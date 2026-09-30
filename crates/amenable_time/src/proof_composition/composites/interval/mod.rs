//! Duration/interval/recurring-interval family aggregate proof propositions.

mod basic;
mod enhanced;
mod recurring;

pub use basic::{
    DurationFormValid, DurationRepresentationSemanticsValid, ExplicitDurationValid,
    ExplicitTimeIntervalValid, IntervalEndpointsOrdered, TimeIntervalValid,
};
pub use enhanced::{
    CompleteIntervalSubstitutionSemanticsValid, EnhancedIntervalLevelOneSemanticsValid,
    EnhancedIntervalLevelTwoSemanticsValid, ExplicitIntervalDurationSubstitutionSemanticsValid,
    ExplicitIntervalEndComponentInheritanceSemanticsValid,
    ExplicitIntervalShiftPropagationSemanticsValid, ExtendedIntervalBoundarySemanticsValid,
    InheritedIntervalEndComponentsSemanticsValid, InheritedIntervalZoneSemanticsValid,
};
pub use recurring::{
    CompleteRecurringIntervalRepresentationSemanticsValid,
    OtherThanCompleteRecurringIntervalRepresentationSemanticsValid, RecurringIntervalFormValid,
    RecurringIntervalWithRepeatRuleValid, RepeatRuleValid,
};
