//! Neutral temporal descriptors — the accord vocabulary that crosses
//! trait boundaries.
//!
//! Ported from `elicit_temporal::types` (259 definitions there). This
//! file holds the minimal set the Phase 0 slice needs; the rest lands in
//! plan Phase 2, at which point the descriptor structs also gain
//! `#[derive(Evidence)]` so they can serve as `Sidecar::Primary`
//! payloads. Descriptors carry **no** construction-time validation — a
//! `month = 13` is rejected by a proof, not a constructor guard
//! (`docs/AMENABLE_TIME_PLAN.md`, decision 5).

use strum::EnumIter;

/// The fractional-second component of a temporal value: the decimal digits
/// after the fractional separator, verbatim.
///
/// The first descriptor carrying real string data — the Phase 0
/// `Exchange` edge (`amenable_kani::time`) threads one through a
/// `Sidecar` to exercise the non-`Copy` payload path. No construction
/// validation (`docs/AMENABLE_TIME_PLAN.md`, decision 5): whether the
/// digits are well-formed is a proof's job, not a constructor's.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    derive_getters::Getters,
    derive_new::new,
    amenable_derive::Evidence,
)]
#[evidence(basis = "Self")]
pub struct FractionalSecond {
    /// The decimal digits after the fractional separator.
    #[new(into)]
    digits: String,
}

/// The smallest named temporal unit relevant to ISO 8601 precision rules.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, EnumIter, derive_more::Display,
)]
pub enum TemporalComponent {
    /// Calendar year.
    #[display("year")]
    Year,
    /// Calendar decade.
    #[display("decade")]
    Decade,
    /// Calendar century.
    #[display("century")]
    Century,
    /// Calendar month.
    #[display("month")]
    Month,
    /// Calendar day.
    #[display("day")]
    Day,
    /// Clock hour.
    #[display("hour")]
    Hour,
    /// Clock minute.
    #[display("minute")]
    Minute,
    /// Clock second.
    #[display("second")]
    Second,
}

/// A standards-governed wire format a temporal value can be serialized to
/// or parsed from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter, derive_more::Display)]
pub enum SerializationProfile {
    /// ISO 8601 basic form, without separators.
    #[display("ISO 8601 basic")]
    Iso8601Basic,
    /// ISO 8601 extended form, with separators.
    #[display("ISO 8601 extended")]
    Iso8601Extended,
    /// The RFC 3339 Internet timestamp profile.
    #[display("RFC 3339")]
    Rfc3339,
    /// The RFC 9557 IXDTF profile.
    #[display("RFC 9557 IXDTF")]
    Ixdtf,
}
