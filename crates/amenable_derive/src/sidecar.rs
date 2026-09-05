//! `#[derive(Sidecar)]`: generates `impl Sidecar<V> for X { .. }` (plus a
//! constructor) from a `#[sidecar(verifier = "..")]` attribute and two
//! field markers (`#[sidecar(primary)]`/`#[sidecar(token)]`).
//!
//! `verifier` is optional, matching `#[amenable_derive::establish]`'s own
//! verifier-less form (see that module's doc comment for the full
//! rationale). When omitted, this generates one backend-generic `impl<V:
//! Verifier> Sidecar<V> for X where Proposition: Evidence + Witness<V>` --
//! `primary()`/`sidecar()`'s own bodies are pure structural glue (`&self.
//! primary`/`self.token.clone()`), with zero backend-specific content
//! either way, so unlike `#[establish]`'s trivial mint body this was
//! *always* possible; it just never had a consumer needing it before a
//! carrier type itself needed to live in a neutral crate (`GAAP_LEDGER_
//! PLAN.md`'s Step 7). One real consequence: with `V` generic, there is no
//! concrete verifier text left to gate `primary_ensures_attr`/`creusot_
//! ensures_attr` on (`quote!(V).to_string()` is just `"V"`, never `
//! "CreusotVerifier"`), so neither is ever generated in this mode --
//! Creusot's own real `#[ensures(..)]` content for `primary()`/`new()`
//! has to move to an `extern_spec!` written in `amenable_creusot` instead,
//! the same relocation `#[establish]`'s own verifier-less form already
//! forced for `establish()`'s real Creusot contract.
//!
//! Every carrier type built so far for an `Exchange`-shaped proof
//! (`amenable_kani::stoplight::Established<T, Token>`,
//! `amenable_kani::ledger::Transfer<S, Token>`, and their per-backend
//! accommodation-model mirrors) is the identical shape: a primary payload
//! field, a token field, an optional phantom proposition marker, and a
//! `Sidecar<V>` impl whose body is always `&self.<primary>`/
//! `self.<token>.clone()`. Collapses that duplication the same way
//! `#[derive(ProofToken)]` collapses the one-line `ProofToken` shape.
//!
//! `proposition` defaults to the primary field's own type (`Established<T,
//! Token>`'s shape, where a state IS its whole payload); set
//! `#[sidecar(proposition = "S")]` when the proposition is a separate,
//! phantom generic parameter instead (`Transfer<S, Token>`'s shape, where
//! `S` never appears as data). Either way, a `Evidence`/`Witness<V>` bound
//! is only added to the where-clause for generic parameters actually
//! declared on the struct -- a *concrete* primary type (`TransferPayload`)
//! needs no such bound restated here; its own `Evidence` impl already
//! exists unconditionally.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    DeriveInput, Error, Field, Fields, GenericParam, Ident, LitStr, Type, Visibility,
    WherePredicate, parse_quote,
};

struct SidecarArgs {
    verifier: Option<Type>,
    proposition: Option<Type>,
    constructor: Visibility,
}

