//! Root evidentiary trait for constitutional propositions.

use crate::{CreusotVerifier, KaniVerifier, VerusVerifier, Witness, WitnessSource};

/// A proposition with explicit lineage and audit artifacts.
///
/// Evidence is produced downstream of a root `Standard`: it reports the
/// lineage of standards and intermediate claims that justify it, plus the
/// audit artifact describing what was done to uphold those obligations.
pub trait Evidence {
    /// Rich lineage artifact for this claim.
    type Lineage;

    /// Rich audit artifact for this claim.
    type Audit;

    /// Concrete witness carrier referenced for downstream proof reporting.
    type WitnessType: WitnessSource<KaniVerifier>
        + WitnessSource<CreusotVerifier>
        + WitnessSource<VerusVerifier>;

    /// Produce the lineage artifact behind this claim.
    fn lineage() -> Self::Lineage;

    /// Produce the audit artifact responsible for upholding the claim.
    fn audit() -> Self::Audit;

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
