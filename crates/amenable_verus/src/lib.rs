//! Verus verifier backend for the `amenable` constitutional trait family.
//!
//! This crate is a downstream consumer of `amenable_core`'s `Verifier`/
//! `Witness` interface: it will supply the concrete
//! `WitnessSource<VerusVerifier>` proof-emission machinery. Not yet
//! implemented — see `AMENABLE_PLAN.md` Phase 3 in the `amenable`
//! repository root.

#![forbid(unsafe_code)]
#![warn(missing_docs)]
