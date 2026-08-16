//! Proof gallery: executable Verus experiments for verifier behavior
//! itself. Mirrors `amenable_kani::gallery`'s own role and discipline
//! (see that module's own doc comment) -- production proofs answer "does
//! this claim hold?"; the gallery answers "what does the verifier do
//! with this modeling pattern?" -- with one necessary structural
//! difference from Kani's gallery, not a stylistic one:
//!
//! `amenable_kani::gallery` registers each case via `inventory::submit!`,
//! so a "false trail" (a case whose *expected* outcome is a Kani
//! failure/timeout) can sit permanently in the crate: Kani proves
//! harnesses one at a time via `cargo kani --harness <name>`, so one
//! failing harness never blocks any other. `amenable_verus` has no
//! `inventory` dependency at all (`lib.rs`'s own doc comment explains
//! why), and `verus --crate-type=lib crates/amenable_verus/src/lib.rs`
//! verifies the *entire* mod-included file tree in one pass -- there is
//! no per-harness selection. A permanently-failing (or, worse,
//! crashing) `verus! {}` block left in this module tree would break
//! `just verify-verus` outright, for everyone, always.
//!
//! So the discipline here is: a gallery case that currently **passes**
//! (a confirmed finding, a best practice, a working accommodation) is a
//! real `mod` below, verified every time `just verify-verus` runs. A
//! gallery case that currently **fails or crashes** documents the exact
//! reproduction in its own file's doc comment/comments, with the actual
//! `verus! {}` content present but not `mod`-included into this file
//! (or gated so it never reaches a real `verus! {}` block) until the
//! finding resolves into something that either passes for real or is
//! confirmed to be a genuine, permanent limitation worth recording as
//! a `mod` anyway with an explicit "known broken, do not un-gate"
//! marker.

// `pub mod`, matching `rust_std`'s own carriers (not `amenable_kani::
// gallery`'s private `mod`): Kani's gallery avoids dead-code lint issues
// because `harness!` gates each proof function behind `#[cfg(kani)]`,
// invisible to ordinary `cargo clippy` entirely. Nothing here is
// cfg-gated -- this crate's whole point is compiling under both plain
// rustc and Verus's driver -- so every item needs a real, public,
// reachable path from the crate root the same way `rust_std`'s carriers
// already have, or `-D warnings` flags it as dead code.
pub mod ensures_contract_bound;
pub mod ensures_macro_generated;
pub mod evidence_self_referential_root;
pub mod proof_token_external_trait_bound;
pub mod stoplight_exchange;
