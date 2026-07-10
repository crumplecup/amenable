//! Root evidentiary trait for constitutional propositions.

use crate::{CreusotVerifier, KaniVerifier, VerusVerifier, Witness, WitnessSource};

/// A proposition with explicit audit lineage.
///
/// Root propositions may implement this directly. Derived propositions should
/// use it to disclose the upstream roots and code surface that justify them.
pub trait Evidence {
    /// Concrete witness carrier referenced for downstream proof reporting.
    type WitnessType: WitnessSource<KaniVerifier>
        + WitnessSource<CreusotVerifier>
        + WitnessSource<VerusVerifier>;

    /// Concise description of the evidence lineage behind this claim.
    fn lineage_summary() -> &'static str;

    /// Code-level audit surface responsible for upholding the claim.
    fn audit_surface() -> &'static [&'static str];

    /// Human-readable identifier for the witness carrier.
    fn witness_type_name() -> &'static str {
        std::any::type_name::<Self::WitnessType>()
    }
}

impl<E> Witness<KaniVerifier> for E
where
    E: Evidence,
{
    type SupportingEvidence = E;
    type ProofArtifact = <E::WitnessType as WitnessSource<KaniVerifier>>::ProofArtifact;

    fn proof() -> Self::ProofArtifact {
        <E::WitnessType as WitnessSource<KaniVerifier>>::proof()
    }
}

impl<E> Witness<CreusotVerifier> for E
where
    E: Evidence,
{
    type SupportingEvidence = E;
    type ProofArtifact = <E::WitnessType as WitnessSource<CreusotVerifier>>::ProofArtifact;

    fn proof() -> Self::ProofArtifact {
        <E::WitnessType as WitnessSource<CreusotVerifier>>::proof()
    }
}

impl<E> Witness<VerusVerifier> for E
where
    E: Evidence,
{
    type SupportingEvidence = E;
    type ProofArtifact = <E::WitnessType as WitnessSource<VerusVerifier>>::ProofArtifact;

    fn proof() -> Self::ProofArtifact {
        <E::WitnessType as WitnessSource<VerusVerifier>>::proof()
    }
}
