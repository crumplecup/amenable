use super::instant::JiffOffsetDateTime;
use amenable_core::{Metadata, OwnedEntry, Provenance, Verifier};
use amenable_time::{
    TemporalCivilProps, TemporalDurationProps, TemporalRecurringIntervalProps,
    TemporalTimeIntervalProps, TemporalZoneProps,
};

// ── Phase 3: Civil ───────────────────────────────────────────────────

/// A [`jiff::civil::Date`] as a calendar (or ordinal) date carrier.
///
/// Reused for BOTH `TemporalCivilProps::CalendarDate` and `::OrdinalDate`
/// — the same real jiff type honestly backs both once resolved to a
/// concrete value, the same "one carrier, several associated-type
/// slots" pattern `amenable_std::StdSystemTime` already uses for its
/// own `Instant`/`OffsetDateTime`.
#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    derive_more::Deref,
    derive_new::new,
)]
#[evidence(basis = "Self")]
pub struct JiffDate(
    /// The wrapped date.
    jiff::civil::Date,
);

/// A [`jiff::civil::Time`] as a local time-of-day carrier.
#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    derive_more::Deref,
    derive_new::new,
)]
#[evidence(basis = "Self")]
pub struct JiffTime(
    /// The wrapped time.
    jiff::civil::Time,
);

/// A [`jiff::civil::DateTime`] as a local date-time carrier.
#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    derive_more::Deref,
    derive_new::new,
)]
#[evidence(basis = "Self")]
pub struct JiffDateTime(
    /// The wrapped date-time.
    jiff::civil::DateTime,
);

/// A [`jiff::civil::ISOWeekDate`] as a week-date carrier.
#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    derive_more::Deref,
    derive_new::new,
)]
#[evidence(basis = "Self")]
pub struct JiffISOWeekDate(
    /// The wrapped ISO week date.
    jiff::civil::ISOWeekDate,
);

/// A reduced-precision calendar date carrier.
///
/// `jiff::civil::Date` has no partial (year-only / year-month-only)
/// representation of its own — this is a real, meaningful shape
/// (mirroring `amenable_time::ReducedCalendarDateDescriptor`'s own two
/// forms exactly, using jiff's own `i16`/`i8` component widths), but its
/// conversion logic isn't wired to any `Exchange` edge yet: no
/// `Temporal*NativeBridge` trait in Phases 1-3's scope needs a
/// `ReducedCalendarDate` realize/reflect pair — only `TemporalParser`'s
/// own `RawInput -> ParsedReducedCalendarDate` edge (Phase 9) will.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::Evidence)]
#[evidence(basis = "Self")]
pub enum JiffReducedCalendarDate {
    /// Year-only calendar date form.
    Year {
        /// Signed calendar year.
        year: i16,
    },
    /// Year-month calendar date form.
    YearMonth {
        /// Signed calendar year.
        year: i16,
        /// Calendar month number.
        month: i8,
    },
}

impl Default for JiffReducedCalendarDate {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::Year { year: 0 }
    }
}

/// A reduced-precision local time-of-day carrier.
///
/// Same "real shape, not yet wired to an `Exchange` edge" status as
/// [`JiffReducedCalendarDate`] — and a genuinely harder case besides:
/// `amenable_time::ReducedLocalTimeDescriptor` allows a fractional
/// component on its own hour/minute field, which `jiff::civil::Time`
/// cannot represent at all (its finest fractional unit is the second).
/// This carrier's own conversion logic, whenever Phase 9 needs it, will
/// have to reject that case the same way `duration_descriptor_to_jiff_span`
/// already rejects a duration fraction on a coarser-than-seconds unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::Evidence)]
#[evidence(basis = "Self")]
pub enum JiffReducedLocalTime {
    /// Hour-only local time form.
    Hour {
        /// Hour of day.
        hour: i8,
    },
    /// Hour-minute local time form.
    HourMinute {
        /// Hour of day.
        hour: i8,
        /// Minute of hour.
        minute: i8,
    },
}

impl Default for JiffReducedLocalTime {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::Hour { hour: 0 }
    }
}

impl TemporalCivilProps for JiffTimeBackend {
    type CalendarDate = JiffDate;
    type ReducedCalendarDate = JiffReducedCalendarDate;
    type OrdinalDate = JiffDate;
    type WeekDate = JiffISOWeekDate;
    type LocalTime = JiffTime;
    type ReducedLocalTime = JiffReducedLocalTime;
    type LocalDateTime = JiffDateTime;
}

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

// ── JiffVerifier ─────────────────────────────────────────────────────

/// Reporting surface for [`JiffVerifier`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct JiffVerifierMetadata;

impl Metadata for JiffVerifierMetadata {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn snapshot(&self) -> Vec<OwnedEntry> {
        vec![
            OwnedEntry::new("verifier_family", "jiff"),
            OwnedEntry::new(
                "kind",
                "runtime execution over jiff's real calendar/zone/parser API",
            ),
            OwnedEntry::new("formal_tool", "none"),
        ]
    }
}

impl Provenance for JiffVerifierMetadata {}

/// A [`Verifier`] marker for the real jiff-backed temporal backend.
///
/// Like [`CanaryVerifier`](amenable_std::CanaryVerifier), it runs no
/// formal tool — but unlike the canary, its `Exchange` bodies exercise
/// jiff's own real implementation (calendar arithmetic, ISO 8601
/// parsing, IANA tzdb lookups where later phases reach them), not a
/// hand-rolled stand-in scoped to what one small standard-library type
/// can honestly back. A violation is still an `Err` at runtime, not a
/// proof obligation, so from this verifier's point of view every
/// machine-checkable contract is a trusted citation exactly like the
/// structural ones.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct JiffVerifier;

