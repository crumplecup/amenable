//! Shared `*Evidence` branch decompositions reused across several
//! aggregates, plus the per-form bundles behind the two
//! multi-credential aggregates. Same `#[derive(Evidence, Witness)]`
//! shape as the [`super::composites`] family.
//!
//! `branches_a.rs`/`branches_b.rs` previously split these 24 types
//! purely alphabetically (their own doc comments named letter ranges:
//! "(C–D)" and "(E–Z)") — the same anti-pattern already fixed in
//! `composites`. Replaced with the same real domains used there:
//! `calendar_date`, `conversion`, `zone`, `clock`, `extension`, and
//! `interval` (split further into `duration`/`complete_substitution`
//! since it holds more than 10 types).

mod calendar_date;
mod clock;
mod conversion;
mod extension;
mod interval;
mod zone;

pub use calendar_date::{
    CombinedDateTimeDateEvidence, CompleteDateEvidence, ExtendedYearBaseEvidence,
    ExtendedYearSignificantDigitsEvidence, ReducedCalendarDatePrecisionEvidence,
};
pub use clock::ReducedLocalTimePrecisionEvidence;
pub use conversion::{MutualAgreementAuthorityScopeEvidence, QualificationPlacementEvidence};
pub use extension::SubYearGroupingKindEvidence;
pub use interval::{
    CompleteDurationEndIntervalSubstitutionEvidence,
    CompleteIntervalDurationRepresentationEvidence,
    CompleteStartDurationIntervalSubstitutionEvidence,
    CompleteStartEndIntervalSubstitutionEvidence, CompleteTimePointDateRepresentationEvidence,
    CompleteTimePointRepresentationEvidence, CompleteTimePointTimeRepresentationEvidence,
    DurationAlternativeFormEvidence, DurationDesignatorRepresentationEvidence,
    DurationWeekFormEvidence, ExplicitDurationRepresentationEvidence,
    ExplicitDurationSemanticEvidence,
};
pub use zone::{NamedZoneAttachmentEvidence, UtcOffsetPrecisionEvidence, ZonedTimestampEvidence};
