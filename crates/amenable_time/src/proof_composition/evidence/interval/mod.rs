//! Duration/complete-interval-substitution evidence branches.

mod complete_substitution;
mod duration;

pub use complete_substitution::{
    CompleteDurationEndIntervalSubstitutionEvidence,
    CompleteIntervalDurationRepresentationEvidence,
    CompleteStartDurationIntervalSubstitutionEvidence,
    CompleteStartEndIntervalSubstitutionEvidence, CompleteTimePointDateRepresentationEvidence,
    CompleteTimePointRepresentationEvidence, CompleteTimePointTimeRepresentationEvidence,
};
pub use duration::{
    DurationAlternativeFormEvidence, DurationDesignatorRepresentationEvidence,
    DurationWeekFormEvidence, ExplicitDurationRepresentationEvidence,
    ExplicitDurationSemanticEvidence,
};
