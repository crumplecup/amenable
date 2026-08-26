//! `KaniWitness` impls for `core::array`.

use std::array::TryFromSliceError;

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
#[cfg(kani)]
use crate::FallibleOperationReportsFailure;
#[cfg(kani)]
use crate::IteratorYieldsNoneWhenExhausted;
use crate::KaniWitness;
use crate::rust_std::macros::{bridge_kani_witness, kani_ensures};

impl KaniWitness for RustStdStandard<TryFromSliceError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_try_from_slice_rejects_a_length_mismatch".to_owned(),
            VERIFY_TRY_FROM_SLICE_REJECTS_A_LENGTH_MISMATCH_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<TryFromSliceError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<TryFromSliceError>",
        "kani",
        || <RustStdStandard<TryFromSliceError> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_TRY_FROM_SLICE_REJECTS_A_LENGTH_MISMATCH_SRC, {
        /// `<[T; N]>::try_from` succeeds only when the slice's length
        /// matches `N` exactly, and round-trips the elements when it
        /// does.
        #[kani::proof]
        fn verify_try_from_slice_rejects_a_length_mismatch() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            let matching: &[i32] = &[a, b];
            let arr: Result<[i32; 2], TryFromSliceError> = matching.try_into();
            assert!(
                matches!(arr, Ok([first, second]) if first == a && second == b),
                "a matching-length slice round-trips into the array"
            );

            let mismatched: &[i32] = &[a, b, a];
            let bad: Result<[i32; 2], TryFromSliceError> = mismatched.try_into();
            assert!(
                FallibleOperationReportsFailure::ensures(bad.is_err()),
                "a mismatched-length slice is rejected"
            );

            let too_short: &[i32] = &[a];
            let short: Result<[i32; 2], TryFromSliceError> = too_short.try_into();
            assert!(
                FallibleOperationReportsFailure::ensures(short.is_err()),
                "a shorter slice is rejected"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<std::array::IntoIter<i32, 3>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_array_into_iter_yields_elements_in_order".to_owned(),
            VERIFY_ARRAY_INTO_ITER_YIELDS_ELEMENTS_IN_ORDER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::array::IntoIter<i32, 3>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<IntoIter<i32, 3>>",
        "kani",
        || <RustStdStandard<std::array::IntoIter<i32, 3>> as KaniWitness>::proof()
            .to_string(),
    )
}

kani_ensures!(
    RustStdStandard<std::array::IntoIter<i32, 3>>,
    "amenable_std::rust_std::RustStdStandard<std::array::IntoIter<i32, 3>>",
    (Option<i32>, Option<i32>),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_ARRAY_INTO_ITER_YIELDS_ELEMENTS_IN_ORDER_SRC, {
        /// `[T; N]::into_iter()` yields the array's elements by value, in
        /// order.
        #[kani::proof]
        fn verify_array_into_iter_yields_elements_in_order() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            let c: i32 = kani::any();
            let mut it = [a, b, c].into_iter();
            assert!(RustStdStandard::<std::array::IntoIter<i32, 3>>::ensures((it.next(), Some(a))));
            assert!(RustStdStandard::<std::array::IntoIter<i32, 3>>::ensures((it.next(), Some(b))));
            assert!(RustStdStandard::<std::array::IntoIter<i32, 3>>::ensures((it.next(), Some(c))));
            assert!(
                IteratorYieldsNoneWhenExhausted::ensures(it.next()),
                "into_iter yields exactly N elements"
            );
        }
    }
}
