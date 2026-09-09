//! Per-method emission-proof composites — each formatter method's
//! output proof(s) folded into one `#[derive(Evidence, Witness)]`
//! struct (the same folding as `proof_composition`), so the `Formatted*`
//! output sidecar keeps its single-`token` shape.

use crate::{
    CenturyValid, CompleteIntervalSubstitutionProofBranch, DateTimeFormulaEvaluationSemanticsValid,
    DateTimeFormulaValid, DateWithShiftValid, DecadeValid, DurationFormValid,
    DurationRepresentationProofBranch, ExtendedIntervalBoundarySemanticsValid, ExtendedYearValid,
    GroupedTimeScaleUnitCarriesOneOrMoreDurationUnits, GroupedTimeScaleUnitConvertsToTimeInterval,
    GroupedTimeScaleUnitDateTimeMayCarryExplicitTimeShift,
    GroupedTimeScaleUnitDefinitionIsContinuous,
    GroupedTimeScaleUnitLowerOrderUnitsRemainWithinGroupBounds,
    GroupedTimeScaleUnitTruncatesOutOfBoundsRemainder, GroupedTimeScaleUnitUsesGroupingDesignators,
    GroupedTimeScaleUnitValid, GroupedTimeScaleUnitValueCarriesExplicitCoefficient,
    IntervalEndComponentInheritanceProofBranch, IntervalZoneInheritanceProofBranch,
    Iso8601BasicFormUsesCompactRepresentation, Iso8601ExtendedFormUsesSeparators,
    IxdtfAdditionalInformationProofBranch, IxdtfCalendarAnnotationProofBranch,
    IxdtfSerializationCarriesNamedZoneAnnotation, IxdtfTimeZoneAnnotationProofBranch,
    IxdtfTimestampValid, LevelOneUnspecifiedDigitsOccupyRightmostPositions,
    LevelTwoUnspecifiedDigitsMayAppearWithinComponent, QualificationPlacementEvidence,
    QualifiedTemporalExpressionValid, QualifiedTemporalValueValid, RecurringIntervalFormValid,
    RecurringIntervalRepresentationProofBranch, SeasonCodeDeclaresNamedSeason,
    SeasonCodeDeclaresSeasonScope, SeasonalExpressionUsesSeasonCodeInMonthSlot,
    SeasonalExpressionUsesYearAndSeasonForm, SeasonalTemporalExpressionValid,
    SerializationCarriesExplicitUtcRelationship,
    SubYearGroupingExpressionUsesGroupingCodeInMonthSlot,
    SubYearGroupingExpressionUsesYearAndGroupingForm, SubYearGroupingExpressionValid,
    SubYearGroupingKindEvidence, TemporalSetExpressionValid, TemporalSetRangeSemanticsValid,
    TimeIntervalValid, TimeOfDayWithShiftValid, UnspecifiedComponentExpressionValid,
    UnspecifiedDigitUsesUppercaseXPlaceholder, UnspecifiedDigitsDeclareUnknownValue,
};

/// Emission proof for [`FormattedCalendarDateExtended`](crate::FormattedCalendarDateExtended) — the `format_calendar_date_extended` output proof(s), folded.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct CalendarDateExtendedFormatted {
    /// The `iso8601_extended_form_uses_separators` sub-claim.
    iso8601_extended_form_uses_separators: Iso8601ExtendedFormUsesSeparators,
}

/// Emission proof for [`FormattedCalendarDateBasic`](crate::FormattedCalendarDateBasic) — the `format_calendar_date_basic` output proof(s), folded.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct CalendarDateBasicFormatted {
    /// The `iso8601_basic_form_uses_compact_representation` sub-claim.
    iso8601_basic_form_uses_compact_representation: Iso8601BasicFormUsesCompactRepresentation,
}

/// Emission proof for [`FormattedReducedCalendarDateExtended`](crate::FormattedReducedCalendarDateExtended) — the `format_reduced_calendar_date_extended` output proof(s), folded.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct ReducedCalendarDateExtendedFormatted {
    /// The `iso8601_extended_form_uses_separators` sub-claim.
    iso8601_extended_form_uses_separators: Iso8601ExtendedFormUsesSeparators,
}

/// Emission proof for [`FormattedReducedCalendarDateBasic`](crate::FormattedReducedCalendarDateBasic) — the `format_reduced_calendar_date_basic` output proof(s), folded.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct ReducedCalendarDateBasicFormatted {
    /// The `iso8601_basic_form_uses_compact_representation` sub-claim.
    iso8601_basic_form_uses_compact_representation: Iso8601BasicFormUsesCompactRepresentation,
}

