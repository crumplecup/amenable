use super::CheckedProof;

use std::net::Shutdown;

use crate::{CreusotVerifier, CreusotWitness, VERIFY_SHUTDOWN_WRITE_PREVENTS_FURTHER_WRITES_SRC};
use amenable_core::{Evidence, Witness};

use amenable_std::RustStdStandard;

macro_rules! bridge_creusot_witness {
    ($ty:ty) => {
        impl Witness<CreusotVerifier> for $ty {
            type SupportingEvidence = <$ty as CreusotWitness>::SupportingEvidence;
            type ProofArtifact = <$ty as CreusotWitness>::ProofArtifact;

            fn proof() -> Self::ProofArtifact {
                <$ty as CreusotWitness>::proof()
            }
        }
    };
}
impl CreusotWitness for RustStdStandard<Shutdown> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_shutdown_write_prevents_further_writes".to_string(),
            VERIFY_SHUTDOWN_WRITE_PREVENTS_FURTHER_WRITES_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<Shutdown>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Shutdown>",
        "creusot",
        || <RustStdStandard<Shutdown> as CreusotWitness>::proof().to_string(),
    )
}
