//! Two macros over the same trivial fact: whether a hand- or
//! derive-generated `impl Evidence` block's own `Basis` is literally
//! `Self` (a root, its own basis) — computed purely syntactically, by
//! comparing the `Basis` item's tokens against the literal `Self`, so
//! neither needs a `TypeId` or a `'static` bound.
//!
//! - **`#[evidence]`** (attribute): for a **hand-written** `impl Evidence
//!   for X { .. }` block whose `basis()`/`audit()` bodies are real,
//!   non-trivial content — computes `is_root()` only, leaving the rest
//!   exactly as authored.
//! - **`#[derive(Evidence)]`** (derive, attrs: `#[evidence(basis = "..",
//!   basis_ctor = "..", bound = "..")]`): for the common case this
//!   crate's own worked examples kept hand-typing regardless —
//!   `GAAP_LEDGER_PLAN.md`'s five contract types (`AmountPositive`/
//!   `SufficientFunds`/`AccountsDistinct`/`BalancedEntries`/
//!   `AccountingEquationHolds`) and `TransferPayload` were each a
//!   byte-for-byte identical `impl Evidence { type Basis = Self; type
//!   Audit = (); fn basis() -> Self::Basis { Self::default() }; fn
//!   audit(&self) {} }`, with only `#[evidence]`'s own `is_root()`
//!   generated — real, provable-not-asserted evidence, but every other
//!   line was exactly the same mechanical shape `#[derive(Standard)]`
//!   already collapses for the asserted-root case. This derive is that
//!   same collapse, minus the `Standard`/`Provenance` half (a provable
//!   claim has no citation to audit — `Audit` is unconditionally `()`).
//!   Also closes a real gap the hand-written form left open: an
//!   `EvidenceLink` registration, the same auto-registration
//!   `#[derive(Standard)]` already does for its own root, non-generic
//!   case — none of these six types had one, so none were discoverable
//!   through the evidence-chain registry at all before this derive
//!   existed.

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    DeriveInput, Error, Expr, ImplItem, ImplItemFn, ItemImpl, LitStr, Type, WherePredicate,
    parse_quote,
};

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

struct EvidenceDeriveArgs {
    basis: Type,
    basis_ctor: Option<Expr>,
    bounds: Vec<WherePredicate>,
}

/// Expand `#[derive(Evidence)]` for a struct carrying `#[evidence(..)]`.
/// See this module's own doc comment for the full rationale; mirrors
/// `#[derive(Standard)]`'s own `basis`/`basis_ctor`/`bound` handling
/// exactly (including the identical default-bound-only-when-the-
/// default's-actually-used discipline for generic types), just without
/// the `Standard`/`Provenance` half, and with `Audit` fixed to `()` —
/// a provable claim has no citation to audit.
pub fn expand_evidence_derive(input: &DeriveInput) -> syn::Result<TokenStream> {
    let args = parse_evidence_derive_args(&input.attrs)?;
    let name = &input.ident;

    let basis_ty = &args.basis;
    let is_root = is_literally_self(basis_ty);
    let basis_ctor: Expr = args
        .basis_ctor
        .clone()
        .unwrap_or_else(|| parse_quote!(<#basis_ty as ::std::default::Default>::default()));

    let mut generics = input.generics.clone();
    {
        let where_clause = generics.make_where_clause();
        if args.basis_ctor.is_none() {
            where_clause
                .predicates
                .push(parse_quote!(#basis_ty: ::std::default::Default));
        }
        where_clause.predicates.extend(args.bounds.clone());
    }
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // See `#[derive(Standard)]`'s own identical registration for why this
    // is root-and-non-generic only: a root's basis is itself, so
    // `stringify!(#name)` alone names both sides of the link, and a
    // generic type needs one registration per concrete instantiation
    // (the same wall `inventory` itself has), which this macro has no
    // way to enumerate.
    let evidence_link_registration = if is_root && input.generics.params.is_empty() {
        quote! {
            ::inventory::submit! {
                ::amenable_core::EvidenceLink {
                    name: concat!(module_path!(), "::", stringify!(#name)),
                    basis: concat!(module_path!(), "::", stringify!(#name)),
                    index: 0,
                }
            }
        }
    } else {
        TokenStream::new()
    };

    Ok(quote! {
        #evidence_link_registration

        impl #impl_generics ::amenable_core::Evidence for #name #ty_generics #where_clause {
            type Basis = #basis_ty;
            type Audit = ();

            fn basis() -> Self::Basis {
                #basis_ctor
            }

            fn audit(&self) {}

            fn is_root() -> bool {
                #is_root
            }
        }
    })
}

fn parse_evidence_derive_args(attrs: &[syn::Attribute]) -> syn::Result<EvidenceDeriveArgs> {
    let mut basis = None;
    let mut basis_ctor = None;
    let mut bounds = Vec::new();

    for attr in attrs.iter().filter(|attr| attr.path().is_ident("evidence")) {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("basis") {
                let value: LitStr = meta.value()?.parse()?;
                basis = Some(value.parse()?);
                return Ok(());
            }

            if meta.path.is_ident("basis_ctor") {
                let value: LitStr = meta.value()?.parse()?;
                basis_ctor = Some(value.parse()?);
                return Ok(());
            }

            if meta.path.is_ident("bound") {
                let value: LitStr = meta.value()?.parse()?;
                bounds.push(value.parse()?);
                return Ok(());
            }

            Err(meta.error("unsupported evidence container attribute"))
        })?;
    }

    let basis = basis
        .ok_or_else(|| Error::new(Span::call_site(), "evidence requires `basis = \"...\"`"))?;

    Ok(EvidenceDeriveArgs {
        basis,
        basis_ctor,
        bounds,
    })
}
