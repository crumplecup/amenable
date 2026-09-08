//! `#[derive(Entry)]`: a vocabulary entry type — one canonically-keyed fact.
//!
//! Emits `impl Entry` (`KEY` + `Value` + `value()`) and `impl Metadata` (the
//! one entry, keyed by `KEY`, holding `self` by clone). `Display` is the type's
//! own responsibility. Container attribute: `#[entry(key = "...", crate =
//! "...")]` (`key` required, `crate` defaults to `amenable_core`).

use quote::quote;
use syn::{Data, DeriveInput, Error, Fields, LitStr, Path, Type, parse_quote};

struct EntryOptions {
    key: String,
    crate_path: Path,
}

#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(attrs)))]
fn parse_options(attrs: &[syn::Attribute]) -> syn::Result<EntryOptions> {
    let mut key = None;
    let mut crate_path: Path = parse_quote!(amenable_core);

    for attr in attrs.iter().filter(|attr| attr.path().is_ident("entry")) {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("key") {
                let value: LitStr = meta.value()?.parse()?;
                key = Some(value.value());
                return Ok(());
            }
            if meta.path.is_ident("crate") {
                let value: LitStr = meta.value()?.parse()?;
                crate_path = value.parse()?;
                return Ok(());
            }
            Err(meta.error("unsupported `entry` container attribute (expected `key` or `crate`)"))
        })?;
    }

    let key = key.ok_or_else(|| {
        Error::new_spanned(
            &attrs[0],
            "`#[derive(Entry)]` needs `#[entry(key = \"...\")]`",
        )
    })?;
    Ok(EntryOptions { key, crate_path })
}

/// The single wrapped field of a newtype, if `data` is a one-field struct.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(data)))]
fn newtype_field(data: &Data) -> Option<(proc_macro2::TokenStream, Type)> {
    let Data::Struct(data) = data else {
        return None;
    };
    match &data.fields {
        Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
            let ty = fields.unnamed[0].ty.clone();
            Some((quote!(self.0), ty))
        }
        Fields::Named(fields) if fields.named.len() == 1 => {
            let field = &fields.named[0];
            let ident = field.ident.as_ref()?;
            Some((quote!(self.#ident), field.ty.clone()))
        }
        _ => None,
    }
}

/// Whether a type path ends in `String` — its `Value` is projected as `str`.
fn is_string(ty: &Type) -> bool {
    matches!(ty, Type::Path(p) if p.path.segments.last().is_some_and(|s| s.ident == "String"))
}

#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(input)))]
pub(crate) fn expand_entry(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let options = parse_options(&input.attrs)?;
    let name = &input.ident;
    let key = &options.key;
    let crate_path = &options.crate_path;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let (value_ty, value_body) = match newtype_field(&input.data) {
        Some((access, ty)) if is_string(&ty) => (quote!(str), quote!(#access.as_str())),
        Some((access, ty)) => (quote!(#ty), quote!(&#access)),
        None => (quote!(Self), quote!(self)),
    };

    Ok(quote! {
        impl #impl_generics ::#crate_path::Entry for #name #ty_generics #where_clause {
            const KEY: &'static str = #key;
            type Value = #value_ty;

            #[cfg_attr(not(kani), ::tracing::instrument(level = "trace", skip(self)))]
            fn value(&self) -> &Self::Value {
                #value_body
            }
        }

        impl #impl_generics ::#crate_path::Metadata for #name #ty_generics #where_clause {
            #[cfg_attr(not(kani), ::tracing::instrument(level = "trace", skip(self)))]
            fn snapshot(&self) -> ::std::vec::Vec<::#crate_path::OwnedEntry> {
                ::std::vec![::#crate_path::OwnedEntry::new(#key, ::core::clone::Clone::clone(self))]
            }
        }
    })
}
