//! Grouped/masked/seasonal precision-variant forms.
//!
//!
//! Ported from `elicit_temporal::contracts::proof_composition`. Each
//! aggregate is folded with its `*Evidence` bundle: the aggregate's
//! fields ARE its sub-claims, and `#[derive(Witness)]` makes the
//! composite proof the structural product of its members' proofs.
//! See [`super`] for the design; leaf `Witness<V>` impls land in the
//! backend crates.

use crate::{
    GroupedTimeScaleUnitCarriesOneOrMoreDurationUnits, GroupedTimeScaleUnitConvertsToTimeInterval,
    GroupedTimeScaleUnitDateTimeMayCarryExplicitTimeShift,
    GroupedTimeScaleUnitDefinitionIsContinuous,
    GroupedTimeScaleUnitLowerOrderUnitsRemainWithinGroupBounds,
    GroupedTimeScaleUnitTruncatesOutOfBoundsRemainder, GroupedTimeScaleUnitUsesGroupingDesignators,
    GroupedTimeScaleUnitValueCarriesExplicitCoefficient,
    LevelOneUnspecifiedDigitsOccupyRightmostPositions,
    LevelTwoUnspecifiedDigitsMayAppearWithinComponent, SeasonCodeDeclaresNamedSeason,
    SeasonCodeDeclaresSeasonScope, SeasonalExpressionUsesSeasonCodeInMonthSlot,
    SeasonalExpressionUsesYearAndSeasonForm, SubYearGroupingExpressionUsesGroupingCodeInMonthSlot,
    SubYearGroupingExpressionUsesYearAndGroupingForm, SubYearGroupingKindEvidence,
    UnspecifiedDigitUsesUppercaseXPlaceholder, UnspecifiedDigitsDeclareUnknownValue,
};

/// Aggregate proof that a grouped time scale unit expression is structurally valid.
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
pub struct GroupedTimeScaleUnitValid {
    /// Grouped units use the `G...U` delimiters.
    designators: GroupedTimeScaleUnitUsesGroupingDesignators,
    /// Grouped units carry one or more duration components.
    units: GroupedTimeScaleUnitCarriesOneOrMoreDurationUnits,
    /// Grouped-unit definitions are continuous.
    continuity: GroupedTimeScaleUnitDefinitionIsContinuous,
    /// Grouped-unit values carry explicit coefficients.
    coefficient: GroupedTimeScaleUnitValueCarriesExplicitCoefficient,
    /// Lower-order units remain within the grouped-unit bounds.
    bounds: GroupedTimeScaleUnitLowerOrderUnitsRemainWithinGroupBounds,
    /// Grouped-unit date-time forms may append an explicit time shift.
    explicit_time_shift: GroupedTimeScaleUnitDateTimeMayCarryExplicitTimeShift,
    /// Out-of-bounds remainder truncates at the original boundary.
    truncation: GroupedTimeScaleUnitTruncatesOutOfBoundsRemainder,
    /// Grouped-unit expressions convert into time-interval semantics.
    interval_semantics: GroupedTimeScaleUnitConvertsToTimeInterval,
}

/// Aggregate proof that a Level 2 sub-year grouping expression is structurally valid.
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
pub struct SubYearGroupingExpressionValid {
    /// The expression uses a year-and-grouping form.
    form: SubYearGroupingExpressionUsesYearAndGroupingForm,
    /// The grouping code occupies the month slot of a year-month-shaped representation.
    month_slot: SubYearGroupingExpressionUsesGroupingCodeInMonthSlot,
    /// The specific grouping family declared by the code.
    grouping: SubYearGroupingKindEvidence,
}

/// Aggregate proof that a seasonal temporal expression is structurally valid.
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
pub struct SeasonalTemporalExpressionValid {
    /// The expression uses a year-and-season form.
    form: SeasonalExpressionUsesYearAndSeasonForm,
    /// The season code occupies the month slot of a year-month-shaped representation.
    month_slot: SeasonalExpressionUsesSeasonCodeInMonthSlot,
    /// The season code declares a named season.
    named_season: SeasonCodeDeclaresNamedSeason,
    /// The season code declares its season scope.
    season_scope: SeasonCodeDeclaresSeasonScope,
}
/// Aggregate proof that masked precision and unspecified-component semantics are structurally valid.
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
pub struct UnspecifiedComponentExpressionValid {
    /// Unspecified digits use the uppercase `X` placeholder.
    placeholder: UnspecifiedDigitUsesUppercaseXPlaceholder,
    /// Each `X` placeholder denotes an unspecified digit or component value.
    unspecified_value: UnspecifiedDigitsDeclareUnknownValue,
    /// Level 1 masking occupies one or more rightmost positions.
    level_one_tail_masking: LevelOneUnspecifiedDigitsOccupyRightmostPositions,
    /// Level 2 masking may occur within a component.
    level_two_component_masking: LevelTwoUnspecifiedDigitsMayAppearWithinComponent,
}
