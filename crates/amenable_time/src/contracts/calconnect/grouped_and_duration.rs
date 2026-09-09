//! CalConnect CC 18011:2018 grouped time scale units (§5), explicit
//! durations (§7), and date-time formulas (§8). See [`super::explicit`]
//! for the tier-B conventions.

use crate::NormativeQuotation;

temporal_standard! {
    /// A grouped unit uses the `G...U` delimiters.
    GroupedTimeScaleUnitUsesGroupingDesignators => (
        document: "CalConnect CC 18011:2018", section: "5.1", body: CalConnect, status: Normative,
        summary: "a grouped time scale unit is delimited by the G ... U grouping designators",
        quotation: NormativeQuotation::ParaphraseOnly, url: "https://standards.calconnect.org/csd/cc-18011.html",
    );
    /// A grouped unit carries one or more positive duration components.
    GroupedTimeScaleUnitCarriesOneOrMoreDurationUnits => (
        document: "CalConnect CC 18011:2018", section: "5.1", body: CalConnect, status: Normative,
        summary: "a grouped time scale unit contains at least one positive duration component",
        quotation: NormativeQuotation::ParaphraseOnly, url: "https://standards.calconnect.org/csd/cc-18011.html",
    );
    /// Adjacent grouped units form a continuous interval.
    GroupedTimeScaleUnitDefinitionIsContinuous => (
        document: "CalConnect CC 18011:2018", section: "5.1", body: CalConnect, status: Normative,
        summary: "consecutive grouped units abut with no gap, defining a continuous interval",
        quotation: NormativeQuotation::ParaphraseOnly, url: "https://standards.calconnect.org/csd/cc-18011.html",
    );
    /// A grouped-unit value carries an explicit coefficient.
    GroupedTimeScaleUnitValueCarriesExplicitCoefficient => (
        document: "CalConnect CC 18011:2018", section: "5.2", body: CalConnect, status: Normative,
        summary: "a grouped-unit value is written with an explicit numeric coefficient",
        quotation: NormativeQuotation::ParaphraseOnly, url: "https://standards.calconnect.org/csd/cc-18011.html",
    );
    /// Lower-order units stay within the group's bounds.
    GroupedTimeScaleUnitLowerOrderUnitsRemainWithinGroupBounds => (
        document: "CalConnect CC 18011:2018", section: "5.3.2", body: CalConnect, status: Normative,
        summary: "components below the grouped unit do not exceed the bounds the grouped unit sets",
        quotation: NormativeQuotation::ParaphraseOnly, url: "https://standards.calconnect.org/csd/cc-18011.html",
    );
    /// Out-of-bounds remainder is truncated at the boundary.
    GroupedTimeScaleUnitTruncatesOutOfBoundsRemainder => (
        document: "CalConnect CC 18011:2018", section: "5.3.4", body: CalConnect, status: Normative,
        summary: "a partial grouped unit that would exceed the boundary is truncated at that boundary",
        quotation: NormativeQuotation::ParaphraseOnly, url: "https://standards.calconnect.org/csd/cc-18011.html",
    );
    /// A grouped-unit date-time may append a time shift.
    GroupedTimeScaleUnitDateTimeMayCarryExplicitTimeShift => (
        document: "CalConnect CC 18011:2018", section: "5.3", body: CalConnect, status: Normative,
        summary: "a grouped-unit date-time representation may carry a trailing explicit time shift",
        quotation: NormativeQuotation::ParaphraseOnly, url: "https://standards.calconnect.org/csd/cc-18011.html",
    );
    /// Grouped-unit expressions convert to time-interval semantics.
    GroupedTimeScaleUnitConvertsToTimeInterval => (
        document: "CalConnect CC 18011:2018", section: "5.3.3", body: CalConnect, status: Normative,
        summary: "a grouped-unit expression has a defined conversion into ordinary time-interval semantics",
        quotation: NormativeQuotation::ParaphraseOnly, url: "https://standards.calconnect.org/csd/cc-18011.html",
    );
    /// An explicit duration uses durational unit designators.
    ExplicitDurationUsesDurationalUnitDesignators => (
        document: "CalConnect CC 18011:2018", section: "7.2", body: CalConnect, status: Normative,
        summary: "an explicit duration writes its components with durational unit designators",
        quotation: NormativeQuotation::ParaphraseOnly, url: "https://standards.calconnect.org/csd/cc-18011.html",
    );
    /// An explicit duration declares its representation kind.
    ExplicitDurationRepresentationKindDeclared => (
        document: "CalConnect CC 18011:2018", section: "7.3", body: CalConnect, status: Normative,
        summary: "an explicit duration states whether it is a simple, composite, or precedence representation",
        quotation: NormativeQuotation::ParaphraseOnly, url: "https://standards.calconnect.org/csd/cc-18011.html",
    );
    /// An explicit duration may use the composite representation.
    ExplicitDurationCompositeRepresentationDeclared => (
        document: "CalConnect CC 18011:2018", section: "7.4", body: CalConnect, status: Normative,
        summary: "an explicit duration may be written in the composite representation family",
        quotation: NormativeQuotation::ParaphraseOnly, url: "https://standards.calconnect.org/csd/cc-18011.html",
    );
    /// A precedence duration keeps its evaluation order.
    ExplicitDurationPrecedenceRepresentationCarriesEvaluationOrder => (
        document: "CalConnect CC 18011:2018", section: "7.5", body: CalConnect, status: Normative,
        summary: "a precedence-representation duration preserves the declared order in which its components are evaluated",
        quotation: NormativeQuotation::ParaphraseOnly, url: "https://standards.calconnect.org/csd/cc-18011.html",
    );
    /// An explicit duration may be negative.
    ExplicitDurationMayBeNegative => (
        document: "CalConnect CC 18011:2018", section: "7.6", body: CalConnect, status: Normative,
        summary: "an explicit duration may carry a negative sign",
        quotation: NormativeQuotation::ParaphraseOnly, url: "https://standards.calconnect.org/csd/cc-18011.html",
    );
    /// An explicit duration may fraction its lowest-order unit.
    ExplicitDurationMayUseFractionalLowestOrderUnit => (
        document: "CalConnect CC 18011:2018", section: "7.7", body: CalConnect, status: Normative,
        summary: "an explicit duration may carry a fractional value on its lowest-order unit",
        quotation: NormativeQuotation::ParaphraseOnly, url: "https://standards.calconnect.org/csd/cc-18011.html",
    );
    /// Exact-duration semantics are declared when used.
    ExactDurationSemanticsDeclared => (
        document: "CalConnect CC 18011:2018", section: "7.8", body: CalConnect, status: Normative,
        summary: "using the exact-duration family is declared explicitly",
        quotation: NormativeQuotation::ParaphraseOnly, url: "https://standards.calconnect.org/csd/cc-18011.html",
    );
    /// Context-dependent-duration semantics are declared when used.
    ContextDependentDurationSemanticsDeclared => (
        document: "CalConnect CC 18011:2018", section: "7.9", body: CalConnect, status: Normative,
        summary: "using the context-dependent-duration family is declared explicitly",
        quotation: NormativeQuotation::ParaphraseOnly, url: "https://standards.calconnect.org/csd/cc-18011.html",
    );
    /// Speculative-duration semantics are declared when used.
    SpeculativeDurationSemanticsDeclared => (
        document: "CalConnect CC 18011:2018", section: "7.10", body: CalConnect, status: Normative,
        summary: "using the speculative-duration family is declared explicitly",
        quotation: NormativeQuotation::ParaphraseOnly, url: "https://standards.calconnect.org/csd/cc-18011.html",
    );
    /// A date-time formula combines a value with a duration.
    DateTimeFormulaCombinesTemporalValueWithDuration => (
        document: "CalConnect CC 18011:2018", section: "8", body: CalConnect, status: Normative,
        summary: "a date-time formula applies an explicit duration to an explicit temporal value",
        quotation: NormativeQuotation::ParaphraseOnly, url: "https://standards.calconnect.org/csd/cc-18011.html",
    );
    /// Formula evaluation carries overflow across boundaries.
    DateTimeFormulaUsesCarryOverSemantics => (
        document: "CalConnect CC 18011:2018", section: "8.1", body: CalConnect, status: Normative,
        summary: "when a component overflows during formula evaluation, the excess carries into the next-higher component",
        quotation: NormativeQuotation::ParaphraseOnly, url: "https://standards.calconnect.org/csd/cc-18011.html",
    );
    /// Formula evaluation truncates at component boundaries.
    DateTimeFormulaTruncatesAtComponentBoundaries => (
        document: "CalConnect CC 18011:2018", section: "8.2", body: CalConnect, status: Normative,
        summary: "formula evaluation truncates a value at a time-scale component boundary rather than carrying past it, where that mode applies",
        quotation: NormativeQuotation::ParaphraseOnly, url: "https://standards.calconnect.org/csd/cc-18011.html",
    );
    /// A formula declares its evaluation mode.
    DateTimeFormulaEvaluationModeDeclared => (
        document: "CalConnect CC 18011:2018", section: "8.3-8.5", body: CalConnect, status: Normative,
        summary: "a date-time formula states whether it follows simple, composite, or precedence duration evaluation rules",
        quotation: NormativeQuotation::ParaphraseOnly, url: "https://standards.calconnect.org/csd/cc-18011.html",
    );
}
