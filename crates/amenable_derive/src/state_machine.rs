//! `#[derive(StateMachine)]`, per `docs/STATE_MACHINE_DERIVATION_PLAN.md`.
//!
//! A derive macro has no type information and cannot read `inventory`
//! registries at expansion time, so it cannot discover which methods on
//! a type are transitions, or infer a `Sidecar<V>` wrapper from context.
//! Every state and edge is declared explicitly:
//!
//! ```ignore
//! #[derive(StateMachine)]
//! #[state_machine(
//!     verifier = "KaniVerifier",
//!     state(name = "Green", carrier = "Established<Green, GreenToken>"),
//!     state(name = "Yellow", carrier = "Established<Yellow, YellowToken>"),
//!     edge(from = "Green", to = "Yellow"),
//! )]
//! struct Stoplight;
//! ```
//!
//! Repeat the whole `#[state_machine(..)]` attribute once per verifier a
//! machine is proven under (Kani, Creusot, Verus), rather than having the
//! macro guess which backends apply. Step 1 emits only compiler-enforced
//! static assertions, one per declared edge, each checking that the real
//! `Exchange<InputCarrier, OutputCarrier, Verifier>` impl already exists
//! for the derived type — the compiler squawks if it doesn't, no macro-
//! side introspection required. Because the carrier is an opaque,
//! caller-supplied type expression, this never needs to know or assume
//! anything about `Established` specifically; any `Sidecar<V>`
//! implementation works identically.
//!
//! Generated aggregate methods (`states()`/`transitions()`), the
//! verifier-generic audit surface, and the runtime cross-check against
//! `ExchangeEdgeRecord` are Step 2, not this one.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, DeriveInput, Error, LitStr, Type};

struct StateDecl {
    name: LitStr,
    carrier: Type,
}

struct EdgeDecl {
    from: LitStr,
    to: LitStr,
}

struct StateMachineBlock {
    verifier: Type,
    states: Vec<StateDecl>,
    edges: Vec<EdgeDecl>,
}

/// Expand `#[derive(StateMachine)]` for a type carrying one or more
/// `#[state_machine(..)]` attributes.
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

    let assertions = blocks
        .iter()
        .map(|block| expand_block_assertions(self_ty, block))
        .collect::<syn::Result<Vec<_>>>()?;

    Ok(quote! { #(#assertions)* })
}

fn expand_block_assertions(
    self_ty: &syn::Ident,
    block: &StateMachineBlock,
) -> syn::Result<TokenStream> {
    let verifier = &block.verifier;

    block
        .edges
        .iter()
        .map(|edge| {
            let from_carrier = find_state_carrier(&block.states, &edge.from)?;
            let to_carrier = find_state_carrier(&block.states, &edge.to)?;

            Ok(quote! {
                const _: fn() = || {
                    fn assert_state_machine_edge<
                        T: ::amenable_core::Exchange<#from_carrier, #to_carrier, #verifier>,
                    >() {
                    }

                    assert_state_machine_edge::<#self_ty>();
                };
            })
        })
        .collect()
}

fn find_state_carrier<'a>(states: &'a [StateDecl], name: &LitStr) -> syn::Result<&'a Type> {
    states
        .iter()
        .find(|state| state.name.value() == name.value())
        .map(|state| &state.carrier)
        .ok_or_else(|| {
            Error::new(
                name.span(),
                format!(
                    "state_machine edge references undeclared state \"{}\"",
                    name.value()
                ),
            )
        })
}

fn parse_state_machine_block(attr: &Attribute) -> syn::Result<StateMachineBlock> {
    let mut verifier: Option<Type> = None;
    let mut states: Vec<StateDecl> = Vec::new();
    let mut edges: Vec<EdgeDecl> = Vec::new();

    attr.parse_nested_meta(|meta| {
        if meta.path.is_ident("verifier") {
            let value: LitStr = meta.value()?.parse()?;
            verifier = Some(value.parse()?);
            return Ok(());
        }

        if meta.path.is_ident("state") {
            states.push(parse_state_decl(&meta)?);
            return Ok(());
        }

        if meta.path.is_ident("edge") {
            edges.push(parse_edge_decl(&meta)?);
            return Ok(());
        }

        Err(meta.error("unsupported state_machine container attribute"))
    })?;

    let verifier = verifier
        .ok_or_else(|| Error::new_spanned(attr, "state_machine requires `verifier = \"...\"`"))?;

    Ok(StateMachineBlock {
        verifier,
        states,
        edges,
    })
}

fn parse_state_decl(meta: &syn::meta::ParseNestedMeta) -> syn::Result<StateDecl> {
    let mut name: Option<LitStr> = None;
    let mut carrier: Option<Type> = None;

    meta.parse_nested_meta(|inner| {
        if inner.path.is_ident("name") {
            let value: LitStr = inner.value()?.parse()?;
            name = Some(value);
            return Ok(());
        }

        if inner.path.is_ident("carrier") {
            let value: LitStr = inner.value()?.parse()?;
            carrier = Some(value.parse()?);
            return Ok(());
        }

        Err(inner.error("unsupported state_machine state attribute"))
    })?;

    Ok(StateDecl {
        name: name.ok_or_else(|| meta.error("state requires `name = \"...\"`"))?,
        carrier: carrier.ok_or_else(|| meta.error("state requires `carrier = \"...\"`"))?,
    })
}

fn parse_edge_decl(meta: &syn::meta::ParseNestedMeta) -> syn::Result<EdgeDecl> {
    let mut from: Option<LitStr> = None;
    let mut to: Option<LitStr> = None;

    meta.parse_nested_meta(|inner| {
        if inner.path.is_ident("from") {
            let value: LitStr = inner.value()?.parse()?;
            from = Some(value);
            return Ok(());
        }

        if inner.path.is_ident("to") {
            let value: LitStr = inner.value()?.parse()?;
            to = Some(value);
            return Ok(());
        }

        Err(inner.error("unsupported state_machine edge attribute"))
    })?;

    Ok(EdgeDecl {
        from: from.ok_or_else(|| meta.error("edge requires `from = \"...\"`"))?,
        to: to.ok_or_else(|| meta.error("edge requires `to = \"...\"`"))?,
    })
}
