//! Backend-conversion/mutual-agreement/qualification evidence branches.
//!
//!
//! Shared `*Evidence` branch decompositions reused across several
//! aggregates, plus the per-form bundles behind the two
//! multi-credential aggregates. Same `#[derive(Evidence, Witness)]`
//! shape as the `proof_composition::composites` family.

use crate::{
    CalendarYearThrough1582RequiresMutualAgreement,
    ComponentQualificationAppliesOnlyToMarkedComponent,
    ComponentQualificationUsesImmediateLeftPlacement,
    ExpandedRepresentationRequiresAdditionalAgreement,
    GroupQualificationAppliesToMarkedAndMoreSignificantComponents,
    GroupQualificationUsesImmediateRightPlacement,
    ProlepticGregorianDatesBefore1583RequireMutualAgreement,
};

/// Evidence branch for a standards-governed mutual-agreement authority scope.
///
/// Normative source: ISO 8601-1:2019, 5.2.2.
/// Open-text cross-checks: ISO/WD 8601-1:2016(E), 3.2.1, 4.1.2.1, and 4.1.2.4.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, amenable_derive::Evidence, amenable_derive::Witness,
)]
#[evidence(basis = "Self")]
pub enum MutualAgreementAuthorityScopeEvidence {
    /// Mutual agreement explicitly covers non-expanded calendar years through 1582.
    CalendarYearThrough1582 {
        /// The governed calendar-year restriction is established.
        clause: CalendarYearThrough1582RequiresMutualAgreement,
    },
    /// Mutual agreement explicitly covers proleptic Gregorian dates before 1583.
    ProlepticGregorianBefore1583 {
        /// The governed proleptic-date restriction is established.
        clause: ProlepticGregorianDatesBefore1583RequireMutualAgreement,
    },
    /// Mutual agreement explicitly covers expanded representations.
    ExpandedRepresentation {
        /// The governed expanded-representation restriction is established.
        clause: ExpandedRepresentationRequiresAdditionalAgreement,
    },
}

impl core::default::Default for MutualAgreementAuthorityScopeEvidence {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::CalendarYearThrough1582 {
            clause: core::default::Default::default(),
        }
    }
}
/// Evidence bundle for a qualified temporal expression.
///
/// Normative source: ISO 8601-2:2019, 8.2.1, 8.2.2, 8.2.3, 8.4.4, 8.4.5,
/// 8.4.6, and 8.5.
/// Informative cross-check: public LOC EDTF Level 1 — Qualification of a date (complete);
/// Level 2 — Qualification
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, amenable_derive::Evidence, amenable_derive::Witness,
)]
#[evidence(basis = "Self")]
pub enum QualificationPlacementEvidence {
    /// The qualification marker appears immediately to the right and propagates leftward.
    GroupRight {
        /// The qualification marker is placed immediately to the right of the marked component.
        placement: GroupQualificationUsesImmediateRightPlacement,
        /// The qualification applies to the marked component and all more significant components.
        propagation: GroupQualificationAppliesToMarkedAndMoreSignificantComponents,
    },
    /// The qualification marker appears immediately to the left and applies only to that component.
    ComponentLeft {
        /// The qualification marker is placed immediately to the left of the marked component.
        placement: ComponentQualificationUsesImmediateLeftPlacement,
        /// The qualification applies only to the marked component.
        propagation: ComponentQualificationAppliesOnlyToMarkedComponent,
    },
}

impl core::default::Default for QualificationPlacementEvidence {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::GroupRight {
            placement: core::default::Default::default(),
            propagation: core::default::Default::default(),
        }
    }
}
