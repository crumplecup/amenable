//! `#[derive(Sidecar)]`: generates `impl Sidecar<V> for X { .. }` (plus a
//! constructor) from a `#[sidecar(verifier = "..")]` attribute and two
//! field markers (`#[sidecar(primary)]`/`#[sidecar(token)]`).
//!
//! Every carrier type built so far for an `Exchange`-shaped proof
//! (`amenable_kani::stoplight::Established<T, Token>`,
//! `amenable_kani::ledger::Transfer<S, Token>`, and their per-backend
//! accommodation-model mirrors) is the identical shape: a primary payload
//! field, a token field, an optional phantom proposition marker, and a
//! `Sidecar<V>` impl whose body is always `&self.<primary>`/
//! `self.<token>.clone()`. Collapses that duplication the same way
//! `#[derive(ProofToken)]` collapses the one-line `ProofToken` shape.
//!
//! `proposition` defaults to the primary field's own type (`Established<T,
//! Token>`'s shape, where a state IS its whole payload); set
//! `#[sidecar(proposition = "S")]` when the proposition is a separate,
//! phantom generic parameter instead (`Transfer<S, Token>`'s shape, where
//! `S` never appears as data). Either way, a `Evidence`/`Witness<V>` bound
//! is only added to the where-clause for generic parameters actually
//! declared on the struct -- a *concrete* primary type (`TransferPayload`)
//! needs no such bound restated here; its own `Evidence` impl already
//! exists unconditionally.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    DeriveInput, Error, Field, Fields, GenericParam, Ident, LitStr, Type, Visibility,
    WherePredicate, parse_quote,
};

struct SidecarArgs {
    verifier: Type,
    proposition: Option<Type>,
    constructor: Visibility,
}

pub fn expand_sidecar(input: &DeriveInput) -> syn::Result<TokenStream> {
    let args = parse_sidecar_args(&input.attrs)?;
    let name = &input.ident;

    let Fields::Named(fields) = struct_fields(input)? else {
        return Err(Error::new_spanned(
            name,
            "Sidecar can only be derived for a struct with named fields",
        ));
    };

    let primary_field = find_marked_field(fields, "primary")?;
    let token_field = find_marked_field(fields, "token")?;
    let phantom_fields: Vec<&Field> = fields
        .named
        .iter()
        .filter(|field| !std::ptr::eq(*field, primary_field) && !std::ptr::eq(*field, token_field))
        .collect();
    for field in &phantom_fields {
        require_phantom_data(field)?;
    }

    let primary_ident = primary_field.ident.as_ref().expect("named field");
    let primary_ty = &primary_field.ty;
    let token_ident = token_field.ident.as_ref().expect("named field");
    let token_ty = &token_field.ty;
    let proposition_ty = args
        .proposition
        .clone()
        .unwrap_or_else(|| primary_ty.clone());
    let verifier = &args.verifier;

    let mut generics = input.generics.clone();
    let evidence_predicate = evidence_bound(&generics, primary_ty, &proposition_ty, verifier);
    let token_predicate: WherePredicate = parse_quote!(#token_ty: ::amenable_core::ProofToken<Proposition = #proposition_ty> + ::std::clone::Clone);
    {
        let where_clause = generics.make_where_clause();
        where_clause.predicates.push(evidence_predicate);
        where_clause.predicates.push(token_predicate);
    }
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let sidecar_impl = quote! {
        impl #impl_generics ::amenable_core::Sidecar<#verifier> for #name #ty_generics #where_clause {
            type Primary = #primary_ty;
            type Proposition = #proposition_ty;
            type SidecarToken = #token_ty;

            fn primary(&self) -> &Self::Primary {
                &self.#primary_ident
            }

            fn sidecar(&self) -> Self::SidecarToken {
                ::std::clone::Clone::clone(&self.#token_ident)
            }
        }
    };

    let constructor_vis = &args.constructor;
    let phantom_idents: Vec<&Ident> = phantom_fields
        .iter()
        .map(|field| field.ident.as_ref().expect("named field"))
        .collect();
    let (struct_impl_generics, struct_ty_generics, struct_where_clause) =
        input.generics.split_for_impl();

    let constructor_impl = quote! {
        impl #struct_impl_generics #name #struct_ty_generics #struct_where_clause {
            #constructor_vis fn new(#primary_ident: #primary_ty, #token_ident: #token_ty) -> Self {
                Self {
                    #primary_ident,
                    #token_ident,
                    #(#phantom_idents: ::core::marker::PhantomData,)*
                }
            }
        }
    };

    Ok(quote! {
        #sidecar_impl
        #constructor_impl
    })
}

