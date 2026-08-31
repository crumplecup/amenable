use super::CheckedProof;

use std::io::SeekFrom;

use crate::{
    CreusotVerifier, CreusotWitness, SEEK_FROM_ROUND_TRIPS_EACH_VARIANTS_OFFSET_SRC,
    VERIFY_SEEK_FROM_ROUND_TRIPS_EACH_VARIANTS_OFFSET_SRC,
};
use amenable_core::{Ensures, Evidence, Witness};

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
impl CreusotWitness for RustStdStandard<SeekFrom> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_seek_from_round_trips_each_variants_offset".to_string(),
            VERIFY_SEEK_FROM_ROUND_TRIPS_EACH_VARIANTS_OFFSET_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<SeekFrom>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<SeekFrom>",
        "creusot",
        || <RustStdStandard<SeekFrom> as CreusotWitness>::proof().to_string(),
    )
}

/// Returns `amenable_creusot::SEEK_FROM_ROUND_TRIPS_EACH_VARIANTS_OFFSET_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn seek_from_round_trips_each_variants_offset` the
/// real site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<SeekFrom> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        SEEK_FROM_ROUND_TRIPS_EACH_VARIANTS_OFFSET_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<SeekFrom>",
        "creusot",
        "ensures",
        || <RustStdStandard<SeekFrom> as Ensures<CreusotVerifier>>::ensures(()),
    )
}