/// Emission proof for [`FormattedOrdinalDateExtended`](crate::FormattedOrdinalDateExtended) — the `format_ordinal_date_extended` output proof(s), folded.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct OrdinalDateExtendedFormatted {
    /// The `iso8601_extended_form_uses_separators` sub-claim.
    iso8601_extended_form_uses_separators: Iso8601ExtendedFormUsesSeparators,
}

/// Emission proof for [`FormattedOrdinalDateBasic`](crate::FormattedOrdinalDateBasic) — the `format_ordinal_date_basic` output proof(s), folded.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct OrdinalDateBasicFormatted {
    /// The `iso8601_basic_form_uses_compact_representation` sub-claim.
    iso8601_basic_form_uses_compact_representation: Iso8601BasicFormUsesCompactRepresentation,
}

/// Emission proof for [`FormattedWeekDateExtended`](crate::FormattedWeekDateExtended) — the `format_week_date_extended` output proof(s), folded.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct WeekDateExtendedFormatted {
    /// The `iso8601_extended_form_uses_separators` sub-claim.
    iso8601_extended_form_uses_separators: Iso8601ExtendedFormUsesSeparators,
}

/// Emission proof for [`FormattedWeekDateBasic`](crate::FormattedWeekDateBasic) — the `format_week_date_basic` output proof(s), folded.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct WeekDateBasicFormatted {
    /// The `iso8601_basic_form_uses_compact_representation` sub-claim.
    iso8601_basic_form_uses_compact_representation: Iso8601BasicFormUsesCompactRepresentation,
}

/// Emission proof for [`FormattedLocalTimeExtended`](crate::FormattedLocalTimeExtended) — the `format_local_time_extended` output proof(s), folded.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct LocalTimeExtendedFormatted {
    /// The `iso8601_extended_form_uses_separators` sub-claim.
    iso8601_extended_form_uses_separators: Iso8601ExtendedFormUsesSeparators,
}

/// Emission proof for [`FormattedLocalTimeBasic`](crate::FormattedLocalTimeBasic) — the `format_local_time_basic` output proof(s), folded.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct LocalTimeBasicFormatted {
    /// The `iso8601_basic_form_uses_compact_representation` sub-claim.
    iso8601_basic_form_uses_compact_representation: Iso8601BasicFormUsesCompactRepresentation,
}

/// Emission proof for [`FormattedReducedLocalTimeExtended`](crate::FormattedReducedLocalTimeExtended) — the `format_reduced_local_time_extended` output proof(s), folded.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct ReducedLocalTimeExtendedFormatted {
    /// The `iso8601_extended_form_uses_separators` sub-claim.
    iso8601_extended_form_uses_separators: Iso8601ExtendedFormUsesSeparators,
}

/// Emission proof for [`FormattedReducedLocalTimeBasic`](crate::FormattedReducedLocalTimeBasic) — the `format_reduced_local_time_basic` output proof(s), folded.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct ReducedLocalTimeBasicFormatted {
    /// The `iso8601_basic_form_uses_compact_representation` sub-claim.
    iso8601_basic_form_uses_compact_representation: Iso8601BasicFormUsesCompactRepresentation,
}

/// Emission proof for [`FormattedUtcOffsetExtended`](crate::FormattedUtcOffsetExtended) — the `format_utc_offset_extended` output proof(s), folded.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct UtcOffsetExtendedFormatted {
    /// The `iso8601_extended_form_uses_separators` sub-claim.
    iso8601_extended_form_uses_separators: Iso8601ExtendedFormUsesSeparators,
}

/// Emission proof for [`FormattedUtcOffsetBasic`](crate::FormattedUtcOffsetBasic) — the `format_utc_offset_basic` output proof(s), folded.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct UtcOffsetBasicFormatted {
    /// The `iso8601_basic_form_uses_compact_representation` sub-claim.
    iso8601_basic_form_uses_compact_representation: Iso8601BasicFormUsesCompactRepresentation,
}

/// Emission proof for [`FormattedLocalDateTimeExtended`](crate::FormattedLocalDateTimeExtended) — the `format_local_date_time_extended` output proof(s), folded.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct LocalDateTimeExtendedFormatted {
    /// The `iso8601_extended_form_uses_separators` sub-claim.
    iso8601_extended_form_uses_separators: Iso8601ExtendedFormUsesSeparators,
}

