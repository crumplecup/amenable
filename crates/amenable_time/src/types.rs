//! Neutral temporal descriptors — the accord vocabulary that crosses
//! trait boundaries, ported from `elicit_temporal::types` (259
//! definitions there).
//!
//! This file holds only the two closed enums the Phase 0 slice needs
//! (`TemporalComponent` for `TemporalReporter`-adjacent typing,
//! `SerializationProfile` for `TemporalReporter` and `TemporalError`).
//! The descriptor structs (`CalendarDateDescriptor`, …) and the
//! `#[derive(Evidence)]` pass land in Phase 2. Descriptors carry **no**
//! construction-time validation — a `month = 13` is rejected by a proof,
//! not a constructor guard (`docs/AMENABLE_TIME_PLAN.md`, decision 5).

use strum::EnumIter;

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
