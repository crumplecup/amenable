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

use amenable_core::{verus_find_fn, verus_literal_clauses};
use quote::{format_ident, quote};
use syn::{Expr, LitStr, Token, Type, parse::Parse, parse::ParseStream};

struct VerusWitnessArgs {
    ty: Type,
    evidence: Expr,
    harness: LitStr,
}

impl Parse for VerusWitnessArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ty: Type = input.parse()?;
        input.parse::<Token![,]>()?;
        let evidence: Expr = input.parse()?;
        input.parse::<Token![,]>()?;
        let harness: LitStr = input.parse()?;
        input.parse::<Option<Token![,]>>()?;
        Ok(Self {
            ty,
            evidence,
            harness,
        })
    }
}

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

    let clauses = verus_literal_clauses(&item_fn, ensures);
    if clauses.is_empty() {
        return Err(syn::Error::new(
            args.harness.span(),
            format!("harness `{name}` has no real `{kind}` clause to derive an impl from"),
        ));
    }

    let ty = &args.ty;
    let evidence = &args.evidence;
    let trait_ident = format_ident!("{}", if ensures { "Ensures" } else { "Requires" });
    let method_ident = format_ident!("{}", kind);

    let registrations = (0..clauses.len()).map(|index| {
        quote! {
            ::amenable_core::__inventory::submit! {
                ::amenable_core::ContractRecord {
                    evidence: #evidence,
                    verifier: "verus",
                    kind: #kind,
                    fragment: || <#ty as ::amenable_core::#trait_ident<crate::VerusVerifier>>::#method_ident(())[#index],
                }
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
