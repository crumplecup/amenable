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

// Private, not `pub use`: gives `crate::VerusVerifier` a real crate-root
// binding to satisfy `amenable_derive`'s `verus_ensures_witness!`/
// `verus_ensures_predicate!` family (their generated code names
// `crate::VerusVerifier` so the same macro works whether the calling
// crate supplies the real marker or, as `amenable_derive`'s own tests
// do, a local stand-in) without re-exporting `amenable_core::VerusVerifier`
// under a second public path — the type's one public path stays
// `amenable_core::VerusVerifier`.
#[cfg(feature = "verus")]
use amenable_core::VerusVerifier;

mod ext_type;
#[cfg(feature = "jiff")]
mod jiff;
// Shared by every target module's registrations (`impl_ext_type!`,
// `register_ext_standard_evidence!`); gated by the union of target features
// rather than compiled unconditionally, since with none active there is
// nothing left to consume it.
#[cfg(feature = "chrono")]
mod chrono;
#[cfg(any(feature = "jiff", feature = "chrono"))]
mod macros;
mod provenance_vocab;
// Fractional-second conversions shared by the jiff and chrono backends.
#[cfg(any(feature = "jiff", feature = "chrono"))]
mod temporal_fraction;

#[cfg(feature = "chrono")]
pub use chrono::{
    ChronoDate, ChronoDateTime, ChronoReducedCalendarDate, ChronoReducedLocalTime, ChronoTime,
    ChronoTimeBackend, ChronoVerifier, ChronoVerifierMetadata,
};
pub use ext_type::{ExtLanguageProvenance, ExtProvenance, ExtStandard, ExtType};
#[cfg(feature = "jiff")]
pub use jiff::{
    JiffDate, JiffDateTime, JiffISOWeekDate, JiffOffset, JiffOffsetDateTime, JiffRecurringInterval,
    JiffReducedCalendarDate, JiffReducedLocalTime, JiffSpan, JiffTime, JiffTimeBackend,
    JiffTimeInterval, JiffTimeIntervalEndpoint, JiffTimeIntervalRepresentation, JiffTimeZone,
    JiffTimestamp, JiffVerifier, JiffVerifierMetadata, JiffZoned,
};
pub use provenance_vocab::{Authority, AuthorityKind, SourceCrate, SourceModule, TypeName};
