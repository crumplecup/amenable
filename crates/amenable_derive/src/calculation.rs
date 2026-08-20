//! `#[calculation]` attribute macro: turns a method into a chain link in
//! the evidence graph.

use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{
    Error, Expr, FnArg, ItemFn, MetaNameValue, Pat, Path, ReturnType, Token, Type,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

/// Parsed `#[calculation(token = TokenType)]` arguments.
pub struct CalculationArgs {
    token: Path,
}

impl Parse for CalculationArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut token = None;
        let pairs = Punctuated::<MetaNameValue, Token![,]>::parse_terminated(input)?;

        for pair in pairs {
            if pair.path.is_ident("token") {
                let Expr::Path(expr_path) = &pair.value else {
                    return Err(Error::new_spanned(&pair.value, "expected a type path"));
                };
                token = Some(expr_path.path.clone());
            } else {
                return Err(Error::new_spanned(
                    &pair.path,
                    "unsupported calculation attribute",
                ));
            }
        }

        token.map(|token| CalculationArgs { token }).ok_or_else(|| {
            Error::new(
                Span::call_site(),
                "calculation requires `token = TokenType`",
            )
        })
    }
}

/// Expand a `#[calculation(token = TokenType)]`-decorated function into its
/// evidence-chain wiring.
pub fn expand_calculation(args: &CalculationArgs, input: &ItemFn) -> syn::Result<TokenStream> {
    if !input.sig.generics.params.is_empty() {
        return Err(Error::new_spanned(
            &input.sig.generics,
            "calculation does not yet support generic functions",
        ));
    }

    let fn_name = &input.sig.ident;
    let impl_name = format_ident!("{}_impl", fn_name);
    let evidence_name = format_ident!("{}Evidence", to_pascal_case(&fn_name.to_string()));
    let token = &args.token;
    let vis = &input.vis;
    let attrs = &input.attrs;
    let inputs = &input.sig.inputs;
    let where_clause = &input.sig.generics.where_clause;
    let block = &input.block;

    let output_ty = match &input.sig.output {
        ReturnType::Type(_, ty) => (**ty).clone(),
        ReturnType::Default => {
            return Err(Error::new_spanned(
                &input.sig,
                "calculation requires an explicit return type",
            ));
        }
    };

    let params = inputs
        .iter()
        .map(|arg| match arg {
            FnArg::Typed(pat_type) => Ok(((*pat_type.ty).clone(), (*pat_type.pat).clone())),
            FnArg::Receiver(receiver) => Err(Error::new_spanned(
                receiver,
                "calculation does not support methods that take `self`",
            )),
        })
        .collect::<syn::Result<Vec<(Type, Pat)>>>()?;

    if params.is_empty() {
        return Err(Error::new_spanned(
            &input.sig,
            "calculation requires at least one argument",
        ));
    }

    let (param_types, param_idents): (Vec<Type>, Vec<Pat>) = params.into_iter().unzip();

    // `Basis` is the tuple of parameter *types themselves*, not whatever
    // those types' own bases happen to be — `AddEvidence`'s prior link is
    // `(Debit, Credit)`, one level up from wherever Debit/Credit
    // themselves rest. Walking further back is `(Debit, Credit)`'s own
    // `Evidence` impl's job (`impl_tuple_evidence!`), the next time
    // `chain()` recurses. So `basis_ctor` needs one canonical instance per
    // parameter type — `Default::default()`, the same placeholder a root's
    // own `basis_ctor` defaults to — not `<Param as Evidence>::basis()`,
    // which returns a value of the *wrong type* (`Param::Basis`, not
    // `Param`) the moment a parameter isn't itself a root.
    let (basis_ty, basis_ctor) = if let [only] = param_types.as_slice() {
        (
            quote! { #only },
            quote! { <#only as ::std::default::Default>::default() },
        )
    } else {
        (
            quote! { ( #(#param_types),* ) },
            quote! { ( #(<#param_types as ::std::default::Default>::default()),* ) },
        )
    };

    // One EvidenceLink per parameter, not one link naming the whole tuple
    // as a single composite basis string: a tuple basis is a fan-out to
    // several independent roots/calculations, not one more link in a
    // straight line, and nothing is ever registered under the tuple's own
    // stringified name for a composite link to resolve against. Chain
    // reconstruction (`amenable_core::chain`) walks multiple same-name
    // links as branches, so this is what lets a multi-argument calculation
    // show up as a tree instead of a chain that silently stops one hop in.
    let evidence_link_registrations = param_types.iter().enumerate().map(|(index, param_ty)| {
        quote! {
            ::inventory::submit! {
                ::amenable_core::EvidenceLink {
                    name: concat!(module_path!(), "::", stringify!(#evidence_name)),
                    basis: concat!(module_path!(), "::", stringify!(#param_ty)),
                    index: #index,
                }
            }
        }
    });

    Ok(quote! {
        #(#attrs)*
        fn #impl_name(#inputs) -> #output_ty #where_clause #block

        #[doc = concat!("Evidence generated by `#[calculation]` for `", stringify!(#fn_name), "`.")]
        #vis struct #evidence_name {
            data: #output_ty,
        }

        impl ::amenable_core::Evidence for #evidence_name {
            type Basis = #basis_ty;
            type Audit = #output_ty;

            fn basis() -> Self::Basis {
                #basis_ctor
            }

            fn audit(&self) -> Self::Audit {
                ::std::clone::Clone::clone(&self.data)
            }
        }

        #(#evidence_link_registrations)*

        #(#attrs)*
        #vis fn #fn_name<__Verifier: ::amenable_core::Verifier>(#inputs) -> #evidence_name
        where
            // The evidence type is its own lawful credential (reflexive
            // `ProofToken`), not the bare output value: the output's own
            // constructor is typically public, so holding one never
            // demonstrates the calculation actually ran. Holding
            // `#evidence_name` does, because outside this module it's only
            // reachable by calling this very function.
            #evidence_name: ::amenable_core::Establish<#evidence_name, __Verifier, Token = #token>,
            // Every argument must carry its own witness for the same
            // verifier before this calculation can be proved: access to
            // a calculation's basis is a proof obligation, not something
            // the calculation's own proof can take on faith. This makes
            // it structural — callers can't reach for `add::<V>()`
            // without the chain underneath already being provable.
            #(#param_types: ::amenable_core::Witness<__Verifier>,)*
        {
            #evidence_name {
                data: #impl_name(#(#param_idents),*),
            }
        }
    })
}

fn to_pascal_case(input: &str) -> String {
    input
        .split('_')
        .filter(|segment| !segment.is_empty())
        .map(|segment| {
            let mut chars = segment.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect()
}
