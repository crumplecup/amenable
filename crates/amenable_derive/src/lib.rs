//! Proc macros for the `amenable` constitutional trait family.

use proc_macro::TokenStream;

use quote::quote;
use syn::{
    Data, DataEnum, DataStruct, DeriveInput, Error, Field, Fields, LitStr, Path, Type, Variant,
    WherePredicate, parse_macro_input, parse_quote,
};

#[proc_macro_derive(Provenance, attributes(provenance))]
pub fn derive_provenance(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match expand_provenance(&input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

struct ContainerOptions {
    crate_path: Path,
    tag: String,
}

impl Default for ContainerOptions {
    fn default() -> Self {
        Self {
            crate_path: parse_quote!(amenable_core),
            tag: "variant".to_string(),
        }
    }
}

#[derive(Default)]
struct MemberOptions {
    rename: Option<String>,
    skip: bool,
}

fn expand_provenance(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let options = parse_container_options(&input.attrs)?;
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
            fn metadata(&self) -> impl ::core::iter::Iterator<Item = ::#crate_path::MetadataEntry> {
                #metadata_body
            }
        }
    })
}

fn expand_struct_metadata(
    crate_path: &Path,
    data: &DataStruct,
) -> syn::Result<proc_macro2::TokenStream> {
    let field_pushes = expand_named_fields(crate_path, &data.fields, |field_name, field_ident| {
        expand_field_entries(crate_path, field_name, quote!(&self.#field_ident))
    })?;

    Ok(quote! {
        let mut entries = ::std::vec::Vec::new();
        #(#field_pushes)*
        entries.into_iter()
    })
}

fn expand_enum_metadata(
    crate_path: &Path,
    data: &DataEnum,
    options: &ContainerOptions,
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
                    field
                        .ident
                        .clone()
                        .expect("named enum-field expansion requires identifiers")
                })
                .collect::<Vec<_>>();
            let field_pushes = fields
                .named
                .iter()
                .map(|field| {
                    let field_options = parse_member_options(&field.attrs)?;
                    if field_options.skip {
                        return Ok(None);
                    }

                    let field_name = field_options.rename.unwrap_or_else(|| {
                        field
                            .ident
                            .as_ref()
                            .expect("named enum-field expansion requires identifiers")
                            .to_string()
                    });
                    let field_ident = field
                        .ident
                        .as_ref()
                        .expect("named enum-field expansion requires identifiers");

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
        Fields::Unit => Ok(quote! {
            Self::#ident => {
                let mut entries = ::std::vec::Vec::new();
                entries.push(::#crate_path::MetadataEntry::new(#tag, #variant_name));
                entries.into_iter()
            }
        }),
        Fields::Unnamed(fields) => Err(Error::new_spanned(
            fields,
            "Provenance derive does not support tuple structs or tuple variants; use named fields instead",
        )),
    }
}

fn expand_named_fields<F>(
    crate_path: &Path,
    fields: &Fields,
    make_push: F,
) -> syn::Result<Vec<proc_macro2::TokenStream>>
where
    F: Fn(String, proc_macro2::TokenStream) -> proc_macro2::TokenStream,
{
    match fields {
        Fields::Named(fields) => fields
            .named
            .iter()
            .map(|field| expand_named_field(field, crate_path, &make_push))
            .collect(),
        Fields::Unit => Ok(Vec::new()),
        Fields::Unnamed(fields) => Err(Error::new_spanned(
            fields,
            "Provenance derive does not support tuple structs or tuple variants; use named fields instead",
        )),
    }
}

fn expand_named_field<F>(
    field: &Field,
    _crate_path: &Path,
    make_push: &F,
) -> syn::Result<proc_macro2::TokenStream>
where
    F: Fn(String, proc_macro2::TokenStream) -> proc_macro2::TokenStream,
{
    let options = parse_member_options(&field.attrs)?;
    if options.skip {
        return Ok(quote! {});
    }

    let field_name = options.rename.unwrap_or_else(|| {
        field
            .ident
            .as_ref()
            .expect("named-field expansion requires field identifiers")
            .to_string()
    });
    let field_ident = field
        .ident
        .as_ref()
        .expect("named-field expansion requires field identifiers");

    Ok(make_push(field_name, quote!(#field_ident)))
}

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

fn collect_field_types(data: &Data) -> syn::Result<Vec<Type>> {
    match data {
        Data::Struct(data) => collect_named_field_types(&data.fields),
        Data::Enum(data) => data
            .variants
            .iter()
            .map(|variant| collect_named_field_types(&variant.fields))
            .collect::<syn::Result<Vec<_>>>()
            .map(|groups| groups.into_iter().flatten().collect()),
        Data::Union(data) => Err(Error::new_spanned(
            data.union_token,
            "Provenance can only be derived for structs and enums",
        )),
    }
}

fn collect_named_field_types(fields: &Fields) -> syn::Result<Vec<Type>> {
    match fields {
        Fields::Named(fields) => fields
            .named
            .iter()
            .filter_map(|field| match parse_member_options(&field.attrs) {
                Ok(options) if options.skip => None,
                Ok(_) => Some(Ok(field.ty.clone())),
                Err(error) => Some(Err(error)),
            })
            .collect(),
        Fields::Unit => Ok(Vec::new()),
        Fields::Unnamed(fields) => Err(Error::new_spanned(
            fields,
            "Provenance derive does not support tuple structs or tuple variants; use named fields instead",
        )),
    }
}

fn parse_container_options(attrs: &[syn::Attribute]) -> syn::Result<ContainerOptions> {
    let mut options = ContainerOptions::default();

    for attr in attrs
        .iter()
        .filter(|attr| attr.path().is_ident("provenance"))
    {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("crate") {
                let value: LitStr = meta.value()?.parse()?;
                options.crate_path = value.parse()?;
                return Ok(());
            }

            if meta.path.is_ident("tag") {
                let value: LitStr = meta.value()?.parse()?;
                options.tag = value.value();
                return Ok(());
            }

            Err(meta.error("unsupported provenance container attribute"))
        })?;
    }

    Ok(options)
}

fn parse_member_options(attrs: &[syn::Attribute]) -> syn::Result<MemberOptions> {
    let mut options = MemberOptions::default();

    for attr in attrs
        .iter()
        .filter(|attr| attr.path().is_ident("provenance"))
    {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("rename") {
                let value: LitStr = meta.value()?.parse()?;
                options.rename = Some(value.value());
                return Ok(());
            }

            if meta.path.is_ident("skip") {
                options.skip = true;
                return Ok(());
            }

            Err(meta.error("unsupported provenance field or variant attribute"))
        })?;
    }

    Ok(options)
}
