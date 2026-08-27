//! `#[capture_exchange_body(evidence = .., creusot_ensures = ..,
//! method_generics = .., kani_ensures = .., kani_requires = ..)]`:
//! registers a real `ExchangeEdgeRecord` for a method that already
//! carries its own real, backend-specific contract by hand -- unlike
//! `#[exchange(..)]` (`exchange.rs`), which generates a concrete-
//! verifier contract/`Witness<V>`/`Exchange<..>` bundle alongside the
//! capture. `GAAP_LEDGER_PLAN.md`'s Step 7 moved `Ledger`'s own methods
//! to a neutral crate (`amenable_gaap`) with a fully generic `Ensures<V>`
//! bound on each -- there is no single concrete verifier left for that
//! bundle to name, and each backend now attaches its own proof
//! separately anyway (Kani: a direct `#[cfg_attr(kani, ..)]` contract;
//! Creusot/Verus: a generated companion reading this very record, the
//! same as before Step 7).
//!
//! **`kani_ensures = "true"` (optional, default `"false"`).**
//! `GAAP_LEDGER_PLAN.md`'s Step 7's own follow-up ("manual bounds are an
//! anti-pattern"): once every real edge's own Kani contract was fixed to
//! call through its target evidence type's registered `Ensures<V>` impl
//! rather than restating the claim inline, all four (`validate`/
//! `commit`/`reject`/`rollback`) converged on the *exact* same
//! mechanical shape -- `|result: &Result<Output, Error>| <Evidence as
//! Ensures<V>>::ensures(result.clone())` -- differing only in which
//! `Output`/`Error`/`Evidence` names get substituted, all three of which
//! this macro already has. So generate it, the same way `#[exchange(..)]`
//! already mechanically generates its own concrete-verifier contract
//! (`exchange.rs`'s own doc comment) -- opt-in, not automatic, since a
//! future caller with a genuinely different Kani contract shape (or none
//! at all) shouldn't be forced into this one. When set, the real method
//! must **not** carry its own hand-written `#[cfg_attr(kani, kani::
//! ensures(..))]` -- this macro injects one onto a clone of the method,
//! matching `#[exchange(..)]`'s own `contracted_method.attrs.push(..)`
//! technique.
//!
//! **`kani_requires = ".."` (optional, only meaningful alongside
//! `kani_ensures = "true"`).** A real precondition expression isn't
//! mechanical in general -- not every edge needs one, and there's no way
//! to derive *which* condition from the method's own signature -- so
//! this stays a real, hand-authored string, spliced into `#[cfg_attr(
//! kani, kani::requires(..))]` verbatim rather than reconstructed.
//!
//! **`kani_requires_evidence = "Type"` (optional, mutually exclusive
//! with `kani_requires`, only meaningful alongside `kani_ensures =
//! "true"`).** For the specific, real case where the precondition is
//! itself a claim some earlier edge already registered as *its own*
//! postcondition -- `commit`'s real precedent: a `Validated`-carrying
//! `Transfer` is exactly the value that flows from `validate`'s output
//! position into `commit`'s input position, so `commit`'s precondition
//! and `validate`'s postcondition can rest on the identical registered
//! `AmountPositive` claim instead of two independently hand-typed copies
//! with nothing enforcing they agree. Generates `<Type as
//! ::amenable_core::Requires<V>>::requires(input.clone())` -- the same
//! "delegate to a registered impl, don't restate" shape `kani_ensures =
//! "true"` already uses for the postcondition half, just for the
//! precondition half, and against a caller-named type rather than always
//! `evidence` (the precondition's real claim may live on a different
//! type than the postcondition's). Relies on the same real,
//! unchecked-by-this-macro assumption `kani_ensures = "true"` already
//! does for the bare `V` identifier: the method's own input parameter is
//! named `input`, matching every real caller so far.
//!
//! Captures the method's own real body verbatim, the identical `Span::
//! source_text()` technique `#[exchange(..)]` uses (shared via that
//! module's own `pub(crate)` helpers, not duplicated), and registers the
//! identical `ExchangeEdgeRecord` shape.

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Error, Expr, FnArg, ImplItem, ItemImpl, LitStr, MetaNameValue, Path, ReturnType, Token,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

