use super::CheckedProof;

use std::alloc::System;

use crate::{
    CreusotVerifier, CreusotWitness, SYSTEM_ALLOCATION_ROUND_TRIPS_SRC,
    VERIFY_SYSTEM_ALLOCATES_AND_DEALLOCATES_A_LAYOUT_SRC,
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
impl CreusotWitness for RustStdStandard<System> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_system_allocates_and_deallocates_a_layout".to_string(),
            VERIFY_SYSTEM_ALLOCATES_AND_DEALLOCATES_A_LAYOUT_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<System>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<System>",
        "creusot",
        || <RustStdStandard<System> as CreusotWitness>::proof().to_string(),
    )
}

/// Returns `amenable_creusot::SYSTEM_ALLOCATION_ROUND_TRIPS_SRC` directly
/// -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn system_allocation_round_trips` the real site calls,
/// not a hand-retyped copy of its expression. The same claim Kani's own
/// `verify_system_allocates_and_deallocates_a_layout` harness checks via
/// `assert_eq!` (out of the contract-bound scanner's reach, since
/// `assert_eq!`'s comparands aren't parsed as a clause).
impl Ensures<CreusotVerifier> for RustStdStandard<System> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        SYSTEM_ALLOCATION_ROUND_TRIPS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<System>",
        "creusot",
        "ensures",
        || <RustStdStandard<System> as Ensures<CreusotVerifier>>::ensures(()),
    )
}
