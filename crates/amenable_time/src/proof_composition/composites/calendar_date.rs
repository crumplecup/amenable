//! Calendar/ordinal/week-date family aggregate proof propositions.
//!
//!
//! Ported from `elicit_temporal::contracts::proof_composition`. Each
//! aggregate is folded with its `*Evidence` bundle: the aggregate's
//! fields ARE its sub-claims, and `#[derive(Witness)]` makes the
//! composite proof the structural product of its members' proofs.
//! See [`super`] for the design; leaf `Witness<V>` impls land in the
//! backend crates.

use crate::{
    BeforeYearOneValueUsesTrailingBSuffix, CalendarDateHasYearMonthDay,
    CalendarDateUsesGregorianCalendar, CalendarDayWithinMonthBounds,
    CalendarMonthInRangeOneToTwelve, CenturyOrdinalInRangeZeroToNinetyNine, CompleteDateEvidence,
    DateIdentifiesPositionWithinCalendar, DecadeOrdinalInRangeZeroToNineHundredNinetyNine,
    ExplicitDateWithShiftUsesDateThenShiftConcatenation, ExplicitTimeShiftValid,
    ExtendedYearBaseEvidence, ExtendedYearSignificantDigitsEvidence, LeapDayOccursOnlyInLeapYear,
    OrdinalDateHasYearAndDayOfYear, OrdinalDayInRangeOneToThreeHundredSixtySix,
    ReducedCalendarDatePrecisionEvidence, WeekDateHasWeekYearWeekAndWeekday,
    WeekNumberInRangeOneToFiftyThree, WeekdayInRangeOneToSeven,
};

/// Aggregate proof that a complete calendar date is structurally valid.
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
pub struct CalendarDateValid {
    /// The representation is a Gregorian calendar date.
    calendar: CalendarDateUsesGregorianCalendar,
    /// The representation carries year, month, and day fields.
    shape: CalendarDateHasYearMonthDay,
    /// The month is within the legal ISO 8601 range.
    month: CalendarMonthInRangeOneToTwelve,
    /// The day is legal for the specific month and year.
    day: CalendarDayWithinMonthBounds,
    /// Leap-day usage satisfies the Gregorian leap-year rule.
    leap_day: LeapDayOccursOnlyInLeapYear,
}

/// Aggregate proof that an ISO 8601 date representation is structurally valid.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, amenable_derive::Evidence, amenable_derive::Witness,
)]
#[evidence(basis = "Self")]
pub enum DateValid {
    /// Gregorian calendar-date representation.
    Calendar {
        /// The calendar-date representation is structurally valid.
        date: CalendarDateValid,
        /// A date identifies a position within the calendar.
        semantics: DateIdentifiesPositionWithinCalendar,
    },
    /// Ordinal-date representation.
    Ordinal {
        /// The ordinal-date representation is structurally valid.
        date: OrdinalDateValid,
        /// A date identifies a position within the calendar.
        semantics: DateIdentifiesPositionWithinCalendar,
    },
    /// Week-date representation.
    Week {
        /// The week-date representation is structurally valid.
        date: WeekDateValid,
        /// A date identifies a position within the calendar.
        semantics: DateIdentifiesPositionWithinCalendar,
    },
}

impl core::default::Default for DateValid {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::Calendar {
            date: core::default::Default::default(),
            semantics: core::default::Default::default(),
        }
    }
}

/// Aggregate proof that a complete explicit-form date with shift is structurally valid.
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
pub struct DateWithShiftValid {
    /// The date branch is one of the lawful explicit complete-date families.
    date: CompleteDateEvidence,
    /// The explicit date precedes the explicit time shift.
    concatenation: ExplicitDateWithShiftUsesDateThenShiftConcatenation,
    /// The attached shift is a lawful explicit-form time shift.
    shift: ExplicitTimeShiftValid,
}

/// Aggregate proof that a Gregorian calendar decade representation is structurally valid.
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
pub struct DecadeValid {
    /// The decade ordinal is within the legal ISO range.
    ordinal: DecadeOrdinalInRangeZeroToNineHundredNinetyNine,
    /// Before-year-one syntax, when used, is declared by a trailing `B` suffix.
    before_year_one: Option<BeforeYearOneValueUsesTrailingBSuffix>,
}

/// Aggregate proof that a Gregorian calendar century representation is structurally valid.
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
pub struct CenturyValid {
    /// The century ordinal is within the legal ISO range.
    ordinal: CenturyOrdinalInRangeZeroToNinetyNine,
    /// Before-year-one syntax, when used, is declared by a trailing `B` suffix.
    before_year_one: Option<BeforeYearOneValueUsesTrailingBSuffix>,
}

/// Aggregate proof that an ISO 8601-2 extended year form is structurally valid.
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
pub struct ExtendedYearValid {
    /// The base year form is structurally valid.
    base: ExtendedYearBaseEvidence,
    /// Optional significant-digit semantics attached to the base form.
    significant_digits: Option<ExtendedYearSignificantDigitsEvidence>,
}

/// Aggregate proof that a complete ordinal date is structurally valid.
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
pub struct OrdinalDateValid {
    /// The representation carries year and day-of-year fields.
    shape: OrdinalDateHasYearAndDayOfYear,
    /// The ordinal day is in range for the target year.
    day: OrdinalDayInRangeOneToThreeHundredSixtySix,
}

/// Aggregate proof that a reduced-precision calendar date is structurally valid.
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
pub struct ReducedCalendarDateValid {
    /// The representation is a Gregorian calendar date.
    calendar: CalendarDateUsesGregorianCalendar,
    /// The declared precision branch is structurally valid.
    precision: ReducedCalendarDatePrecisionEvidence,
}

/// Aggregate proof that a complete week date is structurally valid.
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
pub struct WeekDateValid {
    /// The representation carries week-year, week, and weekday fields.
    shape: WeekDateHasWeekYearWeekAndWeekday,
    /// The week number is in the legal ISO 8601 range.
    week: WeekNumberInRangeOneToFiftyThree,
    /// The weekday is in the legal ISO 8601 range.
    weekday: WeekdayInRangeOneToSeven,
}
