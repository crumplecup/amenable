//! Formatter exchange output sidecars — one `#[derive(Sidecar)]`
//! struct per [`TemporalFormatter`](crate::TemporalFormatter) method
//! (`(String, Established<EmissionProof>)`, named): `#[sidecar(primary)]`
//! is the emitted text, `#[sidecar(token)]` the emission-conformance
//! proof. The `Exchange` impls live in the backend crate.

use crate::{
    CalendarDateBasicFormattedToken, CalendarDateExtendedFormattedToken, CenturyFormattedToken,
    DateTimeFormulaFormattedToken, DateWithShiftFormattedToken, DecadeFormattedToken,
    DurationFormattedToken, ExtendedYearFormattedToken, GroupedTimeScaleUnitFormattedToken,
    IxdtfTimestampFormattedToken, IxdtfZonedTimestampFormattedToken,
    LocalDateTimeBasicFormattedToken, LocalDateTimeExtendedFormattedToken,
    LocalTimeBasicFormattedToken, LocalTimeExtendedFormattedToken,
    OffsetDateTimeBasicFormattedToken, OffsetDateTimeExtendedFormattedToken,
    OrdinalDateBasicFormattedToken, OrdinalDateExtendedFormattedToken,
    QualifiedTemporalValueFormattedToken, RecurringIntervalFormattedToken,
    ReducedCalendarDateBasicFormattedToken, ReducedCalendarDateExtendedFormattedToken,
    ReducedLocalTimeBasicFormattedToken, ReducedLocalTimeExtendedFormattedToken,
    Rfc3339TimestampFormattedToken, SeasonalTemporalExpressionFormattedToken,
    SubYearGroupingExpressionFormattedToken, TemporalSetFormattedToken, TimeIntervalFormattedToken,
    TimeOfDayWithShiftFormattedToken, UnspecifiedComponentExpressionFormattedToken,
    UtcOffsetBasicFormattedToken, UtcOffsetExtendedFormattedToken, WeekDateBasicFormattedToken,
    WeekDateExtendedFormattedToken,
};

/// Output sidecar for the `format_calendar_date_extended` exchange: the emitted text plus
/// a token for [`CalendarDateExtendedFormatted`](crate::CalendarDateExtendedFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::CalendarDateExtendedFormatted",
    constructor = "pub"
)]
pub struct FormattedCalendarDateExtended {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: CalendarDateExtendedFormattedToken,
}

impl FormattedCalendarDateExtended {
    /// Borrow the emitted text.
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}

/// Output sidecar for the `format_calendar_date_basic` exchange: the emitted text plus
/// a token for [`CalendarDateBasicFormatted`](crate::CalendarDateBasicFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::CalendarDateBasicFormatted", constructor = "pub")]
pub struct FormattedCalendarDateBasic {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: CalendarDateBasicFormattedToken,
}

impl FormattedCalendarDateBasic {
    /// Borrow the emitted text.
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}

/// Output sidecar for the `format_reduced_calendar_date_extended` exchange: the emitted text plus
/// a token for [`ReducedCalendarDateExtendedFormatted`](crate::ReducedCalendarDateExtendedFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::ReducedCalendarDateExtendedFormatted",
    constructor = "pub"
)]
pub struct FormattedReducedCalendarDateExtended {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: ReducedCalendarDateExtendedFormattedToken,
}

impl FormattedReducedCalendarDateExtended {
    /// Borrow the emitted text.
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}

/// Output sidecar for the `format_reduced_calendar_date_basic` exchange: the emitted text plus
/// a token for [`ReducedCalendarDateBasicFormatted`](crate::ReducedCalendarDateBasicFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::ReducedCalendarDateBasicFormatted",
    constructor = "pub"
)]
pub struct FormattedReducedCalendarDateBasic {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: ReducedCalendarDateBasicFormattedToken,
}

impl FormattedReducedCalendarDateBasic {
    /// Borrow the emitted text.
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}

/// Output sidecar for the `format_ordinal_date_extended` exchange: the emitted text plus
/// a token for [`OrdinalDateExtendedFormatted`](crate::OrdinalDateExtendedFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::OrdinalDateExtendedFormatted",
    constructor = "pub"
)]
pub struct FormattedOrdinalDateExtended {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: OrdinalDateExtendedFormattedToken,
}

