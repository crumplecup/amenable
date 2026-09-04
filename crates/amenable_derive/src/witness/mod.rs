//! `#[derive(Witness)]`: structural closure over already-witnessed members.
//!
//! The derived artifact is a new nominal proof type for the enclosing data
//! shape. Product types fold child proofs into one larger product proof
//! (`product`); sum types fold per-variant proofs into one larger sum proof
//! (`sum`). Shared field/bounds/marker helpers live in `helpers`.

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Error, Generics, parse_quote};

use crate::attr_options::{parse_provenance_container_options, parse_witness_container_options};

mod helpers;
mod product;
mod sum;

use helpers::{
    add_classified_witness_bounds, add_display_bounds, add_witness_artifact_bounds,
    add_witness_bounds, collect_witness_field_types, default_verus_module_path,
};
use product::expand_struct_proof_type;
use sum::expand_enum_proof_types;

#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(input)))]
pub fn expand_witness(input: &DeriveInput) -> syn::Result<TokenStream> {
    let evidence_ident = &input.ident;
    let proof_ident = format_ident!("{evidence_ident}WitnessProof");
    let variant_prefix = format_ident!("{evidence_ident}WitnessVariant");
    let options = parse_provenance_container_options(&input.attrs)?;
    let witness_options = parse_witness_container_options(&input.attrs)?;
    let verus_module_path = witness_options
        .verus_module()
        .clone()
        .unwrap_or_else(|| default_verus_module_path(evidence_ident));
    let (_, evidence_ty_generics, _) = input.generics.split_for_impl();

    let mut proof_generics = input.generics.clone();
    proof_generics.params.push(parse_quote!(__Verifier));
    add_witness_bounds(
        &mut proof_generics,
        &collect_witness_field_types(&input.data)?,
    )?;

    let mut display_generics = proof_generics.clone();
    add_display_bounds(
        &mut display_generics,
        &collect_witness_field_types(&input.data)?,
    )?;
    let mut artifact_generics = proof_generics.clone();
    add_witness_artifact_bounds(
        &mut artifact_generics,
        &collect_witness_field_types(&input.data)?,
    )?;

    let proof_type_context = ProofTypeContext {
        evidence_ident,
        evidence_ty_generics: evidence_ty_generics.clone(),
        evidence_generics: &input.generics,
        proof_ident: &proof_ident,
        proof_generics: &proof_generics,
        display_generics: &display_generics,
        artifact_generics: &artifact_generics,
        verus_module_path: &verus_module_path,
    };

    let proof_definition = match &input.data {
        Data::Struct(data) => expand_struct_proof_type(&proof_type_context, data)?,
        Data::Enum(data) => {
            expand_enum_proof_types(&proof_type_context, &variant_prefix, data, options.tag())?
        }
        Data::Union(data) => {
            return Err(Error::new_spanned(
                data.union_token,
                "Witness can only be derived for structs and enums",
            ));
        }
    };

    let (_, proof_ty_generics, _) = proof_generics.split_for_impl();
    let proof_turbofish = proof_ty_generics.as_turbofish();
    let witness_type_predicate: syn::WherePredicate =
        parse_quote!(#evidence_ident #evidence_ty_generics: ::amenable_core::Evidence);
    let mut witness_generics = proof_generics.clone();
    witness_generics
        .make_where_clause()
        .predicates
        .push(witness_type_predicate.clone());
    let (witness_impl_generics, _, witness_where_clause) = witness_generics.split_for_impl();

    // A separate impl, not folded into the Witness<__Verifier> one above:
    // ClassifiedWitness<V>: Witness<V> is a supertrait, so requiring every
    // field to satisfy ClassifiedWitness<__Verifier> here already implies
    // each field's (weaker) Witness<__Verifier> bound -- no need to restate
    // it. This only applies (and only lets this evidence type satisfy
    // ClassifiedWitness itself) when every field's own support has actually
    // been classified; a field still on Witness's Opaque default leaves this
    // impl inapplicable, which is what turns an unclassified leaf anywhere
    // in a composed tree into a real `cargo check`-time error at the point
    // something requires ClassifiedWitness of the whole composite (see
    // `amenable_core::register_witness_exports!`).
    let mut classified_generics = input.generics.clone();
    classified_generics.params.push(parse_quote!(__Verifier));
    add_classified_witness_bounds(
        &mut classified_generics,
        &collect_witness_field_types(&input.data)?,
    )?;
    classified_generics
        .make_where_clause()
        .predicates
        .push(witness_type_predicate);
    let (classified_impl_generics, _, classified_where_clause) =
        classified_generics.split_for_impl();

    Ok(quote! {
        #proof_definition

        impl #witness_impl_generics ::amenable_core::Witness<__Verifier>
            for #evidence_ident #evidence_ty_generics
            #witness_where_clause
        {
            type SupportingEvidence = Self;
            type ProofArtifact = #proof_ident #proof_ty_generics;

            fn proof() -> Self::ProofArtifact {
                #proof_ident #proof_turbofish::new()
            }

            fn support() -> ::amenable_core::WitnessSupportSummary {
                #proof_ident #proof_turbofish::new().support
            }
        }

        impl #classified_impl_generics ::amenable_core::ClassifiedWitness<__Verifier>
            for #evidence_ident #evidence_ty_generics
            #classified_where_clause
        {}
    })
}

/// The shared parameter set every proof-type expansion (struct, enum,
/// per-variant) needs from the enclosing `#[derive(Witness)]` invocation --
/// bundled so each expansion function stays under clippy's argument-count
/// lint without losing any of the context.
pub(super) struct ProofTypeContext<'a> {
    evidence_ident: &'a syn::Ident,
    evidence_ty_generics: syn::TypeGenerics<'a>,
    evidence_generics: &'a Generics,
    proof_ident: &'a syn::Ident,
    proof_generics: &'a Generics,
    display_generics: &'a Generics,
    artifact_generics: &'a Generics,
    verus_module_path: &'a str,
}
