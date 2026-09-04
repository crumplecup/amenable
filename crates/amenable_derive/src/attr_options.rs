//! Shared `#[provenance(..)]` / `#[witness(..)]` attribute parsing and
//! field-type collection, used by both `#[derive(Provenance)]` (`provenance`)
//! and `#[derive(Witness)]` (`witness`).

use derive_getters::Getters;
use syn::{Data, Error, Field, Fields, LitStr, Path, Type, parse_quote};

#[derive(Getters)]
pub(crate) struct ProvenanceContainerOptions {
    crate_path: Path,
    tag: String,
}

impl Default for ProvenanceContainerOptions {
    #[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
    fn default() -> Self {
        Self {
            crate_path: parse_quote!(amenable_core),
            tag: "variant".to_string(),
        }
    }
}

#[derive(Default, Getters)]
pub(crate) struct WitnessContainerOptions {
    verus_module: Option<String>,
}

#[derive(Default, Getters)]
pub(crate) struct MemberOptions {
    rename: Option<String>,
    skip: bool,
}

#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(data)))]
pub(crate) fn collect_field_types(data: &Data) -> syn::Result<Vec<Type>> {
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
            "Provenance can only be derived for structs and enums",
        )),
    }
}

#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(fields)))]
pub(crate) fn collect_field_types_from_fields(fields: &Fields) -> syn::Result<Vec<Type>> {
    match fields {
        Fields::Named(fields) => fields
            .named
            .iter()
            .map(collect_field_type)
            .filter_map(Result::transpose)
            .collect(),
        Fields::Unnamed(fields) => fields
            .unnamed
            .iter()
            .map(collect_field_type)
            .filter_map(Result::transpose)
            .collect(),
        Fields::Unit => Ok(Vec::new()),
    }
}

#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(field)))]
fn collect_field_type(field: &Field) -> syn::Result<Option<Type>> {
    let options = parse_member_options(&field.attrs)?;

    if options.skip {
        return Ok(None);
    }

    Ok(Some(field.ty.clone()))
}

#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(field)))]
pub(crate) fn field_name(field: &Field, position: Option<usize>) -> syn::Result<String> {
    let options = parse_member_options(&field.attrs)?;
    if let Some(rename) = options.rename {
        return Ok(rename);
    }

    match (&field.ident, position) {
        (Some(ident), _) => Ok(ident.to_string()),
        (None, Some(index)) => Ok(index.to_string()),
        (None, None) => Err(Error::new_spanned(
            field,
            "tuple fields require an explicit position",
        )),
    }
}

#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(attrs)))]
pub(crate) fn parse_provenance_container_options(
    attrs: &[syn::Attribute],
) -> syn::Result<ProvenanceContainerOptions> {
    let mut options = ProvenanceContainerOptions::default();

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

#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(attrs)))]
pub(crate) fn parse_witness_container_options(
    attrs: &[syn::Attribute],
) -> syn::Result<WitnessContainerOptions> {
    let mut options = WitnessContainerOptions::default();

    for attr in attrs.iter().filter(|attr| attr.path().is_ident("witness")) {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("verus") {
                meta.parse_nested_meta(|meta| {
                    if meta.path.is_ident("module") {
                        let value: LitStr = meta.value()?.parse()?;
                        options.verus_module = Some(value.value());
                        return Ok(());
                    }

                    Err(meta.error("unsupported witness verus attribute"))
                })?;

                return Ok(());
            }

            Err(meta.error("unsupported witness container attribute"))
        })?;
    }

    Ok(options)
}

#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(attrs)))]
pub(crate) fn parse_member_options(attrs: &[syn::Attribute]) -> syn::Result<MemberOptions> {
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
