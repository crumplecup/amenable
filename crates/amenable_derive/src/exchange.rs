//! `#[exchange(...)]` attribute macro: generalizes the by-hand pattern
//! proven in `amenable_kani::stoplight` (see `EXCHANGE_PROOF_DERIVATION_
//! PLAN.md`'s Step 1) for wiring a real `Exchange` transition to its
//! backing `Witness` proof.
//!
//! Deliberately narrow scope, chosen empirically rather than assumed from
//! the plan's original sketch: the plan's Step 3 text imagined the
//! attribute sitting directly on `fn exchange(&self, input) -> Result<...>`
//! and generating the contract too, but Step 1 already found that Kani
//! 0.67.0 can't place a contract on a trait method when the trait itself is
//! generic (`Exchange<Input, Output, V>` is) — the contract and its real
//! body have to live on a plain inherent method, with the trait impl
//! reduced to delegation. So this macro attaches to that inherent method's
//! surrounding `impl SelfType { .. }` block instead.
//!
//! Two things still carry their own real guarantees this macro must not
//! weaken, and it still leaves both alone:
//!
//! - The method **body**: the real transition logic, authored by hand, and
//!   re-emitted exactly as written.
//! - The `amenable_derive::harness! { .. }` invocation: it captures the
//!   harness function's verbatim source via `Span::source_text()`, which
//!   only works when the braced item is written directly at the call site
//!   (see `harness.rs`'s own doc comment). Splicing a user-authored setup
//!   block through this macro's `quote!` output would silently degrade
//!   that capture to a token-reconstructed (whitespace-losing) fallback.
//!   So `harness!` stays a separate, hand-written invocation alongside
//!   this attribute, exactly as it was before Step 3.
//!
//! What changed after Step 3 (see `EXCHANGE_PROOF_DERIVATION_PLAN.md`'s
//! Step 6): the `#[cfg_attr(.., kani::ensures(..))]` contract itself is now
//! **generated**, not hand-written — but this does not weaken the "real
//! proof content stays authored by hand" guarantee, because Step 6 already
//! moved the real claim somewhere else. Once `evidence`'s postcondition
//! lives in a real, separately-registered `Ensures<V>` impl (`kani_
//! ensures!`, still hand-written, still where the actual predicate is
//! authored), the DFCC attribute wiring it into the proof site is 100%
//! mechanical — `|result: &Result<Output, Error>| <Evidence as Ensures<V>>
//! ::ensures(result.clone())` needs no information this macro doesn't
//! already have from `evidence`/the method's own return type (`clone()`,
//! not a `*result` deref-copy: `GAAP_LEDGER_PLAN.md`'s Step 1 found a
//! real second-user case where `Output` carries a `String`-backed field,
//! which can never be `Copy` — `Clone` is a strict superset, so this
//! costs nothing for `Stoplight`'s own already-`Copy` types). Generating
//! it here removes three-edges'-worth of copy-pasted boilerplate (and the
//! drift risk copy-pasting invites) without synthesizing any new claim — the
//! human-authored predicate this attribute calls through to is exactly as
//! hand-written as it was before, just no longer re-typed at each call
//! site. A future edge with no meaningful postcondition beyond type safety
//! is not this macro's problem to anticipate; every real edge today wants
//! this, so the bound is unconditional rather than an opt-in flag guessed
//! in advance of a caller that needs the alternative.
//!
//! What this macro generates, all of it either newly-mechanical (the
//! `ensures` attribute, see above) or, before Step 3, identical boilerplate
//! repeated once per `Stoplight` edge: the `Witness<V>` impl for the
//! transition's target evidence (referencing the harness by name), the
//! `ProofRecord` registration backing it, the `Exchange<Input, Output, V>`
//! impl that delegates to the real inherent method, and the inherent
//! method's own DFCC `ensures` attribute.

