//! Local-time/clock/standard-time family aggregate proof propositions.

mod local;
mod standard_and_shift;

pub use local::{
    LocalTimeScaleValid, LocalTimeSemanticsValid, LocalTimeValid, ReducedLocalTimeValid, TimeValid,
    UtcOfDayValid,
};
pub use standard_and_shift::{
    ExplicitTimeOfDayValid, ExplicitTimeShiftValid, StandardTimeOfDayValid, StandardTimeValid,
    TimeOfDayWithShiftValid,
};
