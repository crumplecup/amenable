//! Parser exchange output sidecars — one `#[derive(Sidecar)]` struct
//! per [`TemporalParser`](crate::TemporalParser) method (the
//! `elicit_temporal` return tuple, named). Field 1 is the descriptor
//! (`#[sidecar(primary)]`), field 2 the proof token
//! (`#[sidecar(token)]`). The `Exchange` impls live in the backend
//! crate (`#[capture_exchange_body]`); `amenable_time` ships the shape.

use crate::{
    CalendarDateDescriptor, CalendarDateValidToken, CenturyDescriptor, CenturyValidToken,
    DateTimeFormulaDescriptor, DateTimeFormulaProofToken, DateWithShiftDescriptor,
    DateWithShiftValidToken, DecadeDescriptor, DecadeValidToken, DurationDescriptor,
    DurationFormValidToken, ExtendedYearDescriptor, ExtendedYearValidToken,
    GroupedTimeScaleUnitDescriptor, GroupedTimeScaleUnitProofToken, IxdtfTimestampDescriptor,
    IxdtfTimestampProofToken, IxdtfZonedTimestampProofToken, LocalDateTimeDescriptor,
    LocalDateTimeProofToken, LocalTimeDescriptor, LocalTimeValidToken, OffsetDateTimeDescriptor,
    OffsetDateTimeProofToken, OrdinalDateDescriptor, OrdinalDateValidToken,
    QualifiedTemporalValueDescriptor, QualifiedTemporalValueProofToken,
    RecurringIntervalDescriptor, RecurringIntervalFormValidToken, ReducedCalendarDateDescriptor,
    ReducedCalendarDateValidToken, ReducedLocalTimeDescriptor, ReducedLocalTimeValidToken,
    Rfc3339TimestampProofToken, SeasonalTemporalExpressionDescriptor,
    SeasonalTemporalExpressionProofToken, SubYearGroupingExpressionDescriptor,
    SubYearGroupingExpressionProofToken, TemporalSetDescriptor, TemporalSetProofToken,
    TimeIntervalDescriptor, TimeIntervalProofToken, TimeOfDayWithShiftDescriptor,
    TimeOfDayWithShiftValidToken, UnspecifiedComponentExpressionDescriptor,
    UnspecifiedComponentExpressionProofToken, UtcOffsetDescriptor, UtcOffsetValidToken,
    WeekDateDescriptor, WeekDateValidToken, ZonedDateTimeDescriptor,
};

/// Output sidecar for the `parse_calendar_date` exchange: [`CalendarDateDescriptor`](crate::CalendarDateDescriptor)
/// plus a token for [`CalendarDateValid`](crate::CalendarDateValid).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::CalendarDateValid", constructor = "pub")]
pub struct ParsedCalendarDate {
    #[sidecar(primary)]
    descriptor: CalendarDateDescriptor,
    #[sidecar(token)]
    token: CalendarDateValidToken,
}

impl ParsedCalendarDate {
    /// Borrow the parsed descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &CalendarDateDescriptor {
        &self.descriptor
    }
}

/// Output sidecar for the `parse_reduced_calendar_date` exchange: [`ReducedCalendarDateDescriptor`](crate::ReducedCalendarDateDescriptor)
/// plus a token for [`ReducedCalendarDateValid`](crate::ReducedCalendarDateValid).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::ReducedCalendarDateValid", constructor = "pub")]
pub struct ParsedReducedCalendarDate {
    #[sidecar(primary)]
    descriptor: ReducedCalendarDateDescriptor,
    #[sidecar(token)]
    token: ReducedCalendarDateValidToken,
}

impl ParsedReducedCalendarDate {
    /// Borrow the parsed descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &ReducedCalendarDateDescriptor {
        &self.descriptor
    }
}

