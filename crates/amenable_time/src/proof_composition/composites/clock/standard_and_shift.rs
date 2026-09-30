//! Standard-time and explicit-form time-of-day/time-shift variants.
//!
//!
//! Ported from `elicit_temporal::contracts::proof_composition`. Each
//! aggregate is folded with its `*Evidence` bundle: the aggregate's
//! fields ARE its sub-claims, and `#[derive(Witness)]` makes the
//! composite proof the structural product of its members' proofs.
//! See [`super`] for the design; leaf `Witness<V>` impls land in the
//! backend crates.

use crate::{
    ExplicitTemporalFormMayOmitZeroValuedComponents,
    ExplicitTemporalPrecisionUsesLowestDenotedComponent,
    ExplicitTimeOfDayForbidsEndOfDayRepresentation,
    ExplicitTimeOfDayUsesHourMinuteSecondUnitDesignators, ExplicitTimeOfDayUsesTimeDesignator,
    ExplicitTimeOfDayWithShiftUsesTimeThenShiftConcatenation,
    ExplicitTimeShiftBareZuluRepresentsUtcZero, ExplicitTimeShiftPayloadUsesExplicitTimeOfDay,
    ExplicitTimeShiftUsesLeadingMinusOnlyWhenBehindUtc, ExplicitTimeShiftUsesZuluDesignator,
    FractionAppliesToLowestOrderComponent, FractionUsesDecimalSign, HourInRangeZeroToTwentyFour,
    LocalTimeValid, MinuteInRangeZeroToFiftyNine, SecondInRangeZeroToSixty,
    StandardTimeDerivedFromUtcByLocalShift, StandardTimeOfDayUsesStandardTimeScale,
    TimeShiftIsConstantDurationBetweenTimeScales, UtcOffsetValid, UtcTimeScaleValid,
};

/// Aggregate proof that a standard-time-of-day representation is structurally valid.
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
pub struct StandardTimeOfDayValid {
    /// The wall-clock time-of-day representation is structurally valid.
    time: LocalTimeValid,
    /// The governing standard-time scale is established.
    scale: StandardTimeValid,
    /// The time-of-day interpretation uses standard time.
    semantics: StandardTimeOfDayUsesStandardTimeScale,
}

/// Aggregate proof that standard-time semantics are established.
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
pub struct StandardTimeValid {
    /// The reference time scale is UTC.
    reference: UtcTimeScaleValid,
    /// The local shift from UTC is carried explicitly.
    shift: UtcOffsetValid,
    /// Standard time is derived from UTC by an applicable local shift.
    derivation: StandardTimeDerivedFromUtcByLocalShift,
    /// The shift is a constant duration between the time scales.
    constant_shift: TimeShiftIsConstantDurationBetweenTimeScales,
}

/// Aggregate proof that a CalConnect explicit local-time-of-day form is structurally valid.
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
pub struct ExplicitTimeOfDayValid {
    /// The representation uses the leading `T` designator.
    designator: ExplicitTimeOfDayUsesTimeDesignator,
    /// The representation uses explicit hour, minute, and second unit designators.
    units: ExplicitTimeOfDayUsesHourMinuteSecondUnitDesignators,
    /// The hour value is in range.
    hour: HourInRangeZeroToTwentyFour,
    /// The minute value is in range when present.
    minute: Option<MinuteInRangeZeroToFiftyNine>,
    /// The second value is in range when present.
    second: Option<SecondInRangeZeroToSixty>,
    /// Fraction syntax uses an ISO 8601 decimal sign when present.
    fraction_sign: Option<FractionUsesDecimalSign>,
    /// Any fraction attaches to the lowest-order present component.
    fraction_target: Option<FractionAppliesToLowestOrderComponent>,
    /// Zero-valued components may be omitted from the lexical form.
    zero_omission: ExplicitTemporalFormMayOmitZeroValuedComponents,
    /// The lowest denoted component declares the precision.
    precision: ExplicitTemporalPrecisionUsesLowestDenotedComponent,
    /// Explicit local time of day does not use an end-of-day representation.
    no_end_of_day: ExplicitTimeOfDayForbidsEndOfDayRepresentation,
}

/// Aggregate proof that a complete explicit-form time of day with shift is structurally valid.
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
pub struct TimeOfDayWithShiftValid {
    /// The explicit local-time-of-day branch is structurally valid.
    time: ExplicitTimeOfDayValid,
    /// The explicit time precedes the explicit time shift.
    concatenation: ExplicitTimeOfDayWithShiftUsesTimeThenShiftConcatenation,
    /// The attached shift is a lawful explicit-form time shift.
    shift: ExplicitTimeShiftValid,
}

/// Aggregate proof that a CalConnect explicit time-shift form is structurally valid.
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
pub struct ExplicitTimeShiftValid {
    /// The representation uses the leading `Z` designator.
    designator: ExplicitTimeShiftUsesZuluDesignator,
    /// A leading minus sign appears only when the shift is behind UTC.
    sign: ExplicitTimeShiftUsesLeadingMinusOnlyWhenBehindUtc,
    /// Any non-empty payload uses the explicit local-time-of-day family.
    payload_shape: ExplicitTimeShiftPayloadUsesExplicitTimeOfDay,
    /// A bare `Z` denotes UTC with zero shift.
    bare_utc: ExplicitTimeShiftBareZuluRepresentsUtcZero,
    /// The explicit time payload when present.
    time: Option<ExplicitTimeOfDayValid>,
}
