//! Temporal-set/grouped-time-scale-unit/date-time-formula formatter output sidecars.
//!
//!
//! One `#[derive(Sidecar)]` struct per
//! [`TemporalFormatter`](crate::TemporalFormatter) method
//! (`(String, Established<EmissionProof>)`, named): `#[sidecar(primary)]`
//! is the emitted text, `#[sidecar(token)]` the emission-conformance
//! proof. The `Exchange` impls live in the backend crate.

use crate::{
    DateTimeFormulaFormattedToken, GroupedTimeScaleUnitFormattedToken, TemporalSetFormattedToken,
};

/// Output sidecar for the `format_temporal_set` exchange: the emitted text plus
/// a token for [`TemporalSetFormatted`](crate::TemporalSetFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::TemporalSetFormatted", constructor = "pub")]
pub struct FormattedTemporalSet {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: TemporalSetFormattedToken,
}

impl FormattedTemporalSet {
    /// Borrow the emitted text.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}
/// Output sidecar for the `format_grouped_time_scale_unit` exchange: the emitted text plus
/// a token for [`GroupedTimeScaleUnitFormatted`](crate::GroupedTimeScaleUnitFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::GroupedTimeScaleUnitFormatted",
    constructor = "pub"
)]
pub struct FormattedGroupedTimeScaleUnit {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: GroupedTimeScaleUnitFormattedToken,
}

impl FormattedGroupedTimeScaleUnit {
    /// Borrow the emitted text.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}
/// Output sidecar for the `format_date_time_formula` exchange: the emitted text plus
/// a token for [`DateTimeFormulaFormatted`](crate::DateTimeFormulaFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::DateTimeFormulaFormatted", constructor = "pub")]
pub struct FormattedDateTimeFormula {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: DateTimeFormulaFormattedToken,
}

impl FormattedDateTimeFormula {
    /// Borrow the emitted text.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}
