//! `#[derive(Metadata)]` / `#[derive(Provenance)]`: a `Metadata` impl whose
//! `snapshot()` projects each field per its `#[entry(..)]` role.
//! `#[derive(Provenance)]` additionally emits the marker `impl Provenance for
//! T {}`.
//!
//! Field roles ([`EntryKind`]):
//!
//! - **`#[entry]`** — leaf: `OwnedEntry::new("<field>", self.<field>.clone())`.
//!   Bound `FieldTy: MetadataValue + Clone`.
//! - **`#[entry(nested)]`** — sub-record: the field's own entries spliced under
//!   a `"<field>."` prefix. Bound `FieldTy: Metadata`.
//! - **`#[entry(flatten)]`** — sub-record spliced with no prefix.
//! - **`#[entry(skip)]`** / **`#[entry(rename = "..")]`**.
//! - **no `#[entry]` attribute** — legacy `Bare`: recurse via
//!   `Metadata::snapshot()`, re-key a `"value"`-keyed child entry to the field
//!   name, else prefix. `#[provenance(skip | rename)]` / `#[metadata(..)]` also
//!   keep a field `Bare`.

use quote::{format_ident, quote};
use syn::{
    Data, DataEnum, DataStruct, DeriveInput, Error, Field, Fields, Index, Path, Variant,
    WherePredicate, parse_quote,
};

use crate::attr_options::{
    EntryKind, ProvenanceContainerOptions, collect_field_bounds, field_name, parse_member_options,
    parse_provenance_container_options,
};

/// `#[derive(Metadata)]`: the `impl Metadata` alone.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(input)))]
pub(crate) fn expand_metadata(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let options = parse_provenance_container_options(&input.attrs)?;
    let name = &input.ident;
    let crate_path = options.crate_path();
    let generics = augmented_generics(input, crate_path)?;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let snapshot_body = match &input.data {
        Data::Struct(data) => expand_struct_metadata(crate_path, data)?,
        Data::Enum(data) => expand_enum_metadata(crate_path, data, &options)?,
        Data::Union(data) => {
            return Err(Error::new_spanned(
                data.union_token,
                "Metadata can only be derived for structs and enums",
            ));
        }
    };

    Ok(quote! {
        impl #impl_generics ::#crate_path::Metadata for #name #ty_generics #where_clause {
            fn snapshot(&self) -> ::std::vec::Vec<::#crate_path::OwnedEntry> {
                #snapshot_body
            }
        }
    })
}

/// `#[derive(Provenance)]`: the same `impl Metadata` the `Metadata` derive
/// emits, plus the marker `impl Provenance for T {}`.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(input)))]
pub(crate) fn expand_provenance(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let metadata_impl = expand_metadata(input)?;

    let options = parse_provenance_container_options(&input.attrs)?;
    let crate_path = options.crate_path();
    let name = &input.ident;
    let generics = augmented_generics(input, crate_path)?;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    Ok(quote! {
        #metadata_impl

        impl #impl_generics ::#crate_path::Provenance for #name #ty_generics #where_clause {}
    })
}

/// The input's generics with a `FieldTy: Metadata` bound added per non-skip
/// field.
#[cfg_attr(
    not(kani),
    tracing::instrument(level = "debug", skip(input, crate_path))
)]
fn augmented_generics(input: &DeriveInput, crate_path: &Path) -> syn::Result<syn::Generics> {
    let mut generics = input.generics.clone();
    let where_clause = generics.make_where_clause();

    for (field_type, kind) in collect_field_bounds(&input.data)? {
        let predicate: WherePredicate = match kind {
            EntryKind::Leaf => {
                parse_quote!(#field_type: ::#crate_path::MetadataValue + ::core::clone::Clone)
            }
            EntryKind::Bare | EntryKind::Nested | EntryKind::Flatten => {
                parse_quote!(#field_type: ::#crate_path::Metadata)
            }
        };
        where_clause.predicates.push(predicate);
    }

    Ok(generics)
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
        entries
    })
}

#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(data, options)))]
fn expand_enum_metadata(
    crate_path: &Path,
    data: &DataEnum,
    options: &ProvenanceContainerOptions,
) -> syn::Result<proc_macro2::TokenStream> {
    let tag = options.tag();
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
    let variant_name = options
        .rename()
        .clone()
        .unwrap_or_else(|| variant.ident.to_string());
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
                    if *field_options.skip() {
                        return Ok(None);
                    }

                    let field_ident = field.ident.as_ref().ok_or_else(|| {
                        Error::new_spanned(field, "named enum-field expansion requires identifiers")
                    })?;
                    let field_name = field_options
                        .rename()
                        .clone()
                        .unwrap_or_else(|| field_ident.to_string());

                    Ok(Some(expand_field_entries(
                        crate_path,
                        field_name,
                        quote!(#field_ident),
                        *field_options.kind(),
                    )))
                })
                .collect::<syn::Result<Vec<_>>>()?
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();

            Ok(quote! {
                Self::#ident { #(#field_idents),* } => {
                    let mut entries = ::std::vec::Vec::new();
                    entries.push(::#crate_path::OwnedEntry::new(#tag, #variant_name));
                    #(#field_pushes)*
                    entries
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
                    if *field_options.skip() {
                        return Ok(None);
                    }

                    let field_name = field_options
                        .rename()
                        .clone()
                        .unwrap_or_else(|| index.to_string());
                    let field_binding = &field_bindings[index];

                    Ok(Some(expand_field_entries(
                        crate_path,
                        field_name,
                        quote!(#field_binding),
                        *field_options.kind(),
                    )))
                })
                .collect::<syn::Result<Vec<_>>>()?
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();

            Ok(quote! {
                Self::#ident(#(#field_bindings),*) => {
                    let mut entries = ::std::vec::Vec::new();
                    entries.push(::#crate_path::OwnedEntry::new(#tag, #variant_name));
                    #(#field_pushes)*
                    entries
                }
            })
        }
        Fields::Unit => Ok(quote! {
            Self::#ident => {
                ::std::vec![::#crate_path::OwnedEntry::new(#tag, #variant_name)]
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
    if *options.skip() {
        return Ok(quote! {});
    }

    Ok(expand_field_entries(
        crate_path,
        field_name(field, position)?,
        field_access,
        *options.kind(),
    ))
}

/// Push one field's entries, per its [`EntryKind`]. `field_access` is a
/// reference to the field (`&self.x` for a struct, a `&T` match binding for an
/// enum variant).
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(field_access)))]
fn expand_field_entries(
    crate_path: &Path,
    field_name: String,
    field_access: proc_macro2::TokenStream,
    kind: EntryKind,
) -> proc_macro2::TokenStream {
    match kind {
        EntryKind::Leaf => quote! {
            entries.push(::#crate_path::OwnedEntry::new(
                #field_name,
                ::core::clone::Clone::clone(#field_access),
            ));
        },
        EntryKind::Nested => quote! {
            for entry in ::#crate_path::Metadata::snapshot(#field_access) {
                entries.push(entry.prefixed(#field_name));
            }
        },
        EntryKind::Flatten => quote! {
            entries.extend(::#crate_path::Metadata::snapshot(#field_access));
        },
        EntryKind::Bare => quote! {
            for entry in ::#crate_path::Metadata::snapshot(#field_access) {
                let key = if ::#crate_path::ErasedEntry::key(&entry) == "value" {
                    ::std::string::String::from(#field_name)
                } else {
                    ::std::format!("{}.{}", #field_name, ::#crate_path::ErasedEntry::key(&entry))
                };

                entries.push(entry.with_key(key));
            }
        },
    }
}
