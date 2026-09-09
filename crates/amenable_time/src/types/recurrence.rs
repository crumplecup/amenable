//! CalConnect CC 18012 recurrence descriptors: date-time formulas,
//! selection expressions, and repeat rules.

use derive_builder::Builder;
use derive_getters::Getters;
use derive_new::new;
use strum::EnumIter;

use crate::{
    DurationDescriptor, ExplicitDurationDescriptor, ExplicitTemporalValueDescriptor,
    ExplicitTimeIntervalDescriptor, TimeIntervalDescriptor, TimeScaleUnitValueDescriptor,
};

/// The declared evaluation family for a date-time formula.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, EnumIter, derive_more::Display,
)]
pub enum DateTimeFormulaEvaluationKindDescriptor {
    /// Simple-duration evaluation.
    #[display("simple")]
    Simple,
    /// Composite-duration evaluation.
    #[display("composite")]
    Composite,
    /// Precedence-duration evaluation.
    #[display("precedence")]
    Precedence,
}

/// An explicit temporal value plus a duration under the CalConnect
/// formula model.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Getters, new)]
pub struct DateTimeFormulaDescriptor {
    /// The explicit date or time value being resolved.
    value: ExplicitTemporalValueDescriptor,
    /// The duration applied to that value.
    duration: ExplicitDurationDescriptor,
    /// Declared evaluation family for the formula.
    #[getter(copy)]
    evaluation_kind: DateTimeFormulaEvaluationKindDescriptor,
}

/// One CalConnect selection-rule component.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SelectionRuleDescriptor {
    /// Restrict by month numbers.
    Months(Vec<u8>),
    /// Restrict by week numbers, allowing negative indexing from the end.
    Weeks(Vec<i32>),
    /// Restrict by day-of-month values, allowing negative indexing from the end.
    DaysOfMonth(Vec<i32>),
    /// Restrict by ISO weekday numbers.
    Weekdays(Vec<u8>),
    /// Restrict by ordinal day-of-year values, allowing negative indexing.
    OrdinalDaysOfYear(Vec<i32>),
    /// Restrict by hour values.
    Hours(Vec<u8>),
    /// Restrict by minute values.
    Minutes(Vec<u8>),
    /// Restrict by second values.
    Seconds(Vec<u8>),
    /// Restrict by positional selection within the prior result set.
    Position(Vec<i32>),
    /// Extend a selection component by an explicit duration window.
    DurationWindow {
        /// The selection component being extended.
        rule: Box<SelectionRuleDescriptor>,
        /// Duration window applied to the selected component.
        duration: DurationDescriptor,
    },
}

/// A full CalConnect selection expression with optional single-instance
/// semantics.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Getters, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct SelectionExpressionDescriptor {
    /// Selection rules in lexical order.
    #[builder(default)]
    rules: Vec<SelectionRuleDescriptor>,
    /// Whether the selection denotes a single instance via the `I` designator.
    #[builder(default)]
    #[getter(copy)]
    selects_single_instance: bool,
}

/// Eligible time intervals used as the repeating cycle for repeat-rule
/// evaluation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Getters, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct EligibleTimeIntervalsDescriptor {
    /// One or more eligible interval unit declarations.
    #[builder(default)]
    units: Vec<TimeScaleUnitValueDescriptor>,
}

/// A repeat-rule payload joining eligible intervals to a selection
/// expression.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Getters, new)]
pub struct RepeatRuleDescriptor {
    /// Repeating cycle that defines eligible intervals.
    eligible_intervals: EligibleTimeIntervalsDescriptor,
    /// Selection rules applied within each eligible interval.
    selection: SelectionExpressionDescriptor,
}

/// The interval family admitted by a complete recurring interval with a
/// repeat rule.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RecurringIntervalWithRepeatRuleIntervalDescriptor {
    /// A complete ISO 8601 interval representation.
    IsoComplete(TimeIntervalDescriptor),
    /// A CalConnect explicit interval representation.
    Explicit(ExplicitTimeIntervalDescriptor),
}

/// A complete recurring-interval representation extended with a repeat
/// rule.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Getters, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct RecurringIntervalWithRepeatRuleDescriptor {
    /// Bounded repetition count; `None` denotes unbounded recurrence.
    #[builder(default)]
    #[getter(copy)]
    repetitions: Option<u32>,
    /// Base complete interval being repeated.
    interval: RecurringIntervalWithRepeatRuleIntervalDescriptor,
    /// Repeat-rule refinement applied to the recurrence.
    repeat_rule: RepeatRuleDescriptor,
}
