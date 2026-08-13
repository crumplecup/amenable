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

use crate::{
    collect_field_types_from_fields, field_name, parse_container_options, parse_member_options,
};

pub fn expand_witness(input: &DeriveInput) -> syn::Result<TokenStream> {
    let evidence_ident = &input.ident;
    let proof_ident = format_ident!("{evidence_ident}WitnessProof");
    let variant_prefix = format_ident!("{evidence_ident}WitnessVariant");
    let options = parse_container_options(&input.attrs)?;
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

    let proof_definition = match &input.data {
        Data::Struct(data) => expand_struct_proof_type(
            evidence_ident,
            evidence_ty_generics.clone(),
            &input.generics,
            &proof_ident,
            &proof_generics,
            &display_generics,
            data,
        )?,
        Data::Enum(data) => expand_enum_proof_types(
            evidence_ident,
            evidence_ty_generics.clone(),
            &input.generics,
            &proof_ident,
            &variant_prefix,
            &proof_generics,
            &display_generics,
            data,
            &options.tag,
        )?,
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
        .push(witness_type_predicate);
    let (witness_impl_generics, _, witness_where_clause) = witness_generics.split_for_impl();

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
        }
    })
}

fn expand_struct_proof_type(
    evidence_ident: &syn::Ident,
    evidence_ty_generics: syn::TypeGenerics<'_>,
    evidence_generics: &Generics,
    proof_ident: &syn::Ident,
    proof_generics: &Generics,
    display_generics: &Generics,
    data: &DataStruct,
) -> syn::Result<TokenStream> {
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
    let report_lines = fields.iter().map(|field| {
        let field_ident = &field.ident;
        let label = &field.label;

        quote! {
            writeln!(f, "member {}: {}", #label, self.#field_ident)?;
        }
    });
    let generics_marker = expand_generics_marker(evidence_generics);

    let (proof_impl_generics, proof_ty_generics, proof_where_clause) =
        proof_generics.split_for_impl();
    let (display_impl_generics, display_ty_generics, display_where_clause) =
        display_generics.split_for_impl();

    Ok(quote! {
        pub struct #proof_ident #proof_impl_generics #proof_where_clause {
            #(pub #field_names: #field_types,)*
            #generics_marker
            pub verifier: ::std::marker::PhantomData<__Verifier>,
        }

        impl #proof_impl_generics #proof_ident #proof_ty_generics #proof_where_clause {
            pub fn new() -> Self {
                Self {
                    #(#constructor_fields,)*
                    __evidence_generics: ::std::marker::PhantomData,
                    verifier: ::std::marker::PhantomData,
                }
            }
        }

        impl #display_impl_generics ::std::fmt::Display for #proof_ident #display_ty_generics #display_where_clause {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                writeln!(f, "verifier: {}", <__Verifier as ::amenable_core::Verifier>::name())?;
                writeln!(f, "evidence: {}", ::std::any::type_name::<#evidence_ident #evidence_ty_generics>())?;
                writeln!(f, "shape: {}", #shape_name)?;
                #(#report_lines)*
                Ok(())
            }
        }
    })
}

