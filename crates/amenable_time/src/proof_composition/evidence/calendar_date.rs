//! Calendar/ordinal/week-date/extended-year evidence branches.
//!
//!
//! Shared `*Evidence` branch decompositions reused across several
//! aggregates, plus the per-form bundles behind the two
//! multi-credential aggregates. Same `#[derive(Evidence, Witness)]`
//! shape as the `proof_composition::composites` family.

use crate::{
    CalendarDateValid, CalendarMonthInRangeOneToTwelve,
    CombinedDateTimePermitsCalendarDateComponent, CombinedDateTimePermitsOrdinalDateComponent,
    CombinedDateTimePermitsWeekDateComponent, ExponentialYearExponentIsPositiveInteger,
    ExponentialYearUsesPowerOfTenNotation, LetterPrefixedCalendarYearMagnitudeExceedsFourDigits,
    LetterPrefixedCalendarYearUsesLeadingYDesignator, NegativeCalendarYearUsesLeadingMinusSign,
    OrdinalDateValid, ReducedCalendarDateHasYearComponent,
    ReducedCalendarDateUsesYearMonthRepresentation, ReducedCalendarDateUsesYearOnlyRepresentation,
    SignificantDigitYearCountIsPositiveInteger, SignificantDigitYearUsesTrailingSSuffix,
    WeekDateValid,
};

/// Evidence branch for the complete date family carried by a combined date-time representation.
///
/// Normative source: ISO 8601-1:2019, 5.4.2 and 5.4.3.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, amenable_derive::Evidence, amenable_derive::Witness,
)]
#[evidence(basis = "Self")]
pub enum CombinedDateTimeDateEvidence {
    /// The combined representation uses the calendar-date family.
    Calendar {
        /// The calendar-date component is structurally valid.
        date: CalendarDateValid,
        /// Clause 4.3 permits the calendar-date family in combined date-time forms.
        family: CombinedDateTimePermitsCalendarDateComponent,
    },
    /// The combined representation uses the ordinal-date family.
    Ordinal {
        /// The ordinal-date component is structurally valid.
        date: OrdinalDateValid,
        /// Clause 4.3 permits the ordinal-date family in combined date-time forms.
        family: CombinedDateTimePermitsOrdinalDateComponent,
    },
    /// The combined representation uses the week-date family.
    Week {
        /// The week-date component is structurally valid.
        date: WeekDateValid,
        /// Clause 4.3 permits the week-date family in combined date-time forms.
        family: CombinedDateTimePermitsWeekDateComponent,
    },
}

impl core::default::Default for CombinedDateTimeDateEvidence {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::Calendar {
            date: core::default::Default::default(),
            family: core::default::Default::default(),
        }
    }
}
/// Evidence branch for the complete date family carried by an explicit date-with-shift value.
///
/// Normative source: CalConnect CC 18011:2018 §4.3 — Date
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, amenable_derive::Evidence, amenable_derive::Witness,
)]
#[evidence(basis = "Self")]
pub enum CompleteDateEvidence {
    /// The complete date uses the calendar-date family.
    Calendar(CalendarDateValid),
    /// The complete date uses the ordinal-date family.
    Ordinal(OrdinalDateValid),
    /// The complete date uses the week-date family.
    Week(WeekDateValid),
}

impl core::default::Default for CompleteDateEvidence {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::Calendar(core::default::Default::default())
    }
}
/// Evidence branch for the base lexical form of an ISO 8601-2 year extension.
///
/// Normative sources: ISO 8601-2:2019, 4.4.1, 4.4.2, 4.7.2, and 4.7.3.
/// Informative cross-check: public LOC EDTF Level 1 — Letter-prefixed calendar year;
/// Negative calendar year. Level 2 — Exponential year
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, amenable_derive::Evidence, amenable_derive::Witness,
)]
#[evidence(basis = "Self")]
pub enum ExtendedYearBaseEvidence {
    /// A four-digit year form, optionally carrying the negative-year lexical law.
    FourDigit {
        /// The representation uses an explicit leading minus sign when negative.
        negative: Option<NegativeCalendarYearUsesLeadingMinusSign>,
    },
    /// A letter-prefixed year form with an integer year payload.
    LetterPrefixed {
        /// The representation uses the leading `Y` designator.
        prefix: LetterPrefixedCalendarYearUsesLeadingYDesignator,
        /// The absolute year magnitude exceeds four digits.
        magnitude: LetterPrefixedCalendarYearMagnitudeExceedsFourDigits,
        /// The representation uses an explicit leading minus sign when negative.
        negative: Option<NegativeCalendarYearUsesLeadingMinusSign>,
    },
    /// A letter-prefixed year form with exponential notation.
    Exponential {
        /// The representation uses the leading `Y` designator.
        prefix: LetterPrefixedCalendarYearUsesLeadingYDesignator,
        /// The representation uses `E` power-of-ten notation.
        exponential: ExponentialYearUsesPowerOfTenNotation,
        /// The exponent is a positive integer.
        exponent: ExponentialYearExponentIsPositiveInteger,
        /// The representation uses an explicit leading minus sign when negative.
        negative: Option<NegativeCalendarYearUsesLeadingMinusSign>,
    },
}

impl core::default::Default for ExtendedYearBaseEvidence {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::FourDigit {
            negative: core::default::Default::default(),
        }
    }
}
/// Evidence branch for the significant-digits suffix on an ISO 8601-2 year form.
///
/// Normative sources: ISO 8601-2:2019, 4.4.3 and 4.7.4.
/// Informative cross-check: public LOC EDTF Level 2 — Significant digits
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
pub struct ExtendedYearSignificantDigitsEvidence {
    /// The year form uses a trailing uppercase `S` suffix.
    suffix: SignificantDigitYearUsesTrailingSSuffix,
    /// The significant-digit count is a positive integer.
    count: SignificantDigitYearCountIsPositiveInteger,
}
/// Evidence branch for the declared reduced calendar-date precision.
///
/// Normative source: ISO 8601-1:2019, 5.2.2.
/// Open-text cross-check: ISO/WD 8601-1:2016(E), 4.1.2.3.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, amenable_derive::Evidence, amenable_derive::Witness,
)]
#[evidence(basis = "Self")]
pub enum ReducedCalendarDatePrecisionEvidence {
    /// The reduced calendar date uses the year-only form.
    YearOnly {
        /// The representation carries a calendar year component.
        year: ReducedCalendarDateHasYearComponent,
        /// The representation uses the year-only lexical form.
        form: ReducedCalendarDateUsesYearOnlyRepresentation,
    },
    /// The reduced calendar date uses the year-month form.
    YearMonth {
        /// The representation carries a calendar year component.
        year: ReducedCalendarDateHasYearComponent,
        /// The representation uses the year-month lexical form.
        form: ReducedCalendarDateUsesYearMonthRepresentation,
        /// The month is within the legal ISO 8601 range.
        month: CalendarMonthInRangeOneToTwelve,
    },
}

impl core::default::Default for ReducedCalendarDatePrecisionEvidence {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::YearOnly {
            year: core::default::Default::default(),
            form: core::default::Default::default(),
        }
    }
}
