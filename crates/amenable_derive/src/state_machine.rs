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
//!     state("Green", "Established<Green, GreenToken>"),
//!     state("Yellow", "Established<Yellow, YellowToken>"),
//!     edge("Green", "Yellow"),
//! )]
//! struct Stoplight;
//! ```
//!
//! `state(name, carrier)`/`edge(from, to)` are positional, not `key =
//! "value"` pairs the way every other macro in this family writes its
//! own arguments — a real, confirmed constraint, not a style choice:
//! `clippy::duplicated_attributes` (clippy 1.97) flags two `edge(from =
//! "Pending", to = ..)` entries sharing the same `from` value as
//! duplicate attributes, comparing only the first key-value pair
//! regardless of the rest — a real false positive for any state with
//! more than one outgoing edge, reproduced in isolation before deciding
//! this wasn't fixable by reordering fields. Positional args don't
//! trigger it at all (confirmed the same way), so that's the shape here.
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
//! **`#[state_machine(generic_over_verifier, ..)]`** (mutually exclusive
//! with `verifier = "..."`): for a type whose `Exchange<Input, Output,
//! V>` impls are themselves generic over `V` (`amenable_gaap::Ledger`'s
//! real shape, once `#[capture_exchange_body(..)]` started generating
//! `impl<V: Verifier> Exchange<..> for Self` — see that macro's own doc
//! comment). A concrete `verifier = "KaniVerifier"` block can't apply to
//! a type like `Ledger`: it lives in a neutral crate with no dependency
//! on any backend crate, so the generated code (which lands in *that*
//! crate, wherever the derive was invoked) could never name a concrete
//! verifier type at all. `generic_over_verifier` sidesteps this
//! entirely — no concrete verifier is ever named anywhere — by emitting
//! a genuinely `for<V: Verifier>`-checked assertion instead of a
//! per-instantiation one:
//!
//! ```ignore
//! fn assert_edge<V: Verifier, T: Exchange<From, To, V>>() {}
//! fn check<V: Verifier>() { assert_edge::<V, Self>(); }
//! ```
//!
//! **No static assertion is generated in this mode — a real correction,
//! not the original design.** The first attempt tried exactly the
//! `check`-forces-resolution-against-unconstrained-`V` trick sketched
//! above, and the compiler was right to reject it: `Ledger`'s real
//! `Exchange` impl is only generic over `V` *conditionally* — bounded by
//! `Witness<V>`/`Ensures<V>`/`Requires<V>` on the specific evidence types
//! each edge touches, e.g. `Validated: Witness<V> + Requires<V, ..>` for
//! `commit` — never for a truly unconstrained `V: Verifier`, which no
//! real verifier-neutral crate could prove (that would mean the claim
//! holds even for a *hypothetical* verifier with no real `Witness`/
//! `Ensures` content backing it at all). There is no way to state "works
//! for every `V` that also satisfies whatever bounds this specific edge
//! happens to need" from here — this derive works from declared strings,
//! it never parses the real method signatures the way `capture_exchange_
//! body` does, so it has no way to know what those bounds are per edge.
//! `capture_exchange_body`'s own generated `impl<V: Verifier> Exchange<
//! ..> for Self where <real bounds>` is already the complete, real
//! compile-time check for its own edge — this mode adds nothing on top
//! of it at compile time; declaration-vs-reality correctness (a typo'd
//! edge, a missing one) is caught by the runtime `ExchangeEdgeRecord`
//! cross-check instead, which needs no universal-over-`V` provability to
//! be real. The generated `impl amenable_core::StateMachine<V> for
//! Self` becomes a single `impl<V: Verifier>` blanket, since `states()`/
//! `transitions()`/`audit_surface()` are all genuinely `V`-independent.
//!
//! Step 2 adds the real `impl amenable_core::StateMachine<Verifier> for
//! Self` alongside the static assertions: `states()`/`transitions()`
//! echo the parsed declarations directly (no registry query needed —
//! the declaration itself is the source of truth for what was
//! declared), and `audit_surface()` queries the real
//! `amenable_core::ExchangeEdgeRecord` registry, filtered by `self_ty`,
//! for real captured transition-method source. The runtime cross-check
//! between declared and registered edges lives in a real test file, not
//! generated code — see `crates/amenable_kani/tests/stoplight_amenable_test.rs`.

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

enum VerifierMode {
    Concrete(Box<Type>),
    Generic,
}

struct StateMachineBlock {
    verifier: VerifierMode,
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

    let expansions = blocks
        .iter()
        .map(|block| expand_block(self_ty, block))
        .collect::<syn::Result<Vec<_>>>()?;

