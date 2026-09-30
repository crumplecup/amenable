//! Local/offset/CalConnect-explicit date-time family aggregate proof propositions.
//!
//!
//! Ported from `elicit_temporal::contracts::proof_composition`. Each
//! aggregate is folded with its `*Evidence` bundle: the aggregate's
//! fields ARE its sub-claims, and `#[derive(Witness)]` makes the
//! composite proof the structural product of its members' proofs.
//! See [`super`] for the design; leaf `Witness<V>` impls land in the
//! backend crates.

use crate::{
    CombinedDateTimeDateComponentMustNotUseReducedAccuracy, CombinedDateTimeDateEvidence,
    CombinedDateTimeUsesSingleFormatAcrossDateAndTimeComponents,
    CombinedDateTimeUsesTimeDesignator, CompleteDateEvidence,
    ExplicitDateTimeTimePortionMayBeReducedPrecision,
    ExplicitDateTimeUsesDateThenTimeConcatenation,
    ExplicitDateTimeWithShiftUsesDateTimeThenShiftConcatenation, ExplicitTimeOfDayValid,
    ExplicitTimeShiftValid, LocalTimeValid, UtcDesignatorIsUppercaseZ, UtcOffsetValid,
};

/// Aggregate proof that a combined local date-time representation is structurally valid.
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
pub struct LocalDateTimeValid {
    /// The date component is valid and belongs to a lawful combined date-time family.
    date: CombinedDateTimeDateEvidence,
    /// The time component is valid.
    time: LocalTimeValid,
    /// The date and time parts are joined with the ISO designator.
    separator: CombinedDateTimeUsesTimeDesignator,
    /// The date branch is complete rather than reduced-accuracy.
    date_precision: CombinedDateTimeDateComponentMustNotUseReducedAccuracy,
    /// The date and time branches use a single ISO format family across the expression.
    component_format: CombinedDateTimeUsesSingleFormatAcrossDateAndTimeComponents,
}

/// Aggregate proof that an offset date-time representation is structurally valid.
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
pub struct OffsetDateTimeValid {
    /// The local date-time component is valid.
    local: LocalDateTimeValid,
    /// The UTC relationship is represented by a valid numeric offset.
    offset: UtcOffsetValid,
    /// The UTC designator, when used instead of a numeric offset, is uppercase `Z`.
    utc_designator: UtcDesignatorIsUppercaseZ,
}

/// Aggregate proof that a CalConnect explicit date-time form is structurally valid.
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
pub struct ExplicitDateTimeValid {
    /// The date branch is one of the lawful explicit complete-date families.
    date: CompleteDateEvidence,
    /// The time branch is a lawful explicit local-time-of-day form.
    time: ExplicitTimeOfDayValid,
    /// The explicit date precedes the explicit time in the combined representation.
    concatenation: ExplicitDateTimeUsesDateThenTimeConcatenation,
    /// The time branch may use reduced precision.
    reduced_precision: ExplicitDateTimeTimePortionMayBeReducedPrecision,
}

/// Aggregate proof that a CalConnect explicit date-time-with-shift form is structurally valid.
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
pub struct ExplicitDateTimeWithShiftValid {
    /// The explicit local date-time branch is structurally valid.
    local: ExplicitDateTimeValid,
    /// The explicit date-time precedes the explicit time shift.
    concatenation: ExplicitDateTimeWithShiftUsesDateTimeThenShiftConcatenation,
    /// The attached shift is a lawful explicit-form time shift.
    shift: ExplicitTimeShiftValid,
}
