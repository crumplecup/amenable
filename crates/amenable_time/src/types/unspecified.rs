//! ISO 8601-2 unspecified-digit (masked) temporal component descriptors.

use derive_builder::Builder;
use derive_getters::Getters;
use strum::EnumIter;

use crate::QualifiedTemporalExpressionDescriptor;

/// One digit of a masked ISO 8601-2 numeric component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, derive_more::Display)]
pub enum UnspecifiedDigitDescriptor {
    /// A concrete decimal digit.
    #[display("{_0}")]
    Digit(u8),
    /// The uppercase `X` placeholder for an unspecified digit.
    #[display("X")]
    Unspecified,
}

/// A neutral digit-vector descriptor for ISO 8601-2 masked numeric
/// components.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Getters, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct MaskedNumericComponentDescriptor {
    /// Component digits in lexical order, preserving any `X` placeholders.
    #[builder(default)]
    digits: Vec<UnspecifiedDigitDescriptor>,
}

/// The declared public-profile masking regime for unspecified-digit
/// expressions.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, EnumIter, derive_more::Display,
)]
pub enum UnspecifiedPrecisionProfileDescriptor {
    /// Level 1 rightmost-only masking.
    #[display("level-1-rightmost")]
    LevelOneRightmost,
    /// Level 2 masking that may appear anywhere within a component.
    #[display("level-2-any-position")]
    LevelTwoAnyPosition,
}

/// A neutral shape vocabulary for ISO 8601-2 unspecified-component
/// expressions.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum UnspecifiedTemporalShapeDescriptor {
    /// A masked calendar-date expression.
    CalendarDate {
        /// Year component digits.
        year: MaskedNumericComponentDescriptor,
        /// Month component digits, when present.
        month: Option<MaskedNumericComponentDescriptor>,
        /// Day component digits, when present.
        day: Option<MaskedNumericComponentDescriptor>,
    },
    /// A masked ordinal-date expression.
    OrdinalDate {
        /// Year component digits.
        year: MaskedNumericComponentDescriptor,
        /// Ordinal day digits.
        day_of_year: MaskedNumericComponentDescriptor,
    },
    /// A masked week-date expression.
    WeekDate {
        /// Week-based year digits.
        week_year: MaskedNumericComponentDescriptor,
        /// Week number digits.
        week: MaskedNumericComponentDescriptor,
        /// Weekday digits, when present.
        weekday: Option<MaskedNumericComponentDescriptor>,
    },
}

/// A neutral ISO 8601-2 unspecified-component expression descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Getters, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct UnspecifiedComponentExpressionDescriptor {
    /// Underlying masked temporal shape.
    shape: UnspecifiedTemporalShapeDescriptor,
    /// Declared masking regime.
    #[getter(copy)]
    profile: UnspecifiedPrecisionProfileDescriptor,
    /// Uncertainty or approximation qualification metadata, when present.
    #[builder(default)]
    #[getter(copy)]
    qualification: Option<QualifiedTemporalExpressionDescriptor>,
}
