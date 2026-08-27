//! `verus_ensures_witness!`/`verus_requires_witness!`: generate a real
//! `impl Ensures<VerusVerifier>`/`impl Requires<VerusVerifier>` block
//! plus one `ContractRecord` registration per real clause, all from one
//! macro invocation naming a type, an evidence string, and a real
//! harness -- rather than a human hand-typing the bound text, picking
//! one clause as "primary," and hand-registering the rest through a
//! bespoke supplementary `ContractRecord` per extra clause (the shape
//! `NonZero<T>`'s registration used before this macro existed).
//!
//! `Bound = &'static [&'static str]`, not a single string: a harness's
//! real postcondition is sometimes more than one real clause (an `iff`
//! becomes two directional `ensures` in Verus, since there's no single
//! iff operator), and every clause is a first-class part of the trait's
//! own value now -- nothing is smuggled in through a side channel.
//! `ContractRecord.fragment` itself stays `fn() -> &'static str` (one
//! fragment per record, unchanged, so `dump-registry`'s JSON schema and
//! `elicit_doc`'s scanner don't need to change) -- this macro just
//! generates the N per-clause registrations mechanically instead of by
//! hand.
//!
//! `verus_ensures_predicate!`/`verus_requires_predicate!` cover the
//! other real shape: a claim that isn't any *one* harness's own clause
//! at all, but a real, named `pub open spec fn` shared across several
//! different harnesses/carrier files (e.g. `write_stores_new_value`,
//! called with different real arguments from `Cell`/`RefCell`/
//! `UnsafeCell`/`OrderedPair`'s own methods) -- there's no single
//! harness to anchor a clause-index selector on, so these derive from
//! the predicate's own real declaration instead, in its own parameter
//! names, not any one caller's argument-substituted instance of it.
//! A type can name a bracketed list of several such predicates (e.g.
//! `IncrementHeadroom`'s tight/single/wide margin variants) instead of
//! just one -- each contributes its own real clause the same way a
//! harness's own multiple clauses do.

use amenable_core::{verus_find_fn, verus_literal_clauses, verus_predicate_body};
use quote::{format_ident, quote};
use syn::{
    Expr, LitInt, LitStr, Token, Type, bracketed,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

struct VerusWitnessArgs {
    ty: Type,
    evidence: Expr,
    harness: LitStr,
    /// Which of the harness's own real clauses this specific type
    /// claims, by 0-based index -- `None` means all of them (the
    /// common case: a leaf's own dedicated harness). `Some` is for a
    /// type that reuses a harness shared with others but only claims
    /// *some* of its real clauses (e.g. `ValidUnicodeScalar` naming
    /// just `verify_char_roundtrip`'s unicode-scalar clause, not its
    /// roundtrip clause too) -- an explicit, uniform selector, not a
    /// bespoke supplementary registration.
    indices: Option<Vec<usize>>,
}

impl Parse for VerusWitnessArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ty: Type = input.parse()?;
        input.parse::<Token![,]>()?;
        let evidence: Expr = input.parse()?;
        input.parse::<Token![,]>()?;
        let harness: LitStr = input.parse()?;

        let indices = if input.parse::<Option<Token![,]>>()?.is_some() && !input.is_empty() {
            let content;
            bracketed!(content in input);
            let items: Punctuated<LitInt, Token![,]> =
                content.parse_terminated(<LitInt as Parse>::parse, Token![,])?;
            Some(
                items
                    .iter()
                    .map(LitInt::base10_parse)
                    .collect::<syn::Result<Vec<usize>>>()?,
            )
        } else {
            None
        };
        input.parse::<Option<Token![,]>>()?;

        Ok(Self {
            ty,
            evidence,
            harness,
            indices,
        })
    }
}