impl FormattedOrdinalDateExtended {
    /// Borrow the emitted text.
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}

/// Output sidecar for the `format_ordinal_date_basic` exchange: the emitted text plus
/// a token for [`OrdinalDateBasicFormatted`](crate::OrdinalDateBasicFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::OrdinalDateBasicFormatted", constructor = "pub")]
pub struct FormattedOrdinalDateBasic {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: OrdinalDateBasicFormattedToken,
}

impl FormattedOrdinalDateBasic {
    /// Borrow the emitted text.
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}

/// Output sidecar for the `format_week_date_extended` exchange: the emitted text plus
/// a token for [`WeekDateExtendedFormatted`](crate::WeekDateExtendedFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::WeekDateExtendedFormatted", constructor = "pub")]
pub struct FormattedWeekDateExtended {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: WeekDateExtendedFormattedToken,
}

impl FormattedWeekDateExtended {
    /// Borrow the emitted text.
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}

/// Output sidecar for the `format_week_date_basic` exchange: the emitted text plus
/// a token for [`WeekDateBasicFormatted`](crate::WeekDateBasicFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::WeekDateBasicFormatted", constructor = "pub")]
pub struct FormattedWeekDateBasic {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: WeekDateBasicFormattedToken,
}

impl FormattedWeekDateBasic {
    /// Borrow the emitted text.
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}

/// Output sidecar for the `format_local_time_extended` exchange: the emitted text plus
/// a token for [`LocalTimeExtendedFormatted`](crate::LocalTimeExtendedFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::LocalTimeExtendedFormatted", constructor = "pub")]
pub struct FormattedLocalTimeExtended {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: LocalTimeExtendedFormattedToken,
}

impl FormattedLocalTimeExtended {
    /// Borrow the emitted text.
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}

/// Output sidecar for the `format_local_time_basic` exchange: the emitted text plus
/// a token for [`LocalTimeBasicFormatted`](crate::LocalTimeBasicFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::LocalTimeBasicFormatted", constructor = "pub")]
pub struct FormattedLocalTimeBasic {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: LocalTimeBasicFormattedToken,
}

impl FormattedLocalTimeBasic {
    /// Borrow the emitted text.
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}

/// Output sidecar for the `format_reduced_local_time_extended` exchange: the emitted text plus
/// a token for [`ReducedLocalTimeExtendedFormatted`](crate::ReducedLocalTimeExtendedFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::ReducedLocalTimeExtendedFormatted",
    constructor = "pub"
)]
pub struct FormattedReducedLocalTimeExtended {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: ReducedLocalTimeExtendedFormattedToken,
}

impl FormattedReducedLocalTimeExtended {
    /// Borrow the emitted text.
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}

/// Output sidecar for the `format_reduced_local_time_basic` exchange: the emitted text plus
/// a token for [`ReducedLocalTimeBasicFormatted`](crate::ReducedLocalTimeBasicFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::ReducedLocalTimeBasicFormatted",
    constructor = "pub"
)]
pub struct FormattedReducedLocalTimeBasic {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: ReducedLocalTimeBasicFormattedToken,
}

impl FormattedReducedLocalTimeBasic {
    /// Borrow the emitted text.
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}

/// Output sidecar for the `format_utc_offset_extended` exchange: the emitted text plus
/// a token for [`UtcOffsetExtendedFormatted`](crate::UtcOffsetExtendedFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::UtcOffsetExtendedFormatted", constructor = "pub")]
pub struct FormattedUtcOffsetExtended {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: UtcOffsetExtendedFormattedToken,
}

impl FormattedUtcOffsetExtended {
    /// Borrow the emitted text.
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}

/// Output sidecar for the `format_utc_offset_basic` exchange: the emitted text plus
/// a token for [`UtcOffsetBasicFormatted`](crate::UtcOffsetBasicFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::UtcOffsetBasicFormatted", constructor = "pub")]
pub struct FormattedUtcOffsetBasic {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: UtcOffsetBasicFormattedToken,
}

impl FormattedUtcOffsetBasic {
    /// Borrow the emitted text.
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}

/// Output sidecar for the `format_local_date_time_extended` exchange: the emitted text plus
/// a token for [`LocalDateTimeExtendedFormatted`](crate::LocalDateTimeExtendedFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::LocalDateTimeExtendedFormatted",
    constructor = "pub"
)]
pub struct FormattedLocalDateTimeExtended {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: LocalDateTimeExtendedFormattedToken,
}

