//! `#[derive(Standard)]`: generates both `Standard` and `Evidence` impls
//! from a `#[standard(...)]` attribute.
//!
//! Every hand-written `Standard` we've built duplicates the same
//! provenance value as both `Standard::Provenance`/`provenance()` and
//! `Evidence::Audit`/`audit()`. This derive collapses that duplication,
//! computing `basis()`/`is_root()` the same syntactic way `#[evidence]`
//! does — no `TypeId`, no `'static`.

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{DeriveInput, Error, Expr, LitStr, Type, WherePredicate, parse_quote};

use crate::evidence::is_literally_self;

struct StandardArgs {
    basis: Type,
    basis_ctor: Option<Expr>,
    provenance: Option<Expr>,
    provenance_type: Option<Type>,
    bounds: Vec<WherePredicate>,
}

/// Expand `#[derive(Standard)]` for a struct carrying `#[standard(...)]`.
pub fn expand_standard(input: &DeriveInput) -> syn::Result<TokenStream> {
    let args = parse_standard_args(&input.attrs)?;
    let name = &input.ident;

    let basis_ty = &args.basis;
    let is_root = is_literally_self(basis_ty);
    let provenance_ty = args
        .provenance_type
        .clone()
        .unwrap_or_else(|| parse_quote!(Self));
    let provenance_expr = args
        .provenance
        .clone()
        .unwrap_or_else(|| parse_quote!(::std::clone::Clone::clone(self)));
    // `basis()` returns a canonical instance of `#basis_ty` itself — the
    // same kind of placeholder a root defaults to for itself — not
    // `<#basis_ty as Evidence>::basis()`, which is one level too far
    // (that's `#basis_ty`'s own basis, not `#basis_ty`). Recursing further
    // back is the *next* `chain()`/`proof_chain()` step's job, once it
    // reaches `#basis_ty` and asks in turn.
    let basis_ctor: Expr = args
        .basis_ctor
        .clone()
        .unwrap_or_else(|| parse_quote!(<#basis_ty as ::std::default::Default>::default()));

    let mut generics = input.generics.clone();
    {
        let where_clause = generics.make_where_clause();
        // The default `provenance_expr`/`basis_ctor` above call `Clone::
        // clone(self)`/`Default::default()` unconditionally inside the
        // generated impl body. For every non-generic `Standard` (every
        // use before a generic one came along) that's invisible: a
        // concrete type either has `Clone`/`Default` or it doesn't, and
        // every one so far does. A *generic* `Standard` exposes the real
        // gap: without an explicit `Self: Clone`/`#basis_ty: Default`
        // bound on the generated impl itself, those calls don't
        // typecheck for any `T` that doesn't happen to make them hold.
        // So add exactly the bound the chosen default actually needs,
        // and only when that default is actually used -- a custom
        // `provenance`/`basis_ctor` expression is the caller's own
        // responsibility, via `bound = ".."`.
        if args.provenance.is_none() {
            where_clause
                .predicates
                .push(parse_quote!(Self: ::std::clone::Clone));
        }
        if args.basis_ctor.is_none() {
            where_clause
                .predicates
                .push(parse_quote!(#basis_ty: ::std::default::Default));
        }
        where_clause.predicates.extend(args.bounds.clone());
    }
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Auto-registration only covers root, non-generic standards: a root's
    // basis is itself, so `stringify!(#name)` alone names both sides of the
    // link. A basis naming another `Evidence` type would need that type's
    // own qualifying module path, which this macro has no way to know: it
    // never resolves paths, only manipulates tokens. Generic standards hit
    // the same wall inventory itself has — one `inventory::submit!` per
    // concrete instantiation, not one for the whole generic family — so
    // those stay on manual, explicit registration too.
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

        impl #impl_generics ::amenable_core::Standard for #name #ty_generics #where_clause {
            type Provenance = #provenance_ty;

            fn provenance(&self) -> Self::Provenance {
                #provenance_expr
            }
        }

        impl #impl_generics ::amenable_core::Evidence for #name #ty_generics #where_clause {
            type Basis = #basis_ty;
            type Audit = #provenance_ty;

            fn basis() -> Self::Basis {
                #basis_ctor
            }

            fn audit(&self) -> Self::Audit {
                #provenance_expr
            }

            fn is_root() -> bool {
                #is_root
            }
        }
    })
}

fn parse_standard_args(attrs: &[syn::Attribute]) -> syn::Result<StandardArgs> {
    let mut basis = None;
    let mut basis_ctor = None;
    let mut provenance = None;
    let mut provenance_type = None;
    let mut bounds = Vec::new();

    for attr in attrs.iter().filter(|attr| attr.path().is_ident("standard")) {
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

            if meta.path.is_ident("provenance") {
                let value: LitStr = meta.value()?.parse()?;
                provenance = Some(value.parse()?);
                return Ok(());
            }

            if meta.path.is_ident("provenance_type") {
                let value: LitStr = meta.value()?.parse()?;
                provenance_type = Some(value.parse()?);
                return Ok(());
            }

            if meta.path.is_ident("bound") {
                let value: LitStr = meta.value()?.parse()?;
                bounds.push(value.parse()?);
                return Ok(());
            }

            Err(meta.error("unsupported standard container attribute"))
        })?;
    }

    let basis = basis
        .ok_or_else(|| Error::new(Span::call_site(), "standard requires `basis = \"...\"`"))?;

    Ok(StandardArgs {
        basis,
        basis_ctor,
        provenance,
        provenance_type,
        bounds,
    })
}
