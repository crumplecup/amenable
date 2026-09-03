//! `#[derive(Witness)]`: structural closure over already-witnessed members.
//!
//! The derived artifact is a new nominal proof type for the enclosing data
//! shape. Product types fold child proofs into one larger product proof;
//! sum types fold per-variant proofs into one larger sum proof. The outer
//! type identity and shape remain explicit, so structurally similar leaves
//! in different enclosing forms still yield distinct proof artifacts.

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    Data, DataEnum, DataStruct, DeriveInput, Error, Fields, GenericParam, Generics, Type, Variant,
    parse_quote,
};

use crate::attr_options::{
    collect_field_types_from_fields, field_name, parse_member_options,
    parse_provenance_container_options, parse_witness_container_options,
};

#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(input)))]
pub fn expand_witness(input: &DeriveInput) -> syn::Result<TokenStream> {
    let evidence_ident = &input.ident;
    let proof_ident = format_ident!("{evidence_ident}WitnessProof");
    let variant_prefix = format_ident!("{evidence_ident}WitnessVariant");
    let options = parse_provenance_container_options(&input.attrs)?;
    let witness_options = parse_witness_container_options(&input.attrs)?;
    let verus_module_path = witness_options
        .verus_module
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
            expand_enum_proof_types(&proof_type_context, &variant_prefix, data, &options.tag)?
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
struct ProofTypeContext<'a> {
    evidence_ident: &'a syn::Ident,
    evidence_ty_generics: syn::TypeGenerics<'a>,
    evidence_generics: &'a Generics,
    proof_ident: &'a syn::Ident,
    proof_generics: &'a Generics,
    display_generics: &'a Generics,
    artifact_generics: &'a Generics,
    verus_module_path: &'a str,
}

#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(ctx, data)))]
fn expand_struct_proof_type(
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

#[cfg_attr(
    not(kani),
    tracing::instrument(level = "info", skip(ctx, variant_prefix, data))
)]
fn expand_enum_proof_types(
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

#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(fields)))]
fn expand_proof_fields(fields: &Fields) -> syn::Result<Vec<ProofField>> {
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
fn expand_proof_field(
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
fn named_proof_field_ident(field: &syn::Field) -> syn::Result<syn::Ident> {
    field.ident.clone().ok_or_else(|| {
        syn::Error::new_spanned(field, "named proof field generation requires identifiers")
    })
}

#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(data)))]
fn collect_witness_field_types(data: &Data) -> syn::Result<Vec<Type>> {
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
fn add_witness_bounds(generics: &mut Generics, field_types: &[Type]) -> syn::Result<()> {
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
fn add_classified_witness_bounds(generics: &mut Generics, field_types: &[Type]) -> syn::Result<()> {
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
fn add_display_bounds(generics: &mut Generics, field_types: &[Type]) -> syn::Result<()> {
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
fn add_witness_artifact_bounds(generics: &mut Generics, field_types: &[Type]) -> syn::Result<()> {
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
fn expand_generics_marker(evidence_generics: &Generics) -> TokenStream {
    let marker_type = generics_marker_type(evidence_generics);

    quote! {
        __evidence_generics: ::std::marker::PhantomData<#marker_type>,
    }
}

#[cfg_attr(
    not(kani),
    tracing::instrument(level = "debug", skip(evidence_generics))
)]
fn generics_marker_type(evidence_generics: &Generics) -> TokenStream {
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
fn generic_marker_member(param: &GenericParam) -> TokenStream {
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
fn default_verus_module_path(evidence_ident: &syn::Ident) -> String {
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

struct ProofField {
    ident: syn::Ident,
    label: String,
    ty: Type,
    component_type: Type,
}

struct EnumProofField {
    field_ident: syn::Ident,
    variant_name: String,
    proof_ident: syn::Ident,
}
