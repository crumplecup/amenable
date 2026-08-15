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
//! surrounding `impl SelfType { .. }` block instead, and deliberately does
//! NOT touch two things that already carry their own real guarantees the
//! macro must not weaken:
//!
//! - The `#[cfg_attr(kani, kani::ensures(..))]` contract itself, and the
//!   method body: both are real proof content, authored by hand, and stay
//!   that way.
//! - The `amenable_derive::harness! { .. }` invocation: it captures the
//!   harness function's verbatim source via `Span::source_text()`, which
//!   only works when the braced item is written directly at the call site
//!   (see `harness.rs`'s own doc comment). Splicing a user-authored setup
//!   block through this macro's `quote!` output would silently degrade
//!   that capture to a token-reconstructed (whitespace-losing) fallback.
//!   So `harness!` stays a separate, hand-written invocation alongside
//!   this attribute, exactly as it was before Step 3.
//!
//! What this macro *does* generate, because none of it carries either
//! guarantee above and all of it was, before Step 3, identical boilerplate
//! repeated once per `Stoplight` edge: the `Witness<V>` impl for the
//! transition's target evidence (referencing the harness by name), the
//! `ProofRecord` registration backing it, and the `Exchange<Input, Output,
//! V>` impl that delegates to the real inherent method.

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{
    Error, Expr, FnArg, GenericArgument, ImplItem, ItemImpl, LitStr, MetaNameValue, Path,
    PathArguments, ReturnType, Token, Type,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

/// Parsed `#[exchange(cfg = .., verifier = .., evidence = .., proof_artifact
/// = .., harness_fn = .., harness_const = .., evidence_id = ..)]` arguments.
pub struct ExchangeArgs {
    cfg: Ident,
    verifier: Path,
    evidence: Path,
    proof_artifact: Path,
    harness_fn: Ident,
    harness_const: Ident,
    evidence_id: Option<LitStr>,
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

        let pairs = Punctuated::<MetaNameValue, Token![,]>::parse_terminated(input)?;

        for pair in pairs {
            if pair.path.is_ident("cfg") {
                cfg = Some(expect_ident(&pair.value)?);
            } else if pair.path.is_ident("verifier") {
                verifier = Some(expect_path(&pair.value)?);
            } else if pair.path.is_ident("evidence") {
                evidence = Some(expect_path(&pair.value)?);
            } else if pair.path.is_ident("proof_artifact") {
                proof_artifact = Some(expect_path(&pair.value)?);
            } else if pair.path.is_ident("harness_fn") {
                harness_fn = Some(expect_ident(&pair.value)?);
            } else if pair.path.is_ident("harness_const") {
                harness_const = Some(expect_ident(&pair.value)?);
            } else if pair.path.is_ident("evidence_id") {
                evidence_id = Some(expect_lit_str(&pair.value)?);
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

fn expect_lit_str(expr: &Expr) -> syn::Result<LitStr> {
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
    } = args;

    let evidence_id_expr = match evidence_id {
        Some(suffix) => {
            quote! { concat!(module_path!(), "::", stringify!(#evidence), "::", #suffix) }
        }
        None => quote! { concat!(module_path!(), "::", stringify!(#evidence)) },
    };

    Ok(quote! {
        #item_impl

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
            ::amenable_core::ProofRecord {
                evidence: #evidence_id_expr,
                verifier: stringify!(#cfg),
                describe: || <#evidence as ::amenable_core::Witness<#verifier>>::proof().to_string(),
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

/// Extract `(T, E)` from a `Result<T, E>` return type.
fn extract_result_generics(ty: &Type) -> syn::Result<(Type, Type)> {
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