#[cfg_attr(not(kani), tracing::instrument(level = "info", skip(input)))]
pub fn expand_sidecar(input: &DeriveInput) -> syn::Result<TokenStream> {
    let args = parse_sidecar_args(&input.attrs)?;
    let name = &input.ident;

    let Fields::Named(fields) = struct_fields(input)? else {
        return Err(Error::new_spanned(
            name,
            "Sidecar can only be derived for a struct with named fields",
        ));
    };

    let primary_field = find_marked_field(fields, "primary")?;
    let token_field = find_marked_field(fields, "token")?;
    let phantom_fields: Vec<&Field> = fields
        .named
        .iter()
        .filter(|field| !std::ptr::eq(*field, primary_field) && !std::ptr::eq(*field, token_field))
        .collect();
    for field in &phantom_fields {
        require_phantom_data(field)?;
    }

    let primary_ident = primary_field
        .ident
        .as_ref()
        .ok_or_else(|| Error::new_spanned(primary_field, "named field"))?;
    let primary_ty = &primary_field.ty;
    let token_ident = token_field
        .ident
        .as_ref()
        .ok_or_else(|| Error::new_spanned(token_field, "named field"))?;
    let token_ty = &token_field.ty;
    let proposition_ty = args
        .proposition
        .clone()
        .unwrap_or_else(|| primary_ty.clone());

    // Verifier-less mode adds a fresh generic parameter `V: Verifier`
    // rather than naming any concrete verifier -- see this module's own
    // doc comment for why this is possible here (unlike `#[establish]`'s
    // trivial-mint case, `Sidecar`'s own body was always backend-neutral;
    // it just never had a caller needing genericity before).
    let fresh_verifier: Type = parse_quote!(V);
    let verifier: &Type = args.verifier.as_ref().unwrap_or(&fresh_verifier);

    // `ty_generics` (the `<S, Token>` in `Transfer<S, Token>`'s own type
    // application) has to come from the struct's *original*, unmodified
    // generics -- `V`, added below only for the `impl<..>` header and
    // where-clause, is never one of `Transfer`'s own type parameters, so
    // splitting a single augmented `Generics` for all three (`impl_
    // generics`/`ty_generics`/`where_clause` together) would incorrectly
    // inject `V` into the type application too (`Transfer<S, Token, V>`,
    // which doesn't exist).
    let (_, ty_generics, _) = input.generics.split_for_impl();

    let mut generics = input.generics.clone();
    if args.verifier.is_none() {
        generics
            .params
            .push(parse_quote!(V: ::amenable_core::Verifier));
    }
    let evidence_predicate = evidence_bound(&generics, primary_ty, &proposition_ty, verifier);
    let token_predicate: WherePredicate = parse_quote!(#token_ty: ::amenable_core::ProofToken<Proposition = #proposition_ty> + ::std::clone::Clone);
    {
        let where_clause = generics.make_where_clause();
        where_clause.predicates.push(evidence_predicate);
        where_clause.predicates.push(token_predicate);
    }
    let (impl_generics, _, where_clause) = generics.split_for_impl();

    // Gated on `verifier`'s own stringified text, not `#[cfg_attr(
    // creusot, ..)]` -- see `creusot_ensures_attr`'s own doc comment,
    // below, for the real reason (a `cargo creusot` build sets `--cfg
    // creusot` across its whole dependency graph, not just the crate it
    // translates). Always `false` in verifier-less mode: `V` is a bare
    // generic parameter with no concrete text to compare, by design --
    // see this module's own doc comment for where Creusot's real content
    // goes instead.
    let is_creusot = args.verifier.is_some() && quote!(#verifier).to_string() == "CreusotVerifier";

    // Real, not decorative, for the identical reason `creusot_ensures_
    // attr` (below) is: `validate`'s own real body extracts data through
    // `input.primary()` before ever reaching a `Transfer::new(..)` this
    // crate's own `Ensures<CreusotVerifier>` impls need to reason about
    // -- without this, nothing downstream could learn what `primary()`'s
    // returned reference actually points at.
    let primary_ensures_attr = if is_creusot {
        quote! {
            #[::creusot_std::macros::ensures(result == &self.#primary_ident)]
        }
    } else {
        TokenStream::new()
    };

    let sidecar_impl = quote! {
        impl #impl_generics ::amenable_core::Sidecar<#verifier> for #name #ty_generics #where_clause {
            type Primary = #primary_ty;
            type Proposition = #proposition_ty;
            type SidecarToken = #token_ty;

            #primary_ensures_attr
            fn primary(&self) -> &Self::Primary {
                &self.#primary_ident
            }

            fn sidecar(&self) -> Self::SidecarToken {
                ::std::clone::Clone::clone(&self.#token_ident)
            }
        }
    };

    let constructor_vis = &args.constructor;
    let phantom_idents: Vec<&Ident> = phantom_fields
        .iter()
        .map(|field| {
            field
                .ident
                .as_ref()
                .ok_or_else(|| Error::new_spanned(*field, "named field"))
        })
        .collect::<syn::Result<Vec<_>>>()?;
    let (struct_impl_generics, struct_ty_generics, struct_where_clause) =
        input.generics.split_for_impl();

    // Real, not decorative: ordinary modular verification only exposes
    // what a function's own `ensures` promises, so without this, nothing
    // downstream could learn a constructed value's own field back from a
    // captured `Exchange` body's final `Ok(X::new(..))` return --
    // confirmed the hard way building `GAAP_LEDGER_PLAN.md`'s Step 6
    // (`Ledger::validate`'s own real body), the first `#[derive(
    // Sidecar)]` consumer whose postcondition actually needs to see this
    // far. Gated on `verifier`'s own *text* being literally
    // `"CreusotVerifier"`, not a blanket `#[cfg_attr(creusot, ..)]`:
    // `cargo creusot`'s own build sets `--cfg creusot` across the whole
    // dependency graph it compiles, not just the crate it translates --
    // confirmed the hard way, a real `` cannot find `creusot_std` in the
    // crate root `` error from `amenable_kani` (an ordinary Cargo
    // dependency of `amenable_creusot`, never itself translated, with no
    // real dependency on `creusot_std` at all) once this attribute was
    // unconditional. Comparing `verifier`'s own stringified tokens is
    // the only signal available at macro-expansion time for which
    // verifier a given `#[derive(Sidecar)]` invocation actually targets.
    let creusot_ensures_attr = if is_creusot {
        quote! {
            #[::creusot_std::macros::ensures(result.#primary_ident == #primary_ident)]
        }
    } else {
        TokenStream::new()
    };

    let constructor_impl = quote! {
        impl #struct_impl_generics #name #struct_ty_generics #struct_where_clause {
            #creusot_ensures_attr
            #constructor_vis fn new(#primary_ident: #primary_ty, #token_ident: #token_ty) -> Self {
                Self {
                    #primary_ident,
                    #token_ident,
                    #(#phantom_idents: ::core::marker::PhantomData,)*
                }
            }
        }
    };

    Ok(quote! {
        #sidecar_impl
        #constructor_impl
    })
}

