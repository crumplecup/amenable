//! `amenable_time`'s trait-layer error -- [`TemporalError`] +
//! [`TemporalErrorKind`], following CLAUDE.md error patterns 2 and 3:
//! the parent boxes an umbrella `Kind` enum, and every `Kind` variant
//! is a clean 1-tuple wrapping a real, named, `Error`-implementing
//! native source -- never a bare `String` or struct-literal payload --
//! so `TemporalError::source()`'s chain always reaches a genuine
//! `Error`-implementing value at each hop, matching the real,
//! already-compliant precedent in `amenable::error::sources::internal`
//! (`InvariantSource`, etc).
//!
//! Ported from `elicit_temporal::error`. `ParseRejectedSource` names
//! the serialization profile as a `String` for now; it takes the real
//! `SerializationProfile` descriptor once that lands (plan Phase 2).
//!
//! Split by domain, matching the real precedent in
//! `amenable::error::sources::{internal,io,numeric,time}`: `parsing`/
//! `validation`/`endpoint`/`capability`/`builders`/`narrowing` hold
//! every backend-neutral native source (`endpoint` split out of
//! `validation` to stay under this crate's per-file line threshold --
//! declared-interval-endpoint and fraction/leap-second validation,
//! as distinct from calendar-component validation); `kind` holds the
//! umbrella [`TemporalErrorKind`] and its [`TemporalError`] parent;
//! `chrono_sources`/`chrono_tz_sources`/`jiff_sources` hold the sources
//! that need a specific backend crate's own type to stay genuine
//! rather than stringified, each gated behind that backend's
//! same-named feature so this crate never depends on a backend it
//! doesn't need.

mod builders;
mod capability;
mod endpoint;
mod kind;
mod narrowing;
mod parsing;
mod unsupported_case;
mod validation;
#[cfg(feature = "chrono")]
mod chrono_sources;
#[cfg(feature = "chrono-tz")]
mod chrono_tz_sources;
#[cfg(feature = "jiff")]
mod jiff_interval_sources;
#[cfg(feature = "jiff")]
mod jiff_sources;

pub use builders::{
    DurationBuilderFailedSource, IxdtfTimestampBuilderFailedSource,
    LocalDateTimeBuilderFailedSource, LocalTimeBuilderFailedSource,
    NamedTimeZoneBuilderFailedSource, OffsetDateTimeBuilderFailedSource,
    RecurringIntervalBuilderFailedSource, UtcOffsetBuilderFailedSource,
    ZonedDateTimeBuilderFailedSource,
};
pub use capability::{
    AmbiguousLocalTimestampSource, LossyConversionRequiresAuthoritySource,
    NamedZoneInconsistencySource, UnsupportedDetail, UnsupportedSource,
};
pub use endpoint::{
    CalendarComponentsPresentSource, DeclaredEndpoint, EndpointHasNoTemporalValueSource,
    LeapSecondNotRepresentableSource, NonDigitFractionByteSource,
};
pub use kind::{TemporalError, TemporalErrorKind, TemporalResult};
pub use unsupported_case::UnsupportedCase;
pub use narrowing::{DurationDayOverflowSource, EpochSecondsEndpointsReversedSource};
pub use parsing::{ParseIntRejectedSource, ParseRejectedSource};
pub use validation::{
    ComponentOutOfRangeSource, FixedOffsetOutOfRangeSource, InvalidCalendarDateSource,
    InvalidIsoWeekdaySource, InvalidLocalTimeSource, InvalidOrdinalDateSource,
    InvalidWeekDateSource, NanosecondOutOfRangeSource, NoUniqueLocalMappingSource,
};
#[cfg(feature = "chrono")]
pub use chrono_sources::{NonPositiveDeltaSource, OffsetDiscoveryRangeReversedSource};
#[cfg(feature = "chrono-tz")]
pub use chrono_tz_sources::{NamedZoneResolutionFailedSource, OffsetInconsistentWithNamedZoneSource};
#[cfg(feature = "jiff")]
pub use jiff_interval_sources::{
    IntervalEndpointsReversedSource, JiffDateTimeConstructionFailedSource,
    JiffInstantPinFailedSource, JiffNamedZoneResolutionFailedSource,
    JiffOffsetInconsistentWithNamedZoneSource, JiffSpanConstructionFailedSource,
    LosslessPrecisionAdjustmentRejectedSource,
};
#[cfg(feature = "jiff")]
pub use jiff_sources::{
    ComponentNarrowingFailedSource, JiffDateConstructionFailedSource,
    JiffOffsetConstructionFailedSource, JiffParseRejectedSource,
    JiffTimeConstructionFailedSource, NarrowingComponent,
};
