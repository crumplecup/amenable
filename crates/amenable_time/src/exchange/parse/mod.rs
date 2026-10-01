//! Parser exchange output sidecars — one `#[derive(Sidecar)]` struct
//! per [`TemporalParser`](crate::TemporalParser) method (the
//! `elicit_temporal` return tuple, named). Field 1 is the descriptor
//! (`#[sidecar(primary)]`), field 2 the proof token
//! (`#[sidecar(token)]`). The `Exchange` impls live in the backend
//! crate (`#[capture_exchange_body]`); `amenable_time` ships the shape.
//!
//! Split by real domain — matching `exchange::establish`'s own split,
//! since every parse-output sidecar here feeds exactly the matching
//! `establish` token in the same domain. `extension` holds more than
//! 10 types, so it's further split by sub-concern into its own
//! directory.

mod calendar_date;
mod clock;
mod datetime;
mod extension;
mod interval;
mod timestamp;

pub use calendar_date::{
    ParsedCalendarDate, ParsedOrdinalDate, ParsedReducedCalendarDate, ParsedWeekDate,
};
pub use clock::{ParsedLocalTime, ParsedReducedLocalTime, ParsedUtcOffset};
pub use datetime::{ParsedLocalDateTime, ParsedOffsetDateTime};
pub use extension::{
    ParsedCentury, ParsedDateTimeFormula, ParsedDateWithShift, ParsedDecade, ParsedExtendedYear,
    ParsedGroupedTimeScaleUnit, ParsedQualifiedTemporalValue, ParsedSeasonalTemporalExpression,
    ParsedSubYearGroupingExpression, ParsedTemporalSet, ParsedTimeOfDayWithShift,
    ParsedUnspecifiedComponentExpression,
};
pub use interval::{ParsedDuration, ParsedRecurringInterval, ParsedTimeInterval};
pub use timestamp::{ParsedIxdtfTimestamp, ParsedIxdtfZonedTimestamp, ParsedRfc3339Timestamp};
