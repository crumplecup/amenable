//! ISO 8601-2 Level 2 seasonal and sub-year grouping descriptors.

use derive_builder::Builder;
use derive_getters::Getters;
use strum::EnumIter;

use crate::QualifiedTemporalExpressionDescriptor;

/// A named season carried by an ISO 8601-2 seasonal temporal expression.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    EnumIter,
    derive_more::Display,
    Default,
)]
pub enum NamedSeasonDescriptor {
    /// Spring season.
    #[display("spring")]
    #[default]
    Spring,
    /// Summer season.
    #[display("summer")]
    Summer,
    /// Autumn season.
    #[display("autumn")]
    Autumn,
    /// Winter season.
    #[display("winter")]
    Winter,
}

/// The scope declared by an ISO 8601-2 season code.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    EnumIter,
    derive_more::Display,
    Default,
)]
pub enum SeasonScopeDescriptor {
    /// Named season without explicit hemisphere qualification.
    #[display("location-independent")]
    #[default]
    LocationIndependent,
    /// Named season qualified to the northern hemisphere.
    #[display("northern-hemisphere")]
    NorthernHemisphere,
    /// Named season qualified to the southern hemisphere.
    #[display("southern-hemisphere")]
    SouthernHemisphere,
}

/// A quarter grouping within a calendar year.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    EnumIter,
    derive_more::Display,
    Default,
)]
pub enum QuarterOfYearDescriptor {
    /// First quarter of the year.
    #[display("Q1")]
    #[default]
    First,
    /// Second quarter of the year.
    #[display("Q2")]
    Second,
    /// Third quarter of the year.
    #[display("Q3")]
    Third,
    /// Fourth quarter of the year.
    #[display("Q4")]
    Fourth,
}

/// A quadrimester grouping within a calendar year.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    EnumIter,
    derive_more::Display,
    Default,
)]
pub enum QuadrimesterOfYearDescriptor {
    /// First quadrimester of the year.
    #[display("quadrimester-1")]
    #[default]
    First,
    /// Second quadrimester of the year.
    #[display("quadrimester-2")]
    Second,
    /// Third quadrimester of the year.
    #[display("quadrimester-3")]
    Third,
}

/// A semestral grouping within a calendar year.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    EnumIter,
    derive_more::Display,
    Default,
)]
pub enum SemestralHalfDescriptor {
    /// First semestral half of the year.
    #[display("semestral-1")]
    #[default]
    First,
    /// Second semestral half of the year.
    #[display("semestral-2")]
    Second,
}

/// A neutral vocabulary for ISO 8601-2 Level 2 sub-year groupings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SubYearGroupingDescriptor {
    /// Season grouping, including hemisphere scope when declared.
    Season {
        /// Named season declared by the code.
        season: NamedSeasonDescriptor,
        /// Whether the season is location-independent or hemisphere-qualified.
        scope: SeasonScopeDescriptor,
    },
    /// Quarter grouping.
    Quarter(QuarterOfYearDescriptor),
    /// Quadrimester grouping.
    Quadrimester(QuadrimesterOfYearDescriptor),
    /// Semestral grouping.
    Semestral(SemestralHalfDescriptor),
}

/// A neutral ISO 8601-2 seasonal temporal expression descriptor.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Getters,
    Builder,
    Default,
    amenable_derive::Evidence,
)]
#[evidence(basis = "Self")]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct SeasonalTemporalExpressionDescriptor {
    /// Signed calendar year paired with the season code.
    #[getter(copy)]
    year: i32,
    /// Named season declared by the expression.
    #[getter(copy)]
    season: NamedSeasonDescriptor,
    /// Whether the season is location-independent or hemisphere-qualified.
    #[getter(copy)]
    scope: SeasonScopeDescriptor,
    /// Uncertainty or approximation qualification metadata, when present.
    #[builder(default)]
    #[getter(copy)]
    qualification: Option<QualifiedTemporalExpressionDescriptor>,
}

/// A neutral ISO 8601-2 Level 2 sub-year grouping expression descriptor.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Getters,
    Builder,
    Default,
    amenable_derive::Evidence,
)]
#[evidence(basis = "Self")]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct SubYearGroupingExpressionDescriptor {
    /// Signed calendar year paired with the grouping code.
    #[getter(copy)]
    year: i32,
    /// Specific sub-year grouping declared by the expression.
    #[getter(copy)]
    grouping: SubYearGroupingDescriptor,
    /// Uncertainty or approximation qualification metadata, when present.
    #[builder(default)]
    #[getter(copy)]
    qualification: Option<QualifiedTemporalExpressionDescriptor>,
}

impl core::default::Default for SubYearGroupingDescriptor {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::Season {
            season: core::default::Default::default(),
            scope: core::default::Default::default(),
        }
    }
}