/// Emission proof for [`FormattedLocalDateTimeBasic`](crate::FormattedLocalDateTimeBasic) — the `format_local_date_time_basic` output proof(s), folded.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct LocalDateTimeBasicFormatted {
    /// The `iso8601_basic_form_uses_compact_representation` sub-claim.
    iso8601_basic_form_uses_compact_representation: Iso8601BasicFormUsesCompactRepresentation,
}

/// Emission proof for [`FormattedOffsetDateTimeExtended`](crate::FormattedOffsetDateTimeExtended) — the `format_offset_date_time_extended` output proof(s), folded.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct OffsetDateTimeExtendedFormatted {
    /// The `iso8601_extended_form_uses_separators` sub-claim.
    iso8601_extended_form_uses_separators: Iso8601ExtendedFormUsesSeparators,
    /// The `serialization_carries_explicit_utc_relationship` sub-claim.
    serialization_carries_explicit_utc_relationship: SerializationCarriesExplicitUtcRelationship,
}

/// Emission proof for [`FormattedOffsetDateTimeBasic`](crate::FormattedOffsetDateTimeBasic) — the `format_offset_date_time_basic` output proof(s), folded.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct OffsetDateTimeBasicFormatted {
    /// The `iso8601_basic_form_uses_compact_representation` sub-claim.
    iso8601_basic_form_uses_compact_representation: Iso8601BasicFormUsesCompactRepresentation,
    /// The `serialization_carries_explicit_utc_relationship` sub-claim.
    serialization_carries_explicit_utc_relationship: SerializationCarriesExplicitUtcRelationship,
}

/// Emission proof for [`FormattedDateWithShift`](crate::FormattedDateWithShift) — the `format_date_with_shift` output proof(s), folded.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct DateWithShiftFormatted {
    /// The `date_with_shift_valid` sub-claim.
    date_with_shift_valid: DateWithShiftValid,
}

/// Emission proof for [`FormattedTimeOfDayWithShift`](crate::FormattedTimeOfDayWithShift) — the `format_time_of_day_with_shift` output proof(s), folded.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct TimeOfDayWithShiftFormatted {
    /// The `time_of_day_with_shift_valid` sub-claim.
    time_of_day_with_shift_valid: TimeOfDayWithShiftValid,
}

/// Emission proof for [`FormattedExtendedYear`](crate::FormattedExtendedYear) — the `format_extended_year` output proof(s), folded.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct ExtendedYearFormatted {
    /// The `extended_year_valid` sub-claim.
    extended_year_valid: ExtendedYearValid,
}

/// Emission proof for [`FormattedDecade`](crate::FormattedDecade) — the `format_decade` output proof(s), folded.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct DecadeFormatted {
    /// The `decade_valid` sub-claim.
    decade_valid: DecadeValid,
}

/// Emission proof for [`FormattedCentury`](crate::FormattedCentury) — the `format_century` output proof(s), folded.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct CenturyFormatted {
    /// The `century_valid` sub-claim.
    century_valid: CenturyValid,
}

/// Emission proof for [`FormattedQualifiedTemporalValue`](crate::FormattedQualifiedTemporalValue) — the `format_qualified_temporal_value` output proof(s), folded.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct QualifiedTemporalValueFormatted {
    /// The `qualified_temporal_value_valid` sub-claim.
    qualified_temporal_value_valid: QualifiedTemporalValueValid,
    /// The `qualified_temporal_expression_valid` sub-claim.
    qualified_temporal_expression_valid: QualifiedTemporalExpressionValid,
    /// The `qualification_placement_evidence` sub-claim.
    qualification_placement_evidence: QualificationPlacementEvidence,
}

/// Emission proof for [`FormattedRfc3339Timestamp`](crate::FormattedRfc3339Timestamp) — the `format_rfc3339_timestamp` output proof(s), folded.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct Rfc3339TimestampFormatted {
    /// The `serialization_carries_explicit_utc_relationship` sub-claim.
    serialization_carries_explicit_utc_relationship: SerializationCarriesExplicitUtcRelationship,
}

/// Emission proof for [`FormattedIxdtfZonedTimestamp`](crate::FormattedIxdtfZonedTimestamp) — the `format_ixdtf_zoned_timestamp` output proof(s), folded.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct IxdtfZonedTimestampFormatted {
    /// The `ixdtf_serialization_carries_named_zone_annotation` sub-claim.
    ixdtf_serialization_carries_named_zone_annotation: IxdtfSerializationCarriesNamedZoneAnnotation,
}

