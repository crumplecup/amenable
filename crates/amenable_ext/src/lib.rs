//! Third-party crate support for the `amenable` trait family.
//!
//! Traits meant to be implemented directly on foreign types must live in
//! a crate visible to both the trait and the type — Rust's orphan rules
//! leave no other option. So, like `amenable_std` for the standard
//! library, this crate defines [`ExtType`]/[`ExtStandard`] and the
//! per-target-crate registrations together, one directory per target
//! behind a same-named feature flag (default `[]` empty): `cargo add
//! amenable_ext --features jiff` sweeps in exactly the `jiff` dependency
//! and its registrations, nothing else.
//!
//! First target: **jiff**, and jiff only for now — see
//! `docs/AMENABLE_EXT_PLAN.md` for the full plan, phasing, and the
//! `cordial` coverage-tooling design this crate's registrations are
//! meant to be checked against.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod ext_type;
#[cfg(feature = "jiff")]
mod jiff;
// Shared by every target module's registrations (`impl_ext_type!`,
// `register_ext_standard_evidence!`); gated by the union of target
// features rather than compiled unconditionally, since with none active
// there is nothing left to consume it -- change to `any(feature =
// "jiff", feature = "chrono", ...)` once a second target lands
// (`any(...)` of a single feature is itself a clippy lint).
#[cfg(feature = "jiff")]
mod macros;
mod provenance_vocab;

pub use ext_type::{ExtLanguageProvenance, ExtProvenance, ExtStandard, ExtType};
pub use provenance_vocab::{Authority, AuthorityKind, SourceCrate, SourceModule, TypeName};
