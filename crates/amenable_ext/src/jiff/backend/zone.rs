use crate::JiffTimeBackend;
use amenable_time::TemporalZoneProps;

// ── Phase 4: Zone ────────────────────────────────────────────────────

/// A [`jiff::tz::TimeZone`] as a named-time-zone carrier.
///
/// `jiff::tz::TimeZone` derives only `Clone, Eq, PartialEq` (confirmed
/// by reading jiff's real source — no `Copy`, no `Hash`, no `Default`,
/// consistent with it being a hand-rolled pointer-tagged union that may
/// own real TZif data) — this wrapper's own derive list matches, and,
/// same as `JiffOffset`'s own doc comment, needs an explicit
/// `basis_ctor` (`TimeZone::UTC`, a real jiff constant) since there is
/// no `Default` to fall back to.
#[derive(
    Debug, Clone, PartialEq, Eq, amenable_derive::Evidence, derive_more::Deref, derive_new::new,
)]
#[evidence(basis = "Self", basis_ctor = "Self(jiff::tz::TimeZone::UTC)")]
pub struct JiffTimeZone(
    /// The wrapped time zone.
    jiff::tz::TimeZone,
);

/// A [`jiff::Zoned`] as a zoned-date-time carrier.
///
/// `jiff::Zoned` derives only `Clone` (confirmed by reading jiff's real
/// source), but DOES have a manual `Default` impl — unlike `JiffTimeZone`
/// above, a bare `#[evidence(basis = "Self")]` works here without a
/// `basis_ctor` override.
#[derive(Debug, Clone, Default, amenable_derive::Evidence, derive_more::Deref, derive_new::new)]
#[evidence(basis = "Self")]
pub struct JiffZoned(
    /// The wrapped zoned date-time.
    jiff::Zoned,
);

impl TemporalZoneProps for JiffTimeBackend {
    type NamedTimeZone = JiffTimeZone;
    type ZonedDateTime = JiffZoned;
}
