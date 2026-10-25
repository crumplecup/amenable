//! The umbrella [`TemporalErrorKind`] enum and the [`TemporalError`]
//! parent that boxes it.

use derive_getters::Getters;
use derive_more::{Display, Error};

use super::{
    AmbiguousLocalTimestampSource, CalendarComponentsPresentSource, ComponentOutOfRangeSource,
    DurationBuilderFailedSource, DurationDayOverflowSource, EndpointHasNoTemporalValueSource,
    EpochSecondsEndpointsReversedSource, FixedOffsetOutOfRangeSource, InvalidCalendarDateSource,
    InvalidIsoWeekdaySource, InvalidLocalTimeSource, InvalidOrdinalDateSource,
    InvalidWeekDateSource, IxdtfTimestampBuilderFailedSource, LeapSecondNotRepresentableSource,
    LocalDateTimeBuilderFailedSource, LocalTimeBuilderFailedSource,
    LossyConversionRequiresAuthoritySource, NamedTimeZoneBuilderFailedSource,
    NamedZoneInconsistencySource, NanosecondOutOfRangeSource, NoUniqueLocalMappingSource,
    NonDigitFractionByteSource, OffsetDateTimeBuilderFailedSource, ParseIntRejectedSource,
    ParseRejectedSource, RecurringIntervalBuilderFailedSource, UnsupportedSource,
    UtcOffsetBuilderFailedSource, ZonedDateTimeBuilderFailedSource,
};
#[cfg(feature = "chrono")]
use super::{NonPositiveDeltaSource, OffsetDiscoveryRangeReversedSource};
#[cfg(feature = "chrono-tz")]
use super::{NamedZoneResolutionFailedSource, OffsetInconsistentWithNamedZoneSource};
#[cfg(feature = "jiff")]
use super::{
    ComponentNarrowingFailedSource, IntervalEndpointsReversedSource,
    JiffDateConstructionFailedSource, JiffDateTimeConstructionFailedSource,
    JiffInstantPinFailedSource, JiffNamedZoneResolutionFailedSource,
    JiffOffsetConstructionFailedSource, JiffOffsetInconsistentWithNamedZoneSource,
    JiffParseRejectedSource, JiffSpanConstructionFailedSource,
    JiffTimeConstructionFailedSource, LosslessPrecisionAdjustmentRejectedSource,
};

