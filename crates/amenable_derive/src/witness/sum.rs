//! `#[derive(Witness)]` for sum types: one per-variant proof type plus an
//! outer enum proof that dispatches to whichever variant is live.

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{DataEnum, Fields, Generics, Variant};

use super::ProofTypeContext;
use super::helpers::{expand_generics_marker, expand_proof_fields};
use crate::attr_options::parse_member_options;

#[cfg_attr(
    not(kani),
    tracing::instrument(level = "info", skip(ctx, variant_prefix, data))
)]
pub(super) fn expand_enum_proof_types(
    ctx: &ProofTypeContext<'_>,
    variant_prefix: &syn::Ident,
    data: &DataEnum,
    tag: &str,
) -> syn::Result<TokenStream> {
    let evidence_ident = ctx.evidence_ident;
    let evidence_ty_generics = &ctx.evidence_ty_generics;
    let evidence_generics = ctx.evidence_generics;
    let proof_ident = ctx.proof_ident;
    let proof_generics = ctx.proof_generics;
    let display_generics = ctx.display_generics;
    let artifact_generics = ctx.artifact_generics;
    let verus_module_path = ctx.verus_module_path;
    let variant_proofs = data
        .variants
        .iter()
        .map(|variant| {
            expand_enum_variant_proof_type(
                variant_prefix,
                evidence_generics,
                proof_generics,
                display_generics,
                artifact_generics,
                variant,
            )
        })
        .collect::<syn::Result<Vec<_>>>()?;
    let outer_fields = data
        .variants
        .iter()
        .map(|variant| {
            let options = parse_member_options(&variant.attrs)?;
            let field_ident = format_ident!("variant_{}", variant.ident.to_string().to_lowercase());
            let variant_name = options.rename.unwrap_or_else(|| variant.ident.to_string());
            let variant_proof_ident = format_ident!("{variant_prefix}{}", variant.ident);

            Ok(EnumProofField {
                field_ident,
                variant_name,
                proof_ident: variant_proof_ident,
            })
        })
        .collect::<syn::Result<Vec<_>>>()?;

    let outer_field_idents = outer_fields.iter().map(|field| &field.field_ident);
    let outer_field_types = outer_fields.iter().map(|field| &field.proof_ident);
    let constructor_bindings = outer_fields.iter().map(|field| {
        let field_ident = &field.field_ident;
        let proof_ident = &field.proof_ident;
        let (_, proof_ty_generics, _) = proof_generics.split_for_impl();
        let proof_turbofish = proof_ty_generics.as_turbofish();

        quote! {
            let #field_ident = #proof_ident #proof_turbofish::new();
        }
    });
    let support_terms = outer_fields.iter().map(|field| {
        let field_ident = &field.field_ident;

        quote! {
            #field_ident.support
        }
    });
    let constructors = outer_fields.iter().map(|field| {
        let field_ident = &field.field_ident;

        quote! {
            #field_ident
        }
    });
    let reports = outer_fields.iter().map(|field| {
        let field_ident = &field.field_ident;
        let variant_name = &field.variant_name;

        quote! {
            writeln!(f, "variant {}: {}", #variant_name, self.#field_ident)?;
        }
    });
    let artifact_variants = outer_fields.iter().map(|field| {
        let field_ident = &field.field_ident;
        let variant_name = &field.variant_name;

        quote! {
            ::amenable_core::WitnessArtifactVariant::new(
                #variant_name.to_owned(),
                ::std::boxed::Box::new(
                    ::amenable_core::WitnessArtifact::witness_artifact(&self.#field_ident)
                ),
            )
        }
    });

    let (proof_impl_generics, proof_ty_generics, proof_where_clause) =
        proof_generics.split_for_impl();
    let (display_impl_generics, display_ty_generics, display_where_clause) =
        display_generics.split_for_impl();
    let (artifact_impl_generics, artifact_ty_generics, artifact_where_clause) =
        artifact_generics.split_for_impl();

    Ok(quote! {
        #(#variant_proofs)*

        pub struct #proof_ident #proof_impl_generics #proof_where_clause {
            #(pub #outer_field_idents: #outer_field_types #proof_ty_generics,)*
            pub support: ::amenable_core::WitnessSupportSummary,
            pub verifier: ::std::marker::PhantomData<__Verifier>,
        }

        impl #proof_impl_generics #proof_ident #proof_ty_generics #proof_where_clause {
            pub const VERUS_MODULE_PATH: &'static str = #verus_module_path;

            pub fn new() -> Self {
                #(#constructor_bindings)*
                let support = ::amenable_core::WitnessSupportSummary::compose(&[
                    #(#support_terms,)*
                ]);

                Self {
                    #(#constructors,)*
                    support,
                    verifier: ::std::marker::PhantomData,
                }
            }
        }

        impl #display_impl_generics ::std::fmt::Display for #proof_ident #display_ty_generics #display_where_clause {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                writeln!(f, "verifier: {}", <__Verifier as ::amenable_core::Verifier>::name())?;
                writeln!(f, "evidence: {}", ::std::any::type_name::<#evidence_ident #evidence_ty_generics>())?;
                writeln!(f, "shape: enum")?;
                writeln!(f, "support: {}", self.support)?;
                writeln!(f, "tag: {}", #tag)?;
                #(#reports)*
                Ok(())
            }
        }

        impl #proof_impl_generics ::amenable_core::WitnessModulePath
            for #proof_ident #proof_ty_generics
            #proof_where_clause
        {
            const MODULE_PATH: &'static str = Self::VERUS_MODULE_PATH;
        }

        impl #artifact_impl_generics ::amenable_core::WitnessArtifact
            for #proof_ident #artifact_ty_generics
            #artifact_where_clause
        {
            fn witness_artifact(&self) -> ::amenable_core::WitnessArtifactNode {
                ::amenable_core::WitnessArtifactNode::enum_variants(
                    self.support,
                    #tag,
                    vec![#(#artifact_variants,)*],
                )
            }
        }
    })
}

#[cfg_attr(
    not(kani),
    tracing::instrument(
        level = "debug",
        skip(
            variant_prefix,
            evidence_generics,
            proof_generics,
            display_generics,
            artifact_generics,
            variant
        )
    )
)]
fn expand_enum_variant_proof_type(
    variant_prefix: &syn::Ident,
    evidence_generics: &Generics,
    proof_generics: &Generics,
    display_generics: &Generics,
    artifact_generics: &Generics,
    variant: &Variant,
) -> syn::Result<TokenStream> {
    let variant_ident = &variant.ident;
    let proof_ident = format_ident!("{variant_prefix}{variant_ident}");
    let shape_name = match &variant.fields {
        Fields::Named(_) => "named_variant",
        Fields::Unnamed(_) => "tuple_variant",
        Fields::Unit => "unit_variant",
    };
    let fields = expand_proof_fields(&variant.fields)?;
    let field_names = fields.iter().map(|field| &field.ident);
    let field_types = fields.iter().map(|field| &field.ty);
    let constructor_fields = fields.iter().map(|field| {
        let field_ident = &field.ident;
        let component_type = &field.component_type;

        quote! {
            #field_ident: <#component_type as ::amenable_core::Witness<__Verifier>>::proof()
        }
    });
    let support_terms = fields.iter().map(|field| {
        let component_type = &field.component_type;

        quote! {
            <#component_type as ::amenable_core::Witness<__Verifier>>::support()
        }
    });
    let report_lines = fields.iter().map(|field| {
        let field_ident = &field.ident;
        let label = &field.label;

        quote! {
            writeln!(f, "member {}: {}", #label, self.#field_ident)?;
        }
    });
    let artifact_members = fields.iter().map(|field| {
        let field_ident = &field.ident;
        let label = &field.label;

        quote! {
            ::amenable_core::WitnessArtifactMember::new(
                #label.to_owned(),
                ::std::boxed::Box::new(
                    ::amenable_core::WitnessArtifact::witness_artifact(&self.#field_ident)
                ),
            )
        }
    });
    let generics_marker = expand_generics_marker(evidence_generics);
    let (proof_impl_generics, proof_ty_generics, proof_where_clause) =
        proof_generics.split_for_impl();
    let (display_impl_generics, display_ty_generics, display_where_clause) =
        display_generics.split_for_impl();
    let (artifact_impl_generics, artifact_ty_generics, artifact_where_clause) =
        artifact_generics.split_for_impl();

    Ok(quote! {
        pub struct #proof_ident #proof_impl_generics #proof_where_clause {
            #(pub #field_names: #field_types,)*
            #generics_marker
            pub support: ::amenable_core::WitnessSupportSummary,
            pub verifier: ::std::marker::PhantomData<__Verifier>,
        }

        impl #proof_impl_generics #proof_ident #proof_ty_generics #proof_where_clause {
            pub fn new() -> Self {
                Self {
                    #(#constructor_fields,)*
                    __evidence_generics: ::std::marker::PhantomData,
                    support: ::amenable_core::WitnessSupportSummary::compose(&[
                        #(#support_terms,)*
                    ]),
                    verifier: ::std::marker::PhantomData,
                }
            }
        }

        impl #display_impl_generics ::std::fmt::Display for #proof_ident #display_ty_generics #display_where_clause {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                writeln!(f, "shape: {}", #shape_name)?;
                writeln!(f, "support: {}", self.support)?;
                writeln!(f, "variant: {}", stringify!(#variant_ident))?;
                #(#report_lines)*
                Ok(())
            }
        }

        impl #artifact_impl_generics ::amenable_core::WitnessArtifact
            for #proof_ident #artifact_ty_generics
            #artifact_where_clause
        {
            fn witness_artifact(&self) -> ::amenable_core::WitnessArtifactNode {
                ::amenable_core::WitnessArtifactNode::with_members(
                    match #shape_name {
                        "named_variant" => ::amenable_core::WitnessArtifactShape::NamedVariant,
                        "tuple_variant" => ::amenable_core::WitnessArtifactShape::TupleVariant,
                        "unit_variant" => ::amenable_core::WitnessArtifactShape::UnitVariant,
                        _ => unreachable!("variant proof shapes are exhaustive"),
                    },
                    self.support,
                    Some(stringify!(#variant_ident).to_owned()),
                    vec![#(#artifact_members,)*],
                )
            }
        }
    })
}

struct EnumProofField {
    field_ident: syn::Ident,
    variant_name: String,
    proof_ident: syn::Ident,
}
