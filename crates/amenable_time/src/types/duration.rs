//! ISO 8601 and CalConnect duration descriptors.

use derive_builder::Builder;
use derive_getters::Getters;
use strum::EnumIter;

use crate::{
    DurationFractionDescriptor, TimeScaleUnitFractionDescriptor, TimeScaleUnitValueDescriptor,
};

/// A neutral ISO 8601 duration descriptor.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Getters, Builder)]
#[builder(pattern = "owned", setter(into, strip_option), default)]
pub struct DurationDescriptor {
    /// Whole years component.
    #[getter(copy)]
    years: u32,
    /// Whole months component.
    #[getter(copy)]
    months: u32,
    /// Whole weeks component.
    #[getter(copy)]
    weeks: u32,
    /// Whole days component.
    #[getter(copy)]
    days: u32,
    /// Whole hours component.
    #[getter(copy)]
    hours: u32,
    /// Whole minutes component.
    #[getter(copy)]
    minutes: u32,
    /// Whole seconds component.
    #[getter(copy)]
    seconds: u32,
    /// Fraction applied to the lowest-order declared component, when present.
    fractional_component: Option<DurationFractionDescriptor>,
}

/// The signedness carried by a CalConnect explicit duration.
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
pub enum ExplicitDurationSignDescriptor {
    /// The duration advances forward in time.
    #[display("positive")]
    #[default]
    Positive,
    /// The duration advances backward in time.
    #[display("negative")]
    Negative,
}

/// The representation family carried by a CalConnect explicit duration.
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
pub enum ExplicitDurationRepresentationKindDescriptor {
    /// A simple duration expression with a single ordered component sequence.
    #[display("simple")]
    #[default]
    Simple,
    /// A composite duration expression.
    #[display("composite")]
    Composite,
    /// A precedence duration expression whose component order is semantically relevant.
    #[display("precedence")]
    Precedence,
}

/// The semantics family declared for a CalConnect explicit duration.
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
pub enum ExplicitDurationSemanticKindDescriptor {
    /// The duration denotes an exact span independent of placement context.
    #[display("exact")]
    #[default]
    Exact,
    /// The duration's realized span depends on placement on the time scale.
    #[display("context-dependent")]
    ContextDependent,
    /// The duration's realized span depends on future knowledge such as leap-second announcements.
    #[display("speculative")]
    Speculative,
}

/// A CalConnect explicit duration descriptor preserving sign, order, and
/// semantic family.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Getters, Builder, Default)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct ExplicitDurationDescriptor {
    /// Whether the duration advances forward or backward in time.
    #[builder(default = "ExplicitDurationSignDescriptor::Positive")]
    #[getter(copy)]
    sign: ExplicitDurationSignDescriptor,
    /// Whether the duration uses simple, composite, or precedence semantics.
    #[builder(default = "ExplicitDurationRepresentationKindDescriptor::Simple")]
    #[getter(copy)]
    representation_kind: ExplicitDurationRepresentationKindDescriptor,
    /// Ordered durational units preserved exactly as exchanged.
    #[builder(default)]
    components: Vec<TimeScaleUnitValueDescriptor>,
    /// Fraction applied to the lowest-order declared unit, when present.
    #[builder(default)]
    fractional_component: Option<TimeScaleUnitFractionDescriptor>,
    /// Exactness family declared for the duration, when present.
    #[builder(default)]
    #[getter(copy)]
    semantic_kind: Option<ExplicitDurationSemanticKindDescriptor>,
}