use proc_macro2::{Ident, TokenStream};
use quote::{ToTokens, quote};
use syn::{
    Error, Expr, FnArg, GenericArgument, ImplItem, ItemImpl, LitStr, MetaNameValue, Path,
    PathArguments, ReturnType, Token, Type,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

/// Parsed `#[exchange(cfg = .., verifier = "..", evidence = "..",
/// proof_artifact = .., harness_fn = .., harness_const = .., evidence_id =
/// ..)]` arguments. `verifier`/`evidence` are string literals, re-parsed as
/// a `Path` -- matching `#[derive(Standard)]`'s `#[standard(basis = "..")]`
/// convention and `#[establish(..)]`'s own fields: a generic evidence type
/// (`Rejected<Pending>`, needed once two edges converge on the same
/// evidence family -- see `ledger.rs`'s `reject`/`rollback` doc comment)
/// is genuinely ambiguous as a bare attribute expression (`<`/`>` read as
/// comparison operators, not generic-argument delimiters). `cfg`/
/// `proof_artifact`/`harness_fn`/`harness_const`/`evidence_id` stay plain
/// identifiers (or, for `evidence_id`, a string naming a registry suffix,
/// unrelated to this): none of them has ever needed to name a generic
/// type.
pub struct ExchangeArgs {
    cfg: Ident,
    verifier: Path,
    evidence: Path,
    proof_artifact: Path,
    harness_fn: Ident,
    harness_const: Ident,
    evidence_id: Option<LitStr>,
    creusot_ensures: Option<LitStr>,
}

impl Parse for ExchangeArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut cfg = None;
        let mut verifier = None;
        let mut evidence = None;
        let mut proof_artifact = None;
        let mut harness_fn = None;
        let mut harness_const = None;
        let mut evidence_id = None;
        let mut creusot_ensures = None;

        let pairs = Punctuated::<MetaNameValue, Token![,]>::parse_terminated(input)?;

        for pair in pairs {
            if pair.path.is_ident("cfg") {
                cfg = Some(expect_ident(&pair.value)?);
            } else if pair.path.is_ident("verifier") {
                verifier = Some(expect_path_lit(&pair.value)?);
            } else if pair.path.is_ident("evidence") {
                evidence = Some(expect_path_lit(&pair.value)?);
            } else if pair.path.is_ident("proof_artifact") {
                proof_artifact = Some(expect_path(&pair.value)?);
            } else if pair.path.is_ident("harness_fn") {
                harness_fn = Some(expect_ident(&pair.value)?);
            } else if pair.path.is_ident("harness_const") {
                harness_const = Some(expect_ident(&pair.value)?);
            } else if pair.path.is_ident("evidence_id") {
                evidence_id = Some(expect_lit_str(&pair.value)?);
            } else if pair.path.is_ident("creusot_ensures") {
                creusot_ensures = Some(expect_lit_str(&pair.value)?);
            } else {
                return Err(Error::new_spanned(
                    &pair.path,
                    "unsupported exchange attribute",
                ));
            }
        }

        Ok(ExchangeArgs {
            cfg: require(cfg, "cfg")?,
            verifier: require(verifier, "verifier")?,
            evidence: require(evidence, "evidence")?,
            proof_artifact: require(proof_artifact, "proof_artifact")?,
            harness_fn: require(harness_fn, "harness_fn")?,
            harness_const: require(harness_const, "harness_const")?,
            evidence_id,
            creusot_ensures,
        })
    }
}

fn require<T>(value: Option<T>, name: &str) -> syn::Result<T> {
    value.ok_or_else(|| {
        Error::new(
            proc_macro2::Span::call_site(),
            format!("exchange requires `{name} = ..`"),
        )
    })
}

fn expect_ident(expr: &Expr) -> syn::Result<Ident> {
    let Expr::Path(expr_path) = expr else {
        return Err(Error::new_spanned(expr, "expected an identifier"));
    };
    expr_path
        .path
        .get_ident()
        .cloned()
        .ok_or_else(|| Error::new_spanned(expr, "expected a single identifier, not a path"))
}

fn expect_path(expr: &Expr) -> syn::Result<Path> {
    let Expr::Path(expr_path) = expr else {
        return Err(Error::new_spanned(expr, "expected a type path"));
    };
    Ok(expr_path.path.clone())
}

/// `pub(crate)`: `capture_exchange_body.rs` reuses this verbatim.
pub(crate) fn expect_path_lit(expr: &Expr) -> syn::Result<Path> {
    let Expr::Lit(expr_lit) = expr else {
        return Err(Error::new_spanned(expr, "expected a string literal"));
    };
    let syn::Lit::Str(lit_str) = &expr_lit.lit else {
        return Err(Error::new_spanned(expr, "expected a string literal"));
    };
    lit_str.parse()
}