/// Only add an `Evidence`/`Witness<V>` bound for a type that's actually a
/// generic parameter of this struct -- a concrete primary type
/// (`TransferPayload`) needs no bound restated here at all.
#[cfg_attr(
    not(kani),
    tracing::instrument(level = "debug", skip(generics, primary_ty, proposition_ty, verifier))
)]
fn evidence_bound(
    generics: &syn::Generics,
    primary_ty: &Type,
    proposition_ty: &Type,
    verifier: &Type,
) -> WherePredicate {
    let is_generic = |ty: &Type| {
        generics.params.iter().any(|param| match param {
            GenericParam::Type(type_param) => {
                matches!(ty, Type::Path(path) if path.path.is_ident(&type_param.ident))
            }
            _ => false,
        })
    };

    if types_match(primary_ty, proposition_ty) {
        // Established<T, Token>'s shape: one generic parameter plays both
        // roles, so it needs both bounds together.
        parse_quote!(#proposition_ty: ::amenable_core::Evidence + ::amenable_core::Witness<#verifier>)
    } else if is_generic(proposition_ty) {
        // Transfer<S, Token>'s shape: the proposition is a separate,
        // phantom generic parameter -- the (possibly concrete) primary
        // type only needs `Evidence` via its own real impl, not restated
        // here.
        parse_quote!(#proposition_ty: ::amenable_core::Evidence + ::amenable_core::Witness<#verifier>)
    } else {
        parse_quote!(#primary_ty: ::amenable_core::Evidence)
    }
}

/// `syn::Type` carries no `PartialEq` impl without the `extra-traits`
/// feature (not enabled here) -- compare by token text instead, the
/// common workaround for this exact situation.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(left, right)))]
fn types_match(left: &Type, right: &Type) -> bool {
    quote!(#left).to_string() == quote!(#right).to_string()
}

#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(input)))]
fn struct_fields(input: &DeriveInput) -> syn::Result<&Fields> {
    match &input.data {
        syn::Data::Struct(data) => Ok(&data.fields),
        _ => Err(Error::new_spanned(
            &input.ident,
            "Sidecar can only be derived for a struct",
        )),
    }
}

#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(fields)))]
fn find_marked_field<'a>(fields: &'a syn::FieldsNamed, marker: &str) -> syn::Result<&'a Field> {
    let mut found = None;
    for field in &fields.named {
        if has_marker(field, marker)? {
            if found.is_some() {
                return Err(Error::new_spanned(
                    field,
                    format!("only one field may carry #[sidecar({marker})]"),
                ));
            }
            found = Some(field);
        }
    }

    found.ok_or_else(|| {
        Error::new_spanned(
            &fields.named,
            format!("Sidecar requires exactly one field marked #[sidecar({marker})]"),
        )
    })
}

#[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(field)))]
fn has_marker(field: &Field, marker: &str) -> syn::Result<bool> {
    let mut found = false;
    for attr in field
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident("sidecar"))
    {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident(marker) {
                found = true;
                return Ok(());
            }
            if meta.path.is_ident("primary") || meta.path.is_ident("token") {
                return Ok(());
            }
            Err(meta.error("unsupported sidecar field attribute"))
        })?;
    }
    Ok(found)
}

#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(field)))]
fn require_phantom_data(field: &Field) -> syn::Result<()> {
    let is_phantom = matches!(
        &field.ty,
        Type::Path(path)
            if path.path.segments.last().is_some_and(|segment| segment.ident == "PhantomData")
    );

    if is_phantom {
        Ok(())
    } else {
        Err(Error::new_spanned(
            field,
            "Sidecar only supports a third field when it's a std::marker::PhantomData proposition marker",
        ))
    }
}

#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(attrs)))]
fn parse_sidecar_args(attrs: &[syn::Attribute]) -> syn::Result<SidecarArgs> {
    let mut verifier = None;
    let mut proposition = None;
    let mut constructor = None;

    for attr in attrs.iter().filter(|attr| attr.path().is_ident("sidecar")) {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("verifier") {
                let value: LitStr = meta.value()?.parse()?;
                verifier = Some(value.parse()?);
                return Ok(());
            }
            if meta.path.is_ident("proposition") {
                let value: LitStr = meta.value()?.parse()?;
                proposition = Some(value.parse()?);
                return Ok(());
            }
            if meta.path.is_ident("constructor") {
                let value: LitStr = meta.value()?.parse()?;
                constructor = Some(value.parse()?);
                return Ok(());
            }

            Err(meta.error("unsupported sidecar container attribute"))
        })?;
    }

    Ok(SidecarArgs {
        verifier,
        proposition,
        constructor: constructor.unwrap_or(Visibility::Public(Default::default())),
    })
}
