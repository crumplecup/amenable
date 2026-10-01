//! Qualified-value/seasonal/sub-year-grouping/unspecified-component formatter output sidecars.
//!
//!
//! One `#[derive(Sidecar)]` struct per
//! [`TemporalFormatter`](crate::TemporalFormatter) method
//! (`(String, Established<EmissionProof>)`, named): `#[sidecar(primary)]`
//! is the emitted text, `#[sidecar(token)]` the emission-conformance
//! proof. The `Exchange` impls live in the backend crate.

use crate::{
    QualifiedTemporalValueFormattedToken, SeasonalTemporalExpressionFormattedToken,
    SubYearGroupingExpressionFormattedToken, UnspecifiedComponentExpressionFormattedToken,
};

/// Output sidecar for the `format_qualified_temporal_value` exchange: the emitted text plus
/// a token for [`QualifiedTemporalValueFormatted`](crate::QualifiedTemporalValueFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::QualifiedTemporalValueFormatted",
    constructor = "pub"
)]
pub struct FormattedQualifiedTemporalValue {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: QualifiedTemporalValueFormattedToken,
}

impl FormattedQualifiedTemporalValue {
    /// Borrow the emitted text.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}
/// Output sidecar for the `format_seasonal_temporal_expression` exchange: the emitted text plus
/// a token for [`SeasonalTemporalExpressionFormatted`](crate::SeasonalTemporalExpressionFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::SeasonalTemporalExpressionFormatted",
    constructor = "pub"
)]
pub struct FormattedSeasonalTemporalExpression {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: SeasonalTemporalExpressionFormattedToken,
}

impl FormattedSeasonalTemporalExpression {
    /// Borrow the emitted text.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}
/// Output sidecar for the `format_sub_year_grouping_expression` exchange: the emitted text plus
/// a token for [`SubYearGroupingExpressionFormatted`](crate::SubYearGroupingExpressionFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::SubYearGroupingExpressionFormatted",
    constructor = "pub"
)]
pub struct FormattedSubYearGroupingExpression {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: SubYearGroupingExpressionFormattedToken,
}

impl FormattedSubYearGroupingExpression {
    /// Borrow the emitted text.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}
/// Output sidecar for the `format_unspecified_component_expression` exchange: the emitted text plus
/// a token for [`UnspecifiedComponentExpressionFormatted`](crate::UnspecifiedComponentExpressionFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::UnspecifiedComponentExpressionFormatted",
    constructor = "pub"
)]
pub struct FormattedUnspecifiedComponentExpression {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: UnspecifiedComponentExpressionFormattedToken,
}

impl FormattedUnspecifiedComponentExpression {
    /// Borrow the emitted text.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}
