//! `#[derive(KaniCompose)]`: generate bounded Kani-facing constructors by
//! delegating every field to its own `KaniCompose` implementation.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataEnum, DataStruct, DeriveInput, Error, Fields, Type, parse_quote};

/// Expand `#[derive(KaniCompose)]` for structs and enums.
pub fn expand_kani_compose(input: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &input.ident;
    let mut generics = input.generics.clone();
    let where_clause = generics.make_where_clause();

    for field_type in collect_field_types(&input.data) {
        where_clause
            .predicates
            .push(parse_quote!(#field_type: ::amenable_kani::KaniCompose));
    }

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let (depth0, depth1, depth2, any) = match &input.data {
        Data::Struct(data) => (
            struct_ctor(data, compose_method("kani_depth0"))?,
            struct_ctor(data, compose_method("kani_depth1"))?,
            struct_ctor(data, compose_method("kani_depth2"))?,
            struct_ctor(data, compose_method("kani_any"))?,
        ),
        Data::Enum(data) => (
            enum_ctor(data, compose_method("kani_depth0"), true)?,
            enum_ctor(data, compose_method("kani_depth1"), true)?,
            enum_ctor(data, compose_method("kani_depth2"), true)?,
            enum_ctor(data, compose_method("kani_depth2"), false)?,
        ),
        Data::Union(data) => {
            return Err(Error::new_spanned(
                data.union_token,
                "KaniCompose can only be derived for structs and enums",
            ));
        }
    };

    Ok(quote! {
        // `#[cfg(kani)]` here is meaningless to any downstream consumer
        // that never declares the `kani` cfg name -- the `allow`+`const`
        // wrapper is the only placement that suppresses `unexpected_cfgs`
        // there without affecting what actually compiles under a real
        // `kani` build (confirmed against this exact generic/where-clause
        // shape in an isolated scratch crate before landing here; see
        // `docs/CFG_HYGIENE_PLAN.md`'s Step 1).
        #[allow(unexpected_cfgs)]
        const _: () = {
            #[cfg(kani)]
            impl #impl_generics ::amenable_kani::KaniCompose for #name #ty_generics #where_clause {
                fn kani_depth0() -> Self {
                    #depth0
                }

                fn kani_depth1() -> Self {
                    #depth1
                }

                fn kani_depth2() -> Self {
                    #depth2
                }

                fn kani_any() -> Self {
                    #any
                }
            }
        };
    })
}

fn collect_field_types(data: &Data) -> Vec<Type> {
    let mut field_types = Vec::new();

    match data {
        Data::Struct(data) => field_types.extend(data.fields.iter().map(|field| field.ty.clone())),
        Data::Enum(data) => {
            for variant in &data.variants {
                field_types.extend(variant.fields.iter().map(|field| field.ty.clone()));
            }
        }
        Data::Union(_) => {}
    }

    field_types
}

fn struct_ctor(data: &DataStruct, method: syn::Ident) -> syn::Result<TokenStream> {
    fields_ctor(&data.fields, method, None)
}

fn enum_ctor(data: &DataEnum, method: syn::Ident, fixed_variant: bool) -> syn::Result<TokenStream> {
    let variants = data.variants.iter().collect::<Vec<_>>();
    if variants.is_empty() {
        return Err(Error::new_spanned(
            data.enum_token,
            "KaniCompose requires at least one enum variant",
        ));
    }

    let default_index = variants
        .iter()
        .position(|variant| {
            variant
                .attrs
                .iter()
                .any(|attr| attr.path().is_ident("default"))
        })
        .unwrap_or(0);

    if fixed_variant {
        return variant_ctor(variants[default_index], method, None);
    }

    let selector_arms = variants
        .iter()
        .enumerate()
        .map(|(index, variant)| {
            let ctor = variant_ctor(variant, method.clone(), None)?;
            Ok(quote! { #index => #ctor, })
        })
        .collect::<syn::Result<Vec<_>>>()?;
    let variant_count = variants.len();

    Ok(quote! {{
        let selector: usize = kani::any();
        kani::assume(selector < #variant_count);

        match selector {
            #(#selector_arms)*
            _ => unreachable!("selector is bounded by kani::assume"),
        }
    }})
}

fn variant_ctor(
    variant: &syn::Variant,
    method: syn::Ident,
    self_ty: Option<&syn::Ident>,
) -> syn::Result<TokenStream> {
    let variant_ident = &variant.ident;
    let prefix = match self_ty {
        Some(self_ty) => quote!(#self_ty::#variant_ident),
        None => quote!(Self::#variant_ident),
    };
    fields_ctor(&variant.fields, method, Some(prefix))
}

fn fields_ctor(
    fields: &Fields,
    method: syn::Ident,
    prefix: Option<TokenStream>,
) -> syn::Result<TokenStream> {
    match fields {
        Fields::Named(fields) => {
            let members = fields
                .named
                .iter()
                .map(|field| {
                    let ident = field.ident.as_ref().ok_or_else(|| {
                        Error::new_spanned(field, "named-field expansion requires identifiers")
                    })?;
                    let ty = &field.ty;
                    Ok(quote!(#ident: <#ty as ::amenable_kani::KaniCompose>::#method()))
                })
                .collect::<syn::Result<Vec<TokenStream>>>()?;
            let head = prefix.unwrap_or_else(|| quote!(Self));
            Ok(quote!(#head { #(#members,)* }))
        }
        Fields::Unnamed(fields) => {
            let members = fields.unnamed.iter().map(|field| {
                let ty = &field.ty;
                quote!(<#ty as ::amenable_kani::KaniCompose>::#method())
            });
            let head = prefix.unwrap_or_else(|| quote!(Self));
            Ok(quote!(#head ( #(#members,)* )))
        }
        Fields::Unit => Ok(match prefix {
            Some(prefix) => quote!(#prefix),
            None => quote!(Self),
        }),
    }
}

fn compose_method(name: &str) -> syn::Ident {
    syn::Ident::new(name, proc_macro2::Span::call_site())
}
