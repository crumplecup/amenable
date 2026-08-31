use super::CheckedProof;

use std::mem::ManuallyDrop;

use crate::{
    CreusotVerifier, CreusotWitness, MANUALLY_DROP_DEREFS_AND_INTO_INNER_ROUND_TRIP_HOLDS_SRC,
    VERIFY_MANUALLY_DROP_DEREFS_AND_INTO_INNER_ROUND_TRIP_SRC,
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
// Bare `ManuallyDrop<i32>`, matching `amenable_std::rust_std::mem`'s own
// registration exactly (confirmed against the checklist's own
// `evidence_name` column: `RustStdStandard<ManuallyDrop<i32>>`).
impl CreusotWitness for RustStdStandard<ManuallyDrop<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_manually_drop_derefs_and_into_inner_round_trip".to_string(),
            VERIFY_MANUALLY_DROP_DEREFS_AND_INTO_INNER_ROUND_TRIP_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<ManuallyDrop<i32>>);

/// Returns
/// `amenable_creusot::MANUALLY_DROP_DEREFS_AND_INTO_INNER_ROUND_TRIP_HOLDS_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn
/// manually_drop_derefs_and_into_inner_round_trip_holds` the real site
/// calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<ManuallyDrop<i32>> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        MANUALLY_DROP_DEREFS_AND_INTO_INNER_ROUND_TRIP_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<ManuallyDrop<i32>>",
        "creusot",
        "ensures",
        ||
            <RustStdStandard<ManuallyDrop<i32>> as Ensures<CreusotVerifier>>::ensures(()),
    )
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<ManuallyDrop<i32>>",
        "creusot",
        || {
            <RustStdStandard<ManuallyDrop<i32>> as CreusotWitness>::proof().to_string()
        },
    )
}
