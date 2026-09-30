//! Local-to-zone ambiguity/gap zone-transition-resolution propositions.
//!
//!
//! Ported from `elicit_temporal::contracts::proof_composition`. Each
//! aggregate is folded with its `*Evidence` bundle: the aggregate's
//! fields ARE its sub-claims, and `#[derive(Witness)]` makes the
//! composite proof the structural product of its members' proofs.
//! See [`super`] for the design; leaf `Witness<V>` impls land in the
//! backend crates.

use crate::{
    LocalDateTimeMayBeAmbiguousAtZoneTransition, LocalDateTimeMayFallInZoneTransitionGap,
    LocalDateTimeRequiresZoneOrOffsetForInstant, LocalDateTimeValid,
    NamedTimeZoneIdentifierExcludesDotSegments, NamedTimeZoneUsesIanaIdentifier,
    NamedZoneAttachmentEvidence, ZoneTransitionAmbiguityDeclared,
    ZoneTransitionDisambiguationAuthorityDeclared, ZoneTransitionGapDeclared,
    ZoneTransitionGapHandlingAuthorityDeclared, ZonedTimestampEvidence,
};

/// Aggregate proof that ambiguous local-time resolution semantics are explicit and lawful.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct ZoneTransitionAmbiguitySemanticsValid {
    /// The local date-time is still awaiting zone authority to identify an instant.
    local_semantics: LocalDateTimeDoesNotIdentifyFixedInstant,
    /// The local date-time can be ambiguous at a transition boundary.
    ambiguity_possibility: LocalDateTimeMayBeAmbiguousAtZoneTransition,
    /// The producer explicitly declares that a transition ambiguity is in play.
    ambiguity_declared: ZoneTransitionAmbiguityDeclared,
    /// The selected disambiguation and gap-handling policy is explicit.
    resolution_authority: ZoneTransitionResolutionAuthorityValid,
}

/// Aggregate proof that skipped local-time gap semantics are explicit and lawful.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct ZoneTransitionGapSemanticsValid {
    /// The local date-time is still awaiting zone authority to identify an instant.
    local_semantics: LocalDateTimeDoesNotIdentifyFixedInstant,
    /// The local date-time can fall inside a skipped transition gap.
    gap_possibility: LocalDateTimeMayFallInZoneTransitionGap,
    /// The producer explicitly declares that a transition gap is in play.
    gap_declared: ZoneTransitionGapDeclared,
    /// The selected disambiguation and gap-handling policy is explicit.
    resolution_authority: ZoneTransitionResolutionAuthorityValid,
}

/// Aggregate proof that explicit local-to-zone resolution authority is fully declared.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct ZoneTransitionResolutionAuthorityValid {
    /// The named zone is carried by an IANA time-zone identifier.
    zone_identifier: NamedTimeZoneUsesIanaIdentifier,
    /// The zone identifier excludes the forbidden `"."` and `".."` segments.
    zone_segments: NamedTimeZoneIdentifierExcludesDotSegments,
    /// Ambiguous local times use explicit disambiguation authority.
    disambiguation: ZoneTransitionDisambiguationAuthorityDeclared,
    /// Skipped local times use explicit gap-handling authority.
    gap_handling: ZoneTransitionGapHandlingAuthorityDeclared,
}

/// Aggregate proof that a timestamp carries named-zone identity.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, amenable_derive::Evidence, amenable_derive::Witness,
)]
#[evidence(basis = "Self")]
pub enum ZonedDateTimeHasNamedZone {
    /// Established via the `NamedZoneAttachmentEvidence` decomposition.
    NamedZoneAttachment(NamedZoneAttachmentEvidence),
    /// Established via the `ZonedTimestampEvidence` decomposition.
    ZonedTimestamp(ZonedTimestampEvidence),
}

impl core::default::Default for ZonedDateTimeHasNamedZone {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::NamedZoneAttachment(core::default::Default::default())
    }
}

/// Aggregate proof that a local date-time does not, by itself, identify a fixed instant.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct LocalDateTimeDoesNotIdentifyFixedInstant {
    /// The local date-time representation is structurally valid.
    local_date_time: LocalDateTimeValid,
    /// Additional zone or offset authority is required to identify an instant.
    requires_authority: LocalDateTimeRequiresZoneOrOffsetForInstant,
}
