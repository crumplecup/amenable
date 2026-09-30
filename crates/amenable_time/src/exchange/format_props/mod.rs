//! Per-method emission-proof composites — each formatter method's
//! output proof(s) folded into one `#[derive(Evidence, Witness)]`
//! struct (the same folding as `proof_composition`), so the `Formatted*`
//! output sidecar keeps its single-`token` shape.
//!
//! Split by family: `calendar_date`, `clock`, `datetime`, `timestamp`,
//! `interval`, and `extension` (the CalConnect/ISO 8601-2 constructs)
//! — matching `exchange::format`'s own split, since every type here
//! is the proof composite for that module's matching sidecar.

mod calendar_date;
mod clock;
mod datetime;
mod extension;
mod interval;
mod timestamp;

pub use calendar_date::{
    CalendarDateBasicFormatted, CalendarDateExtendedFormatted, OrdinalDateBasicFormatted,
    OrdinalDateExtendedFormatted, ReducedCalendarDateBasicFormatted,
    ReducedCalendarDateExtendedFormatted, WeekDateBasicFormatted, WeekDateExtendedFormatted,
};
pub use clock::{
    LocalTimeBasicFormatted, LocalTimeExtendedFormatted, ReducedLocalTimeBasicFormatted,
    ReducedLocalTimeExtendedFormatted, UtcOffsetBasicFormatted, UtcOffsetExtendedFormatted,
};
pub use datetime::{
    LocalDateTimeBasicFormatted, LocalDateTimeExtendedFormatted, OffsetDateTimeBasicFormatted,
    OffsetDateTimeExtendedFormatted,
};
pub use extension::{
    CenturyFormatted, DateTimeFormulaFormatted, DateWithShiftFormatted, DecadeFormatted,
    ExtendedYearFormatted, GroupedTimeScaleUnitFormatted, QualifiedTemporalValueFormatted,
    SeasonalTemporalExpressionFormatted, SubYearGroupingExpressionFormatted, TemporalSetFormatted,
    TimeOfDayWithShiftFormatted, UnspecifiedComponentExpressionFormatted,
};
pub use interval::{DurationFormatted, RecurringIntervalFormatted, TimeIntervalFormatted};
pub use timestamp::{
    IxdtfTimestampFormatted, IxdtfZonedTimestampFormatted, Rfc3339TimestampFormatted,
};
