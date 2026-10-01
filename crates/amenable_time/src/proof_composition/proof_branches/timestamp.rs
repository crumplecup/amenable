//! RFC 9557 / IXDTF annotation proof-branch discriminants.
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
    IxdtfAdditionalInformationSemanticsValid, IxdtfTimestampHasPreferredPresentationCalendar,
    OffsetTimeZoneAnnotationConsistentWithTimestamp, ZonedDateTimeHasNamedZone,
    ZonedTimestampEvidence,
};

/// Explicit RFC 9557 time-zone annotation proof branch carried by IXDTF exchanges.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, Default, amenable_derive::Evidence, amenable_derive::Witness,
)]
#[evidence(basis = "Self")]
pub enum IxdtfTimeZoneAnnotationProofBranch {
    /// No RFC 9557 time-zone annotation is present.
    #[default]
    None,
    /// A named-zone annotation is present and carries civil-rule identity.
    Named {
        /// Aggregate proof that the timestamp carries named-zone identity.
        zoned: ZonedDateTimeHasNamedZone,
        /// Evidence bundle for the named-zone branch.
        evidence: ZonedTimestampEvidence,
    },
    /// An offset time-zone annotation is present and follows compatibility semantics.
    Offset {
        /// Aggregate proof that the offset annotation is consistent with the timestamp.
        semantics: OffsetTimeZoneAnnotationConsistentWithTimestamp,
    },
}
/// Explicit RFC 9557 preferred-calendar proof branch carried by IXDTF exchanges.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, Default, amenable_derive::Evidence, amenable_derive::Witness,
)]
#[evidence(basis = "Self")]
pub enum IxdtfCalendarAnnotationProofBranch {
    /// No preferred-presentation calendar annotation is present.
    #[default]
    None,
    /// A preferred-presentation calendar annotation is present.
    Present {
        /// Aggregate proof that the timestamp declares a preferred presentation calendar.
        preferred_calendar: IxdtfTimestampHasPreferredPresentationCalendar,
    },
}
/// Explicit RFC 9557 additional-information proof branch carried by IXDTF exchanges.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, Default, amenable_derive::Evidence, amenable_derive::Witness,
)]
#[evidence(basis = "Self")]
pub enum IxdtfAdditionalInformationProofBranch {
    /// No additional-information annotations are present.
    #[default]
    None,
    /// Additional-information annotations are present and semantically validated.
    Present {
        /// Aggregate proof that the additional-information semantics are valid.
        semantics: IxdtfAdditionalInformationSemanticsValid,
    },
}