/// Only add an `Evidence`/`Witness<V>` bound for a type that's actually a
/// generic parameter of this struct -- a concrete primary type
/// (`TransferPayload`) needs no bound restated here at all.
fn evidence_bound(
    generics: &syn::Generics,
    primary_ty: &Type,
    proposition_ty: &Type,
    verifier: &Type,
) -> WherePredicate {
    let is_generic = |ty: &Type| {
        generics.params.iter().any(|param| match param {
            GenericParam::Type(type_param) => {
                matches!(ty, Type::Path(path) if path.path.is_ident(&type_param.ident))
            }
            _ => false,
        })
    };

    if types_match(primary_ty, proposition_ty) {
        // Established<T, Token>'s shape: one generic parameter plays both
        // roles, so it needs both bounds together.
        parse_quote!(#proposition_ty: ::amenable_core::Evidence + ::amenable_core::Witness<#verifier>)
    } else if is_generic(proposition_ty) {
        // Transfer<S, Token>'s shape: the proposition is a separate,
        // phantom generic parameter -- the (possibly concrete) primary
        // type only needs `Evidence` via its own real impl, not restated
        // here.
        parse_quote!(#proposition_ty: ::amenable_core::Evidence + ::amenable_core::Witness<#verifier>)
    } else {
        parse_quote!(#primary_ty: ::amenable_core::Evidence)
    }
}

/// `syn::Type` carries no `PartialEq` impl without the `extra-traits`
/// feature (not enabled here) -- compare by token text instead, the
/// common workaround for this exact situation.
fn types_match(left: &Type, right: &Type) -> bool {
    quote!(#left).to_string() == quote!(#right).to_string()
}

fn struct_fields(input: &DeriveInput) -> syn::Result<&Fields> {
    match &input.data {
        syn::Data::Struct(data) => Ok(&data.fields),
        _ => Err(Error::new_spanned(
            &input.ident,
            "Sidecar can only be derived for a struct",
        )),
    }
}

fn find_marked_field<'a>(fields: &'a syn::FieldsNamed, marker: &str) -> syn::Result<&'a Field> {
    let mut found = None;
    for field in &fields.named {
        if has_marker(field, marker)? {
            if found.is_some() {
                return Err(Error::new_spanned(
                    field,
                    format!("only one field may carry #[sidecar({marker})]"),
                ));
            }
            found = Some(field);
        }
    }

    found.ok_or_else(|| {
        Error::new_spanned(
            &fields.named,
            format!("Sidecar requires exactly one field marked #[sidecar({marker})]"),
        )
    })
}

fn has_marker(field: &Field, marker: &str) -> syn::Result<bool> {
    let mut found = false;
    for attr in field
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident("sidecar"))
    {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident(marker) {
                found = true;
                return Ok(());
            }
            if meta.path.is_ident("primary") || meta.path.is_ident("token") {
                return Ok(());
            }
            Err(meta.error("unsupported sidecar field attribute"))
        })?;
    }
    Ok(found)
}

fn require_phantom_data(field: &Field) -> syn::Result<()> {
    let is_phantom = matches!(
        &field.ty,
        Type::Path(path)
            if path.path.segments.last().is_some_and(|segment| segment.ident == "PhantomData")
    );

    if is_phantom {
        Ok(())
    } else {
        Err(Error::new_spanned(
            field,
            "Sidecar only supports a third field when it's a std::marker::PhantomData proposition marker",
        ))
    }
}

fn parse_sidecar_args(attrs: &[syn::Attribute]) -> syn::Result<SidecarArgs> {
    let mut verifier = None;
    let mut proposition = None;
    let mut constructor = None;

    for attr in attrs.iter().filter(|attr| attr.path().is_ident("sidecar")) {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("verifier") {
                let value: LitStr = meta.value()?.parse()?;
                verifier = Some(value.parse()?);
                return Ok(());
            }
            if meta.path.is_ident("proposition") {
                let value: LitStr = meta.value()?.parse()?;
                proposition = Some(value.parse()?);
                return Ok(());
            }
            if meta.path.is_ident("constructor") {
                let value: LitStr = meta.value()?.parse()?;
                constructor = Some(value.parse()?);
                return Ok(());
            }

            Err(meta.error("unsupported sidecar container attribute"))
        })?;
    }

    Ok(SidecarArgs {
        verifier: verifier.ok_or_else(|| {
            Error::new(
                proc_macro2::Span::call_site(),
                "sidecar requires `verifier = \"...\"`",
            )
        })?,
        proposition,
        constructor: constructor.unwrap_or(Visibility::Public(Default::default())),
    })
}