impl FormattedLocalDateTimeExtended {
    /// Borrow the emitted text.
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}

/// Output sidecar for the `format_local_date_time_basic` exchange: the emitted text plus
/// a token for [`LocalDateTimeBasicFormatted`](crate::LocalDateTimeBasicFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::LocalDateTimeBasicFormatted",
    constructor = "pub"
)]
pub struct FormattedLocalDateTimeBasic {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: LocalDateTimeBasicFormattedToken,
}

impl FormattedLocalDateTimeBasic {
    /// Borrow the emitted text.
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}

/// Output sidecar for the `format_offset_date_time_extended` exchange: the emitted text plus
/// a token for [`OffsetDateTimeExtendedFormatted`](crate::OffsetDateTimeExtendedFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::OffsetDateTimeExtendedFormatted",
    constructor = "pub"
)]
pub struct FormattedOffsetDateTimeExtended {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: OffsetDateTimeExtendedFormattedToken,
}

impl FormattedOffsetDateTimeExtended {
    /// Borrow the emitted text.
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}

/// Output sidecar for the `format_offset_date_time_basic` exchange: the emitted text plus
/// a token for [`OffsetDateTimeBasicFormatted`](crate::OffsetDateTimeBasicFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::OffsetDateTimeBasicFormatted",
    constructor = "pub"
)]
pub struct FormattedOffsetDateTimeBasic {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: OffsetDateTimeBasicFormattedToken,
}

impl FormattedOffsetDateTimeBasic {
    /// Borrow the emitted text.
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}

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
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}

/// Output sidecar for the `format_rfc3339_timestamp` exchange: the emitted text plus
/// a token for [`Rfc3339TimestampFormatted`](crate::Rfc3339TimestampFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::Rfc3339TimestampFormatted", constructor = "pub")]
pub struct FormattedRfc3339Timestamp {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: Rfc3339TimestampFormattedToken,
}

impl FormattedRfc3339Timestamp {
    /// Borrow the emitted text.
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}

/// Output sidecar for the `format_ixdtf_zoned_timestamp` exchange: the emitted text plus
/// a token for [`IxdtfZonedTimestampFormatted`](crate::IxdtfZonedTimestampFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::IxdtfZonedTimestampFormatted",
    constructor = "pub"
)]
pub struct FormattedIxdtfZonedTimestamp {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: IxdtfZonedTimestampFormattedToken,
}

impl FormattedIxdtfZonedTimestamp {
    /// Borrow the emitted text.
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}

/// Output sidecar for the `format_ixdtf_timestamp` exchange: the emitted text plus
/// a token for [`IxdtfTimestampFormatted`](crate::IxdtfTimestampFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::IxdtfTimestampFormatted", constructor = "pub")]
pub struct FormattedIxdtfTimestamp {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: IxdtfTimestampFormattedToken,
}

impl FormattedIxdtfTimestamp {
    /// Borrow the emitted text.
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
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}

/// Output sidecar for the `format_duration` exchange: the emitted text plus
/// a token for [`DurationFormatted`](crate::DurationFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::DurationFormatted", constructor = "pub")]
pub struct FormattedDuration {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: DurationFormattedToken,
}

impl FormattedDuration {
    /// Borrow the emitted text.
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}

/// Output sidecar for the `format_recurring_interval` exchange: the emitted text plus
/// a token for [`RecurringIntervalFormatted`](crate::RecurringIntervalFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::RecurringIntervalFormatted", constructor = "pub")]
pub struct FormattedRecurringInterval {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: RecurringIntervalFormattedToken,
}

impl FormattedRecurringInterval {
    /// Borrow the emitted text.
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}

/// Output sidecar for the `format_time_interval` exchange: the emitted text plus
/// a token for [`TimeIntervalFormatted`](crate::TimeIntervalFormatted).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::TimeIntervalFormatted", constructor = "pub")]
pub struct FormattedTimeInterval {
    #[sidecar(primary)]
    text: crate::FormattedTemporalText,
    #[sidecar(token)]
    token: TimeIntervalFormattedToken,
}

impl FormattedTimeInterval {
    /// Borrow the emitted text.
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}
