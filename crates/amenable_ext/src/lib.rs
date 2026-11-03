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
#[cfg(all(feature = "verus", any(feature = "jiff", feature = "chrono")))]
use amenable_core::VerusVerifier;

mod ext_type;
// Shared by every target's own `verus_witness` submodule (`jiff::
// verus_witness`, `chrono::verus_witness`): the trusted/checked macros
// and the `ExtCheckedProof` proof artifact, target-agnostic. Gated by
// the union of target features too, the same as `macros` below --
// with no target active there's nothing to consume it.
#[cfg(all(feature = "verus", any(feature = "jiff", feature = "chrono")))]
mod ext_verus_witness;
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
// Kani-only proof scaffolding for `temporal_fraction`'s encode
// direction -- split out once that module grew large enough to be its
// own cohesive concern. Gated the same way as `temporal_fraction`
// itself, since it calls back into it.
#[cfg(all(any(feature = "jiff", feature = "chrono"), kani))]
mod fractional_digit_chain;

#[cfg(feature = "chrono-tz")]
pub use chrono::{
    BoundedStepWalk, BoundedStepWalkToken, ChronoTz, ChronoZoned, DISCOVERED_GAP_TABLE,
    DISCOVERED_OFFSET_TABLE, DiscoveredGapTableRow, DiscoveredOffsetEntry, DiscoveredOffsetTable,
    DiscoveredOffsetTableData, DiscoveredOffsetTableRow, OffsetDiscoveryParams,
    OffsetDiscoveryRequest,
};
#[cfg(all(feature = "chrono", kani))]
pub use chrono::{
    RefinedFixedOffset, build_refined_fixed_offset_in_range,
    fixed_offset_to_utc_offset_descriptor_infallible,
    naive_date_time_to_local_date_time_descriptor_infallible,
    naive_date_time_to_local_date_time_descriptor_no_fraction,
    utc_offset_descriptor_to_fixed_offset_infallible,
};
#[cfg(feature = "chrono")]
pub use chrono::{
    ChronoDate, ChronoDateTime, ChronoInstant, ChronoOffset, ChronoOffsetDateTime,
    ChronoReducedCalendarDate, ChronoReducedLocalTime, ChronoTime, ChronoTimeBackend,
    ChronoVerifier, ChronoVerifierMetadata, build_fixed_offset_from_signed_seconds,
    build_utc_offset_descriptor_from_parts, chrono_to_offset_date_time_descriptor,
    fixed_offset_sign_and_magnitude, fixed_offset_to_utc_offset_descriptor,
    local_date_time_descriptor_to_naive, naive_date_time_to_local_date_time_descriptor,
    narrow_to_u8, narrow_to_u8_in_range, offset_date_time_descriptor_to_chrono,
    offset_descriptor_parts_2, offset_descriptor_parts_3, offset_descriptor_parts_4,
    signed_seconds_parts_2, signed_seconds_parts_3, signed_seconds_parts_4,
    utc_offset_descriptor_to_fixed_offset,
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
#[cfg(any(feature = "jiff", feature = "chrono"))]
pub use temporal_fraction::{
    fractional_digit_bytes_to_nanos, fractional_seconds_digits_to_nanos,
    nanos_to_fractional_digit_bytes, nanos_to_fractional_seconds_digits,
};
#[cfg(all(any(feature = "jiff", feature = "chrono"), kani))]
pub use temporal_fraction::digit_bytes_to_string_unconditional;
#[cfg(all(any(feature = "jiff", feature = "chrono"), kani))]
pub use fractional_digit_chain::{
    ValidFractionalDigits, combine_digit_2, combine_digit_3, combine_digit_4, combine_digit_5,
    combine_digit_6, combine_digit_7, combine_digit_8, combine_digit_9,
    fractional_digit_bytes_depth_1, fractional_digit_bytes_depth_2, fractional_digit_bytes_depth_3,
    fractional_digit_bytes_depth_4, fractional_digit_bytes_depth_5,
    fractional_digit_bytes_depth_6, fractional_digit_bytes_depth_7,
    fractional_digit_bytes_depth_8, fractional_digit_bytes_depth_9, fractional_digit_round_trip,
};
