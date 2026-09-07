//! Shared `#[metadata(..)]` / `#[provenance(..)]` / `#[witness(..)]` attribute
//! parsing and field-type collection, used by `#[derive(Metadata)]`
//! (`metadata`), `#[derive(Provenance)]` (`provenance`) and `#[derive(Witness)]`
//! (`witness`).

use derive_getters::Getters;
use syn::{Data, Error, Field, Fields, LitStr, Path, Type, parse_quote};

/// Whether an attribute is one of the two interchangeable schema attributes
/// (`#[metadata(..)]` on a `#[derive(Metadata)]`, `#[provenance(..)]` on a
/// `#[derive(Provenance)]`).
fn is_schema_attr(attr: &syn::Attribute) -> bool {
    attr.path().is_ident("metadata") || attr.path().is_ident("provenance")
}

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

/// How `#[derive(Metadata)]` / `#[derive(Provenance)]` projects one field.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub(crate) enum EntryKind {
    /// No `#[entry(..)]` attribute: legacy behaviour — recurse via the
    /// field's `Metadata::snapshot()` and re-key `"value"` to the field name,
    /// else prefix with `"<field>."`. Requires `FieldTy: Metadata`.
    #[default]
    Bare,
    /// `#[entry]` — the field is a `MetadataValue` leaf: one entry keyed by
    /// the field name. Requires `FieldTy: MetadataValue + Clone`.
    Leaf,
    /// `#[entry(nested)]` — the field is a sub-record: its entries are
    /// spliced in under a `"<field>."` prefix. Requires `FieldTy: Metadata`.
    Nested,
    /// `#[entry(flatten)]` — a sub-record spliced with no prefix. Requires
    /// `FieldTy: Metadata`.
    Flatten,
}

#[derive(Default, Getters)]
pub(crate) struct MemberOptions {
    rename: Option<String>,
    skip: bool,
    kind: EntryKind,
}

/// Every non-`skip` field's type paired with the [`EntryKind`] that governs
/// which trait bound `#[derive(Metadata)]` needs on it.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(data)))]
pub(crate) fn collect_field_bounds(data: &Data) -> syn::Result<Vec<(Type, EntryKind)>> {
    match data {
        Data::Struct(data) => collect_field_bounds_from_fields(&data.fields),
        Data::Enum(data) => data
            .variants
            .iter()
            .map(|variant| collect_field_bounds_from_fields(&variant.fields))
            .collect::<syn::Result<Vec<_>>>()
            .map(|groups| groups.into_iter().flatten().collect()),
        Data::Union(data) => Err(Error::new_spanned(
            data.union_token,
            "Metadata can only be derived for structs and enums",
        )),
    }
}

/// Every non-`skip` field's type (kind-agnostic), used by `#[derive(Witness)]`.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(fields)))]
pub(crate) fn collect_field_types_from_fields(fields: &Fields) -> syn::Result<Vec<Type>> {
    Ok(collect_field_bounds_from_fields(fields)?
        .into_iter()
        .map(|(ty, _)| ty)
        .collect())
}

#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(fields)))]
fn collect_field_bounds_from_fields(fields: &Fields) -> syn::Result<Vec<(Type, EntryKind)>> {
    let fields = match fields {
        Fields::Named(fields) => &fields.named,
        Fields::Unnamed(fields) => &fields.unnamed,
        Fields::Unit => return Ok(Vec::new()),
    };

    fields
        .iter()
        .map(|field| {
            let options = parse_member_options(&field.attrs)?;
            if options.skip {
                return Ok(None);
            }
            Ok(Some((field.ty.clone(), options.kind)))
        })
        .filter_map(Result::transpose)
        .collect()
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

    for attr in attrs.iter().filter(|attr| is_schema_attr(attr)) {
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

    // `#[provenance(..)]` / `#[metadata(..)]` — legacy: `rename` / `skip` only,
    // `kind` stays `Bare`.
    for attr in attrs.iter().filter(|attr| is_schema_attr(attr)) {
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

    // `#[entry]` / `#[entry(nested | flatten | skip | rename = "..")]` — the
    // explicit leaf/sub-record model. Any `#[entry]` attribute opts the field
    // into it, defaulting to `Leaf`.
    for attr in attrs.iter().filter(|attr| attr.path().is_ident("entry")) {
        options.kind = EntryKind::Leaf;

        if matches!(attr.meta, syn::Meta::Path(_)) {
            continue;
        }

        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("nested") {
                if options.kind != EntryKind::Flatten {
                    options.kind = EntryKind::Nested;
                }
                return Ok(());
            }

            if meta.path.is_ident("flatten") {
                options.kind = EntryKind::Flatten;
                return Ok(());
            }

            if meta.path.is_ident("skip") {
                options.skip = true;
                return Ok(());
            }

            if meta.path.is_ident("rename") {
                let value: LitStr = meta.value()?.parse()?;
                options.rename = Some(value.value());
                return Ok(());
            }

            Err(meta.error(
                "unsupported `entry` attribute (expected `nested`, \
                            `flatten`, `skip`, or `rename = \"..\"`)",
            ))
        })?;
    }

    Ok(options)
}
