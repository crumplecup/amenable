//! The temporal exchange surface — every `elicit_temporal` trait method
//! re-expressed as an [`Exchange`](amenable_core::Exchange): a proven
//! [`Sidecar`](amenable_core::Sidecar) in, a proven `Sidecar` out.
//!
//! `amenable_time` owns the *shape* — the sidecar types, the boundary
//! tokens, the [`Establish`](amenable_core::Establish) edges, and the
//! [`TemporalParser<V>`](crate::TemporalParser) bundle (whose contract
//! *is* "be the 24 parse exchanges"). The `Exchange` **impls** live in
//! the backend crate: a downstream `Jiff` writes one inherent
//! `fn parse_x(&self, RawInput) -> Result<ParsedX, TemporalError>` per
//! method and `#[amenable_derive::capture_exchange_body]` generates its
//! `impl<V> Exchange<..> for Jiff`. `amenable_time` cannot provide those
//! — the orphan rule forbids `impl<T: …, V> Exchange<RawInput, …, V> for
//! T` (foreign trait, uncovered `Self`).
//!
//! - [`markers`] / [`tokens`] — boundary `Evidence` markers, root input token.
//! - [`establish`] — the 24 output tokens + their `Establish` edges.
//! - [`sidecars`] — [`RawInput`], the shared input sidecar.
//! - [`parse_props`] — per-method composite propositions (multi-proof methods).
//! - [`parse`] — the 24 per-method output sidecars.

mod establish;
mod markers;
mod parse;
mod parse_props;
mod sidecars;
mod tokens;

pub use establish::{
    CalendarDateValidToken, CenturyValidToken, DateTimeFormulaProofToken, DateWithShiftValidToken,
    DecadeValidToken, ExtendedYearValidToken, GroupedTimeScaleUnitProofToken,
    IxdtfTimestampProofToken, LocalDateTimeProofToken, LocalTimeValidToken,
    OffsetDateTimeProofToken, OrdinalDateValidToken, QualifiedTemporalValueProofToken,
    ReducedCalendarDateValidToken, ReducedLocalTimeValidToken, Rfc3339TimestampProofToken,
    SeasonalTemporalExpressionProofToken, SubYearGroupingExpressionProofToken,
    TemporalSetProofToken, TimeIntervalProofToken, TimeOfDayWithShiftValidToken,
    UnspecifiedComponentExpressionProofToken, UtcOffsetValidToken, WeekDateValidToken,
};
pub use markers::{RawTemporalText, TemporalInputReceived};
pub use parse::{
    ParsedCalendarDate, ParsedCentury, ParsedDateTimeFormula, ParsedDateWithShift, ParsedDecade,
    ParsedExtendedYear, ParsedGroupedTimeScaleUnit, ParsedIxdtfTimestamp, ParsedLocalDateTime,
    ParsedLocalTime, ParsedOffsetDateTime, ParsedOrdinalDate, ParsedQualifiedTemporalValue,
    ParsedReducedCalendarDate, ParsedReducedLocalTime, ParsedRfc3339Timestamp,
    ParsedSeasonalTemporalExpression, ParsedSubYearGroupingExpression, ParsedTemporalSet,
    ParsedTimeInterval, ParsedTimeOfDayWithShift, ParsedUnspecifiedComponentExpression,
    ParsedUtcOffset, ParsedWeekDate,
};
pub use parse_props::{
    DateTimeFormulaProof, GroupedTimeScaleUnitProof, IxdtfTimestampProof, LocalDateTimeProof,
    OffsetDateTimeProof, QualifiedTemporalValueProof, Rfc3339TimestampProof,
    SeasonalTemporalExpressionProof, SubYearGroupingExpressionProof, TemporalSetProof,
    TimeIntervalProof, UnspecifiedComponentExpressionProof,
};
pub use sidecars::RawInput;
pub use tokens::TemporalInputToken;
