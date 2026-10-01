//! Proof-branch discriminants — the 13 `*ProofBranch` enums from
//! `elicit_temporal::types`, deferred from Phase 2 because they carry
//! `Established<T>` / `*Evidence` payloads. Folded the same way as the
//! [`super::composites`] family: `Established<X>` → `X`, and the
//! elicit_temporal `evidence:` sidecar field is dropped where it is
//! type-identical to the aggregate it accompanies (the folded composite
//! already carries its own decomposition). Each is `#[derive(Evidence,
//! Witness)]` — a proof-branch discriminant for the semantic bundles /
//! exchange outputs of Phases 4–5.
//!
//! Split by the same real domains as `composites`: `timestamp`, `zone`,
//! `interval`.

mod interval;
mod timestamp;
mod zone;

pub use interval::{
    CompleteIntervalSubstitutionProofBranch, DurationRepresentationProofBranch,
    ExplicitIntervalDurationSubstitutionProofBranch,
    ExplicitIntervalEndComponentInheritanceProofBranch,
    ExplicitIntervalShiftPropagationProofBranch, IntervalEndComponentInheritanceProofBranch,
    IntervalZoneInheritanceProofBranch, RecurringIntervalRepresentationProofBranch,
    RecurringIntervalWithRepeatRuleIntervalProofBranch,
};
pub use timestamp::{
    IxdtfAdditionalInformationProofBranch, IxdtfCalendarAnnotationProofBranch,
    IxdtfTimeZoneAnnotationProofBranch,
};
pub use zone::LocalTimeZoneResolutionProofBranch;
