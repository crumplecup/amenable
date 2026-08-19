//! Verus proof content for Rust standard-library carriers, plus the
//! `amenable_core` constitutional trait family itself.
//!
//! This crate contains *only* what `verus --crate-type=lib
//! crates/amenable_verus/src/lib.rs` needs to check: real `verus! { ... }`
//! spec functions, nothing else. Unlike `amenable_creusot` (which still
//! depends on `amenable_core`/`amenable_derive`, since `cargo creusot`
//! resolves ordinary Cargo dependencies), Verus is invoked as a bare
//! compiler over a single file tree — it never sees `Cargo.toml`, so it
//! cannot resolve `amenable_core`, `inventory`, or any proc-macro crate at
//! all as an ordinary Cargo dependency. Confirmed empirically: pointing
//! `verus` at the old, pre-split version of this crate (which depended on
//! all three) failed immediately with unresolved-crate errors, not proof
//! errors.
//!
//! So this crate's own `Cargo.toml` has exactly two dependencies —
//! `verus_builtin_macros` and `vstd` — mirroring `elicitation_verus`'s own
//! real, working structure (confirmed by reading `~/repos/elicitation/
//! crates/elicitation_verus/src/*.rs`: every file there is equally
//! dependency-free). But a Cargo dependency isn't the only way to bring
//! real source into a single-file-tree `verus` compilation: `#[path]` mod
//! declarations below pull in `amenable_core`'s own trait-family files
//! *verbatim*, unmodified, as ordinary modules of *this* crate — no
//! cross-crate `--extern` resolution needed, since Verus never sees more
//! than one crate's worth of `mod`-linked files regardless. Confirmed real
//! and load-bearing, not merely compiling: `gallery::
//! evidence_self_referential_root` verifies the real, unmodified
//! `Evidence`/`Witness`/`ProofToken`/`Establish` trait family — including
//! a genuine self-referential root — end to end (`351 verified, 0
//! errors`). `witness.rs` is the one exception (mixes the `Witness<V>`
//! trait's own definition with `inventory`-dependent registry code in the
//! same file); `witness_accommodation` is a hand-trimmed mirror of just
//! the trait mechanics, pending a real split of `witness.rs` itself.
//!
//! `char` and `String` are real Rust primitives/std types with genuine
//! `vstd` spec support (`char as u32` casts and `String`'s `View` impl
//! giving `s@`/`s@.len()` both verified empirically against a real Verus
//! install) — no shadow-struct workaround needed for either, unlike most
//! of `elicitation_verus`'s own std-adjacent proofs (their `strings.rs`
//! deliberately avoids storing a real `String` field at all, citing "Verus
//! doesn't have specs for it" — true for their `Contract` wrapper types'
//! needs at the time, but `elicitation_verus`'s own `gallery::level5`
//! demonstrates the real `s@.len()` pattern this crate uses directly).

pub mod derived_witness;
pub mod exchange_support;
pub mod gallery;
pub mod rust_std;
mod witness_accommodation;

// The real, unmodified `amenable_core` trait-family source, mod-included
// verbatim -- see this file's own doc comment above for why `#[path]`
// achieves this without any Cargo dependency at all. Named to match
// `amenable_core::lib`'s own flat mod/pub-use structure exactly (not
// nested under a submodule) because these real files use bare `crate::X`
// imports, expecting to be compiled at a crate root shaped like their
// own.
#[path = "../../amenable_core/src/cert.rs"]
mod amenable_core_cert;
#[path = "../../amenable_core/src/contract.rs"]
mod amenable_core_contract;
#[path = "../../amenable_core/src/evidence.rs"]
mod amenable_core_evidence;
#[path = "../../amenable_core/src/exchange.rs"]
mod amenable_core_exchange;
#[path = "../../amenable_core/src/provenance.rs"]
mod amenable_core_provenance;
#[path = "../../amenable_core/src/roles.rs"]
mod amenable_core_roles;
#[path = "../../amenable_core/src/state_machine.rs"]
mod amenable_core_state_machine;
#[path = "../../amenable_core/src/verifier.rs"]
mod amenable_core_verifier;

pub use amenable_core_cert::{Certificate, Registry, RegistryReport};
pub use amenable_core_contract::{Ensures, Requires};
pub use amenable_core_evidence::Evidence;
pub use amenable_core_exchange::{Establish, Exchange, ProofToken, Sidecar};
pub use amenable_core_provenance::{
    MetadataEntry, OwnedProvenanceReport, Provenance, ProvenanceReport,
};
pub use amenable_core_roles::{AsStandard, Standard};
pub use amenable_core_state_machine::{StateMachine, Transition, TransitionAudit};
pub use amenable_core_verifier::Verifier;
pub use witness_accommodation::Witness;