#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(input)))]
pub(crate) fn expand_verus_witness(
    input: proc_macro2::TokenStream,
    ensures: bool,
) -> syn::Result<proc_macro2::TokenStream> {
    let args: VerusWitnessArgs = syn::parse2(input)?;
    let name = args.harness.value();
    let kind = if ensures { "ensures" } else { "requires" };

    let (_, _, item_fn) = verus_find_fn(&name).ok_or_else(|| {
        syn::Error::new(
            args.harness.span(),
            format!(
                "no real, public `pub fn {name}` found under `crates/amenable_verus/src` -- \
                 this macro derives its impl from the real carrier source, so the harness must \
                 exist there"
            ),
        )
    })?;

    let all_clauses = verus_literal_clauses(&item_fn, ensures);
    if all_clauses.is_empty() {
        return Err(syn::Error::new(
            args.harness.span(),
            format!("harness `{name}` has no real `{kind}` clause to derive an impl from"),
        ));
    }

    let selected = match &args.indices {
        Some(indices) => indices.clone(),
        None => (0..all_clauses.len()).collect(),
    };
    let clauses = selected
        .iter()
        .map(|&index| {
            all_clauses.get(index).cloned().ok_or_else(|| {
                syn::Error::new(
                    args.harness.span(),
                    format!(
                        "harness `{name}` only has {} real `{kind}` clause(s), no index {index}",
                        all_clauses.len()
                    ),
                )
            })
        })
        .collect::<syn::Result<Vec<_>>>()?;

    codegen(&args.ty, &args.evidence, ensures, &clauses)
}

struct VerusPredicateWitnessArgs {
    ty: Type,
    evidence: Expr,
    /// One real spec fn's own name is the common case. A type can also
    /// name several -- e.g. `IncrementHeadroom`'s tight/single/wide
    /// margin predicates, three real, distinctly-named `open spec fn`s
    /// each already shared across several carrier files -- rather than
    /// picking one as "canonical" and hand-registering the rest as
    /// bespoke supplementary `ContractRecord`s.
    predicates: Vec<LitStr>,
}

impl Parse for VerusPredicateWitnessArgs {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(input)))]
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ty: Type = input.parse()?;
        input.parse::<Token![,]>()?;
        let evidence: Expr = input.parse()?;
        input.parse::<Token![,]>()?;

        let predicates = if input.peek(syn::token::Bracket) {
            let content;
            bracketed!(content in input);
            let items: Punctuated<LitStr, Token![,]> =
                content.parse_terminated(<LitStr as Parse>::parse, Token![,])?;
            items.into_iter().collect()
        } else {
            vec![input.parse::<LitStr>()?]
        };
        input.parse::<Option<Token![,]>>()?;

        Ok(Self {
            ty,
            evidence,
            predicates,
        })
    }
}

#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(input)))]
pub(crate) fn expand_verus_predicate_witness(
    input: proc_macro2::TokenStream,
    ensures: bool,
) -> syn::Result<proc_macro2::TokenStream> {
    let args: VerusPredicateWitnessArgs = syn::parse2(input)?;

    let clauses = args
        .predicates
        .iter()
        .map(|predicate| {
            let name = predicate.value();

            let (_, _, item_fn) = verus_find_fn(&name).ok_or_else(|| {
                syn::Error::new(
                    predicate.span(),
                    format!(
                        "no real, public `pub open spec fn {name}` found under \
                         `crates/amenable_verus/src` -- this macro derives its impl from the \
                         real predicate declaration, so it must exist there"
                    ),
                )
            })?;

            verus_predicate_body(&item_fn).map_err(|reason| {
                syn::Error::new(
                    predicate.span(),
                    format!(
                        "predicate `{name}` {reason}, so its declaration can't be its own claim \
                         verbatim"
                    ),
                )
            })
        })
        .collect::<syn::Result<Vec<_>>>()?;

    codegen(&args.ty, &args.evidence, ensures, &clauses)
}

#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(ty, evidence)))]
fn codegen(
    ty: &Type,
    evidence: &Expr,
    ensures: bool,
    clauses: &[String],
) -> syn::Result<proc_macro2::TokenStream> {
    let kind = if ensures { "ensures" } else { "requires" };
    let trait_ident = format_ident!("{}", if ensures { "Ensures" } else { "Requires" });
    let method_ident = format_ident!("{}", kind);

    let registrations = (0..clauses.len()).map(|index| {
        quote! {
            ::amenable_core::__inventory::submit! {
                ::amenable_core::ContractRecord::new(
                    #evidence,
                    "verus",
                    #kind,
                    || <#ty as ::amenable_core::#trait_ident<crate::VerusVerifier>>::#method_ident(())[#index],
                )
            }
        }
    });

    Ok(quote! {
        impl ::amenable_core::#trait_ident<crate::VerusVerifier> for #ty {
            type Input = ();
            type Bound = &'static [&'static str];

            fn #method_ident(_: ()) -> &'static [&'static str] {
                &[#(#clauses),*]
            }
        }

        #(#registrations)*
    })
}