/// Output sidecar for the `parse_extended_year` exchange: [`ExtendedYearDescriptor`](crate::ExtendedYearDescriptor)
/// plus a token for [`ExtendedYearValid`](crate::ExtendedYearValid).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::ExtendedYearValid", constructor = "pub")]
pub struct ParsedExtendedYear {
    #[sidecar(primary)]
    descriptor: ExtendedYearDescriptor,
    #[sidecar(token)]
    token: ExtendedYearValidToken,
}

impl ParsedExtendedYear {
    /// Borrow the parsed descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &ExtendedYearDescriptor {
        &self.descriptor
    }
}

/// Output sidecar for the `parse_decade` exchange: [`DecadeDescriptor`](crate::DecadeDescriptor)
/// plus a token for [`DecadeValid`](crate::DecadeValid).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::DecadeValid", constructor = "pub")]
pub struct ParsedDecade {
    #[sidecar(primary)]
    descriptor: DecadeDescriptor,
    #[sidecar(token)]
    token: DecadeValidToken,
}

impl ParsedDecade {
    /// Borrow the parsed descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &DecadeDescriptor {
        &self.descriptor
    }
}

/// Output sidecar for the `parse_century` exchange: [`CenturyDescriptor`](crate::CenturyDescriptor)
/// plus a token for [`CenturyValid`](crate::CenturyValid).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::CenturyValid", constructor = "pub")]
pub struct ParsedCentury {
    #[sidecar(primary)]
    descriptor: CenturyDescriptor,
    #[sidecar(token)]
    token: CenturyValidToken,
}

impl ParsedCentury {
    /// Borrow the parsed descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &CenturyDescriptor {
        &self.descriptor
    }
}

/// Output sidecar for the `parse_qualified_temporal_value` exchange: [`QualifiedTemporalValueDescriptor`](crate::QualifiedTemporalValueDescriptor)
/// plus a token for [`QualifiedTemporalValueProof`](crate::QualifiedTemporalValueProof).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::QualifiedTemporalValueProof",
    constructor = "pub"
)]
pub struct ParsedQualifiedTemporalValue {
    #[sidecar(primary)]
    descriptor: QualifiedTemporalValueDescriptor,
    #[sidecar(token)]
    token: QualifiedTemporalValueProofToken,
}

impl ParsedQualifiedTemporalValue {
    /// Borrow the parsed descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &QualifiedTemporalValueDescriptor {
        &self.descriptor
    }
}

/// Output sidecar for the `parse_ordinal_date` exchange: [`OrdinalDateDescriptor`](crate::OrdinalDateDescriptor)
/// plus a token for [`OrdinalDateValid`](crate::OrdinalDateValid).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::OrdinalDateValid", constructor = "pub")]
pub struct ParsedOrdinalDate {
    #[sidecar(primary)]
    descriptor: OrdinalDateDescriptor,
    #[sidecar(token)]
    token: OrdinalDateValidToken,
}

impl ParsedOrdinalDate {
    /// Borrow the parsed descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &OrdinalDateDescriptor {
        &self.descriptor
    }
}

/// Output sidecar for the `parse_week_date` exchange: [`WeekDateDescriptor`](crate::WeekDateDescriptor)
/// plus a token for [`WeekDateValid`](crate::WeekDateValid).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::WeekDateValid", constructor = "pub")]
pub struct ParsedWeekDate {
    #[sidecar(primary)]
    descriptor: WeekDateDescriptor,
    #[sidecar(token)]
    token: WeekDateValidToken,
}

impl ParsedWeekDate {
    /// Borrow the parsed descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &WeekDateDescriptor {
        &self.descriptor
    }
}

/// Output sidecar for the `parse_local_time` exchange: [`LocalTimeDescriptor`](crate::LocalTimeDescriptor)
/// plus a token for [`LocalTimeValid`](crate::LocalTimeValid).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::LocalTimeValid", constructor = "pub")]
pub struct ParsedLocalTime {
    #[sidecar(primary)]
    descriptor: LocalTimeDescriptor,
    #[sidecar(token)]
    token: LocalTimeValidToken,
}

impl ParsedLocalTime {
    /// Borrow the parsed descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &LocalTimeDescriptor {
        &self.descriptor
    }
}

