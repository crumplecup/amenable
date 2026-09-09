//! Calendar-, ordinal-, week-, decade-, century-, and extended-year date
//! descriptors, plus the umbrella date families that range over them.

use derive_builder::Builder;
use derive_getters::Getters;
use derive_new::new;

/// A complete Gregorian calendar date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Getters, new)]
pub struct CalendarDateDescriptor {
    /// Signed calendar year.
    #[getter(copy)]
    year: i32,
    /// Calendar month number.
    #[getter(copy)]
    month: u8,
    /// Day of month.
    #[getter(copy)]
    day: u8,
}

/// A reduced-precision Gregorian calendar date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ReducedCalendarDateDescriptor {
    /// Year-only calendar date form.
    Year {
        /// Signed calendar year.
        year: i32,
    },
    /// Year-month calendar date form.
    YearMonth {
        /// Signed calendar year.
        year: i32,
        /// Calendar month number.
        month: u8,
    },
}

/// A Gregorian calendar decade.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Getters, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct DecadeDescriptor {
    /// Three-digit ordinal number for the Gregorian calendar decade.
    #[getter(copy)]
    ordinal: u16,
    /// Whether the expression refers to a value before year one using the `B` suffix.
    #[builder(default)]
    #[getter(copy)]
    before_year_one: bool,
}

/// A Gregorian calendar century.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Getters, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct CenturyDescriptor {
    /// Two-digit ordinal number for the Gregorian calendar century.
    #[getter(copy)]
    ordinal: u8,
    /// Whether the expression refers to a value before year one using the `B` suffix.
    #[builder(default)]
    #[getter(copy)]
    before_year_one: bool,
}

/// The base lexical form for an ISO 8601-2 extended year expression.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ExtendedYearBaseDescriptor {
    /// Four-digit year form, which may also carry ISO 8601-2 extensions
    /// such as negative-year or significant-digit semantics.
    FourDigit {
        /// Signed year value.
        year: i32,
    },
    /// Letter-prefixed year form using a leading `Y` designator.
    LetterPrefixed {
        /// Signed year value.
        year: i64,
    },
    /// Letter-prefixed exponential year form.
    Exponential {
        /// Signed significand before the `E` separator.
        significand: i64,
        /// Positive exponent after the `E` separator.
        exponent: u32,
    },
}

/// A neutral ISO 8601-2 extended year descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Getters, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct ExtendedYearDescriptor {
    /// The base year form carried by the expression.
    #[getter(copy)]
    base: ExtendedYearBaseDescriptor,
    /// Number of significant digits declared by a trailing `S` suffix, when present.
    #[builder(default)]
    #[getter(copy)]
    significant_digits: Option<u32>,
}

/// A complete ordinal date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Getters, new)]
pub struct OrdinalDateDescriptor {
    /// Signed calendar year.
    #[getter(copy)]
    year: i32,
    /// Day number within the year.
    #[getter(copy)]
    day_of_year: u16,
}

/// A complete week date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Getters, new)]
pub struct WeekDateDescriptor {
    /// ISO week-based year.
    #[getter(copy)]
    week_year: i32,
    /// ISO week number.
    #[getter(copy)]
    week: u8,
    /// ISO weekday number (`1` = Monday, `7` = Sunday).
    #[getter(copy)]
    weekday: u8,
}

/// The complete-date representation family recognized by the
/// explicit-form standards.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CompleteDateDescriptor {
    /// A complete Gregorian calendar date.
    Calendar(CalendarDateDescriptor),
    /// A complete ordinal date.
    Ordinal(OrdinalDateDescriptor),
    /// A complete week date.
    Week(WeekDateDescriptor),
}

/// The date representation family for the ISO 8601 umbrella date concept.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DateDescriptor {
    /// A Gregorian calendar date.
    Calendar(CalendarDateDescriptor),
    /// An ordinal date.
    Ordinal(OrdinalDateDescriptor),
    /// A week date.
    Week(WeekDateDescriptor),
}
