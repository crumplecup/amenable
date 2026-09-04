//! `KaniWitness` impls and `NonZero::new`/`get` harnesses for the signed
//! `NonZero<T>` widths (`i8`/`i16`/`i32`/`i64`/`i128`/`isize`).
//!
//! Each width is written out literally rather than generated through a
//! wrapping `macro_rules!`: `amenable_derive::harness!` captures a harness's
//! verbatim source via the group's span, and a span produced by a
//! `macro_rules!` expansion resolves back to the *defining* macro's on-disk
//! text -- so a generator macro would capture its own placeholders instead
//! of each width's real harness.

use std::num::NonZero;

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use crate::KaniWitness;
#[cfg(kani)]
use crate::NonZeroGetRoundTrips;
use crate::rust_std::CheckedProof;
use crate::rust_std::bridge_kani_witness;

impl KaniWitness for RustStdStandard<NonZero<i8>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_nonzero_i8".to_owned(),
            VERIFY_NONZERO_I8_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<NonZero<i8>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<NonZero<i8>>",
        "kani",
        || <RustStdStandard<NonZero<i8>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_NONZERO_I8_SRC, {
        /// `NonZero::new` succeeds iff the input is nonzero, and `.get()`
        /// round-trips the wrapped value unchanged. The first assertion
        /// calls `RustStdStandard::<NonZero<T>>::ensures` directly
        /// (defined below, in `impl_nonzero_ensures_kani!`) rather than
        /// restating `value != 0`.
        #[kani::proof]
        fn verify_nonzero_i8() {
            let value: i8 = kani::any();
            match NonZero::<i8>::new(value) {
                Some(nz) => {
                    assert!(
                        <RustStdStandard<NonZero<i8>> as Ensures<crate::KaniVerifier>>::ensures(value),
                        "NonZero::new succeeds only for nonzero values"
                    );
                    assert!(
                        NonZeroGetRoundTrips::ensures((nz.get(), value)),
                        "NonZero round-trips its wrapped value"
                    );
                }
                None => {
                    assert!(
                        !<RustStdStandard<NonZero<i8>> as Ensures<crate::KaniVerifier>>::ensures(value),
                        "NonZero::new fails only for zero"
                    );
                }
            }
        }
    }
}

impl KaniWitness for RustStdStandard<NonZero<i16>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_nonzero_i16".to_owned(),
            VERIFY_NONZERO_I16_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<NonZero<i16>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<NonZero<i16>>",
        "kani",
        || <RustStdStandard<NonZero<i16>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_NONZERO_I16_SRC, {
        /// `NonZero::new` succeeds iff the input is nonzero, and `.get()`
        /// round-trips the wrapped value unchanged. The first assertion
        /// calls `RustStdStandard::<NonZero<T>>::ensures` directly
        /// (defined below, in `impl_nonzero_ensures_kani!`) rather than
        /// restating `value != 0`.
        #[kani::proof]
        fn verify_nonzero_i16() {
            let value: i16 = kani::any();
            match NonZero::<i16>::new(value) {
                Some(nz) => {
                    assert!(
                        <RustStdStandard<NonZero<i16>> as Ensures<crate::KaniVerifier>>::ensures(value),
                        "NonZero::new succeeds only for nonzero values"
                    );
                    assert!(
                        NonZeroGetRoundTrips::ensures((nz.get(), value)),
                        "NonZero round-trips its wrapped value"
                    );
                }
                None => {
                    assert!(
                        !<RustStdStandard<NonZero<i16>> as Ensures<crate::KaniVerifier>>::ensures(value),
                        "NonZero::new fails only for zero"
                    );
                }
            }
        }
    }
}

impl KaniWitness for RustStdStandard<NonZero<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_nonzero_i32".to_owned(),
            VERIFY_NONZERO_I32_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<NonZero<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<NonZero<i32>>",
        "kani",
        || <RustStdStandard<NonZero<i32>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_NONZERO_I32_SRC, {
        /// `NonZero::new` succeeds iff the input is nonzero, and `.get()`
        /// round-trips the wrapped value unchanged. The first assertion
        /// calls `RustStdStandard::<NonZero<T>>::ensures` directly
        /// (defined below, in `impl_nonzero_ensures_kani!`) rather than
        /// restating `value != 0`.
        #[kani::proof]
        fn verify_nonzero_i32() {
            let value: i32 = kani::any();
            match NonZero::<i32>::new(value) {
                Some(nz) => {
                    assert!(
                        <RustStdStandard<NonZero<i32>> as Ensures<crate::KaniVerifier>>::ensures(value),
                        "NonZero::new succeeds only for nonzero values"
                    );
                    assert!(
                        NonZeroGetRoundTrips::ensures((nz.get(), value)),
                        "NonZero round-trips its wrapped value"
                    );
                }
                None => {
                    assert!(
                        !<RustStdStandard<NonZero<i32>> as Ensures<crate::KaniVerifier>>::ensures(value),
                        "NonZero::new fails only for zero"
                    );
                }
            }
        }
    }
}