/// Emission proof for [`FormattedIxdtfTimestamp`](crate::FormattedIxdtfTimestamp) — the `format_ixdtf_timestamp` output proof(s), folded.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct IxdtfTimestampFormatted {
    /// The `ixdtf_timestamp_valid` sub-claim.
    ixdtf_timestamp_valid: IxdtfTimestampValid,
    /// The `ixdtf_time_zone_annotation_proof_branch` sub-claim.
    ixdtf_time_zone_annotation_proof_branch: IxdtfTimeZoneAnnotationProofBranch,
    /// The `ixdtf_calendar_annotation_proof_branch` sub-claim.
    ixdtf_calendar_annotation_proof_branch: IxdtfCalendarAnnotationProofBranch,
    /// The `ixdtf_additional_information_proof_branch` sub-claim.
    ixdtf_additional_information_proof_branch: IxdtfAdditionalInformationProofBranch,
}

/// Emission proof for [`FormattedSeasonalTemporalExpression`](crate::FormattedSeasonalTemporalExpression) — the `format_seasonal_temporal_expression` output proof(s), folded.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct SeasonalTemporalExpressionFormatted {
    /// The `seasonal_temporal_expression_valid` sub-claim.
    seasonal_temporal_expression_valid: SeasonalTemporalExpressionValid,
    /// The `seasonal_expression_uses_year_and_season_form` sub-claim.
    seasonal_expression_uses_year_and_season_form: SeasonalExpressionUsesYearAndSeasonForm,
    /// The `seasonal_expression_uses_season_code_in_month_slot` sub-claim.
    seasonal_expression_uses_season_code_in_month_slot: SeasonalExpressionUsesSeasonCodeInMonthSlot,
    /// The `season_code_declares_named_season` sub-claim.
    season_code_declares_named_season: SeasonCodeDeclaresNamedSeason,
    /// The `season_code_declares_season_scope` sub-claim.
    season_code_declares_season_scope: SeasonCodeDeclaresSeasonScope,
}

/// Emission proof for [`FormattedSubYearGroupingExpression`](crate::FormattedSubYearGroupingExpression) — the `format_sub_year_grouping_expression` output proof(s), folded.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct SubYearGroupingExpressionFormatted {
    /// The `sub_year_grouping_expression_valid` sub-claim.
    sub_year_grouping_expression_valid: SubYearGroupingExpressionValid,
    /// The `sub_year_grouping_expression_uses_year_and_grouping_form` sub-claim.
    sub_year_grouping_expression_uses_year_and_grouping_form:
        SubYearGroupingExpressionUsesYearAndGroupingForm,
    /// The `sub_year_grouping_expression_uses_grouping_code_in_month_slot` sub-claim.
    sub_year_grouping_expression_uses_grouping_code_in_month_slot:
        SubYearGroupingExpressionUsesGroupingCodeInMonthSlot,
    /// The `sub_year_grouping_kind_evidence` sub-claim.
    sub_year_grouping_kind_evidence: SubYearGroupingKindEvidence,
}

/// Emission proof for [`FormattedUnspecifiedComponentExpression`](crate::FormattedUnspecifiedComponentExpression) — the `format_unspecified_component_expression` output proof(s), folded.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct UnspecifiedComponentExpressionFormatted {
    /// The `unspecified_component_expression_valid` sub-claim.
    unspecified_component_expression_valid: UnspecifiedComponentExpressionValid,
    /// The `unspecified_digit_uses_uppercase_x_placeholder` sub-claim.
    unspecified_digit_uses_uppercase_x_placeholder: UnspecifiedDigitUsesUppercaseXPlaceholder,
    /// The `unspecified_digits_declare_unknown_value` sub-claim.
    unspecified_digits_declare_unknown_value: UnspecifiedDigitsDeclareUnknownValue,
    /// The `level_one_unspecified_digits_occupy_rightmost_positions` sub-claim.
    level_one_unspecified_digits_occupy_rightmost_positions:
        LevelOneUnspecifiedDigitsOccupyRightmostPositions,
    /// The `level_two_unspecified_digits_may_appear_within_component` sub-claim.
    level_two_unspecified_digits_may_appear_within_component:
        LevelTwoUnspecifiedDigitsMayAppearWithinComponent,
}

/// Emission proof for [`FormattedTemporalSet`](crate::FormattedTemporalSet) — the `format_temporal_set` output proof(s), folded.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct TemporalSetFormatted {
    /// The `temporal_set_expression_valid` sub-claim.
    temporal_set_expression_valid: TemporalSetExpressionValid,
    /// The `temporal_set_range_semantics_valid` sub-claim.
    temporal_set_range_semantics_valid: TemporalSetRangeSemanticsValid,
}

