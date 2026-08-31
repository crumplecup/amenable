use super::CheckedProof;

use crate::{
    CreusotVerifier, CreusotWitness, FN_POINTER_CALLS_THE_UNDERLYING_FUNCTION_SRC,
    TUPLE_FIELD_ACCESS_HOLDS_SRC, VERIFY_CONST_POINTER_CAST_PRESERVES_THE_ADDRESS_SRC,
    VERIFY_FN_POINTER_CALLS_THE_UNDERLYING_FUNCTION_SRC,
    VERIFY_MUT_POINTER_CAST_PRESERVES_THE_ADDRESS_SRC, VERIFY_TUPLE_FIELD_ACCESS_SRC,
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
impl CreusotWitness for RustStdStandard<(i32, i32)> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_tuple_field_access".to_string(),
            VERIFY_TUPLE_FIELD_ACCESS_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<(i32, i32)>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<(i32, i32)>",
        "creusot",
        || <RustStdStandard<(i32, i32)> as CreusotWitness>::proof().to_string(),
    )
}

/// Returns `amenable_creusot::TUPLE_FIELD_ACCESS_HOLDS_SRC` directly --
/// the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn tuple_field_access_holds` the real site calls,
/// not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<(i32, i32)> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        TUPLE_FIELD_ACCESS_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<(i32, i32)>",
        "creusot",
        "ensures",
        || <RustStdStandard<(i32, i32)> as Ensures<CreusotVerifier>>::ensures(()),
    )
}

impl CreusotWitness for RustStdStandard<fn(i32) -> i32> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_fn_pointer_calls_the_underlying_function".to_string(),
            VERIFY_FN_POINTER_CALLS_THE_UNDERLYING_FUNCTION_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<fn(i32) -> i32>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<fn(i32) -> i32>",
        "creusot",
        || <RustStdStandard<fn(i32) -> i32> as CreusotWitness>::proof().to_string(),
    )
}

/// Returns `amenable_creusot::FN_POINTER_CALLS_THE_UNDERLYING_FUNCTION_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn fn_pointer_calls_the_underlying_function` the
/// real site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<fn(i32) -> i32> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        FN_POINTER_CALLS_THE_UNDERLYING_FUNCTION_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<fn(i32) -> i32>",
        "creusot",
        "ensures",
        || <RustStdStandard<fn(i32) -> i32> as Ensures<CreusotVerifier>>::ensures(()),
    )
}

impl CreusotWitness for RustStdStandard<*const i32> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_const_pointer_cast_preserves_the_address".to_string(),
            VERIFY_CONST_POINTER_CAST_PRESERVES_THE_ADDRESS_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<*const i32>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<*const i32>",
        "creusot",
        || <RustStdStandard<*const i32> as CreusotWitness>::proof().to_string(),
    )
}

impl CreusotWitness for RustStdStandard<*mut i32> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_mut_pointer_cast_preserves_the_address".to_string(),
            VERIFY_MUT_POINTER_CAST_PRESERVES_THE_ADDRESS_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<*mut i32>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<*mut i32>",
        "creusot",
        || <RustStdStandard<*mut i32> as CreusotWitness>::proof().to_string(),
    )
}