fn expand_enum_proof_types(
    evidence_ident: &syn::Ident,
    evidence_ty_generics: syn::TypeGenerics<'_>,
    evidence_generics: &Generics,
    proof_ident: &syn::Ident,
    variant_prefix: &syn::Ident,
    proof_generics: &Generics,
    display_generics: &Generics,
    data: &DataEnum,
    tag: &str,
) -> syn::Result<TokenStream> {
    let variant_proofs = data
        .variants
        .iter()
        .map(|variant| {
            expand_enum_variant_proof_type(
                variant_prefix,
                evidence_generics,
                proof_generics,
                display_generics,
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
    let constructors = outer_fields.iter().map(|field| {
        let field_ident = &field.field_ident;
        let proof_ident = &field.proof_ident;
        let (_, proof_ty_generics, _) = proof_generics.split_for_impl();
        let proof_turbofish = proof_ty_generics.as_turbofish();

        quote! {
            #field_ident: #proof_ident #proof_turbofish::new()
        }
    });
    let reports = outer_fields.iter().map(|field| {
        let field_ident = &field.field_ident;
        let variant_name = &field.variant_name;

        quote! {
            writeln!(f, "variant {}: {}", #variant_name, self.#field_ident)?;
        }
    });

    let (proof_impl_generics, proof_ty_generics, proof_where_clause) =
        proof_generics.split_for_impl();
    let (display_impl_generics, display_ty_generics, display_where_clause) =
        display_generics.split_for_impl();

    Ok(quote! {
        #(#variant_proofs)*

        pub struct #proof_ident #proof_impl_generics #proof_where_clause {
            #(pub #outer_field_idents: #outer_field_types #proof_ty_generics,)*
            pub verifier: ::std::marker::PhantomData<__Verifier>,
        }

        impl #proof_impl_generics #proof_ident #proof_ty_generics #proof_where_clause {
            pub fn new() -> Self {
                Self {
                    #(#constructors,)*
                    verifier: ::std::marker::PhantomData,
                }
            }
        }

        impl #display_impl_generics ::std::fmt::Display for #proof_ident #display_ty_generics #display_where_clause {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                writeln!(f, "verifier: {}", <__Verifier as ::amenable_core::Verifier>::name())?;
                writeln!(f, "evidence: {}", ::std::any::type_name::<#evidence_ident #evidence_ty_generics>())?;
                writeln!(f, "shape: enum")?;
                writeln!(f, "tag: {}", #tag)?;
                #(#reports)*
                Ok(())
            }
        }
    })
}

fn expand_enum_variant_proof_type(
    variant_prefix: &syn::Ident,
    evidence_generics: &Generics,
    proof_generics: &Generics,
    display_generics: &Generics,
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
    let report_lines = fields.iter().map(|field| {
        let field_ident = &field.ident;
        let label = &field.label;

        quote! {
            writeln!(f, "member {}: {}", #label, self.#field_ident)?;
        }
    });
    let generics_marker = expand_generics_marker(evidence_generics);
    let (proof_impl_generics, proof_ty_generics, proof_where_clause) =
        proof_generics.split_for_impl();
    let (display_impl_generics, display_ty_generics, display_where_clause) =
        display_generics.split_for_impl();

    Ok(quote! {
        pub struct #proof_ident #proof_impl_generics #proof_where_clause {
            #(pub #field_names: #field_types,)*
            #generics_marker
            pub verifier: ::std::marker::PhantomData<__Verifier>,
        }

        impl #proof_impl_generics #proof_ident #proof_ty_generics #proof_where_clause {
            pub fn new() -> Self {
                Self {
                    #(#constructor_fields,)*
                    __evidence_generics: ::std::marker::PhantomData,
                    verifier: ::std::marker::PhantomData,
                }
            }
        }

        impl #display_impl_generics ::std::fmt::Display for #proof_ident #display_ty_generics #display_where_clause {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                writeln!(f, "shape: {}", #shape_name)?;
                writeln!(f, "variant: {}", stringify!(#variant_ident))?;
                #(#report_lines)*
                Ok(())
            }
        }
    })
}

fn expand_proof_fields(fields: &Fields) -> syn::Result<Vec<ProofField>> {
    match fields {
        Fields::Named(fields) => fields
            .named
            .iter()
            .map(|field| expand_proof_field(field, None, named_proof_field_ident(field)))
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

fn named_proof_field_ident(field: &syn::Field) -> syn::Ident {
    field
        .ident
        .clone()
        .expect("named proof field generation requires identifiers")
}

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

fn add_display_bounds(generics: &mut Generics, field_types: &[Type]) -> syn::Result<()> {
    let where_clause = generics.make_where_clause();

    for field_type in field_types {
        where_clause.predicates.push(parse_quote!(
            <#field_type as ::amenable_core::Witness<__Verifier>>::ProofArtifact: ::std::fmt::Display
        ));
    }

    Ok(())
}

fn expand_generics_marker(evidence_generics: &Generics) -> TokenStream {
    let marker_type = generics_marker_type(evidence_generics);

    quote! {
        __evidence_generics: ::std::marker::PhantomData<#marker_type>,
    }
}

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