impl KaniWitness for RustStdStandard<NonZero<i64>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_nonzero_i64".to_owned(),
            VERIFY_NONZERO_I64_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<NonZero<i64>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<NonZero<i64>>",
        "kani",
        || <RustStdStandard<NonZero<i64>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_NONZERO_I64_SRC, {
        /// `NonZero::new` succeeds iff the input is nonzero, and `.get()`
        /// round-trips the wrapped value unchanged. The first assertion
        /// calls `RustStdStandard::<NonZero<T>>::ensures` directly
        /// (defined below, in `impl_nonzero_ensures_kani!`) rather than
        /// restating `value != 0`.
        #[kani::proof]
        fn verify_nonzero_i64() {
            let value: i64 = kani::any();
            match NonZero::<i64>::new(value) {
                Some(nz) => {
                    assert!(
                        <RustStdStandard<NonZero<i64>> as Ensures<crate::KaniVerifier>>::ensures(value),
                        "NonZero::new succeeds only for nonzero values"
                    );
                    assert!(
                        NonZeroGetRoundTrips::ensures((nz.get(), value)),
                        "NonZero round-trips its wrapped value"
                    );
                }
                None => {
                    assert!(
                        !<RustStdStandard<NonZero<i64>> as Ensures<crate::KaniVerifier>>::ensures(value),
                        "NonZero::new fails only for zero"
                    );
                }
            }
        }
    }
}

impl KaniWitness for RustStdStandard<NonZero<i128>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_nonzero_i128".to_owned(),
            VERIFY_NONZERO_I128_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<NonZero<i128>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<NonZero<i128>>",
        "kani",
        || <RustStdStandard<NonZero<i128>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_NONZERO_I128_SRC, {
        /// `NonZero::new` succeeds iff the input is nonzero, and `.get()`
        /// round-trips the wrapped value unchanged. The first assertion
        /// calls `RustStdStandard::<NonZero<T>>::ensures` directly
        /// (defined below, in `impl_nonzero_ensures_kani!`) rather than
        /// restating `value != 0`.
        #[kani::proof]
        fn verify_nonzero_i128() {
            let value: i128 = kani::any();
            match NonZero::<i128>::new(value) {
                Some(nz) => {
                    assert!(
                        <RustStdStandard<NonZero<i128>> as Ensures<crate::KaniVerifier>>::ensures(value),
                        "NonZero::new succeeds only for nonzero values"
                    );
                    assert!(
                        NonZeroGetRoundTrips::ensures((nz.get(), value)),
                        "NonZero round-trips its wrapped value"
                    );
                }
                None => {
                    assert!(
                        !<RustStdStandard<NonZero<i128>> as Ensures<crate::KaniVerifier>>::ensures(value),
                        "NonZero::new fails only for zero"
                    );
                }
            }
        }
    }
}

impl KaniWitness for RustStdStandard<NonZero<isize>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_nonzero_isize".to_owned(),
            VERIFY_NONZERO_ISIZE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<NonZero<isize>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<NonZero<isize>>",
        "kani",
        || <RustStdStandard<NonZero<isize>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_NONZERO_ISIZE_SRC, {
        /// `NonZero::new` succeeds iff the input is nonzero, and `.get()`
        /// round-trips the wrapped value unchanged. The first assertion
        /// calls `RustStdStandard::<NonZero<T>>::ensures` directly
        /// (defined below, in `impl_nonzero_ensures_kani!`) rather than
        /// restating `value != 0`.
        #[kani::proof]
        fn verify_nonzero_isize() {
            let value: isize = kani::any();
            match NonZero::<isize>::new(value) {
                Some(nz) => {
                    assert!(
                        <RustStdStandard<NonZero<isize>> as Ensures<crate::KaniVerifier>>::ensures(value),
                        "NonZero::new succeeds only for nonzero values"
                    );
                    assert!(
                        NonZeroGetRoundTrips::ensures((nz.get(), value)),
                        "NonZero round-trips its wrapped value"
                    );
                }
                None => {
                    assert!(
                        !<RustStdStandard<NonZero<isize>> as Ensures<crate::KaniVerifier>>::ensures(value),
                        "NonZero::new fails only for zero"
                    );
                }
            }
        }
    }
}
