//! Local-to-zone resolution proof-branch discriminant.
//!
//!
//! Proof-branch discriminants -- the 13 `*ProofBranch` enums from
//! `elicit_temporal::types`, deferred from Phase 2 because they carry
//! `Established<T>` / `*Evidence` payloads. Folded the same way as the
//! `proof_composition::composites` family: `Established<X>` -> `X`, and
//! the elicit_temporal `evidence:` sidecar field is dropped where it is
//! type-identical to the aggregate it accompanies (the folded composite
//! already carries its own decomposition). Each is `#[derive(Evidence,
//! Witness)]` -- a proof-branch discriminant for the semantic bundles /
//! exchange outputs of Phases 4-5.

use crate::{
    LocalDateTimeMayBeAmbiguousAtZoneTransition, LocalDateTimeMayFallInZoneTransitionGap,
    ZoneTransitionAmbiguitySemanticsValid, ZoneTransitionGapSemanticsValid,
};

/// Explicit proof branch for local-to-zone resolution across transition edge cases.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, Default, amenable_derive::Evidence, amenable_derive::Witness,
)]
#[evidence(basis = "Self")]
pub enum LocalTimeZoneResolutionProofBranch {
    /// The local wall-clock time mapped to a single instant without transition special handling.
    #[default]
    Unambiguous,
    /// The local wall-clock time was ambiguous and required explicit disambiguation authority.
    Ambiguous {
        /// The local timestamp can be ambiguous at a zone transition.
        possibility: LocalDateTimeMayBeAmbiguousAtZoneTransition,
        /// Aggregate proof that ambiguity semantics were handled explicitly and lawfully.
        semantics: ZoneTransitionAmbiguitySemanticsValid,
    },
    /// The local wall-clock time fell inside a skipped transition gap.
    Gap {
        /// The local timestamp can fall inside a skipped zone-transition gap.
        possibility: LocalDateTimeMayFallInZoneTransitionGap,
        /// Aggregate proof that gap semantics were handled explicitly and lawfully.
        semantics: ZoneTransitionGapSemanticsValid,
    },
}
