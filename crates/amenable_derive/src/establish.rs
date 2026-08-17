//! `#[establish(credential = .., verifier = .., proposition = ..)]`
//! attribute macro: generates the trivial-token-minting half of `impl
//! Establish<C, V> for Y { type Token = Self; fn establish(_credential: C)
//! -> Self::Token { Self(()) } }`.
//!
//! Not a `#[derive(..)]`, unlike [`crate::proof_token`], even though it
//! sits on the same kind of item (a unit-tuple token struct) — a derive's
//! generated impl always targets the type being derived on, and this one
//! can't: `Establish<C, V>` is implemented **for the proposition**
//! (`Validated`, `Committed`, ..), a different type, usually defined in a
//! different, upstream crate (`amenable_gaap`) than the token/verifier
//! pair that can actually name it (`amenable_kani`). Only the token's own
//! crate has both halves of the information this needs, so the annotation
//! has to live there — matching `#[amenable_derive::exchange]`'s own
//! precedent of an attribute macro (not a derive) wherever the generated
//! impl targets a type other than `Self`.
//!
//! Every hand-written `Establish` impl in this workspace mints its token
//! the identical trivial way: ignore the credential, construct `Self(())`.
//! That's not a coincidence to paper over — it's what "the credential
//! proves the transition is lawful; the token is just proof it happened"
//! actually looks like in code once the real claim lives in a `Witness`/
//! `Ensures` proof elsewhere (see `stoplight.rs`'s and `ledger.rs`'s own
//! doc comments). A token that needs to carry real data from its
//! credential is a different, richer shape this macro deliberately
//! doesn't try to cover — hand-write that `Establish` impl instead, the
//! same "cover the common case, allow divergence" precedent `#[derive(
//! Standard)]` already sets.

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    Error, Expr, Fields, ItemStruct, MetaNameValue, Path, Token, Type,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

/// Parsed `#[establish(credential = .., verifier = .., proposition = ..)]`
/// arguments.
pub struct EstablishArgs {
    credential: Path,
    verifier: Path,
    proposition: Path,
}

impl Parse for EstablishArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut credential = None;
        let mut verifier = None;
        let mut proposition = None;

        let pairs = Punctuated::<MetaNameValue, Token![,]>::parse_terminated(input)?;

        for pair in pairs {
            if pair.path.is_ident("credential") {
                credential = Some(expect_path(&pair.value)?);
            } else if pair.path.is_ident("verifier") {
                verifier = Some(expect_path(&pair.value)?);
            } else if pair.path.is_ident("proposition") {
                proposition = Some(expect_path(&pair.value)?);
            } else {
                return Err(Error::new_spanned(
                    &pair.path,
                    "unsupported establish attribute",
                ));
            }
        }

        Ok(EstablishArgs {
            credential: require(credential, "credential")?,
            verifier: require(verifier, "verifier")?,
            proposition: require(proposition, "proposition")?,
        })
    }
}

fn require<T>(value: Option<T>, name: &str) -> syn::Result<T> {
    value.ok_or_else(|| {
        Error::new(
            Span::call_site(),
            format!("establish requires `{name} = ..`"),
        )
    })
}

fn expect_path(expr: &Expr) -> syn::Result<Path> {
    let Expr::Path(expr_path) = expr else {
        return Err(Error::new_spanned(expr, "expected a type path"));
    };
    Ok(expr_path.path.clone())
}

/// Expand `#[establish(..)]` on `struct TokenName(());`.
pub fn expand_establish(args: &EstablishArgs, item: &ItemStruct) -> syn::Result<TokenStream> {
    let Fields::Unnamed(fields) = &item.fields else {
        return Err(Error::new_spanned(
            &item.fields,
            "establish requires a single-field tuple struct, e.g. `struct TokenName(());`",
        ));
    };
    let Some(field) = fields
        .unnamed
        .iter()
        .next()
        .filter(|_| fields.unnamed.len() == 1)
    else {
        return Err(Error::new_spanned(
            fields,
            "establish requires exactly one field, e.g. `struct TokenName(());`",
        ));
    };
    let Type::Tuple(tuple) = &field.ty else {
        return Err(Error::new_spanned(
            &field.ty,
            "establish requires the field to be `()`, e.g. `struct TokenName(());` -- a \
             token that carries real data from its credential needs a hand-written Establish \
             impl instead",
        ));
    };
    if !tuple.elems.is_empty() {
        return Err(Error::new_spanned(
            tuple,
            "establish requires the field to be `()`, e.g. `struct TokenName(());`",
        ));
    }

    let name = &item.ident;
    let EstablishArgs {
        credential,
        verifier,
        proposition,
    } = args;

    Ok(quote! {
        #item

        impl ::amenable_core::Establish<#credential, #verifier> for #proposition {
            type Token = #name;

            fn establish(_credential: #credential) -> Self::Token {
                #name(())
            }
        }
    })
}
