//! Output tokens and their [`Establish`](amenable_core::Establish) edges.
//!
//! One private-field unit struct + `#[amenable_derive::establish]` per
//! parse-method proposition (a `proof_composition` `*Valid` for the
//! single-proof methods, a [`super::parse_props`] composite for the
//! multi-proof ones). The verifier-less form generates `impl<V: Verifier>
//! Establish<TemporalInputToken, V> for P where P: Witness<V>` -- the
//! `amenable_gaap::tokens` pattern. A backend's `Exchange` body mints the
//! output token via `<P as Establish<TemporalInputToken, V>>::establish(
//! input.sidecar())`.
//!
//! Split by real domain -- matching `exchange::format`/`format_props`'s
//! own split, since every parse-side token here feeds exactly the
//! formatted-output token(s) in the same domain (its `credential`
//! names the parse token directly). `calendar_date` and `extension`
//! each hold more than 10 types, so each is further split by
//! sub-concern into its own directory.

mod calendar_date;
mod clock;
mod datetime;
mod extension;
mod interval;
mod timestamp;

pub use calendar_date::{
    CalendarDateBasicFormattedToken, CalendarDateExtendedFormattedToken, CalendarDateValidToken,
    OrdinalDateBasicFormattedToken, OrdinalDateExtendedFormattedToken, OrdinalDateValidToken,
    ReducedCalendarDateBasicFormattedToken, ReducedCalendarDateExtendedFormattedToken,
    ReducedCalendarDateValidToken, WeekDateBasicFormattedToken, WeekDateExtendedFormattedToken,
    WeekDateValidToken,
};
pub use clock::{
    LocalTimeBasicFormattedToken, LocalTimeExtendedFormattedToken, LocalTimeValidToken,
    ReducedLocalTimeBasicFormattedToken, ReducedLocalTimeExtendedFormattedToken,
    ReducedLocalTimeValidToken, UtcOffsetBasicFormattedToken, UtcOffsetExtendedFormattedToken,
    UtcOffsetValidToken,
};
pub use datetime::{
    LocalDateTimeBasicFormattedToken, LocalDateTimeExtendedFormattedToken, LocalDateTimeProofToken,
    OffsetDateTimeBasicFormattedToken, OffsetDateTimeExtendedFormattedToken,
    OffsetDateTimeProofToken,
};
pub use extension::{
    CenturyFormattedToken, CenturyValidToken, DateTimeFormulaFormattedToken,
    DateTimeFormulaProofToken, DateWithShiftFormattedToken, DateWithShiftValidToken,
    DecadeFormattedToken, DecadeValidToken, ExtendedYearFormattedToken, ExtendedYearValidToken,
    GroupedTimeScaleUnitFormattedToken, GroupedTimeScaleUnitProofToken,
    QualifiedTemporalValueFormattedToken, QualifiedTemporalValueProofToken,
    SeasonalTemporalExpressionFormattedToken, SeasonalTemporalExpressionProofToken,
    SubYearGroupingExpressionFormattedToken, SubYearGroupingExpressionProofToken,
    TemporalSetFormattedToken, TemporalSetProofToken, TimeOfDayWithShiftFormattedToken,
    TimeOfDayWithShiftValidToken, UnspecifiedComponentExpressionFormattedToken,
    UnspecifiedComponentExpressionProofToken,
};
pub use interval::{
    DurationFormValidToken, DurationFormattedToken, RecurringIntervalFormValidToken,
    RecurringIntervalFormattedToken, TimeIntervalFormattedToken, TimeIntervalProofToken,
};
pub use timestamp::{
    IxdtfTimestampFormattedToken, IxdtfTimestampProofToken, IxdtfZonedTimestampFormattedToken,
    IxdtfZonedTimestampProofToken, Rfc3339TimestampFormattedToken, Rfc3339TimestampProofToken,
};
