//! `#[derive(ProofToken)]`: generates `impl ProofToken for X { type
//! Proposition = Y; }` from a `#[proof_token(proposition = "Y")]`
//! attribute.
//!
//! Every hand-written `ProofToken` impl in this workspace (`stoplight.rs`'s
//! `GreenToken`/`YellowToken`/`RedToken`, `ledger.rs`'s `PendingToken`/
//! `ValidatedToken`/`CommittedToken`, and the much larger `rust_std` corpus)
//! is the identical one-line shape: name the proposition, done. This derive
//! collapses that duplication the same way `#[derive(Standard)]` collapses
//! `Standard`/`Evidence`'s shared provenance value — the impl targets the
//! type being derived on (`Self`), matching ordinary derive semantics,
//! unlike [`crate::establish`], which cannot (its impl targets the
//! *proposition*, a different, usually foreign type).

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{DeriveInput, Error, LitStr, Type};

/// Expand `#[derive(ProofToken)]` for a struct carrying
/// `#[proof_token(proposition = "...")]`.
pub fn expand_proof_token(input: &DeriveInput) -> syn::Result<TokenStream> {
    let proposition = parse_proof_token_args(&input.attrs)?;
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics ::amenable_core::ProofToken for #name #ty_generics #where_clause {
            type Proposition = #proposition;
        }
    })
}

fn parse_proof_token_args(attrs: &[syn::Attribute]) -> syn::Result<Type> {
    let mut proposition = None;

    for attr in attrs
        .iter()
        .filter(|attr| attr.path().is_ident("proof_token"))
    {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("proposition") {
                let value: LitStr = meta.value()?.parse()?;
                proposition = Some(value.parse()?);
                return Ok(());
            }

            Err(meta.error("unsupported proof_token container attribute"))
        })?;
    }

    proposition.ok_or_else(|| {
        Error::new(
            Span::call_site(),
            "proof_token requires `proposition = \"...\"`",
        )
    })
}
