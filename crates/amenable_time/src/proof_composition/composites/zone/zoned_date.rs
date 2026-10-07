//! Date-with-named-zone composite, for values shaped like chrono's `Date<Tz>`.

use crate::{DateValid, NamedTimeZoneIdentityValid};

/// Aggregate proof that a calendar date is attached to a named zone.
///
/// Models a date with a named zone and no time component: the shape of
/// chrono's `Date<Tz>`, which stores a `NaiveDate` and a zone offset.
///
/// Covers the date (`DateValid`) and the zone identity
/// (`NamedTimeZoneIdentityValid`). It does not cover the zone offset
/// stored with the date. No existing contract states that the offset
/// matches the zone's rules for that date: `OffsetConsistentWithNamedZone`
/// is about a fixed instant, and a date does not denote one. That claim is
/// a gap, not covered by this composite.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct ZonedDateValid {
    /// The date is structurally and semantically valid.
    date: DateValid,
    /// The zone is a valid named zone identity.
    zone: NamedTimeZoneIdentityValid,
}
