use std::num::NonZero;

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use crate::KaniWitness;
use crate::rust_std::CheckedProof;
use crate::rust_std::macros::bridge_kani_witness;

impl KaniWitness for RustStdStandard<NonZero<i8>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

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

/// The [`RustStdStandard<NonZero<T>>`] witnesses above all reuse their own
/// harness for [`amenable_core::Ensures<crate::KaniVerifier>`] — each
/// harness calls `RustStdStandard::<NonZero<T>>::ensures(value)` directly
/// rather than restating `value != 0`, identical across every width for
/// the same reason the witnesses themselves are literal per-width blocks
/// (see this module's own doc comment): a generator macro's captured span
/// would resolve back to unsubstituted placeholders, not the real
/// per-width text. Nothing here needs `harness!`'s span capture, though,
/// so this half is a plain macro.
macro_rules! impl_nonzero_ensures_kani {
    ($($ty:ty),* $(,)?) => {
        $(
            impl amenable_core::Ensures<crate::KaniVerifier> for RustStdStandard<NonZero<$ty>> {
                type Input = $ty;
                type Bound = bool;

                fn ensures(value: $ty) -> bool {
                    value != 0
                }
            }

            ::inventory::submit! {
                ::amenable_core::ContractRecord::new(
                    concat!(
                        "amenable_std::rust_std::RustStdStandard<NonZero<",
                        stringify!($ty),
                        ">>"
                    ),
                    "kani",
                    "ensures",
                    || "value != 0",
                )
            }
        )*
    };
}

impl_nonzero_ensures_kani!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);

/// `NonZero<T>::get()` round-trips its wrapped value — a distinct claim
/// from `impl_nonzero_ensures_kani`'s `RustStdStandard<NonZero<T>>`
/// impls above (those check `NonZero::new`'s *construction*
/// precondition, `value != 0`; this checks the *accessor*
/// postcondition), so it can't reuse that carrier's slot:
/// `RustStdStandard<NonZero<T>>` already has its one
/// `Ensures<KaniVerifier>` impl claimed by the precondition.
///
/// Generic over the wrapped width rather than twelve separate concrete
/// types (`NonZeroI8GetRoundTrips`, `NonZeroI16GetRoundTrips`, ...): every
/// one of those independently registered the identical fragment
/// `actual == expected`, the same trivial-equality claim this session's
/// other access-pattern types (`DerefReflectsTheStoredValue`,
/// `IndexRecoversTheStoredElement`, ...) already generalize over `T`.
/// Lives in `amenable_kani` rather than `amenable_std` — no Creusot/Verus
/// coverage of `NonZero::get()` exists yet, and every other Kani-only
/// generic contract type this session landed in `amenable_kani` for the
/// same reason.
pub struct NonZeroGetRoundTrips<T>(std::marker::PhantomData<T>);

impl<T> amenable_core::Standard for NonZeroGetRoundTrips<T> {
    type Provenance = amenable_std::RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn provenance(&self) -> Self::Provenance {
        <i32 as amenable_std::RustStdType>::provenance()
    }
}

impl<T> Evidence for NonZeroGetRoundTrips<T> {
    type Basis = RustStdStandard<i32>;
    type Audit = amenable_std::RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn basis() -> Self::Basis {
        RustStdStandard::<i32>::new()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn audit(&self) -> Self::Audit {
        <i32 as amenable_std::RustStdType>::provenance()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", ret))]
    fn is_root() -> bool {
        false
    }
}

impl<T> KaniWitness for NonZeroGetRoundTrips<T> {
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

impl<T> amenable_core::Witness<crate::KaniVerifier> for NonZeroGetRoundTrips<T> {
    type SupportingEvidence = <Self as KaniWitness>::SupportingEvidence;
    type ProofArtifact = <Self as KaniWitness>::ProofArtifact;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        <Self as KaniWitness>::proof()
    }
}

impl<T: PartialEq> amenable_core::Ensures<crate::KaniVerifier> for NonZeroGetRoundTrips<T> {
    type Input = (T, T);
    type Bound = bool;

    fn ensures((actual, expected): (T, T)) -> bool {
        actual == expected
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_kani::NonZeroGetRoundTrips",
        "kani",
        "ensures",
        || stringify!(actual == expected),
    )
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::NonZeroGetRoundTrips",
        "kani",
        || <NonZeroGetRoundTrips<i8> as KaniWitness>::proof().to_string(),
    )
}
