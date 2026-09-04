//! `#[derive(Witness)]` for product types: fold each field's child proof
//! into one struct proof artifact.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataStruct, Fields};

use super::ProofTypeContext;
use super::helpers::{expand_generics_marker, expand_proof_fields};

#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(ctx, data)))]
pub(super) fn expand_struct_proof_type(
    ctx: &ProofTypeContext<'_>,
    data: &DataStruct,
) -> syn::Result<TokenStream> {
    let evidence_ident = ctx.evidence_ident;
    let evidence_ty_generics = &ctx.evidence_ty_generics;
    let evidence_generics = ctx.evidence_generics;
    let proof_ident = ctx.proof_ident;
    let proof_generics = ctx.proof_generics;
    let display_generics = ctx.display_generics;
    let artifact_generics = ctx.artifact_generics;
    let verus_module_path = ctx.verus_module_path;
    let shape_name = match &data.fields {
        Fields::Named(_) => "named_struct",
        Fields::Unnamed(_) => "tuple_struct",
        Fields::Unit => "unit_struct",
    };
    let fields = expand_proof_fields(&data.fields)?;
    let field_names = fields.iter().map(|field| field.ident());
    let field_types = fields.iter().map(|field| field.ty());
    let constructor_fields = fields.iter().map(|field| {
        let field_ident = field.ident();
        let component_type = field.component_type();

        quote! {
            #field_ident: <#component_type as ::amenable_core::Witness<__Verifier>>::proof()
        }
    });
    let support_terms = fields.iter().map(|field| {
        let component_type = field.component_type();

        quote! {
            <#component_type as ::amenable_core::Witness<__Verifier>>::support()
        }
    });
    let report_lines = fields.iter().map(|field| {
        let field_ident = field.ident();
        let label = field.label();

        quote! {
            writeln!(f, "member {}: {}", #label, self.#field_ident)?;
        }
    });
    let artifact_members = fields.iter().map(|field| {
        let field_ident = field.ident();
        let label = field.label();

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
            pub const VERUS_MODULE_PATH: &'static str = #verus_module_path;

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
                writeln!(f, "verifier: {}", <__Verifier as ::amenable_core::Verifier>::name())?;
                writeln!(f, "evidence: {}", ::std::any::type_name::<#evidence_ident #evidence_ty_generics>())?;
                writeln!(f, "shape: {}", #shape_name)?;
                writeln!(f, "support: {}", self.support)?;
                #(#report_lines)*
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
                ::amenable_core::WitnessArtifactNode::with_members(
                    match #shape_name {
                        "named_struct" => ::amenable_core::WitnessArtifactShape::NamedStruct,
                        "tuple_struct" => ::amenable_core::WitnessArtifactShape::TupleStruct,
                        "unit_struct" => ::amenable_core::WitnessArtifactShape::UnitStruct,
                        _ => unreachable!("struct proof shapes are exhaustive"),
                    },
                    self.support,
                    None,
                    vec![#(#artifact_members,)*],
                )
            }
        }
    })
}
