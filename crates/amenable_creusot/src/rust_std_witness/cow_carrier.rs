use super::CheckedProof;

use std::borrow::Cow;

use crate::{
    COW_DESTRUCTURE_RECOVERS_THE_WRAPPED_VALUE_SRC, CreusotVerifier, CreusotWitness,
    VERIFY_COW_DESTRUCTURE_RECOVERS_THE_WRAPPED_VALUE_SRC,
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
// Bare `Cow<'static, i32>`, matching `amenable_std::rust_std::
// alloc_borrow`'s own registration exactly (confirmed against the
// checklist's own `evidence_name` column:
// `RustStdStandard<Cow<'static, i32>>`).
impl CreusotWitness for RustStdStandard<Cow<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_cow_destructure_recovers_the_wrapped_value".to_string(),
            VERIFY_COW_DESTRUCTURE_RECOVERS_THE_WRAPPED_VALUE_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<Cow<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Cow<'static, i32>>",
        "creusot",
        || <RustStdStandard<Cow<'static, i32>> as CreusotWitness>::proof().to_string(),
    )
}

/// Returns
/// `amenable_creusot::COW_DESTRUCTURE_RECOVERS_THE_WRAPPED_VALUE_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn cow_destructure_recovers_the_wrapped_value` the
/// real site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<Cow<'static, i32>> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        COW_DESTRUCTURE_RECOVERS_THE_WRAPPED_VALUE_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<Cow<'static, i32>>",
        "creusot",
        "ensures",
        || <RustStdStandard<Cow<'static, i32>> as Ensures<CreusotVerifier>>::ensures(()),
    )
}
