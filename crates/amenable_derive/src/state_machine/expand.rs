use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Error};

use super::decl::StateMachineBlock;
use super::emit::{expand_block_assertions, expand_block_state_machine_impl};
use super::parse::parse_state_machine_block;

/// Expand `#[derive(StateMachine)]` for a type carrying one or more
/// `#[state_machine(..)]` attributes.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(input)))]
pub fn expand_state_machine(input: &DeriveInput) -> syn::Result<TokenStream> {
    let self_ty = &input.ident;

    let blocks = input
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident("state_machine"))
        .map(parse_state_machine_block)
        .collect::<syn::Result<Vec<_>>>()?;

    if blocks.is_empty() {
        return Err(Error::new_spanned(
            self_ty,
            "derive(StateMachine) requires at least one #[state_machine(..)] attribute",
        ));
    }

    let expansions = blocks
        .iter()
        .map(|block| expand_block(self_ty, block))
        .collect::<syn::Result<Vec<_>>>()?;

    Ok(quote! { #(#expansions)* })
}

#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self_ty, block)))]
fn expand_block(self_ty: &syn::Ident, block: &StateMachineBlock) -> syn::Result<TokenStream> {
    let assertions = expand_block_assertions(self_ty, block)?;
    let state_machine_impl = expand_block_state_machine_impl(self_ty, block);

    Ok(quote! {
        #assertions
        #state_machine_impl
    })
}
