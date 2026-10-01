//! Emission-proof composites for the CalConnect/ISO 8601-2 extension family.
//!
//! Each formatter method's output proof(s) folded into one
//! `#[derive(Evidence, Witness)]` struct (the same folding as
//! `proof_composition`), so the matching `Formatted*` output sidecar
//! keeps its single-`token` shape.
//!
//! Split further by sub-concern since this domain holds more than 10
//! types: `with_shift`, `extended_year_forms`, `precision_variants`,
//! `derived` — matching `exchange::establish`/`::parse`'s own
//! extension sub-split.

mod derived;
mod extended_year_forms;
mod precision_variants;
mod with_shift;

pub use derived::{DateTimeFormulaFormatted, GroupedTimeScaleUnitFormatted, TemporalSetFormatted};
pub use extended_year_forms::{CenturyFormatted, DecadeFormatted, ExtendedYearFormatted};
pub use precision_variants::{
    QualifiedTemporalValueFormatted, SeasonalTemporalExpressionFormatted,
    SubYearGroupingExpressionFormatted, UnspecifiedComponentExpressionFormatted,
};
pub use with_shift::{DateWithShiftFormatted, TimeOfDayWithShiftFormatted};
