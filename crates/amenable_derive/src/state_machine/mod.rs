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
//!
//! **`translator_cfg = "creusot"`** (optional; omit unless the derived
//! type's real `Exchange` impls live *inside* a crate a translator-based
//! backend actually translates). `audit_surface()`'s `inventory::iter`
//! call is safe by default — every backend crate this derive was
//! designed against (`amenable_kani`, `amenable_gaap`) is ordinary
//! Cargo-built and never translated by anything. `amenable_creusot`
//! itself is the one real exception: its own `Stoplight` mirror lives
//! inside the crate `cargo creusot` translates, and a first version of
//! this derive that always emitted an ungated `inventory::iter` call
//! there hit a real `creusot-rustc` ICE (a compiler panic, not a lint).
//! `translator_cfg` splits `audit_surface()` into two `#[cfg(..)]`-gated
//! definitions (real content when the named cfg is absent, an honestly
//! empty `Vec::new()` when present — `inventory` genuinely cannot run
//! under real translation) — but only for the one block that opts in.
//! This is deliberately **not** unconditional: an earlier version always
//! emitted the cfg split, which meant every crate using this derive —
//! including ones with nothing to do with Creusot — needed `cfg(creusot)`
//! added to their own `Cargo.toml`'s `check-cfg` list to silence an
//! `unexpected_cfgs` warning, a real, direct correction: leaking
//! cross-backend cfg awareness into `amenable_kani`/`amenable_gaap` is
//! exactly the "verifier backends never depend on each other" violation
//! this codebase has already caught and reverted twice before, just
//! restated one level down (a cfg *name*, not a Cargo dependency).
//! `translator_cfg`'s value is spliced in as a bare identifier, not
//! hardcoded to `creusot` specifically, so a future Verus-side use (a
//! genuinely different toolchain-resolution problem, likely never
//! reachable this way at all — see this file's own module for that
//! finding) isn't blocked by a hardcoded name.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Error, LitStr, Type};

mod emit;
mod parse;

use emit::{expand_block_assertions, expand_block_state_machine_impl};
use parse::parse_state_machine_block;

pub(super) struct StateDecl {
    name: LitStr,
    carrier: Type,
    root: Option<RootDecl>,
}

/// A declared root constructor: the real, compile-time-checked path
/// (for the `const _: fn(..) -> Carrier = #path;` assertion), its
/// original literal text (for `root_entries()`'s `constructor` string
/// -- kept alongside the parsed `syn::Path` rather than re-stringifying
/// it via `quote!`, which normalizes token spacing, e.g. `Established::
/// <Green, GreenToken>::root` becomes `Established :: < Green,
/// GreenToken > :: root`, a technically-equivalent but uglier string
/// than the one actually written in the declaration), and an optional
/// seed: the real argument type a data-needing root's constructor
/// requires, parsed and stringified the same paired way.
pub(super) struct RootDecl {
    path: syn::Path,
    path_lit: LitStr,
    seed: Option<(Type, LitStr)>,
}

pub(super) struct EdgeDecl {
    from: LitStr,
    to: LitStr,
}

pub(super) enum VerifierMode {
    Concrete(Box<Type>),
    Generic,
}

pub(super) struct StateMachineBlock {
    verifier: VerifierMode,
    states: Vec<StateDecl>,
    edges: Vec<EdgeDecl>,
    translator_cfg: Option<LitStr>,
}

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
