//! Shared field-expansion, generic-bounds, and generics-marker helpers for
//! `#[derive(Witness)]`'s product and sum expansions.

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, Error, Fields, GenericParam, Generics, Type, parse_quote};

use crate::attr_options::{collect_field_types_from_fields, field_name, parse_member_options};

#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(fields)))]
pub(super) fn expand_proof_fields(fields: &Fields) -> syn::Result<Vec<ProofField>> {
    match fields {
        Fields::Named(fields) => fields
            .named
            .iter()
            .map(|field| {
                let ident = named_proof_field_ident(field)?;
                expand_proof_field(field, None, ident)
            })
            .filter_map(Result::transpose)
            .collect(),
        Fields::Unnamed(fields) => fields
            .unnamed
            .iter()
            .enumerate()
            .map(|(index, field)| {
                expand_proof_field(field, Some(index), format_ident!("field_{index}"))
            })
            .filter_map(Result::transpose)
            .collect(),
        Fields::Unit => Ok(Vec::new()),
    }
}

#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(field, ident)))]
pub(super) fn expand_proof_field(
    field: &syn::Field,
    position: Option<usize>,
    ident: syn::Ident,
) -> syn::Result<Option<ProofField>> {
    if parse_member_options(&field.attrs)?.skip {
        return Ok(None);
    }

    let label = field_name(field, position)?;
    let component_type = field.ty.clone();

    Ok(Some(ProofField {
        ident,
        label,
        ty: parse_quote!(
            <#component_type as ::amenable_core::Witness<__Verifier>>::ProofArtifact
        ),
        component_type,
    }))
}

#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(field)))]
pub(super) fn named_proof_field_ident(field: &syn::Field) -> syn::Result<syn::Ident> {
    field.ident.clone().ok_or_else(|| {
        syn::Error::new_spanned(field, "named proof field generation requires identifiers")
    })
}

#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(data)))]
pub(super) fn collect_witness_field_types(data: &Data) -> syn::Result<Vec<Type>> {
    match data {
        Data::Struct(data) => collect_field_types_from_fields(&data.fields),
        Data::Enum(data) => data
            .variants
            .iter()
            .map(|variant| collect_field_types_from_fields(&variant.fields))
            .collect::<syn::Result<Vec<_>>>()
            .map(|groups| groups.into_iter().flatten().collect()),
        Data::Union(data) => Err(Error::new_spanned(
            data.union_token,
            "Witness can only be derived for structs and enums",
        )),
    }
}

#[cfg_attr(
    not(kani),
    tracing::instrument(level = "debug", skip(generics, field_types))
)]
pub(super) fn add_witness_bounds(generics: &mut Generics, field_types: &[Type]) -> syn::Result<()> {
    let where_clause = generics.make_where_clause();
    where_clause
        .predicates
        .push(parse_quote!(__Verifier: ::amenable_core::Verifier));

    for field_type in field_types {
        where_clause
            .predicates
            .push(parse_quote!(#field_type: ::amenable_core::Witness<__Verifier>));
    }

    Ok(())
}

#[cfg_attr(
    not(kani),
    tracing::instrument(level = "debug", skip(generics, field_types))
)]
pub(super) fn add_classified_witness_bounds(
    generics: &mut Generics,
    field_types: &[Type],
) -> syn::Result<()> {
    let where_clause = generics.make_where_clause();
    where_clause
        .predicates
        .push(parse_quote!(__Verifier: ::amenable_core::Verifier));

    for field_type in field_types {
        where_clause
            .predicates
            .push(parse_quote!(#field_type: ::amenable_core::ClassifiedWitness<__Verifier>));
    }

    Ok(())
}

#[cfg_attr(
    not(kani),
    tracing::instrument(level = "debug", skip(generics, field_types))
)]
pub(super) fn add_display_bounds(generics: &mut Generics, field_types: &[Type]) -> syn::Result<()> {
    let where_clause = generics.make_where_clause();

    for field_type in field_types {
        where_clause.predicates.push(parse_quote!(
            <#field_type as ::amenable_core::Witness<__Verifier>>::ProofArtifact: ::std::fmt::Display
        ));
    }

    Ok(())
}

#[cfg_attr(
    not(kani),
    tracing::instrument(level = "debug", skip(generics, field_types))
)]
pub(super) fn add_witness_artifact_bounds(
    generics: &mut Generics,
    field_types: &[Type],
) -> syn::Result<()> {
    let where_clause = generics.make_where_clause();

    for field_type in field_types {
        where_clause.predicates.push(parse_quote!(
            <#field_type as ::amenable_core::Witness<__Verifier>>::ProofArtifact: ::amenable_core::WitnessArtifact
        ));
    }

    Ok(())
}

#[cfg_attr(
    not(kani),
    tracing::instrument(level = "debug", skip(evidence_generics))
)]
pub(super) fn expand_generics_marker(evidence_generics: &Generics) -> TokenStream {
    let marker_type = generics_marker_type(evidence_generics);

    quote! {
        __evidence_generics: ::std::marker::PhantomData<#marker_type>,
    }
}

#[cfg_attr(
    not(kani),
    tracing::instrument(level = "debug", skip(evidence_generics))
)]
pub(super) fn generics_marker_type(evidence_generics: &Generics) -> TokenStream {
    let marker_members = evidence_generics
        .params
        .iter()
        .map(generic_marker_member)
        .collect::<Vec<_>>();

    if marker_members.is_empty() {
        quote!(())
    } else {
        quote!((#(#marker_members,)*))
    }
}

#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(param)))]
pub(super) fn generic_marker_member(param: &GenericParam) -> TokenStream {
    match param {
        GenericParam::Type(type_param) => {
            let ident = &type_param.ident;
            quote!(#ident)
        }
        GenericParam::Lifetime(lifetime_param) => {
            let lifetime = &lifetime_param.lifetime;
            quote!(&#lifetime ())
        }
        GenericParam::Const(const_param) => {
            let ident = &const_param.ident;
            quote!([(); #ident])
        }
    }
}

#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(evidence_ident)))]
pub(super) fn default_verus_module_path(evidence_ident: &syn::Ident) -> String {
    format!(
        "crate::derived_witness::{}_witness",
        to_snake_case(&evidence_ident.to_string())
    )
}

#[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
fn to_snake_case(name: &str) -> String {
    let chars = name.chars().collect::<Vec<_>>();
    let mut snake = String::new();

    for (index, ch) in chars.iter().copied().enumerate() {
        if ch.is_uppercase() {
            let previous = index
                .checked_sub(1)
                .and_then(|position| chars.get(position));
            let next = chars.get(index + 1);
            let needs_separator = index > 0
                && previous.is_some_and(|ch| ch.is_lowercase() || ch.is_ascii_digit())
                || previous.is_some_and(|ch| ch.is_uppercase())
                    && next.is_some_and(|ch| ch.is_lowercase());

            if needs_separator {
                snake.push('_');
            }

            for lowercase in ch.to_lowercase() {
                snake.push(lowercase);
            }
        } else {
            snake.push(ch);
        }
    }

    snake
}

pub(super) struct ProofField {
    pub(super) ident: syn::Ident,
    pub(super) label: String,
    pub(super) ty: Type,
    pub(super) component_type: Type,
}
