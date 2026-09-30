//! Formatter output sidecars for the CalConnect/ISO 8601-2 extension family (reduced-precision year forms, explicit shifts, and the qualified/seasonal/grouped/set/formula constructs).
//!
//! One `#[derive(Sidecar)]` struct per
//! [`TemporalFormatter`](crate::TemporalFormatter) method
//! (`(String, Established<EmissionProof>)`, named): `#[sidecar(primary)]`
//! is the emitted text, `#[sidecar(token)]` the emission-conformance
//! proof. The `Exchange` impls live in the backend crate.

use crate::{
    CenturyFormattedToken, DateTimeFormulaFormattedToken, DateWithShiftFormattedToken,
    DecadeFormattedToken, ExtendedYearFormattedToken, GroupedTimeScaleUnitFormattedToken,
    QualifiedTemporalValueFormattedToken, SeasonalTemporalExpressionFormattedToken,
    SubYearGroupingExpressionFormattedToken, TemporalSetFormattedToken,
    TimeOfDayWithShiftFormattedToken, UnspecifiedComponentExpressionFormattedToken,
};

/// Output sidecar for the `format_date_with_shift` exchange: the emitted text plus
/// a token for [`DateWithShiftFormatted`](crate::DateWithShiftFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::DateWithShiftFormatted", constructor = "pub")]
pub struct FormattedDateWithShift {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: DateWithShiftFormattedToken,
}

impl FormattedDateWithShift {
    /// Borrow the emitted text.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}

/// Output sidecar for the `format_time_of_day_with_shift` exchange: the emitted text plus
/// a token for [`TimeOfDayWithShiftFormatted`](crate::TimeOfDayWithShiftFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::TimeOfDayWithShiftFormatted",
    constructor = "pub"
)]
pub struct FormattedTimeOfDayWithShift {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: TimeOfDayWithShiftFormattedToken,
}

impl FormattedTimeOfDayWithShift {
    /// Borrow the emitted text.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}

/// Output sidecar for the `format_extended_year` exchange: the emitted text plus
/// a token for [`ExtendedYearFormatted`](crate::ExtendedYearFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::ExtendedYearFormatted", constructor = "pub")]
pub struct FormattedExtendedYear {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: ExtendedYearFormattedToken,
}

impl FormattedExtendedYear {
    /// Borrow the emitted text.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}

/// Output sidecar for the `format_decade` exchange: the emitted text plus
/// a token for [`DecadeFormatted`](crate::DecadeFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::DecadeFormatted", constructor = "pub")]
pub struct FormattedDecade {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: DecadeFormattedToken,
}

impl FormattedDecade {
    /// Borrow the emitted text.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}

/// Output sidecar for the `format_century` exchange: the emitted text plus
/// a token for [`CenturyFormatted`](crate::CenturyFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::CenturyFormatted", constructor = "pub")]
pub struct FormattedCentury {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: CenturyFormattedToken,
}

impl FormattedCentury {
    /// Borrow the emitted text.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}

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