/// Output sidecar for the `parse_reduced_local_time` exchange: [`ReducedLocalTimeDescriptor`](crate::ReducedLocalTimeDescriptor)
/// plus a token for [`ReducedLocalTimeValid`](crate::ReducedLocalTimeValid).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::ReducedLocalTimeValid", constructor = "pub")]
pub struct ParsedReducedLocalTime {
    #[sidecar(primary)]
    descriptor: ReducedLocalTimeDescriptor,
    #[sidecar(token)]
    token: ReducedLocalTimeValidToken,
}

impl ParsedReducedLocalTime {
    /// Borrow the parsed descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &ReducedLocalTimeDescriptor {
        &self.descriptor
    }
}

/// Output sidecar for the `parse_utc_offset` exchange: [`UtcOffsetDescriptor`](crate::UtcOffsetDescriptor)
/// plus a token for [`UtcOffsetValid`](crate::UtcOffsetValid).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::UtcOffsetValid", constructor = "pub")]
pub struct ParsedUtcOffset {
    #[sidecar(primary)]
    descriptor: UtcOffsetDescriptor,
    #[sidecar(token)]
    token: UtcOffsetValidToken,
}

impl ParsedUtcOffset {
    /// Borrow the parsed descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &UtcOffsetDescriptor {
        &self.descriptor
    }
}

/// Output sidecar for the `parse_local_date_time` exchange: [`LocalDateTimeDescriptor`](crate::LocalDateTimeDescriptor)
/// plus a token for [`LocalDateTimeProof`](crate::LocalDateTimeProof).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::LocalDateTimeProof", constructor = "pub")]
pub struct ParsedLocalDateTime {
    #[sidecar(primary)]
    descriptor: LocalDateTimeDescriptor,
    #[sidecar(token)]
    token: LocalDateTimeProofToken,
}

impl ParsedLocalDateTime {
    /// Borrow the parsed descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &LocalDateTimeDescriptor {
        &self.descriptor
    }
}

/// Output sidecar for the `parse_offset_date_time` exchange: [`OffsetDateTimeDescriptor`](crate::OffsetDateTimeDescriptor)
/// plus a token for [`OffsetDateTimeProof`](crate::OffsetDateTimeProof).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::OffsetDateTimeProof", constructor = "pub")]
pub struct ParsedOffsetDateTime {
    #[sidecar(primary)]
    descriptor: OffsetDateTimeDescriptor,
    #[sidecar(token)]
    token: OffsetDateTimeProofToken,
}

impl ParsedOffsetDateTime {
    /// Borrow the parsed descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &OffsetDateTimeDescriptor {
        &self.descriptor
    }
}

/// Output sidecar for the `parse_date_with_shift` exchange: [`DateWithShiftDescriptor`](crate::DateWithShiftDescriptor)
/// plus a token for [`DateWithShiftValid`](crate::DateWithShiftValid).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::DateWithShiftValid", constructor = "pub")]
pub struct ParsedDateWithShift {
    #[sidecar(primary)]
    descriptor: DateWithShiftDescriptor,
    #[sidecar(token)]
    token: DateWithShiftValidToken,
}

impl ParsedDateWithShift {
    /// Borrow the parsed descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &DateWithShiftDescriptor {
        &self.descriptor
    }
}

/// Output sidecar for the `parse_time_of_day_with_shift` exchange: [`TimeOfDayWithShiftDescriptor`](crate::TimeOfDayWithShiftDescriptor)
/// plus a token for [`TimeOfDayWithShiftValid`](crate::TimeOfDayWithShiftValid).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::TimeOfDayWithShiftValid", constructor = "pub")]
pub struct ParsedTimeOfDayWithShift {
    #[sidecar(primary)]
    descriptor: TimeOfDayWithShiftDescriptor,
    #[sidecar(token)]
    token: TimeOfDayWithShiftValidToken,
}

impl ParsedTimeOfDayWithShift {
    /// Borrow the parsed descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &TimeOfDayWithShiftDescriptor {
        &self.descriptor
    }
}

