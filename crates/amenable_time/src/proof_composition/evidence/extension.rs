//! Sub-year-grouping-kind evidence branch.
//!
//!
//! Shared `*Evidence` branch decompositions reused across several
//! aggregates, plus the per-form bundles behind the two
//! multi-credential aggregates. Same `#[derive(Evidence, Witness)]`
//! shape as the `proof_composition::composites` family.

use crate::{
    SeasonCodeDeclaresNamedSeason, SeasonCodeDeclaresSeasonScope,
    SubYearGroupingCodeDeclaresQuadrimester, SubYearGroupingCodeDeclaresQuarter,
    SubYearGroupingCodeDeclaresSemestral,
};

/// Branch evidence for the specific Level 2 sub-year grouping family in use.
///
/// Normative source: ISO 8601-2:2019, 4.8.1, 4.8.2, and 4.8.3.
/// Informative cross-check: public LOC EDTF Level 2 — Sub-year groupings
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, amenable_derive::Evidence, amenable_derive::Witness,
)]
#[evidence(basis = "Self")]
pub enum SubYearGroupingKindEvidence {
    /// Seasonal grouping semantics, including hemisphere scope when declared.
    Season {
        /// The code declares a named season.
        named_season: SeasonCodeDeclaresNamedSeason,
        /// The code declares its season scope.
        season_scope: SeasonCodeDeclaresSeasonScope,
    },
    /// Quarter grouping semantics.
    Quarter {
        /// The code declares a quarter grouping.
        quarter: SubYearGroupingCodeDeclaresQuarter,
    },
    /// Quadrimester grouping semantics.
    Quadrimester {
        /// The code declares a quadrimester grouping.
        quadrimester: SubYearGroupingCodeDeclaresQuadrimester,
    },
    /// Semestral grouping semantics.
    Semestral {
        /// The code declares a semestral grouping.
        semestral: SubYearGroupingCodeDeclaresSemestral,
    },
}

impl core::default::Default for SubYearGroupingKindEvidence {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::Season {
            named_season: core::default::Default::default(),
            season_scope: core::default::Default::default(),
        }
    }
}
