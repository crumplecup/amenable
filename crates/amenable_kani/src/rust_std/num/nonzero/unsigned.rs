//! `KaniWitness` impls and `NonZero::new`/`get` harnesses for the unsigned
//! `NonZero<T>` widths (`u8`/`u16`/`u32`/`u64`/`u128`/`usize`).
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
use crate::rust_std::macros::bridge_kani_witness;

impl KaniWitness for RustStdStandard<NonZero<u8>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_nonzero_u8".to_owned(),
            VERIFY_NONZERO_U8_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<NonZero<u8>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<NonZero<u8>>",
        "kani",
        || <RustStdStandard<NonZero<u8>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_NONZERO_U8_SRC, {
        /// `NonZero::new` succeeds iff the input is nonzero, and `.get()`
        /// round-trips the wrapped value unchanged. The first assertion
        /// calls `RustStdStandard::<NonZero<T>>::ensures` directly
        /// (defined below, in `impl_nonzero_ensures_kani!`) rather than
        /// restating `value != 0`.
        #[kani::proof]
        fn verify_nonzero_u8() {
            let value: u8 = kani::any();
            match NonZero::<u8>::new(value) {
                Some(nz) => {
                    assert!(
                        <RustStdStandard<NonZero<u8>> as Ensures<crate::KaniVerifier>>::ensures(value),
                        "NonZero::new succeeds only for nonzero values"
                    );
                    assert!(
                        NonZeroGetRoundTrips::ensures((nz.get(), value)),
                        "NonZero round-trips its wrapped value"
                    );
                }
                None => {
                    assert!(
                        !<RustStdStandard<NonZero<u8>> as Ensures<crate::KaniVerifier>>::ensures(value),
                        "NonZero::new fails only for zero"
                    );
                }
            }
        }
    }
}

impl KaniWitness for RustStdStandard<NonZero<u16>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_nonzero_u16".to_owned(),
            VERIFY_NONZERO_U16_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<NonZero<u16>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<NonZero<u16>>",
        "kani",
        || <RustStdStandard<NonZero<u16>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_NONZERO_U16_SRC, {
        /// `NonZero::new` succeeds iff the input is nonzero, and `.get()`
        /// round-trips the wrapped value unchanged. The first assertion
        /// calls `RustStdStandard::<NonZero<T>>::ensures` directly
        /// (defined below, in `impl_nonzero_ensures_kani!`) rather than
        /// restating `value != 0`.
        #[kani::proof]
        fn verify_nonzero_u16() {
            let value: u16 = kani::any();
            match NonZero::<u16>::new(value) {
                Some(nz) => {
                    assert!(
                        <RustStdStandard<NonZero<u16>> as Ensures<crate::KaniVerifier>>::ensures(value),
                        "NonZero::new succeeds only for nonzero values"
                    );
                    assert!(
                        NonZeroGetRoundTrips::ensures((nz.get(), value)),
                        "NonZero round-trips its wrapped value"
                    );
                }
                None => {
                    assert!(
                        !<RustStdStandard<NonZero<u16>> as Ensures<crate::KaniVerifier>>::ensures(value),
                        "NonZero::new fails only for zero"
                    );
                }
            }
        }
    }
}

impl KaniWitness for RustStdStandard<NonZero<u32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_nonzero_u32".to_owned(),
            VERIFY_NONZERO_U32_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<NonZero<u32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<NonZero<u32>>",
        "kani",
        || <RustStdStandard<NonZero<u32>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_NONZERO_U32_SRC, {
        /// `NonZero::new` succeeds iff the input is nonzero, and `.get()`
        /// round-trips the wrapped value unchanged. The first assertion
        /// calls `RustStdStandard::<NonZero<T>>::ensures` directly
        /// (defined below, in `impl_nonzero_ensures_kani!`) rather than
        /// restating `value != 0`.
        #[kani::proof]
        fn verify_nonzero_u32() {
            let value: u32 = kani::any();
            match NonZero::<u32>::new(value) {
                Some(nz) => {
                    assert!(
                        <RustStdStandard<NonZero<u32>> as Ensures<crate::KaniVerifier>>::ensures(value),
                        "NonZero::new succeeds only for nonzero values"
                    );
                    assert!(
                        NonZeroGetRoundTrips::ensures((nz.get(), value)),
                        "NonZero round-trips its wrapped value"
                    );
                }
                None => {
                    assert!(
                        !<RustStdStandard<NonZero<u32>> as Ensures<crate::KaniVerifier>>::ensures(value),
                        "NonZero::new fails only for zero"
                    );
                }
            }
        }
    }
}