/// Output sidecar for the `parse_rfc3339_timestamp` exchange: [`OffsetDateTimeDescriptor`](crate::OffsetDateTimeDescriptor)
/// plus a token for [`Rfc3339TimestampProof`](crate::Rfc3339TimestampProof).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::Rfc3339TimestampProof", constructor = "pub")]
pub struct ParsedRfc3339Timestamp {
    #[sidecar(primary)]
    descriptor: OffsetDateTimeDescriptor,
    #[sidecar(token)]
    token: Rfc3339TimestampProofToken,
}

impl ParsedRfc3339Timestamp {
    /// Borrow the parsed descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &OffsetDateTimeDescriptor {
        &self.descriptor
    }
}

/// Output sidecar for the `parse_ixdtf_timestamp` exchange: [`IxdtfTimestampDescriptor`](crate::IxdtfTimestampDescriptor)
/// plus a token for [`IxdtfTimestampProof`](crate::IxdtfTimestampProof).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::IxdtfTimestampProof", constructor = "pub")]
pub struct ParsedIxdtfTimestamp {
    #[sidecar(primary)]
    descriptor: IxdtfTimestampDescriptor,
    #[sidecar(token)]
    token: IxdtfTimestampProofToken,
}

impl ParsedIxdtfTimestamp {
    /// Borrow the parsed descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &IxdtfTimestampDescriptor {
        &self.descriptor
    }
}

/// Output sidecar for the `parse_seasonal_temporal_expression` exchange: [`SeasonalTemporalExpressionDescriptor`](crate::SeasonalTemporalExpressionDescriptor)
/// plus a token for [`SeasonalTemporalExpressionProof`](crate::SeasonalTemporalExpressionProof).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::SeasonalTemporalExpressionProof",
    constructor = "pub"
)]
pub struct ParsedSeasonalTemporalExpression {
    #[sidecar(primary)]
    descriptor: SeasonalTemporalExpressionDescriptor,
    #[sidecar(token)]
    token: SeasonalTemporalExpressionProofToken,
}

impl ParsedSeasonalTemporalExpression {
    /// Borrow the parsed descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &SeasonalTemporalExpressionDescriptor {
        &self.descriptor
    }
}

/// Output sidecar for the `parse_sub_year_grouping_expression` exchange: [`SubYearGroupingExpressionDescriptor`](crate::SubYearGroupingExpressionDescriptor)
/// plus a token for [`SubYearGroupingExpressionProof`](crate::SubYearGroupingExpressionProof).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::SubYearGroupingExpressionProof",
    constructor = "pub"
)]
pub struct ParsedSubYearGroupingExpression {
    #[sidecar(primary)]
    descriptor: SubYearGroupingExpressionDescriptor,
    #[sidecar(token)]
    token: SubYearGroupingExpressionProofToken,
}

impl ParsedSubYearGroupingExpression {
    /// Borrow the parsed descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &SubYearGroupingExpressionDescriptor {
        &self.descriptor
    }
}

/// Output sidecar for the `parse_unspecified_component_expression` exchange: [`UnspecifiedComponentExpressionDescriptor`](crate::UnspecifiedComponentExpressionDescriptor)
/// plus a token for [`UnspecifiedComponentExpressionProof`](crate::UnspecifiedComponentExpressionProof).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::UnspecifiedComponentExpressionProof",
    constructor = "pub"
)]
pub struct ParsedUnspecifiedComponentExpression {
    #[sidecar(primary)]
    descriptor: UnspecifiedComponentExpressionDescriptor,
    #[sidecar(token)]
    token: UnspecifiedComponentExpressionProofToken,
}

impl ParsedUnspecifiedComponentExpression {
    /// Borrow the parsed descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &UnspecifiedComponentExpressionDescriptor {
        &self.descriptor
    }
}

