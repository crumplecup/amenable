//! The fixed scoring and recommendation vocabulary a reviewer chooses from.

use crate::{AmenableError, AmenableResult};
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use tracing::instrument;

/// A reviewer's recommended next action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "snake_case")]
pub(super) enum Recommendation {
    /// The proof is persuasive evidence in its current form.
    Accept,
    /// The proof should be strengthened while retaining its current direction.
    Strengthen,
    /// The proof should be replaced with a substantively different argument.
    Replace,
    /// The proof should no longer be relied upon.
    Retire,
}

impl Recommendation {
    #[instrument(level = "debug", skip(self))]
    pub(super) fn as_str(self) -> &'static str {
        match self {
            Self::Accept => "accept",
            Self::Strengthen => "strengthen",
            Self::Replace => "replace",
            Self::Retire => "retire",
        }
    }
}

/// The operational path for acting on an assessment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "snake_case")]
pub(super) enum ResolutionPath {
    /// Keep the proof as-is and continue relying on it.
    #[value(name = "keep_current_proof")]
    KeepCurrentProof,
    /// Extend the current proof without changing its fundamental approach.
    #[value(name = "strengthen_current_proof")]
    StrengthenCurrentProof,
    /// Replace the proof with a narrower proof-specific model.
    #[value(name = "replace_with_proof_specific_model")]
    ReplaceWithProofSpecificModel,
    /// Replace the proof with an accommodation model backed by the standards.
    #[value(name = "replace_with_accommodation_model")]
    ReplaceWithAccommodationModel,
    /// Stop relying on the current claim.
    #[value(name = "retire_claim")]
    RetireClaim,
}

impl ResolutionPath {
    #[instrument(level = "debug", skip(self))]
    pub(super) fn as_str(self) -> &'static str {
        match self {
            Self::KeepCurrentProof => "keep_current_proof",
            Self::StrengthenCurrentProof => "strengthen_current_proof",
            Self::ReplaceWithProofSpecificModel => "replace_with_proof_specific_model",
            Self::ReplaceWithAccommodationModel => "replace_with_accommodation_model",
            Self::RetireClaim => "retire_claim",
        }
    }
}

/// The dimension used when grouping assessment counts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, ValueEnum)]
#[serde(rename_all = "snake_case")]
pub(super) enum SummaryDimension {
    #[value(name = "recommendation")]
    Recommendation,
    #[value(name = "resolution_path")]
    ResolutionPath,
}

impl SummaryDimension {
    #[instrument(level = "debug", skip(self))]
    pub(super) fn as_str(self) -> &'static str {
        match self {
            Self::Recommendation => "recommendation",
            Self::ResolutionPath => "resolution_path",
        }
    }
}

/// Six independently scored dimensions of proof quality.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(super) struct Rubric {
    claim_alignment: u8,
    assumption_adequacy: u8,
    model_fidelity: u8,
    assertion_strength: u8,
    adversarial_coverage: u8,
    clarity: u8,
}

impl Rubric {
    /// Assemble the six rubric scores, exactly as recorded by
    /// `--claim-alignment` and its five sibling flags.
    #[instrument(level = "debug")]
    pub(super) fn new(
        claim_alignment: u8,
        assumption_adequacy: u8,
        model_fidelity: u8,
        assertion_strength: u8,
        adversarial_coverage: u8,
        clarity: u8,
    ) -> Self {
        Self {
            claim_alignment,
            assumption_adequacy,
            model_fidelity,
            assertion_strength,
            adversarial_coverage,
            clarity,
        }
    }

    #[instrument(level = "debug", skip(self))]
    pub(super) fn validate(self) -> AmenableResult<()> {
        for (name, score) in [
            ("claim_alignment", self.claim_alignment),
            ("assumption_adequacy", self.assumption_adequacy),
            ("model_fidelity", self.model_fidelity),
            ("assertion_strength", self.assertion_strength),
            ("adversarial_coverage", self.adversarial_coverage),
            ("clarity", self.clarity),
        ] {
            if score > 4 {
                return Err(AmenableError::invariant(format!(
                    "invalid {name} score {score}; expected an integer from 0 to 4"
                )));
            }
        }

        Ok(())
    }

    #[instrument(level = "debug", skip(self))]
    pub(super) fn values(self) -> [(String, u8); 6] {
        [
            ("claim alignment".to_owned(), self.claim_alignment),
            ("assumption adequacy".to_owned(), self.assumption_adequacy),
            ("model fidelity".to_owned(), self.model_fidelity),
            ("assertion strength".to_owned(), self.assertion_strength),
            ("adversarial coverage".to_owned(), self.adversarial_coverage),
            ("clarity".to_owned(), self.clarity),
        ]
    }
}
