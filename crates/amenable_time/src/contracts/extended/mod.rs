//! ISO 8601-2:2019 extension propositions (48 in `elicit_temporal`'s
//! `extended.rs`): qualification, extended year forms, seasons and
//! sub-year groupings, unspecified-digit placeholders, temporal sets.
//!
//! Primary source is ISO 8601-2:2019 throughout — tier C, so every
//! contract is `NormativeQuotation::ParaphraseOnly` with `status:
//! Normative` and the ISO 8601-2 catalog URL. The publicly-available
//! Library of Congress EDTF profile and the ISO/WD 8601-2:2016(E)
//! working draft appear only as `cross_check`s.

mod qualification;
mod unspecified_and_sets;
mod year_and_season;

pub use qualification::{
    ApproximationQualificationDeclared, BeforeOrAfterQualificationIsLevelTwoOnly,
    BeforeOrOnDateUsesLeadingDoubleDotQualifier,
    ComponentQualificationAppliesOnlyToMarkedComponent,
    ComponentQualificationUsesImmediateLeftPlacement,
    EnhancedIntervalLevelOnePermitsTerminalBoundaryQualification,
    EnhancedIntervalLevelTwoPermitsBeforeOrAfterBoundaryQualification,
    EnhancedIntervalLevelTwoPermitsInternalBoundaryQualification,
    EnhancedIntervalLevelTwoPermitsInternalBoundaryUnspecifiedDigits,
    GroupQualificationAppliesToMarkedAndMoreSignificantComponents,
    GroupQualificationUsesImmediateRightPlacement, OnOrAfterDateUsesTrailingDoubleDotQualifier,
    OpenIntervalBoundaryDeclared, QualificationScopeDeclared,
    UncertaintyAndApproximationMayBeCombined, UncertaintyQualificationDeclared,
    UnknownIntervalBoundaryDeclared,
};
pub use unspecified_and_sets::{
    LevelOneUnspecifiedDigitsOccupyRightmostPositions,
    LevelTwoUnspecifiedDigitsMayAppearWithinComponent, TemporalChoiceSetUsesSquareBrackets,
    TemporalInclusiveSetUsesCurlyBraces, TemporalSetCarriesMultipleMembers,
    TemporalSetDeclaresAlternativeSemantics, TemporalSetDeclaresInclusiveMemberSemantics,
    TemporalSetForbidsInternalWhitespace, TemporalSetMemberSeparatorDeclared,
    TemporalSetOpenRangeUsesBoundaryDoubleDot, TemporalSetRangeNeighborhoodSharesPrecision,
    TemporalSetRangeUsesInclusiveDoubleDotSemantics, UnspecifiedDigitUsesUppercaseXPlaceholder,
    UnspecifiedDigitsDeclareUnknownValue,
};
pub use year_and_season::{
    BeforeYearOneValueUsesTrailingBSuffix, ExponentialYearExponentIsPositiveInteger,
    ExponentialYearUsesPowerOfTenNotation, LetterPrefixedCalendarYearMagnitudeExceedsFourDigits,
    LetterPrefixedCalendarYearUsesLeadingYDesignator, NegativeCalendarYearUsesLeadingMinusSign,
    SeasonCodeDeclaresNamedSeason, SeasonCodeDeclaresSeasonScope,
    SeasonalExpressionUsesSeasonCodeInMonthSlot, SeasonalExpressionUsesYearAndSeasonForm,
    SignificantDigitYearCountIsPositiveInteger, SignificantDigitYearUsesTrailingSSuffix,
    SubYearGroupingCodeDeclaresQuadrimester, SubYearGroupingCodeDeclaresQuarter,
    SubYearGroupingCodeDeclaresSemestral, SubYearGroupingExpressionUsesGroupingCodeInMonthSlot,
    SubYearGroupingExpressionUsesYearAndGroupingForm,
};
