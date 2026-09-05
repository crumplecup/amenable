//! `#[establish(credential = "..", verifier = "..", proposition = "..")]`
//! attribute macro: generates the trivial-token-minting half of `impl
//! Establish<C, V> for Y { type Token = Self; fn establish(_credential: C)
//! -> Self::Token { Self(()) } }`.
//!
//! `verifier` is optional. When given, the impl targets exactly that one
//! concrete verifier -- the original, still-used shape (every existing
//! call site names one). When omitted, this generates a single **backend-
//! generic** blanket impl instead: `impl<V: Verifier> Establish<C, V> for Y
//! where Y: Witness<V> { .. }`. This is the real fix for the mirror-token
//! cascade `GAAP_LEDGER_PLAN.md`'s Step 6 ran into: `establish()`'s body
//! has to construct the token's own private field, which forces it to live
//! in the token's own defining crate -- so as long as the token type (and
//! its establish impl) lived only in `amenable_kani`, no other backend
//! could ever mint a *real* one, and each resorted to a separate mirror
//! token just to have something it was allowed to construct. Moving the
//! token type (and this blanket impl) into the neutral crate the
//! proposition already lives in (`amenable_gaap`) removes the need for a
//! concrete verifier at the impl site at all: orphan-rule legality here
//! comes from the proposition (`Self`/`Y`) being local to that crate, not
//! from `V`'s locality, so `V` can stay a bare, unconstrained generic
//! parameter. The `where Y: Witness<V>` bound is the actual gate --
//! `Establish<C, V>`'s own supertrait already requires it, restated
//! explicitly here because a generic impl needs it spelled out to
//! typecheck -- and it's satisfied per-verifier exactly where each
//! backend's own real proof lives (`impl Witness<KaniVerifier> for
//! Validated` in `amenable_kani`, `impl Witness<CreusotVerifier> for
//! Validated` in `amenable_creusot`, ..., each `#[cfg(..)]`-gated to that
//! backend already). No new verifier-specific code is needed here at all
//! when a fourth backend arrives; it only needs its own `Witness<V>` impl.
//!
//! Arguments are string literals, re-parsed as a `Path`, not bare
//! identifiers — matching `#[derive(Standard)]`'s own `#[standard(basis
//! = "..")]` convention, for the same reason: `proposition = Rejected<
//! Pending>` (a generic proposition, needed once two edges converge on
//! the same evidence family — see `amenable_gaap::ledger`'s own module
//! doc comment, `ledger/mod.rs`, for the real `reject`/`rollback` edges)
//! is genuinely ambiguous as a bare attribute expression (`<`/`>` read
//! as comparison operators, not generic-argument delimiters);
//! `"Rejected<Pending>"` parsed fresh as a `Path` has no such ambiguity.
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
//! `Ensures` proof elsewhere (see `stoplight.rs`'s and `ledger/mod.rs`'s
//! own doc comments). A token that needs to carry real data from its
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

/// Parsed `#[establish(credential = "..", verifier = "..", proposition =
/// "..")]` arguments. `verifier` is optional -- see this module's own doc
/// comment for what its absence generates.
pub struct EstablishArgs {
    credential: Path,
    verifier: Option<Path>,
    proposition: Path,
}

impl Parse for EstablishArgs {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(input)))]
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
            verifier,
            proposition: require(proposition, "proposition")?,
        })
    }
}

#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(value)))]
fn require<T>(value: Option<T>, name: &str) -> syn::Result<T> {
    value.ok_or_else(|| {
        Error::new(
            Span::call_site(),
            format!("establish requires `{name} = ..`"),
        )
    })
}

#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(expr)))]
fn expect_path(expr: &Expr) -> syn::Result<Path> {
    let Expr::Lit(expr_lit) = expr else {
        return Err(Error::new_spanned(expr, "expected a string literal"));
    };
    let syn::Lit::Str(lit_str) = &expr_lit.lit else {
        return Err(Error::new_spanned(expr, "expected a string literal"));
    };
    lit_str.parse()
}

/// Expand `#[establish(..)]` on `struct TokenName(());`.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(args, item)))]
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

    let establish_impl = match verifier {
        Some(verifier) => quote! {
            impl ::amenable_core::Establish<#credential, #verifier> for #proposition {
                type Token = #name;

                #[track_caller]
                fn establish(_credential: #credential) -> Self::Token {
                    #name(())
                }
            }
        },
        None => quote! {
            impl<V: ::amenable_core::Verifier> ::amenable_core::Establish<#credential, V> for #proposition
            where
                #proposition: ::amenable_core::Witness<V>,
            {
                type Token = #name;

                #[track_caller]
                fn establish(_credential: #credential) -> Self::Token {
                    #name(())
                }
            }

            // Only the verifier-less (generic, `amenable_gaap`-style) form
            // registers -- the concrete-verifier form (Stoplight's own
            // per-backend tokens) has no codegen consumer needing it yet.
            // Strictly richer than `#[derive(ProofToken)]`'s own
            // registration for the same token (`credential: Some(..)`, not
            // `None`): a codegen consumer reading both keeps this one. See
            // `ProofTokenMintRecord`'s own doc comment for why this exists
            // at all.
            ::inventory::submit! {
                ::amenable_core::ProofTokenMintRecord::new(
                    stringify!(#name),
                    stringify!(#proposition),
                    Some(stringify!(#credential)),
                )
            }
        },
    };

    Ok(quote! {
        #item

        #establish_impl
    })
}