/// Output sidecar for the `parse_temporal_set` exchange: [`TemporalSetDescriptor`](crate::TemporalSetDescriptor)
/// plus a token for [`TemporalSetProof`](crate::TemporalSetProof).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::TemporalSetProof", constructor = "pub")]
pub struct ParsedTemporalSet {
    #[sidecar(primary)]
    descriptor: TemporalSetDescriptor,
    #[sidecar(token)]
    token: TemporalSetProofToken,
}

impl ParsedTemporalSet {
    /// Borrow the parsed descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &TemporalSetDescriptor {
        &self.descriptor
    }
}

/// Output sidecar for the `parse_grouped_time_scale_unit` exchange: [`GroupedTimeScaleUnitDescriptor`](crate::GroupedTimeScaleUnitDescriptor)
/// plus a token for [`GroupedTimeScaleUnitProof`](crate::GroupedTimeScaleUnitProof).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::GroupedTimeScaleUnitProof", constructor = "pub")]
pub struct ParsedGroupedTimeScaleUnit {
    #[sidecar(primary)]
    descriptor: GroupedTimeScaleUnitDescriptor,
    #[sidecar(token)]
    token: GroupedTimeScaleUnitProofToken,
}

impl ParsedGroupedTimeScaleUnit {
    /// Borrow the parsed descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &GroupedTimeScaleUnitDescriptor {
        &self.descriptor
    }
}

/// Output sidecar for the `parse_date_time_formula` exchange: [`DateTimeFormulaDescriptor`](crate::DateTimeFormulaDescriptor)
/// plus a token for [`DateTimeFormulaProof`](crate::DateTimeFormulaProof).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::DateTimeFormulaProof", constructor = "pub")]
pub struct ParsedDateTimeFormula {
    #[sidecar(primary)]
    descriptor: DateTimeFormulaDescriptor,
    #[sidecar(token)]
    token: DateTimeFormulaProofToken,
}

impl ParsedDateTimeFormula {
    /// Borrow the parsed descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &DateTimeFormulaDescriptor {
        &self.descriptor
    }
}

/// Output sidecar for the `parse_time_interval` exchange: [`TimeIntervalDescriptor`](crate::TimeIntervalDescriptor)
/// plus a token for [`TimeIntervalProof`](crate::TimeIntervalProof).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::TimeIntervalProof", constructor = "pub")]
pub struct ParsedTimeInterval {
    #[sidecar(primary)]
    descriptor: TimeIntervalDescriptor,
    #[sidecar(token)]
    token: TimeIntervalProofToken,
}

impl ParsedTimeInterval {
    /// Borrow the parsed descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &TimeIntervalDescriptor {
        &self.descriptor
    }
}

/// Proven-descriptor input sidecar for the matching formatter method(s).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::IxdtfZonedTimestampProof", constructor = "pub")]
pub struct ParsedIxdtfZonedTimestamp {
    #[sidecar(primary)]
    descriptor: ZonedDateTimeDescriptor,
    #[sidecar(token)]
    token: IxdtfZonedTimestampProofToken,
}

impl ParsedIxdtfZonedTimestamp {
    /// Borrow the descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &ZonedDateTimeDescriptor {
        &self.descriptor
    }
}

/// Proven-descriptor input sidecar for the matching formatter method(s).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::DurationFormValid", constructor = "pub")]
pub struct ParsedDuration {
    #[sidecar(primary)]
    descriptor: DurationDescriptor,
    #[sidecar(token)]
    token: DurationFormValidToken,
}

impl ParsedDuration {
    /// Borrow the descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &DurationDescriptor {
        &self.descriptor
    }
}

/// Proven-descriptor input sidecar for the matching formatter method(s).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::RecurringIntervalFormValid", constructor = "pub")]
pub struct ParsedRecurringInterval {
    #[sidecar(primary)]
    descriptor: RecurringIntervalDescriptor,
    #[sidecar(token)]
    token: RecurringIntervalFormValidToken,
}

impl ParsedRecurringInterval {
    /// Borrow the descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &RecurringIntervalDescriptor {
        &self.descriptor
    }
}