/// `pub(crate)`: `capture_exchange_body.rs` reuses this verbatim.
pub(crate) fn expect_lit_str(expr: &Expr) -> syn::Result<LitStr> {
    let Expr::Lit(expr_lit) = expr else {
        return Err(Error::new_spanned(expr, "expected a string literal"));
    };
    let syn::Lit::Str(lit_str) = &expr_lit.lit else {
        return Err(Error::new_spanned(expr, "expected a string literal"));
    };
    Ok(lit_str.clone())
}

/// Expand `#[exchange(..)]` on `impl SelfType { fn method(&self, input:
/// Input) -> Result<Output, Error> { .. } }`.
pub fn expand_exchange(args: &ExchangeArgs, item_impl: &ItemImpl) -> syn::Result<TokenStream> {
    if item_impl.trait_.is_some() {
        return Err(Error::new_spanned(
            item_impl,
            "exchange must be applied to an inherent impl block, not a trait impl",
        ));
    }
    if !item_impl.generics.params.is_empty() {
        return Err(Error::new_spanned(
            &item_impl.generics,
            "exchange does not yet support generic impl blocks",
        ));
    }

    let method = match item_impl.items.as_slice() {
        [ImplItem::Fn(method)] => method,
        _ => {
            return Err(Error::new_spanned(
                item_impl,
                "exchange requires exactly one method in the impl block",
            ));
        }
    };

    let method_ident = &method.sig.ident;
    let input_ty = match method.sig.inputs.iter().collect::<Vec<_>>().as_slice() {
        [FnArg::Receiver(_), FnArg::Typed(pat_type)] => (*pat_type.ty).clone(),
        _ => {
            return Err(Error::new_spanned(
                &method.sig.inputs,
                "exchange requires a method of the shape `fn method(&self, input: Input) -> \
                 Result<Output, Error>`",
            ));
        }
    };

    let ReturnType::Type(_, return_ty) = &method.sig.output else {
        return Err(Error::new_spanned(
            &method.sig,
            "exchange requires an explicit `Result<Output, Error>` return type",
        ));
    };
    let (output_ty, error_ty) = extract_result_generics(return_ty)?;

    let self_ty = &item_impl.self_ty;
    let ExchangeArgs {
        cfg,
        verifier,
        evidence,
        proof_artifact,
        harness_fn,
        harness_const,
        evidence_id,
        creusot_ensures,
    } = args;

    let creusot_ensures_lit = match creusot_ensures {
        Some(lit) => lit.clone(),
        None => LitStr::new("true", proc_macro2::Span::call_site()),
    };

    let evidence_id_expr = match evidence_id {
        Some(suffix) => {
            quote! { concat!(module_path!(), "::", stringify!(#evidence), "::", #suffix) }
        }
        None => quote! { concat!(module_path!(), "::", stringify!(#evidence)) },
    };

    // Verbatim source of the method's own body -- the same `Span::
    // source_text()` technique `harness!` uses for its own braced group,
    // applied here to the parsed method's `block` instead. Feeds
    // `ExchangeEdgeRecord::body` below: the one piece of real, hand-
    // authored content a different backend's codegen tool needs to
    // generate its own companion from, without duplicating it by hand.
    let body_source = method
        .block
        .brace_token
        .span
        .join()
        .source_text()
        .map(|text| trim_braces(&text).to_owned())
        .unwrap_or_else(|| method.block.to_token_stream().to_string());

    // Inject the DFCC contract onto a clone of the method rather than
    // requiring it hand-written at the call site (see this file's own doc
    // comment for why this doesn't weaken the "real proof content stays
    // hand-authored" guarantee: the actual predicate still lives in
    // `evidence`'s own, separately-registered `Ensures<V>` impl, not here
    // — this is purely the mechanical call-through). Fully-qualified
    // (`<#evidence as ::amenable_core::Ensures<#verifier>>::ensures`)
    // rather than a bare `Evidence::ensures` call so the generated code
    // needs no `use amenable_core::Ensures;` in scope at the call site.
    let mut contracted_impl = item_impl.clone();
    let Some(ImplItem::Fn(contracted_method)) = contracted_impl.items.first_mut() else {
        return Err(Error::new_spanned(
            item_impl,
            "exchange requires exactly one method in the impl block",
        ));
    };
    contracted_method.attrs.push(syn::parse_quote! {
        #[cfg_attr(
            #cfg,
            #cfg::ensures(
                |result: &::std::result::Result<#output_ty, #error_ty>|
                    <#evidence as ::amenable_core::Ensures<#verifier>>::ensures(
                        ::std::clone::Clone::clone(result)
                    )
            )
        )]
    });

    Ok(quote! {
        #contracted_impl

        impl ::amenable_core::Witness<#verifier> for #evidence {
            type SupportingEvidence = Self;
            type ProofArtifact = #proof_artifact;

            fn proof() -> Self::ProofArtifact {
                #proof_artifact {
                    harness: ::std::borrow::ToOwned::to_owned(stringify!(#harness_fn)),
                    claim: ::std::borrow::ToOwned::to_owned(#harness_const),
                }
            }
        }

        ::inventory::submit! {
            ::amenable_core::ProofRecord::new(
                #evidence_id_expr,
                stringify!(#cfg),
                || <#evidence as ::amenable_core::Witness<#verifier>>::proof().to_string(),
            )
        }

        // Always registered, regardless of `#cfg` -- this crate is
        // ordinary Cargo-built and never translated by anything, so
        // `inventory::submit!` here carries none of the ICE risk a
        // translator-based backend's own crate would (see `amenable_core::
        // ExchangeEdgeRecord`'s own doc comment). A different backend's
        // codegen tool queries this to generate its own companion from the
        // real body, without a Cargo dependency on this crate at all.
        ::inventory::submit! {
            ::amenable_core::ExchangeEdgeRecord {
                self_ty: stringify!(#self_ty),
                input_ty: stringify!(#input_ty),
                output_ty: stringify!(#output_ty),
                error_ty: stringify!(#error_ty),
                evidence: stringify!(#evidence),
                method_name: stringify!(#method_ident),
                body: #body_source,
                creusot_ensures: #creusot_ensures_lit,
                // Every `#[exchange(..)]`-decorated edge today is a
                // non-generic method (`Ledger::validate`/`::commit`
                // moved to `#[capture_exchange_body(..)]` once they
                // gained a real `<V: Verifier>` parameter, precisely
                // because this macro requires a concrete `verifier`
                // -- see `ExchangeEdgeRecord::method_generics`'s own
                // doc comment).
                method_generics: "",
            }
        }

        impl ::amenable_core::Exchange<#input_ty, #output_ty, #verifier> for #self_ty {
            type Error = #error_ty;

            fn exchange(
                &self,
                input: #input_ty,
            ) -> ::std::result::Result<#output_ty, Self::Error> {
                self.#method_ident(input)
            }
        }
    })
}

/// Strip the outer `{`/`}` a brace-spanning `source_text()` includes,
/// along with the whitespace immediately inside them -- the same helper
/// `harness.rs` defines for its own, structurally identical need.
/// `pub(crate)`: `capture_exchange_body.rs` reuses this verbatim rather
/// than duplicating it -- the identical span-capture technique, applied
/// to a method with no injected contract/`Witness`/`Exchange` bundle.
pub(crate) fn trim_braces(text: &str) -> &str {
    text.trim()
        .strip_prefix('{')
        .and_then(|text| text.strip_suffix('}'))
        .map(str::trim)
        .unwrap_or(text)
}

/// Extract `(T, E)` from a `Result<T, E>` return type. `pub(crate)`: see
/// [`trim_braces`]'s own doc comment.
pub(crate) fn extract_result_generics(ty: &Type) -> syn::Result<(Type, Type)> {
    let Type::Path(type_path) = ty else {
        return Err(Error::new_spanned(ty, "expected `Result<Output, Error>`"));
    };
    let Some(segment) = type_path.path.segments.last() else {
        return Err(Error::new_spanned(ty, "expected `Result<Output, Error>`"));
    };
    if segment.ident != "Result" {
        return Err(Error::new_spanned(ty, "expected `Result<Output, Error>`"));
    }
    let PathArguments::AngleBracketed(generics) = &segment.arguments else {
        return Err(Error::new_spanned(ty, "expected `Result<Output, Error>`"));
    };
    let args: Vec<_> = generics.args.iter().collect();
    let [
        GenericArgument::Type(output_ty),
        GenericArgument::Type(error_ty),
    ] = args.as_slice()
    else {
        return Err(Error::new_spanned(
            &generics.args,
            "expected `Result<Output, Error>` with exactly two type arguments",
        ));
    };
    Ok((output_ty.clone(), error_ty.clone()))
}
