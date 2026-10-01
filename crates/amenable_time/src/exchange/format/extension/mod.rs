//! Formatter output sidecars for the CalConnect/ISO 8601-2 extension family
//! (reduced-precision year forms, explicit shifts, and the
//! qualified/seasonal/grouped/set/formula constructs).
//!
//! One `#[derive(Sidecar)]` struct per
//! [`TemporalFormatter`](crate::TemporalFormatter) method
//! (`(String, Established<EmissionProof>)`, named): `#[sidecar(primary)]`
//! is the emitted text, `#[sidecar(token)]` the emission-conformance
//! proof. The `Exchange` impls live in the backend crate.
//!
//! Split further by sub-concern since this domain holds more than 10
//! types: `with_shift`, `extended_year_forms`, `precision_variants`,
//! `derived` — matching `format_props::extension`'s own split.

mod derived;
mod extended_year_forms;
mod precision_variants;
mod with_shift;

pub use derived::{FormattedDateTimeFormula, FormattedGroupedTimeScaleUnit, FormattedTemporalSet};
pub use extended_year_forms::{FormattedCentury, FormattedDecade, FormattedExtendedYear};
pub use precision_variants::{
    FormattedQualifiedTemporalValue, FormattedSeasonalTemporalExpression,
    FormattedSubYearGroupingExpression, FormattedUnspecifiedComponentExpression,
};
pub use with_shift::{FormattedDateWithShift, FormattedTimeOfDayWithShift};