/// Emission proof for [`FormattedGroupedTimeScaleUnit`](crate::FormattedGroupedTimeScaleUnit) — the `format_grouped_time_scale_unit` output proof(s), folded.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct GroupedTimeScaleUnitFormatted {
    /// The `grouped_time_scale_unit_valid` sub-claim.
    grouped_time_scale_unit_valid: GroupedTimeScaleUnitValid,
    /// The `grouped_time_scale_unit_uses_grouping_designators` sub-claim.
    grouped_time_scale_unit_uses_grouping_designators: GroupedTimeScaleUnitUsesGroupingDesignators,
    /// The `grouped_time_scale_unit_carries_one_or_more_duration_units` sub-claim.
    grouped_time_scale_unit_carries_one_or_more_duration_units:
        GroupedTimeScaleUnitCarriesOneOrMoreDurationUnits,
    /// The `grouped_time_scale_unit_definition_is_continuous` sub-claim.
    grouped_time_scale_unit_definition_is_continuous: GroupedTimeScaleUnitDefinitionIsContinuous,
    /// The `grouped_time_scale_unit_value_carries_explicit_coefficient` sub-claim.
    grouped_time_scale_unit_value_carries_explicit_coefficient:
        GroupedTimeScaleUnitValueCarriesExplicitCoefficient,
    /// The `grouped_time_scale_unit_lower_order_units_remain_within_group_bounds` sub-claim.
    grouped_time_scale_unit_lower_order_units_remain_within_group_bounds:
        GroupedTimeScaleUnitLowerOrderUnitsRemainWithinGroupBounds,
    /// The `grouped_time_scale_unit_date_time_may_carry_explicit_time_shift` sub-claim.
    grouped_time_scale_unit_date_time_may_carry_explicit_time_shift:
        GroupedTimeScaleUnitDateTimeMayCarryExplicitTimeShift,
    /// The `grouped_time_scale_unit_truncates_out_of_bounds_remainder` sub-claim.
    grouped_time_scale_unit_truncates_out_of_bounds_remainder:
        GroupedTimeScaleUnitTruncatesOutOfBoundsRemainder,
    /// The `grouped_time_scale_unit_converts_to_time_interval` sub-claim.
    grouped_time_scale_unit_converts_to_time_interval: GroupedTimeScaleUnitConvertsToTimeInterval,
}

/// Emission proof for [`FormattedDateTimeFormula`](crate::FormattedDateTimeFormula) — the `format_date_time_formula` output proof(s), folded.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct DateTimeFormulaFormatted {
    /// The `date_time_formula_valid` sub-claim.
    date_time_formula_valid: DateTimeFormulaValid,
    /// The `date_time_formula_evaluation_semantics_valid` sub-claim.
    date_time_formula_evaluation_semantics_valid: DateTimeFormulaEvaluationSemanticsValid,
}

/// Emission proof for [`FormattedDuration`](crate::FormattedDuration) — the `format_duration` output proof(s), folded.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct DurationFormatted {
    /// The `duration_form_valid` sub-claim.
    duration_form_valid: DurationFormValid,
    /// The `duration_representation_proof_branch` sub-claim.
    duration_representation_proof_branch: DurationRepresentationProofBranch,
}

/// Emission proof for [`FormattedRecurringInterval`](crate::FormattedRecurringInterval) — the `format_recurring_interval` output proof(s), folded.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct RecurringIntervalFormatted {
    /// The `recurring_interval_form_valid` sub-claim.
    recurring_interval_form_valid: RecurringIntervalFormValid,
    /// The `recurring_interval_representation_proof_branch` sub-claim.
    recurring_interval_representation_proof_branch: RecurringIntervalRepresentationProofBranch,
}

/// Emission proof for [`FormattedTimeInterval`](crate::FormattedTimeInterval) — the `format_time_interval` output proof(s), folded.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct TimeIntervalFormatted {
    /// The `time_interval_valid` sub-claim.
    time_interval_valid: TimeIntervalValid,
    /// The `extended_interval_boundary_semantics_valid` sub-claim.
    extended_interval_boundary_semantics_valid: ExtendedIntervalBoundarySemanticsValid,
    /// The `interval_end_component_inheritance_proof_branch` sub-claim.
    interval_end_component_inheritance_proof_branch: IntervalEndComponentInheritanceProofBranch,
    /// The `interval_zone_inheritance_proof_branch` sub-claim.
    interval_zone_inheritance_proof_branch: IntervalZoneInheritanceProofBranch,
    /// The `complete_interval_substitution_proof_branch` sub-claim.
    complete_interval_substitution_proof_branch: CompleteIntervalSubstitutionProofBranch,
}
