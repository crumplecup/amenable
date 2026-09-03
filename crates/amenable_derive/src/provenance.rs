//! `#[derive(Provenance)]`: a `Provenance` impl whose `metadata()` walks
//! every non-`#[provenance(skip)]` field's own `Provenance::metadata()`.

use quote::{format_ident, quote};
use syn::{
    Data, DataEnum, DataStruct, DeriveInput, Error, Field, Fields, Index, Path, Variant,
    WherePredicate, parse_quote,
};

use crate::attr_options::{
    ProvenanceContainerOptions, collect_field_types, field_name, parse_member_options,
    parse_provenance_container_options,
};

#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(input)))]
pub(crate) fn expand_provenance(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let options = parse_provenance_container_options(&input.attrs)?;
    let name = &input.ident;
    let crate_path = &options.crate_path;
    let field_types = collect_field_types(&input.data)?;
    let mut generics = input.generics.clone();
    let where_clause = generics.make_where_clause();

    for field_type in field_types {
        let predicate: WherePredicate = parse_quote!(#field_type: ::#crate_path::Provenance);
        where_clause.predicates.push(predicate);
    }

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let metadata_body = match &input.data {
        Data::Struct(data) => expand_struct_metadata(crate_path, data)?,
        Data::Enum(data) => expand_enum_metadata(crate_path, data, &options)?,
        Data::Union(data) => {
            return Err(Error::new_spanned(
                data.union_token,
                "Provenance can only be derived for structs and enums",
            ));
        }
    };

    Ok(quote! {
        impl #impl_generics ::#crate_path::Provenance for #name #ty_generics #where_clause {
            type MetadataIter = ::std::boxed::Box<dyn ::core::iter::Iterator<Item = ::#crate_path::MetadataEntry>>;

            fn metadata(&self) -> Self::MetadataIter {
                ::std::boxed::Box::new({ #metadata_body })
            }
        }
    })
}

#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(data)))]
fn expand_struct_metadata(
    crate_path: &Path,
    data: &DataStruct,
) -> syn::Result<proc_macro2::TokenStream> {
    let field_pushes = expand_struct_field_pushes(crate_path, &data.fields)?;

    Ok(quote! {
        let mut entries = ::std::vec::Vec::new();
        #(#field_pushes)*
        entries.into_iter()
    })
}

#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(data, options)))]
fn expand_enum_metadata(
    crate_path: &Path,
    data: &DataEnum,
    options: &ProvenanceContainerOptions,
) -> syn::Result<proc_macro2::TokenStream> {
    let tag = &options.tag;
    let arms = data
        .variants
        .iter()
        .map(|variant| expand_variant_arm(crate_path, variant, tag))
        .collect::<syn::Result<Vec<_>>>()?;

    Ok(quote! {
        match self {
            #(#arms),*
        }
    })
}

#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(variant)))]
fn expand_variant_arm(
    crate_path: &Path,
    variant: &Variant,
    tag: &str,
) -> syn::Result<proc_macro2::TokenStream> {
    let options = parse_member_options(&variant.attrs)?;
    let variant_name = options.rename.unwrap_or_else(|| variant.ident.to_string());
    let ident = &variant.ident;

    match &variant.fields {
        Fields::Named(fields) => {
            let field_idents = fields
                .named
                .iter()
                .map(|field| {
                    field.ident.clone().ok_or_else(|| {
                        Error::new_spanned(field, "named enum-field expansion requires identifiers")
                    })
                })
                .collect::<syn::Result<Vec<_>>>()?;
            let field_pushes = fields
                .named
                .iter()
                .map(|field| {
                    let field_options = parse_member_options(&field.attrs)?;
                    if field_options.skip {
                        return Ok(None);
                    }

                    let field_ident = field.ident.as_ref().ok_or_else(|| {
                        Error::new_spanned(field, "named enum-field expansion requires identifiers")
                    })?;
                    let field_name = field_options
                        .rename
                        .unwrap_or_else(|| field_ident.to_string());

                    Ok(Some(quote! {
                        for entry in ::#crate_path::Provenance::metadata(#field_ident) {
                            let key = if entry.key() == "value" {
                                ::std::borrow::ToOwned::to_owned(#field_name)
                            } else {
                                ::std::format!("{}.{}", #field_name, entry.key())
                            };

                            entries.push(::#crate_path::MetadataEntry::new(
                                key,
                                entry.value().to_owned(),
                            ));
                        }
                    }))
                })
                .collect::<syn::Result<Vec<_>>>()?
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();

            Ok(quote! {
                Self::#ident { #(#field_idents),* } => {
                    let mut entries = ::std::vec::Vec::new();
                    entries.push(::#crate_path::MetadataEntry::new(#tag, #variant_name));
                    #(#field_pushes)*
                    entries.into_iter()
                }
            })
        }
        Fields::Unnamed(fields) => {
            let field_bindings = fields
                .unnamed
                .iter()
                .enumerate()
                .map(|(index, _field)| format_ident!("__field_{index}"))
                .collect::<Vec<_>>();
            let field_pushes = fields
                .unnamed
                .iter()
                .enumerate()
                .map(|(index, field)| {
                    let field_options = parse_member_options(&field.attrs)?;
                    if field_options.skip {
                        return Ok(None);
                    }

                    let field_name = field_options.rename.unwrap_or_else(|| index.to_string());
                    let field_binding = &field_bindings[index];

                    Ok(Some(expand_field_entries(
                        crate_path,
                        field_name,
                        quote!(#field_binding),
                    )))
                })
                .collect::<syn::Result<Vec<_>>>()?
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();

            Ok(quote! {
                Self::#ident(#(#field_bindings),*) => {
                    let mut entries = ::std::vec::Vec::new();
                    entries.push(::#crate_path::MetadataEntry::new(#tag, #variant_name));
                    #(#field_pushes)*
                    entries.into_iter()
                }
            })
        }
        Fields::Unit => Ok(quote! {
            Self::#ident => {
                let mut entries = ::std::vec::Vec::new();
                entries.push(::#crate_path::MetadataEntry::new(#tag, #variant_name));
                entries.into_iter()
            }
        }),
    }
}

#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(fields)))]
fn expand_struct_field_pushes(
    crate_path: &Path,
    fields: &Fields,
) -> syn::Result<Vec<proc_macro2::TokenStream>> {
    match fields {
        Fields::Named(fields) => fields
            .named
            .iter()
            .map(|field| {
                let field_ident = field.ident.as_ref().ok_or_else(|| {
                    Error::new_spanned(field, "named-field expansion requires field identifiers")
                })?;
                expand_struct_field_push(crate_path, field, None, quote!(&self.#field_ident))
            })
            .collect(),
        Fields::Unnamed(fields) => fields
            .unnamed
            .iter()
            .enumerate()
            .map(|(index, field)| {
                let tuple_index = Index::from(index);
                expand_struct_field_push(crate_path, field, Some(index), quote!(&self.#tuple_index))
            })
            .collect(),
        Fields::Unit => Ok(Vec::new()),
    }
}

#[cfg_attr(
    not(kani),
    tracing::instrument(level = "debug", skip(field, field_access))
)]
fn expand_struct_field_push(
    crate_path: &Path,
    field: &Field,
    position: Option<usize>,
    field_access: proc_macro2::TokenStream,
) -> syn::Result<proc_macro2::TokenStream> {
    let options = parse_member_options(&field.attrs)?;
    if options.skip {
        return Ok(quote! {});
    }

    Ok(expand_field_entries(
        crate_path,
        field_name(field, position)?,
        field_access,
    ))
}

#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(field_access)))]
fn expand_field_entries(
    crate_path: &Path,
    field_name: String,
    field_access: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    quote! {
        for entry in ::#crate_path::Provenance::metadata(#field_access) {
            let key = if entry.key() == "value" {
                ::std::borrow::ToOwned::to_owned(#field_name)
            } else {
                ::std::format!("{}.{}", #field_name, entry.key())
            };

            entries.push(::#crate_path::MetadataEntry::new(
                key,
                entry.value().to_owned(),
            ));
        }
    }
}