    Ok(quote! { #(#expansions)* })
}

fn expand_block(self_ty: &syn::Ident, block: &StateMachineBlock) -> syn::Result<TokenStream> {
    let assertions = expand_block_assertions(self_ty, block)?;
    let state_machine_impl = expand_block_state_machine_impl(self_ty, block);

    Ok(quote! {
        #assertions
        #state_machine_impl
    })
}

fn expand_block_assertions(
    self_ty: &syn::Ident,
    block: &StateMachineBlock,
) -> syn::Result<TokenStream> {
    block
        .edges
        .iter()
        .map(|edge| {
            let from_carrier = find_state_carrier(&block.states, &edge.from)?;
            let to_carrier = find_state_carrier(&block.states, &edge.to)?;

            Ok(match &block.verifier {
                VerifierMode::Concrete(verifier) => quote! {
                    const _: fn() = || {
                        fn assert_state_machine_edge<
                            T: ::amenable_core::Exchange<#from_carrier, #to_carrier, #verifier>,
                        >() {
                        }

                        assert_state_machine_edge::<#self_ty>();
                    };
                },
                // No static assertion here -- see this module's own doc
                // comment for why a "for every V: Verifier" check is
                // provably too strong (real edges are only generic over
                // V conditionally, bounded by real Witness/Ensures/
                // Requires impls this derive has no way to know per
                // edge) and why that's fine: capture_exchange_body's own
                // generated impl is already the real compile-time check.
                VerifierMode::Generic => quote! {},
            })
        })
        .collect()
}

fn expand_block_state_machine_impl(self_ty: &syn::Ident, block: &StateMachineBlock) -> TokenStream {
    let self_ty_str = self_ty.to_string();

    let state_names = block.states.iter().map(|state| &state.name);
    let transitions = block.edges.iter().map(|edge| {
        let from = &edge.from;
        let to = &edge.to;
        quote! { ::amenable_core::Transition { from: #from, to: #to } }
    });

    let (impl_generics, verifier) = match &block.verifier {
        VerifierMode::Concrete(verifier) => (quote! {}, quote! { #verifier }),
        VerifierMode::Generic => (quote! { <V: ::amenable_core::Verifier> }, quote! { V }),
    };

    quote! {
        impl #impl_generics ::amenable_core::StateMachine<#verifier> for #self_ty {
            fn states() -> &'static [&'static str] {
                &[#(#state_names),*]
            }

            fn transitions() -> &'static [::amenable_core::Transition] {
                &[#(#transitions),*]
            }

            fn audit_surface() -> ::std::vec::Vec<::amenable_core::TransitionAudit> {
                let mut audits: ::std::vec::Vec<::amenable_core::TransitionAudit> =
                    ::inventory::iter::<::amenable_core::ExchangeEdgeRecord>()
                        .filter(|record| record.self_ty == #self_ty_str)
                        .map(|record| ::amenable_core::TransitionAudit {
                            to: record.evidence,
                            method_name: record.method_name,
                            body: record.body,
                        })
                        .collect();

                audits.sort_by(|left, right| {
                    (left.to, left.method_name).cmp(&(right.to, right.method_name))
                });

                audits
            }
        }
    }
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
    let mut generic_over_verifier = false;
    let mut states: Vec<StateDecl> = Vec::new();
    let mut edges: Vec<EdgeDecl> = Vec::new();

    attr.parse_nested_meta(|meta| {
        if meta.path.is_ident("verifier") {
            let value: LitStr = meta.value()?.parse()?;
            verifier = Some(value.parse()?);
            return Ok(());
        }

        if meta.path.is_ident("generic_over_verifier") {
            generic_over_verifier = true;
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

    let verifier = match (verifier, generic_over_verifier) {
        (Some(verifier), false) => VerifierMode::Concrete(Box::new(verifier)),
        (None, true) => VerifierMode::Generic,
        (Some(_), true) => {
            return Err(Error::new_spanned(
                attr,
                "state_machine accepts at most one of `verifier`/`generic_over_verifier`, not both",
            ));
        }
        (None, false) => {
            return Err(Error::new_spanned(
                attr,
                "state_machine requires either `verifier = \"...\"` or `generic_over_verifier`",
            ));
        }
    };

    Ok(StateMachineBlock {
        verifier,
        states,
        edges,
    })
}

fn parse_state_decl(meta: &syn::meta::ParseNestedMeta) -> syn::Result<StateDecl> {
    let content;
    syn::parenthesized!(content in meta.input);

    let name: LitStr = content.parse()?;
    content.parse::<syn::Token![,]>()?;
    let carrier_lit: LitStr = content.parse()?;

    Ok(StateDecl {
        name,
        carrier: carrier_lit.parse()?,
    })
}

fn parse_edge_decl(meta: &syn::meta::ParseNestedMeta) -> syn::Result<EdgeDecl> {
    let content;
    syn::parenthesized!(content in meta.input);

    let from: LitStr = content.parse()?;
    content.parse::<syn::Token![,]>()?;
    let to: LitStr = content.parse()?;

    Ok(EdgeDecl { from, to })
}
