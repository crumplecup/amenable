//! Plain local/UTC time-of-day forms and the `TimeValid` form-selection enum.
//!
//!
//! Ported from `elicit_temporal::contracts::proof_composition`. Each
//! aggregate is folded with its `*Evidence` bundle: the aggregate's
//! fields ARE its sub-claims, and `#[derive(Witness)]` makes the
//! composite proof the structural product of its members' proofs.
//! See [`super`] for the design; leaf `Witness<V>` impls land in the
//! backend crates.

use crate::{
    FractionAppliesToLowestOrderComponent, FractionUsesDecimalSign, HourInRangeZeroToTwentyFour,
    LeapSecondOccursOnlyAtUtcBoundary, LocalTimeHasHourMinuteSecond,
    LocalTimeScaleMayBeStandardOrNonUtcBased, LocalTimeUsesLocallyApplicableTimeScale,
    MinuteInRangeZeroToFiftyNine, ReducedLocalTimePrecisionEvidence, SecondInRangeZeroToSixty,
    StandardTimeOfDayValid, StandardTimeValid, TimeOfDayOccursWithinCalendarDay,
    TwentyFourHourRequiresZeroMinuteSecondAndFraction, TwentyFourHourReservedForEndOfDay,
    UtcDesignatorIsUppercaseZ, UtcOfDayIdentifiesTimeWithinUtcCalendarDay,
    UtcOfDayUsesTrailingZuluDesignatorImmediately, UtcTimeScaleValid,
};

/// Aggregate proof that a local time representation is structurally valid.
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
pub struct LocalTimeValid {
    /// The representation carries hour, minute, and second fields.
    shape: LocalTimeHasHourMinuteSecond,
    /// The hour value is in range.
    hour: HourInRangeZeroToTwentyFour,
    /// The minute value is in range.
    minute: MinuteInRangeZeroToFiftyNine,
    /// The second value is in range.
    second: SecondInRangeZeroToSixty,
    /// End-of-day usage of hour 24 is legal.
    end_of_day: TwentyFourHourReservedForEndOfDay,
    /// End-of-day hour 24 uses terminal zero minute, second, and fraction values only.
    terminal_end_of_day: TwentyFourHourRequiresZeroMinuteSecondAndFraction,
    /// Leap-second usage is legal.
    leap_second: LeapSecondOccursOnlyAtUtcBoundary,
    /// Fraction syntax uses an ISO 8601 decimal sign.
    fraction_sign: FractionUsesDecimalSign,
    /// Any fraction attaches to the lowest-order present component.
    fraction_target: FractionAppliesToLowestOrderComponent,
}
/// Aggregate proof that a local time-scale interpretation is established.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, amenable_derive::Evidence, amenable_derive::Witness,
)]
#[evidence(basis = "Self")]
pub enum LocalTimeScaleValid {
    /// The local time uses a UTC-derived standard-time scale.
    Standard {
        /// The standard-time interpretation is established explicitly.
        standard_time: StandardTimeValid,
        /// The standard permits local time scales to be UTC-derived standard time.
        locality: LocalTimeScaleMayBeStandardOrNonUtcBased,
    },
    /// The local time uses another non-UTC-based local time scale.
    NonUtcBased {
        /// The standard permits local time scales to be non-UTC-based.
        locality: LocalTimeScaleMayBeStandardOrNonUtcBased,
    },
}

impl core::default::Default for LocalTimeScaleValid {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::Standard {
            standard_time: core::default::Default::default(),
            locality: core::default::Default::default(),
        }
    }
}

/// Aggregate proof that local-time semantics are established.
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
pub struct LocalTimeSemanticsValid {
    /// The wall-clock time-of-day representation is structurally valid.
    time: LocalTimeValid,
    /// The locally applicable time scale is established explicitly.
    scale: LocalTimeScaleValid,
    /// The time is interpreted against a locally applicable time scale.
    semantics: LocalTimeUsesLocallyApplicableTimeScale,
}

/// Aggregate proof that a reduced-accuracy local time representation is structurally valid.
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
pub struct ReducedLocalTimeValid {
    /// The hour value is in range.
    hour: HourInRangeZeroToTwentyFour,
    /// The declared precision branch is structurally valid.
    precision: ReducedLocalTimePrecisionEvidence,
    /// End-of-day usage of hour 24 is legal.
    end_of_day: TwentyFourHourReservedForEndOfDay,
    /// End-of-day hour 24 uses terminal zero minute, second, and fraction values only.
    terminal_end_of_day: TwentyFourHourRequiresZeroMinuteSecondAndFraction,
    /// Fraction syntax uses an ISO 8601 decimal sign.
    fraction_sign: FractionUsesDecimalSign,
    /// Any fraction attaches to the lowest-order present component.
    fraction_target: FractionAppliesToLowestOrderComponent,
}

/// Aggregate proof that a UTC-of-day representation is structurally valid.
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
pub struct UtcOfDayValid {
    /// The carried time-of-day representation is structurally valid.
    time: LocalTimeValid,
    /// The representation is anchored to the UTC reference time scale.
    scale: UtcTimeScaleValid,
    /// The value identifies a position within a UTC calendar day.
    position: UtcOfDayIdentifiesTimeWithinUtcCalendarDay,
    /// The UTC time-of-day payload is followed immediately by the `Z` designator.
    designator: UtcOfDayUsesTrailingZuluDesignatorImmediately,
    /// The UTC designator uses uppercase `Z`.
    uppercase_z: UtcDesignatorIsUppercaseZ,
}

/// Aggregate proof that an ISO 8601 time-of-day representation is structurally valid.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, amenable_derive::Evidence, amenable_derive::Witness,
)]
#[evidence(basis = "Self")]
pub enum TimeValid {
    /// Local clock-time representation.
    Local {
        /// The local clock-time representation is structurally valid.
        time: LocalTimeValid,
        /// A time of day occurs within a calendar day.
        semantics: TimeOfDayOccursWithinCalendarDay,
    },
    /// Reduced-accuracy local clock-time representation.
    ReducedLocal {
        /// The reduced local-time representation is structurally valid.
        time: ReducedLocalTimeValid,
        /// A time of day occurs within a calendar day.
        semantics: TimeOfDayOccursWithinCalendarDay,
    },
    /// UTC-of-day representation.
    Utc {
        /// The UTC-of-day representation is structurally valid.
        time: UtcOfDayValid,
        /// A time of day occurs within a calendar day.
        semantics: TimeOfDayOccursWithinCalendarDay,
    },
    /// Standard-time-of-day representation.
    Standard {
        /// The standard-time-of-day representation is structurally valid.
        time: StandardTimeOfDayValid,
        /// A time of day occurs within a calendar day.
        semantics: TimeOfDayOccursWithinCalendarDay,
    },
}

impl core::default::Default for TimeValid {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::Local {
            time: core::default::Default::default(),
            semantics: core::default::Default::default(),
        }
    }
}