impl KaniWitness for RustStdStandard<NonZero<u64>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_nonzero_u64".to_owned(),
            VERIFY_NONZERO_U64_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<NonZero<u64>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<NonZero<u64>>",
        "kani",
        || <RustStdStandard<NonZero<u64>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_NONZERO_U64_SRC, {
        /// `NonZero::new` succeeds iff the input is nonzero, and `.get()`
        /// round-trips the wrapped value unchanged. The first assertion
        /// calls `RustStdStandard::<NonZero<T>>::ensures` directly
        /// (defined below, in `impl_nonzero_ensures_kani!`) rather than
        /// restating `value != 0`.
        #[kani::proof]
        fn verify_nonzero_u64() {
            let value: u64 = kani::any();
            match NonZero::<u64>::new(value) {
                Some(nz) => {
                    assert!(
                        <RustStdStandard<NonZero<u64>> as Ensures<crate::KaniVerifier>>::ensures(value),
                        "NonZero::new succeeds only for nonzero values"
                    );
                    assert!(
                        NonZeroGetRoundTrips::ensures((nz.get(), value)),
                        "NonZero round-trips its wrapped value"
                    );
                }
                None => {
                    assert!(
                        !<RustStdStandard<NonZero<u64>> as Ensures<crate::KaniVerifier>>::ensures(value),
                        "NonZero::new fails only for zero"
                    );
                }
            }
        }
    }
}

impl KaniWitness for RustStdStandard<NonZero<u128>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_nonzero_u128".to_owned(),
            VERIFY_NONZERO_U128_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<NonZero<u128>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<NonZero<u128>>",
        "kani",
        || <RustStdStandard<NonZero<u128>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_NONZERO_U128_SRC, {
        /// `NonZero::new` succeeds iff the input is nonzero, and `.get()`
        /// round-trips the wrapped value unchanged. The first assertion
        /// calls `RustStdStandard::<NonZero<T>>::ensures` directly
        /// (defined below, in `impl_nonzero_ensures_kani!`) rather than
        /// restating `value != 0`.
        #[kani::proof]
        fn verify_nonzero_u128() {
            let value: u128 = kani::any();
            match NonZero::<u128>::new(value) {
                Some(nz) => {
                    assert!(
                        <RustStdStandard<NonZero<u128>> as Ensures<crate::KaniVerifier>>::ensures(value),
                        "NonZero::new succeeds only for nonzero values"
                    );
                    assert!(
                        NonZeroGetRoundTrips::ensures((nz.get(), value)),
                        "NonZero round-trips its wrapped value"
                    );
                }
                None => {
                    assert!(
                        !<RustStdStandard<NonZero<u128>> as Ensures<crate::KaniVerifier>>::ensures(value),
                        "NonZero::new fails only for zero"
                    );
                }
            }
        }
    }
}

impl KaniWitness for RustStdStandard<NonZero<usize>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_nonzero_usize".to_owned(),
            VERIFY_NONZERO_USIZE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<NonZero<usize>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<NonZero<usize>>",
        "kani",
        || <RustStdStandard<NonZero<usize>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_NONZERO_USIZE_SRC, {
        /// `NonZero::new` succeeds iff the input is nonzero, and `.get()`
        /// round-trips the wrapped value unchanged. The first assertion
        /// calls `RustStdStandard::<NonZero<T>>::ensures` directly
        /// (defined below, in `impl_nonzero_ensures_kani!`) rather than
        /// restating `value != 0`.
        #[kani::proof]
        fn verify_nonzero_usize() {
            let value: usize = kani::any();
            match NonZero::<usize>::new(value) {
                Some(nz) => {
                    assert!(
                        <RustStdStandard<NonZero<usize>> as Ensures<crate::KaniVerifier>>::ensures(value),
                        "NonZero::new succeeds only for nonzero values"
                    );
                    assert!(
                        NonZeroGetRoundTrips::ensures((nz.get(), value)),
                        "NonZero round-trips its wrapped value"
                    );
                }
                None => {
                    assert!(
                        !<RustStdStandard<NonZero<usize>> as Ensures<crate::KaniVerifier>>::ensures(value),
                        "NonZero::new fails only for zero"
                    );
                }
            }
        }
    }
}