use crate::exchange::{expect_lit_str, expect_path_lit, extract_result_generics, trim_braces};

/// Parsed `#[capture_exchange_body(evidence = "..", creusot_ensures =
/// "..", method_generics = "..", kani_ensures = "..", kani_requires =
/// "..")]` arguments -- a narrow subset of `ExchangeArgs`: no `cfg`/
/// `verifier`/`proof_artifact`/`harness_fn`/`harness_const`/`evidence_id`,
/// since nothing here generates a `Witness<V>` impl or a `ProofRecord`
/// for any of those to name.
pub struct CaptureExchangeBodyArgs {
    evidence: Path,
    creusot_ensures: Option<LitStr>,
    method_generics: Option<LitStr>,
    kani_ensures: Option<LitStr>,
    kani_requires: Option<Expr>,
    kani_requires_evidence: Option<Path>,
}

impl Parse for CaptureExchangeBodyArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut evidence = None;
        let mut creusot_ensures = None;
        let mut method_generics = None;
        let mut kani_ensures = None;
        let mut kani_requires = None;
        let mut kani_requires_evidence = None;

        let pairs = Punctuated::<MetaNameValue, Token![,]>::parse_terminated(input)?;
        for pair in pairs {
            if pair.path.is_ident("evidence") {
                evidence = Some(expect_path_lit(&pair.value)?);
            } else if pair.path.is_ident("creusot_ensures") {
                creusot_ensures = Some(expect_lit_str(&pair.value)?);
            } else if pair.path.is_ident("method_generics") {
                method_generics = Some(expect_lit_str(&pair.value)?);
            } else if pair.path.is_ident("kani_ensures") {
                kani_ensures = Some(expect_lit_str(&pair.value)?);
            } else if pair.path.is_ident("kani_requires") {
                let lit = expect_lit_str(&pair.value)?;
                kani_requires = Some(lit.parse()?);
            } else if pair.path.is_ident("kani_requires_evidence") {
                kani_requires_evidence = Some(expect_path_lit(&pair.value)?);
            } else {
                return Err(Error::new_spanned(
                    &pair.path,
                    "unsupported capture_exchange_body attribute",
                ));
            }
        }

        if kani_requires.is_some() && kani_requires_evidence.is_some() {
            return Err(Error::new(
                proc_macro2::Span::call_site(),
                "capture_exchange_body accepts at most one of `kani_requires`/\
                 `kani_requires_evidence`, not both",
            ));
        }

        Ok(CaptureExchangeBodyArgs {
            evidence: evidence.ok_or_else(|| {
                Error::new(
                    proc_macro2::Span::call_site(),
                    "capture_exchange_body requires `evidence = ..`",
                )
            })?,
            creusot_ensures,
            method_generics,
            kani_ensures,
            kani_requires,
            kani_requires_evidence,
        })
    }
}

