//! Formatter exchange output sidecars — one `#[derive(Sidecar)]`
//! struct per [`TemporalFormatter`](crate::TemporalFormatter) method
//! (`(String, Established<EmissionProof>)`, named): `#[sidecar(primary)]`
//! is the emitted text, `#[sidecar(token)]` the emission-conformance
//! proof. The `Exchange` impls live in the backend crate.
//!
//! Split by family: `calendar_date`, `clock`, `datetime`, `timestamp`,
//! `interval`, and `extension` (the CalConnect/ISO 8601-2 constructs).

mod calendar_date;
mod clock;
mod datetime;
mod extension;
mod interval;
mod timestamp;

pub use calendar_date::{
    FormattedCalendarDateBasic, FormattedCalendarDateExtended, FormattedOrdinalDateBasic,
    FormattedOrdinalDateExtended, FormattedReducedCalendarDateBasic,
    FormattedReducedCalendarDateExtended, FormattedWeekDateBasic, FormattedWeekDateExtended,
};
pub use clock::{
    FormattedLocalTimeBasic, FormattedLocalTimeExtended, FormattedReducedLocalTimeBasic,
    FormattedReducedLocalTimeExtended, FormattedUtcOffsetBasic, FormattedUtcOffsetExtended,
};
pub use datetime::{
    FormattedLocalDateTimeBasic, FormattedLocalDateTimeExtended, FormattedOffsetDateTimeBasic,
    FormattedOffsetDateTimeExtended,
};
pub use extension::{
    FormattedCentury, FormattedDateTimeFormula, FormattedDateWithShift, FormattedDecade,
    FormattedExtendedYear, FormattedGroupedTimeScaleUnit, FormattedQualifiedTemporalValue,
    FormattedSeasonalTemporalExpression, FormattedSubYearGroupingExpression, FormattedTemporalSet,
    FormattedTimeOfDayWithShift, FormattedUnspecifiedComponentExpression,
};
pub use interval::{FormattedDuration, FormattedRecurringInterval, FormattedTimeInterval};
pub use timestamp::{
    FormattedIxdtfTimestamp, FormattedIxdtfZonedTimestamp, FormattedRfc3339Timestamp,
};
