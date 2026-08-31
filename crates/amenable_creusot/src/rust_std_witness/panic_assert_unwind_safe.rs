use super::CheckedProof;

use std::panic::AssertUnwindSafe;

use crate::{
    ASSERT_UNWIND_SAFE_DEREFS_TRANSPARENTLY_SRC, CreusotVerifier, CreusotWitness,
    VERIFY_ASSERT_UNWIND_SAFE_DEREFS_TRANSPARENTLY_SRC,
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
impl CreusotWitness for RustStdStandard<AssertUnwindSafe<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_assert_unwind_safe_derefs_transparently".to_string(),
            VERIFY_ASSERT_UNWIND_SAFE_DEREFS_TRANSPARENTLY_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<AssertUnwindSafe<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<AssertUnwindSafe<i32>>",
        "creusot",
        || <RustStdStandard<AssertUnwindSafe<i32>> as CreusotWitness>::proof().to_string(),
    )
}

/// Returns `amenable_creusot::ASSERT_UNWIND_SAFE_DEREFS_TRANSPARENTLY_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn assert_unwind_safe_derefs_transparently` the
/// real site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<AssertUnwindSafe<i32>> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        ASSERT_UNWIND_SAFE_DEREFS_TRANSPARENTLY_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<AssertUnwindSafe<i32>>",
        "creusot",
        "ensures",
        || <RustStdStandard<AssertUnwindSafe<i32>> as Ensures<CreusotVerifier>>::ensures(()),
    )
}