/// Specific error conditions for temporal trait operations. Every
/// variant is a clean 1-tuple wrapping a real, named,
/// `Error`-implementing source type.
#[derive(Debug, Display, Error)]
pub enum TemporalErrorKind {
    /// Input text does not match the requested profile's lexical shape.
    #[display("{_0}")]
    ParseRejected(ParseRejectedSource),
    /// Input text failed to parse as an integer sub-component.
    #[display("{_0}")]
    ParseIntRejected(ParseIntRejectedSource),
    /// Input text was rejected by jiff's own real parser.
    #[cfg(feature = "jiff")]
    #[display("{_0}")]
    JiffParseRejected(JiffParseRejectedSource),
    /// A temporal component's real value does not fit in a `u8`.
    #[display("{_0}")]
    ComponentOutOfRange(ComponentOutOfRangeSource),
    /// A raw ISO weekday number is outside `1..=7`.
    #[display("{_0}")]
    InvalidIsoWeekday(InvalidIsoWeekdaySource),
    /// A calendar-date descriptor does not resolve to a real calendar date.
    #[display("{_0}")]
    InvalidCalendarDate(InvalidCalendarDateSource),
    /// An ordinal-date descriptor does not resolve to a real calendar date.
    #[display("{_0}")]
    InvalidOrdinalDate(InvalidOrdinalDateSource),
    /// A week-date descriptor does not resolve to a real ISO week date.
    #[display("{_0}")]
    InvalidWeekDate(InvalidWeekDateSource),
    /// A fractional-second nanosecond count does not fit a `u32`.
    #[display("{_0}")]
    NanosecondOutOfRange(NanosecondOutOfRangeSource),
    /// An hour/minute/second/nanosecond combination is not a real local time.
    #[display("{_0}")]
    InvalidLocalTime(InvalidLocalTimeSource),
    /// A UTC-offset magnitude is outside `chrono::FixedOffset`'s representable range.
    #[display("{_0}")]
    FixedOffsetOutOfRange(FixedOffsetOutOfRangeSource),
    /// A local date-time has no single, unique mapping under a given fixed offset.
    #[display("{_0}")]
    NoUniqueLocalMapping(NoUniqueLocalMappingSource),
    /// A declared-but-valueless time-interval endpoint carries no temporal value.
    #[display("{_0}")]
    EndpointHasNoTemporalValue(EndpointHasNoTemporalValueSource),
    /// A fractional-second digit string contains a non-digit byte.
    #[display("{_0}")]
    NonDigitFractionByte(NonDigitFractionByteSource),
    /// A duration descriptor carries calendar-variable components with no fixed `std::time::Duration`.
    #[display("{_0}")]
    CalendarComponentsPresent(CalendarComponentsPresentSource),
    /// A local date-time's sub-second precision represents a leap
    /// second, which the descriptor form cannot represent.
    #[display("{_0}")]
    LeapSecondNotRepresentable(LeapSecondNotRepresentableSource),
    /// The requested operation is unsupported by this backend.
    #[display("{_0}")]
    Unsupported(UnsupportedSource),
    /// The operation would require explicit authority for a lossy conversion.
    #[display("{_0}")]
    LossyConversionRequiresAuthority(LossyConversionRequiresAuthoritySource),
    /// A local timestamp is ambiguous at a zone transition.
    #[display("{_0}")]
    AmbiguousLocalTimestamp(AmbiguousLocalTimestampSource),
    /// A named-zone annotation is inconsistent with the represented instant.
    #[display("{_0}")]
    NamedZoneInconsistency(NamedZoneInconsistencySource),
    /// A `UtcOffsetDescriptorBuilder::build` call failed validation.
    #[display("{_0}")]
    UtcOffsetBuilderFailed(UtcOffsetBuilderFailedSource),
    /// A `LocalTimeDescriptorBuilder::build` call failed validation.
    #[display("{_0}")]
    LocalTimeBuilderFailed(LocalTimeBuilderFailedSource),
    /// A `LocalDateTimeDescriptorBuilder::build` call failed validation.
    #[display("{_0}")]
    LocalDateTimeBuilderFailed(LocalDateTimeBuilderFailedSource),
    /// An `OffsetDateTimeDescriptorBuilder::build` call failed validation.
    #[display("{_0}")]
    OffsetDateTimeBuilderFailed(OffsetDateTimeBuilderFailedSource),
    /// A `NamedTimeZoneDescriptorBuilder::build` call failed validation.
    #[display("{_0}")]
    NamedTimeZoneBuilderFailed(NamedTimeZoneBuilderFailedSource),
    /// A `ZonedDateTimeDescriptorBuilder::build` call failed validation.
    #[display("{_0}")]
    ZonedDateTimeBuilderFailed(ZonedDateTimeBuilderFailedSource),
    /// A `DurationDescriptorBuilder::build` call failed validation.
    #[display("{_0}")]
    DurationBuilderFailed(DurationBuilderFailedSource),
    /// An `IxdtfTimestampDescriptorBuilder::build` call failed validation.
    #[display("{_0}")]
    IxdtfTimestampBuilderFailed(IxdtfTimestampBuilderFailedSource),
    /// A `RecurringIntervalDescriptorBuilder::build` call failed validation.
    #[display("{_0}")]
    RecurringIntervalBuilderFailed(RecurringIntervalBuilderFailedSource),
    /// A duration's whole-day count does not fit a `u32`.
    #[display("{_0}")]
    DurationDayOverflow(DurationDayOverflowSource),
    /// A time interval's two epoch-second endpoints are reversed.
    #[display("{_0}")]
    EpochSecondsEndpointsReversed(EpochSecondsEndpointsReversedSource),
    /// A chrono-tz offset-discovery request's step size is not positive.
    #[cfg(feature = "chrono")]
    #[display("{_0}")]
    NonPositiveDelta(NonPositiveDeltaSource),
    /// A chrono-tz offset-discovery request's range is reversed.
    #[cfg(feature = "chrono")]
    #[display("{_0}")]
    OffsetDiscoveryRangeReversed(OffsetDiscoveryRangeReversedSource),
    /// A chrono-tz IANA zone identifier failed to resolve.
    #[cfg(feature = "chrono-tz")]
    #[display("{_0}")]
    NamedZoneResolutionFailed(NamedZoneResolutionFailedSource),
    /// A computed offset is inconsistent with a named zone's own rules.
    #[cfg(feature = "chrono-tz")]
    #[display("{_0}")]
    OffsetInconsistentWithNamedZone(OffsetInconsistentWithNamedZoneSource),
    /// A backend-native numeric component does not fit the target
    /// integer width.
    #[cfg(feature = "jiff")]
    #[display("{_0}")]
    ComponentNarrowingFailed(ComponentNarrowingFailedSource),
    /// A `jiff::tz::Offset` could not be constructed from a raw
    /// seconds value.
    #[cfg(feature = "jiff")]
    #[display("{_0}")]
    JiffOffsetConstructionFailed(JiffOffsetConstructionFailedSource),
    /// A `jiff::civil::Date` could not be constructed.
    #[cfg(feature = "jiff")]
    #[display("{_0}")]
    JiffDateConstructionFailed(JiffDateConstructionFailedSource),
    /// A `jiff::civil::Time` could not be constructed.
    #[cfg(feature = "jiff")]
    #[display("{_0}")]
    JiffTimeConstructionFailed(JiffTimeConstructionFailedSource),
    /// A `jiff::Span` could not be constructed.
    #[cfg(feature = "jiff")]
    #[display("{_0}")]
    JiffSpanConstructionFailed(JiffSpanConstructionFailedSource),
    /// A local civil date-time could not be pinned to a concrete
    /// instant.
    #[cfg(feature = "jiff")]
    #[display("{_0}")]
    JiffInstantPinFailed(JiffInstantPinFailedSource),
    /// A recurring interval's two offset endpoints are reversed.
    #[cfg(feature = "jiff")]
    #[display("{_0}")]
    IntervalEndpointsReversed(IntervalEndpointsReversedSource),
    /// A jiff IANA zone identifier failed to resolve.
    #[cfg(feature = "jiff")]
    #[display("{_0}")]
    JiffNamedZoneResolutionFailed(JiffNamedZoneResolutionFailedSource),
    /// A computed jiff offset is inconsistent with a named zone's own
    /// rules.
    #[cfg(feature = "jiff")]
    #[display("{_0}")]
    JiffOffsetInconsistentWithNamedZone(JiffOffsetInconsistentWithNamedZoneSource),
    /// A lossless precision adjustment would discard real precision.
    #[cfg(feature = "jiff")]
    #[display("{_0}")]
    LosslessPrecisionAdjustmentRejected(LosslessPrecisionAdjustmentRejectedSource),
    /// A `jiff::civil::DateTime` could not be rebuilt.
    #[cfg(feature = "jiff")]
    #[display("{_0}")]
    JiffDateTimeConstructionFailed(JiffDateTimeConstructionFailedSource),
}

/// Temporal trait-layer error with call-site location.
#[derive(Debug, Display, Error, Getters)]
#[display("{kind} at {file}:{line}")]
pub struct TemporalError {
    /// The specific error kind, boxed to keep `TemporalError` itself
    /// small regardless of how large any one `TemporalErrorKind`
    /// variant grows.
    #[error(source)]
    kind: Box<TemporalErrorKind>,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl TemporalError {
    /// Create a [`TemporalError`] capturing the call-site location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(kind)))]
    pub fn new(kind: TemporalErrorKind) -> Self {
        #[cfg(kani)]
        let (line, file) = (0, String::new());
        #[cfg(not(kani))]
        let (line, file) = {
            let loc = std::panic::Location::caller();
            (loc.line(), loc.file().to_string())
        };
        Self {
            kind: Box::new(kind),
            line,
            file,
        }
    }
}

/// Result type for temporal operations.
pub type TemporalResult<T> = Result<T, TemporalError>;