impl Verifier for JiffVerifier {
    type Metadata = JiffVerifierMetadata;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn name() -> &'static str {
        "jiff"
    }
}

// ── Native carriers ──────────────────────────────────────────────────

/// A [`jiff::Span`] as a temporal duration carrier.
///
/// `jiff::Span` derives only `Clone, Copy, Default` (a manual, non-derived
/// `Debug` impl, and deliberately no `PartialEq`/`Eq`/`Hash` — confirmed
/// by reading jiff's real source; see `amenable_ext::jiff`'s own
/// `SpanFieldwise` witness for why equal-elapsed spans can legitimately
/// compare unequal fieldwise). This wrapper mirrors that: no
/// `PartialEq`/`Eq`/`Hash` derive here either, and comparisons in tests
/// go through `jiff::Span`'s own getters, not `==` on the whole carrier.
#[derive(
    Debug, Clone, Copy, Default, amenable_derive::Evidence, derive_more::Deref, derive_new::new,
)]
#[evidence(basis = "Self")]
pub struct JiffSpan(
    /// The wrapped span.
    jiff::Span,
);

// ── The backend ──────────────────────────────────────────────────────

/// The real jiff-backed temporal backend.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct JiffTimeBackend;

impl TemporalDurationProps for JiffTimeBackend {
    type Duration = JiffSpan;
}

// ── Time interval / recurring interval (Phase 8) ────────────────────
//
// `TemporalTimeIntervalProps`/`TemporalRecurringIntervalProps` +
// their `NativeBridge`s, plus `TemporalNativeIntervalFactory`'s own
// `order_offset_endpoints_native` edge.
//
// `TimeIntervalEndpoint::Value` wraps `QualifiedOrBareTemporalValueDescriptor`,
// itself wrapping `TemporalValueDescriptor` -- a broad enum spanning
// every temporal form in the whole accord. This backend honestly backs
// the real, jiff-representable forms this session already built native
// carriers for (calendar/ordinal/week dates fold to one `jiff::civil::
// Date` variant, per Phase 3's own precision-not-preserved-on-reflect
// convention; local/offset/zoned date-times keep their own distinct
// jiff type) and rejects every other form -- the CalConnect/ISO 8601-2
// extension family (`ReducedCalendarDate`/`Decade`/`Century`/
// `ExtendedYear`/`DateWithShift`/`TimeOfDayWithShift`/`SubYearGrouping`/
// `Seasonal`/`Unspecified`, plus any explicitly `Qualified` value) --
// with a real, honest `Unsupported`, not a silent default.

/// A real jiff-backed time-interval boundary.
#[derive(Debug, Clone, Default, amenable_derive::Evidence)]
#[evidence(basis = "Self")]
pub enum JiffTimeIntervalEndpoint {
    /// An explicit open boundary.
    #[default]
    Open,
    /// An explicit unknown boundary.
    Unknown,
    /// A calendar date (also backs the ordinal- and week-date forms,
    /// which canonicalize to this same jiff type on realize).
    CalendarDate(jiff::civil::Date),
    /// A local date-time.
    LocalDateTime(jiff::civil::DateTime),
    /// An offset date-time.
    OffsetDateTime(JiffOffsetDateTime),
    /// A zoned date-time.
    ZonedDateTime(jiff::Zoned),
}

/// The top-level real jiff-backed interval representation form.
#[derive(Debug, Clone)]
pub enum JiffTimeIntervalRepresentation {
    /// Concrete start and end boundaries.
    StartEnd {
        /// Interval start boundary.
        start: JiffTimeIntervalEndpoint,
        /// Interval end boundary.
        end: JiffTimeIntervalEndpoint,
    },
    /// Concrete start boundary and a duration.
    StartDuration {
        /// Interval start boundary.
        start: JiffTimeIntervalEndpoint,
        /// Interval duration.
        duration: jiff::Span,
    },
    /// Duration followed by a concrete end boundary.
    DurationEnd {
        /// Interval duration.
        duration: jiff::Span,
        /// Interval end boundary.
        end: JiffTimeIntervalEndpoint,
    },
}

impl Default for JiffTimeIntervalRepresentation {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::StartEnd {
            start: JiffTimeIntervalEndpoint::default(),
            end: JiffTimeIntervalEndpoint::default(),
        }
    }
}

/// A real jiff-backed time-interval carrier.
#[derive(Debug, Clone, amenable_derive::Evidence, derive_more::Deref, derive_new::new)]
#[evidence(basis = "Self")]
pub struct JiffTimeInterval(
    /// The wrapped representation.
    JiffTimeIntervalRepresentation,
);

impl Default for JiffTimeInterval {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self(JiffTimeIntervalRepresentation::default())
    }
}

/// A real jiff-backed recurring-interval carrier.
#[derive(Debug, Clone, amenable_derive::Evidence, derive_getters::Getters, derive_new::new)]
#[evidence(basis = "Self")]
pub struct JiffRecurringInterval {
    /// Bounded repetition count; `None` denotes unbounded recurrence.
    repetitions: Option<u32>,
    /// Repeated interval payload.
    interval: JiffTimeInterval,
}

impl Default for JiffRecurringInterval {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self {
            repetitions: None,
            interval: JiffTimeInterval::default(),
        }
    }
}

impl TemporalTimeIntervalProps for JiffTimeBackend {
    type TimeInterval = JiffTimeInterval;
}

impl TemporalRecurringIntervalProps for JiffTimeBackend {
    type RecurringInterval = JiffRecurringInterval;
}
