//! `#[evidence]` attribute macro: computes `is_root` from a hand-written
//! `impl Evidence` block's own `Basis` declaration, at compile time. Purely
//! syntactic — compares the `Basis` item's tokens against the literal
//! `Self`, so it needs no `TypeId` and no `'static` bound.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Error, ImplItem, ImplItemFn, ItemImpl, Type, parse_quote};

/// Expand `#[evidence]` on an `impl Evidence for ...` block.
pub fn expand_evidence(mut input: ItemImpl) -> syn::Result<TokenStream> {
    let already_has_is_root = input
        .items
        .iter()
        .any(|item| matches!(item, ImplItem::Fn(function) if function.sig.ident == "is_root"));

    if !already_has_is_root {
        let basis_ty = find_basis_type(&input)?;
        let is_root = is_literally_self(basis_ty);

        let method: ImplItemFn = parse_quote! {
            fn is_root() -> bool {
                #is_root
            }
        };

        input.items.push(ImplItem::Fn(method));
    }

    Ok(quote! { #input })
}

fn find_basis_type(input: &ItemImpl) -> syn::Result<&Type> {
    input
        .items
        .iter()
        .find_map(|item| match item {
            ImplItem::Type(item_type) if item_type.ident == "Basis" => Some(&item_type.ty),
            _ => None,
        })
        .ok_or_else(|| Error::new_spanned(input, "evidence requires a `type Basis = ...` item"))
}

/// Whether a `Basis`/`type` reference is literally the token `Self` —
/// shared with the `Standard` derive, which computes `is_root` the same
/// syntactic way.
pub(crate) fn is_literally_self(ty: &Type) -> bool {
    matches!(
        ty,
        Type::Path(type_path) if type_path.qself.is_none() && type_path.path.is_ident("Self")
    )
}