/// Expand `#[capture_exchange_body(..)]` on `impl SelfType { fn
/// method(&self, input: Input) -> Result<Output, Error> { .. } }` --
/// the same shape `#[exchange(..)]` requires (see that macro's own doc
/// comment for why generics on the *impl block* itself aren't
/// supported; generics on the *method* -- `Ledger::validate<V: Verifier>`
/// -- are unaffected, since only the impl block's own generics are
/// checked here).
pub fn expand_capture_exchange_body(
    args: &CaptureExchangeBodyArgs,
    item_impl: &ItemImpl,
) -> syn::Result<TokenStream> {
    if item_impl.trait_.is_some() {
        return Err(Error::new_spanned(
            item_impl,
            "capture_exchange_body must be applied to an inherent impl block, not a trait impl",
        ));
    }
    if !item_impl.generics.params.is_empty() {
        return Err(Error::new_spanned(
            &item_impl.generics,
            "capture_exchange_body does not yet support generic impl blocks",
        ));
    }

    let method = match item_impl.items.as_slice() {
        [ImplItem::Fn(method)] => method,
        _ => {
            return Err(Error::new_spanned(
                item_impl,
                "capture_exchange_body requires exactly one method in the impl block",
            ));
        }
    };

    let method_ident = &method.sig.ident;
    let input_ty = match method.sig.inputs.iter().collect::<Vec<_>>().as_slice() {
        [FnArg::Receiver(_), FnArg::Typed(pat_type)] => (*pat_type.ty).clone(),
        _ => {
            return Err(Error::new_spanned(
                &method.sig.inputs,
                "capture_exchange_body requires a method of the shape `fn method(&self, input: \
                 Input) -> Result<Output, Error>`",
            ));
        }
    };

    let ReturnType::Type(_, return_ty) = &method.sig.output else {
        return Err(Error::new_spanned(
            &method.sig,
            "capture_exchange_body requires an explicit `Result<Output, Error>` return type",
        ));
    };
    let (output_ty, error_ty) = extract_result_generics(return_ty)?;
    let method_where_clause = &method.sig.generics.where_clause;
    match method
        .sig
        .generics
        .params
        .iter()
        .collect::<Vec<_>>()
        .as_slice()
    {
        [syn::GenericParam::Type(type_param)] if type_param.ident == "V" => {}
        _ => {
            return Err(Error::new_spanned(
                &method.sig.generics,
                "capture_exchange_body requires a method generic over exactly one type \
                 parameter, named `V` -- the generated `Exchange<Input, Output, V>` impl below \
                 assumes that name, matching this macro family's existing convention",
            ));
        }
    }

    let self_ty = &item_impl.self_ty;
    let CaptureExchangeBodyArgs {
        evidence,
        creusot_ensures,
        method_generics,
        kani_ensures,
        kani_requires,
        kani_requires_evidence,
    } = args;
    let creusot_ensures_lit = creusot_ensures
        .clone()
        .unwrap_or_else(|| LitStr::new("true", proc_macro2::Span::call_site()));
    let method_generics_lit = method_generics
        .clone()
        .unwrap_or_else(|| LitStr::new("", proc_macro2::Span::call_site()));
    let generate_kani_ensures = kani_ensures
        .as_ref()
        .is_some_and(|flag| flag.value() == "true");

    // Verbatim source of the method's own body -- identical technique to
    // `#[exchange(..)]`'s own capture (`exchange.rs`'s own doc comment),
    // shared via `trim_braces`/`extract_result_generics` rather than
    // duplicated. Captured *before* any attribute injection below --
    // `Span::source_text()` reads the real file, unaffected by tokens
    // this macro adds to a cloned copy afterward.
    let body_source = method
        .block
        .brace_token
        .span
        .join()
        .source_text()
        .map(|text| trim_braces(&text).to_owned())
        .unwrap_or_else(|| method.block.to_token_stream().to_string());

    // Inject the mechanical Kani contract onto a clone of the method,
    // the identical `contracted_method.attrs.push(..)` technique
    // `#[exchange(..)]`'s own `expand_exchange` uses -- see this
    // module's own doc comment for why this specific shape (delegate to
    // the target evidence type's own `Ensures<V>` impl) is safe to
    // generate mechanically now, and why `kani_requires` stays a real,
    // hand-authored string instead. `#[cfg_attr(kani, kani::requires(
    // ..))]` is pushed before `#[cfg_attr(kani, kani::ensures(..))]` to
    // match every hand-written call site's own existing convention
    // (`Ledger::commit`'s real precedent). The bare identifier `V`
    // (below) is a real, unchecked assumption -- this macro doesn't
    // read the method's own generic parameter list, it just names the
    // one every real caller so far already uses, matching `method_
    // generics = "V"`'s own identical hardcoded convention; a caller
    // whose sole generic parameter is spelled differently should write
    // the contract by hand instead of setting `kani_ensures = "true"`.
    let contracted_impl = if generate_kani_ensures {
        let mut contracted_impl = item_impl.clone();
        let Some(ImplItem::Fn(contracted_method)) = contracted_impl.items.first_mut() else {
            return Err(Error::new_spanned(
                item_impl,
                "capture_exchange_body requires exactly one method in the impl block",
            ));
        };
        if let Some(requires_expr) = kani_requires {
            contracted_method.attrs.push(syn::parse_quote! {
                #[cfg_attr(kani, kani::requires(#requires_expr))]
            });
        }
        if let Some(requires_evidence) = kani_requires_evidence {
            contracted_method.attrs.push(syn::parse_quote! {
                #[cfg_attr(
                    kani,
                    kani::requires(
                        <#requires_evidence as ::amenable_core::Requires<V>>::requires(
                            ::std::clone::Clone::clone(&input)
                        )
                    )
                )]
            });
        }
        contracted_method.attrs.push(syn::parse_quote! {
            #[cfg_attr(
                kani,
                kani::ensures(
                    |result: &::std::result::Result<#output_ty, #error_ty>|
                        <#evidence as ::amenable_core::Ensures<V>>::ensures(
                            ::std::clone::Clone::clone(result)
                        )
                )
            )]
        });
        // `kani` here is meaningless to a downstream consumer that
        // never declares that cfg name -- the `allow`+`const` wrapper
        // is the only placement that suppresses `unexpected_cfgs`
        // there without affecting what actually compiles under a real
        // `kani` build. The method stays resolvable at its original
        // `#self_ty::#method_ident` path regardless of the enclosing
        // impl block's own `const` nesting (confirmed with a real
        // external call site in an isolated scratch crate for
        // `#[exchange(..)]`'s identical shape; see `docs/
        // CFG_HYGIENE_PLAN.md`'s Step 1 -- this macro's only structural
        // difference is the method's own extra `V` type parameter,
        // which the impl block itself doesn't carry).
        quote! {
            #[allow(unexpected_cfgs)]
            const _: () = {
                #contracted_impl
            };
        }
    } else {
        item_impl.clone().to_token_stream()
    };

    Ok(quote! {
        #contracted_impl

        // A real `Exchange<Input, Output, V>` impl, generic over `V`
        // rather than tied to one concrete backend -- unlike `#[exchange(
        // ..)]`'s own bundle, which needs a concrete verifier because it
        // also generates a `Witness<V>` impl and `ProofRecord`
        // registration for one. This method already has everything real
        // Kani/Creusot/Verus contracts need attached directly (the
        // `#[cfg_attr(kani, ..)]` contract above; each backend's own
        // generated companion reading `ExchangeEdgeRecord` below), so the
        // only thing missing to make this a real, derive-checkable
        // `Exchange` edge -- the actual trait impl -- is exactly as
        // mechanical here as it is for `#[exchange(..)]`: delegate to the
        // real method, generic over the same `V` the method itself
        // already requires. The method's own `where` clause is copied
        // verbatim, since `exchange`'s body calling through to it needs
        // the identical bounds to type-check.
        impl<V: ::amenable_core::Verifier> ::amenable_core::Exchange<#input_ty, #output_ty, V>
            for #self_ty
        #method_where_clause
        {
            type Error = #error_ty;

            fn exchange(
                &self,
                input: #input_ty,
            ) -> ::std::result::Result<#output_ty, Self::Error> {
                self.#method_ident::<V>(input)
            }
        }

        // Always registered, regardless of any `#[cfg]` -- this crate
        // (`amenable_gaap`) is ordinary Cargo-built and never translated
        // by anything itself, so `inventory::submit!` here carries none
        // of the ICE risk a translator-based backend's own crate would
        // (see `amenable_core::ExchangeEdgeRecord`'s own doc comment). A
        // different backend's codegen tool queries this to generate its
        // own companion from the real body, without a Cargo dependency
        // on this crate's method at all.
        ::inventory::submit! {
            ::amenable_core::ExchangeEdgeRecord::new(
                stringify!(#self_ty),
                stringify!(#input_ty),
                stringify!(#output_ty),
                stringify!(#error_ty),
                stringify!(#evidence),
                stringify!(#method_ident),
                #body_source,
            )
            .with_creusot_ensures(#creusot_ensures_lit)
            .with_method_generics(#method_generics_lit)
        }
    })
}
