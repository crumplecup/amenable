//! `#[capture_exchange_body(evidence = .., creusot_ensures = ..)]`:
//! registers a real `ExchangeEdgeRecord` for a method that already
//! carries its own real, backend-specific contract by hand -- unlike
//! `#[exchange(..)]` (`exchange.rs`), which generates a concrete-
//! verifier contract/`Witness<V>`/`Exchange<..>` bundle alongside the
//! capture. `GAAP_LEDGER_PLAN.md`'s Step 7 moved `Ledger`'s own methods
//! to a neutral crate (`amenable_gaap`) with a fully generic `Ensures<V>`
//! bound on each -- there is no single concrete verifier left for that
//! bundle to name, and each backend now attaches its own proof
//! separately anyway (Kani: a direct `#[cfg_attr(kani, ..)]` contract,
//! hand-written on the real method itself; Creusot/Verus: a generated
//! companion reading this very record, the same as before Step 7).
//!
//! Captures the method's own real body verbatim, the identical `Span::
//! source_text()` technique `#[exchange(..)]` uses (shared via that
//! module's own `pub(crate)` helpers, not duplicated), and registers the
//! identical `ExchangeEdgeRecord` shape -- but leaves the method itself
//! completely untouched (no injected attribute), and generates nothing
//! else at all.

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Error, FnArg, ImplItem, ItemImpl, LitStr, MetaNameValue, Path, ReturnType, Token,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

use crate::exchange::{expect_lit_str, expect_path_lit, extract_result_generics, trim_braces};

/// Parsed `#[capture_exchange_body(evidence = "..", creusot_ensures =
/// "..")]` arguments -- a narrow subset of `ExchangeArgs`: no `cfg`/
/// `verifier`/`proof_artifact`/`harness_fn`/`harness_const`/`evidence_id`,
/// since nothing here generates a contract, a `Witness<V>` impl, or a
/// `ProofRecord` for any of those to name.
pub struct CaptureExchangeBodyArgs {
    evidence: Path,
    creusot_ensures: Option<LitStr>,
    method_generics: Option<LitStr>,
}

impl Parse for CaptureExchangeBodyArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut evidence = None;
        let mut creusot_ensures = None;
        let mut method_generics = None;

        let pairs = Punctuated::<MetaNameValue, Token![,]>::parse_terminated(input)?;
        for pair in pairs {
            if pair.path.is_ident("evidence") {
                evidence = Some(expect_path_lit(&pair.value)?);
            } else if pair.path.is_ident("creusot_ensures") {
                creusot_ensures = Some(expect_lit_str(&pair.value)?);
            } else if pair.path.is_ident("method_generics") {
                method_generics = Some(expect_lit_str(&pair.value)?);
            } else {
                return Err(Error::new_spanned(
                    &pair.path,
                    "unsupported capture_exchange_body attribute",
                ));
            }
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

    let self_ty = &item_impl.self_ty;
    let CaptureExchangeBodyArgs {
        evidence,
        creusot_ensures,
        method_generics,
    } = args;
    let creusot_ensures_lit = creusot_ensures
        .clone()
        .unwrap_or_else(|| LitStr::new("true", proc_macro2::Span::call_site()));
    let method_generics_lit = method_generics
        .clone()
        .unwrap_or_else(|| LitStr::new("", proc_macro2::Span::call_site()));

    // Verbatim source of the method's own body -- identical technique to
    // `#[exchange(..)]`'s own capture (`exchange.rs`'s own doc comment),
    // shared via `trim_braces`/`extract_result_generics` rather than
    // duplicated.
    let body_source = method
        .block
        .brace_token
        .span
        .join()
        .source_text()
        .map(|text| trim_braces(&text).to_owned())
        .unwrap_or_else(|| method.block.to_token_stream().to_string());

    Ok(quote! {
        #item_impl

        // Always registered, regardless of any `#[cfg]` -- this crate
        // (`amenable_gaap`) is ordinary Cargo-built and never translated
        // by anything itself, so `inventory::submit!` here carries none
        // of the ICE risk a translator-based backend's own crate would
        // (see `amenable_core::ExchangeEdgeRecord`'s own doc comment). A
        // different backend's codegen tool queries this to generate its
        // own companion from the real body, without a Cargo dependency
        // on this crate's method at all.
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
                method_generics: #method_generics_lit,
            }
        }
    })
}
