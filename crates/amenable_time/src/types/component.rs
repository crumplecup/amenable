//! Temporal-component vocabulary: the smallest named units and the
//! fraction / qualification descriptors that hang directly off them.

use derive_getters::Getters;
use derive_new::new;
use strum::EnumIter;

/// The smallest named temporal unit relevant to ISO 8601 precision rules.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, EnumIter, derive_more::Display,
)]
pub enum TemporalComponent {
    /// Calendar year.
    #[display("year")]
    Year,
    /// Calendar decade.
    #[display("decade")]
    Decade,
    /// Calendar century.
    #[display("century")]
    Century,
    /// Calendar month.
    #[display("month")]
    Month,
    /// Calendar day.
    #[display("day")]
    Day,
    /// Clock hour.
    #[display("hour")]
    Hour,
    /// Clock minute.
    #[display("minute")]
    Minute,
    /// Clock second.
    #[display("second")]
    Second,
}

/// Time-scale units recognized by the broader explicit and recurrence
/// standards family (ISO 8601 plus CalConnect), which admit a calendar
/// week alongside the [`TemporalComponent`] set.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, EnumIter, derive_more::Display,
)]
pub enum TimeScaleUnitDescriptor {
    /// Calendar year.
    #[display("year")]
    Year,
    /// Calendar decade.
    #[display("decade")]
    Decade,
    /// Calendar century.
    #[display("century")]
    Century,
    /// Calendar month.
    #[display("month")]
    Month,
    /// Calendar week.
    #[display("week")]
    Week,
    /// Calendar day.
    #[display("day")]
    Day,
    /// Clock hour.
    #[display("hour")]
    Hour,
    /// Clock minute.
    #[display("minute")]
    Minute,
    /// Clock second.
    #[display("second")]
    Second,
}

/// Lexical placement family for ISO 8601-2 qualification markers.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, EnumIter, derive_more::Display,
)]
pub enum QualificationPlacementDescriptor {
    /// The qualification marker appears immediately to the right of the marked component.
    #[display("group-right")]
    GroupRight,
    /// The qualification marker appears immediately to the left of the marked component.
    #[display("component-left")]
    ComponentLeft,
}

/// Declared scope for an ISO 8601-2 qualification marker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum QualificationScopeDescriptor {
    /// The qualification applies to the named component and every more significant component.
    ThroughComponent(TemporalComponent),
    /// The qualification applies only to the named component.
    OnlyComponent(TemporalComponent),
}

/// Explicit ISO 8601-2 uncertainty or approximation qualification metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Getters, new)]
pub struct QualifiedTemporalExpressionDescriptor {
    /// Whether uncertainty is explicitly declared.
    #[getter(copy)]
    uncertainty: bool,
    /// Whether approximation is explicitly declared.
    #[getter(copy)]
    approximation: bool,
    /// Lexical placement family used by the qualification marker.
    #[getter(copy)]
    placement: QualificationPlacementDescriptor,
    /// Scope to which the qualification applies.
    #[getter(copy)]
    scope: QualificationScopeDescriptor,
}

/// Fractional-second digits preserved as a lexical decimal suffix.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Getters, new)]
pub struct FractionalSecondDescriptor {
    /// Decimal digits appearing after the fractional separator.
    #[new(into)]
    digits: String,
}

/// One valued time-scale unit within grouped-unit or recurrence semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Getters, new)]
pub struct TimeScaleUnitValueDescriptor {
    /// The unit family being counted.
    #[getter(copy)]
    unit: TimeScaleUnitDescriptor,
    /// Positive coefficient attached to that unit.
    #[getter(copy)]
    value: u32,
}

/// A fractional time-scale unit represented lexically for explicit-duration forms.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Getters, new)]
pub struct TimeScaleUnitFractionDescriptor {
    /// Lowest-order unit to which the fraction applies.
    #[getter(copy)]
    unit: TimeScaleUnitDescriptor,
    /// Decimal digits appearing after the fractional separator.
    #[new(into)]
    digits: String,
}

/// A fractional duration component represented lexically.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Getters, new)]
pub struct DurationFractionDescriptor {
    /// Lowest-order component to which the fraction applies.
    #[getter(copy)]
    component: TemporalComponent,
    /// Decimal digits appearing after the fractional separator.
    #[new(into)]
    digits: String,
}
