#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use crate::CheckedProof;
#[cfg(kani)]
use crate::FieldAccessRecoversTheStoredValue;
use crate::KaniWitness;
use crate::rust_std::bridge_kani_witness;

impl KaniWitness for RustStdStandard<(i32, i32)> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_tuple_field_access".to_owned(),
            VERIFY_TUPLE_FIELD_ACCESS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<(i32, i32)>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<(i32, i32)>",
        "kani",
        || <RustStdStandard<(i32, i32)> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_TUPLE_FIELD_ACCESS_SRC, {
        /// A tuple's `.0`/`.1` recover exactly the values it was
        /// constructed with, in position order.
        #[kani::proof]
        fn verify_tuple_field_access() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            let t = (a, b);
            assert!(FieldAccessRecoversTheStoredValue::ensures((t.0, a)));
            assert!(FieldAccessRecoversTheStoredValue::ensures((t.1, b)));
        }
    }
}

impl KaniWitness for RustStdStandard<fn(i32) -> i32> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_fn_pointer_calls_the_underlying_function".to_owned(),
            VERIFY_FN_POINTER_CALLS_THE_UNDERLYING_FUNCTION_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<fn(i32) -> i32>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<fn(i32) -> i32>",
        "kani",
        || <RustStdStandard<fn(i32) -> i32> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_FN_POINTER_CALLS_THE_UNDERLYING_FUNCTION_SRC, {
        /// Calling through a `fn` pointer invokes exactly the function it
        /// was assigned from.
        #[kani::proof]
        fn verify_fn_pointer_calls_the_underlying_function() {
            fn increment(x: i32) -> i32 {
                x.wrapping_add(1)
            }
            let f: fn(i32) -> i32 = increment;
            let value: i32 = kani::any();
            assert!(
                RustStdStandard::<i32>::ensures((f(value), increment(value))),
                "the fn pointer calls the function it was assigned from"
            );
        }
    }
}
