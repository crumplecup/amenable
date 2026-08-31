use std::iter::Flatten;
use std::ops::Range;

use crate::{CreusotVerifier, CreusotWitness};
use amenable_core::{Evidence, Provenance, Witness};

use amenable_std::{RustStdProvenance, RustStdStandard};

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
impl CreusotWitness for RustStdStandard<Flatten<std::vec::IntoIter<Range<i32>>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = RustStdProvenance;

    fn proof() -> Self::ProofArtifact {
        <Self::SupportingEvidence as Evidence>::basis().audit()
    }
}

bridge_creusot_witness!(RustStdStandard<Flatten<std::vec::IntoIter<Range<i32>>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Flatten<IntoIter<Range<i32>>>>",
        "creusot",
        || {
            <RustStdStandard<Flatten<std::vec::IntoIter<Range<i32>>>> as CreusotWitness>::proof()
                .report()
                .to_string()
        },
    )
}
