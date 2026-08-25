//! `KaniWitness` impls for `core::num`.
//!
//! Each `NonZero<T>` instantiation is written out literally rather than
//! generated through a wrapping `macro_rules!`: `amenable_derive::harness!`
//! captures a harness's verbatim source via the group's span, and a span
//! produced by a `macro_rules!` expansion resolves back to the *defining*
//! macro's on-disk text — so a generator macro would capture its own
//! `$ty`/`$harness_fn` placeholders, unsubstituted, instead of each type's
//! real harness. Twelve literal blocks is the price of an honest `claim`.

use std::num::{
    FpCategory, IntErrorKind, NonZero, ParseFloatError, ParseIntError, Saturating, TryFromIntError,
    Wrapping,
};

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
#[cfg(kani)]
use crate::FallibleOperationReportsFailure;
use crate::KaniWitness;
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
                ::amenable_core::ContractRecord {
                    evidence: concat!(
                        "amenable_std::rust_std::RustStdStandard<NonZero<",
                        stringify!($ty),
                        ">>"
                    ),
                    verifier: "kani",
                    kind: "ensures",
                    fragment: || "value != 0",
                }
            }
        )*
    };
}

impl_nonzero_ensures_kani!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);

/// `NonZero<T>::get()` round-trips its wrapped value — a distinct claim
/// from [`impl_nonzero_ensures_kani`]'s `RustStdStandard<NonZero<T>>`
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

    fn provenance(&self) -> Self::Provenance {
        <i32 as amenable_std::RustStdType>::provenance()
    }
}

impl<T> Evidence for NonZeroGetRoundTrips<T> {
    type Basis = RustStdStandard<i32>;
    type Audit = amenable_std::RustStdProvenance;

    fn basis() -> Self::Basis {
        RustStdStandard::<i32>::new()
    }

    fn audit(&self) -> Self::Audit {
        <i32 as amenable_std::RustStdType>::provenance()
    }

    fn is_root() -> bool {
        false
    }
}

impl<T> KaniWitness for NonZeroGetRoundTrips<T> {
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

impl<T> amenable_core::Witness<crate::KaniVerifier> for NonZeroGetRoundTrips<T> {
    type SupportingEvidence = <Self as KaniWitness>::SupportingEvidence;
    type ProofArtifact = <Self as KaniWitness>::ProofArtifact;

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
    ::amenable_core::ContractRecord {
        evidence: "amenable_kani::NonZeroGetRoundTrips",
        verifier: "kani",
        kind: "ensures",
        fragment: || stringify!(actual == expected),
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::NonZeroGetRoundTrips",
        "kani",
        || <NonZeroGetRoundTrips<i8> as KaniWitness>::proof().to_string(),
    )
}

impl KaniWitness for RustStdStandard<Wrapping<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_wrapping_add_matches_the_inner_wrapping_add".to_owned(),
            VERIFY_WRAPPING_ADD_MATCHES_THE_INNER_WRAPPING_ADD_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Wrapping<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Wrapping<i32>>",
        "kani",
        || <RustStdStandard<Wrapping<i32>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_WRAPPING_ADD_MATCHES_THE_INNER_WRAPPING_ADD_SRC, {
        /// `Wrapping<T>`'s `+` operator wraps on overflow exactly like the
        /// inner type's `wrapping_add`.
        #[kani::proof]
        fn verify_wrapping_add_matches_the_inner_wrapping_add() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            let result = Wrapping(a) + Wrapping(b);
            assert_eq!(
                result.0,
                a.wrapping_add(b),
                "Wrapping<T>'s + operator matches the inner type's wrapping_add"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<Saturating<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_saturating_add_matches_the_inner_saturating_add".to_owned(),
            VERIFY_SATURATING_ADD_MATCHES_THE_INNER_SATURATING_ADD_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Saturating<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Saturating<i32>>",
        "kani",
        || <RustStdStandard<Saturating<i32>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_SATURATING_ADD_MATCHES_THE_INNER_SATURATING_ADD_SRC, {
        /// `Saturating<T>`'s `+` operator saturates at the numeric bounds
        /// exactly like the inner type's `saturating_add`.
        #[kani::proof]
        fn verify_saturating_add_matches_the_inner_saturating_add() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            let result = Saturating(a) + Saturating(b);
            assert_eq!(
                result.0,
                a.saturating_add(b),
                "Saturating<T>'s + operator matches the inner type's saturating_add"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<TryFromIntError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_try_from_int_error_occurs_exactly_when_out_of_range".to_owned(),
            VERIFY_TRY_FROM_INT_ERROR_OCCURS_EXACTLY_WHEN_OUT_OF_RANGE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<TryFromIntError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::num::TryFromIntError>",
        "kani",
        || <RustStdStandard<TryFromIntError> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_TRY_FROM_INT_ERROR_OCCURS_EXACTLY_WHEN_OUT_OF_RANGE_SRC, {
        /// `u8::try_from(i32)` fails with `TryFromIntError` exactly when the
        /// source value doesn't fit in `u8`, and succeeds with the same
        /// value otherwise.
        #[kani::proof]
        fn verify_try_from_int_error_occurs_exactly_when_out_of_range() {
            let value: i32 = kani::any();
            let result = u8::try_from(value);
            if (0..=i32::from(u8::MAX)).contains(&value) {
                assert_eq!(
                    result,
                    Ok(value as u8),
                    "try_from succeeds and preserves the value when it fits the target type"
                );
            } else {
                assert!(
                    FallibleOperationReportsFailure::ensures(result.is_err()),
                    "try_from fails with TryFromIntError when the value doesn't fit the target type"
                );
            }
        }
    }
}

impl KaniWitness for RustStdStandard<IntErrorKind> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_int_error_kind_classifies_parse_failures".to_owned(),
            VERIFY_INT_ERROR_KIND_CLASSIFIES_PARSE_FAILURES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<IntErrorKind>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::num::IntErrorKind>",
        "kani",
        || <RustStdStandard<IntErrorKind> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_INT_ERROR_KIND_CLASSIFIES_PARSE_FAILURES_SRC, {
        /// Each representative integer-parse failure mode produces the
        /// matching `IntErrorKind` variant.
        #[kani::proof]
        fn verify_int_error_kind_classifies_parse_failures() {
            assert_eq!(
                "".parse::<i32>().unwrap_err().kind(),
                &IntErrorKind::Empty,
                "an empty string parses with IntErrorKind::Empty"
            );
            assert_eq!(
                "not a number".parse::<i32>().unwrap_err().kind(),
                &IntErrorKind::InvalidDigit,
                "a non-digit string parses with IntErrorKind::InvalidDigit"
            );
            assert_eq!(
                "99999999999999999999".parse::<i32>().unwrap_err().kind(),
                &IntErrorKind::PosOverflow,
                "a value above i32::MAX parses with IntErrorKind::PosOverflow"
            );
            assert_eq!(
                "-99999999999999999999".parse::<i32>().unwrap_err().kind(),
                &IntErrorKind::NegOverflow,
                "a value below i32::MIN parses with IntErrorKind::NegOverflow"
            );
            assert_eq!(
                "0".parse::<NonZero<i32>>().unwrap_err().kind(),
                &IntErrorKind::Zero,
                "zero parses as NonZero<i32> with IntErrorKind::Zero"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<ParseIntError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_parse_int_error_reports_the_kind_of_the_failure".to_owned(),
            VERIFY_PARSE_INT_ERROR_REPORTS_THE_KIND_OF_THE_FAILURE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<ParseIntError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::num::ParseIntError>",
        "kani",
        || <RustStdStandard<ParseIntError> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_PARSE_INT_ERROR_REPORTS_THE_KIND_OF_THE_FAILURE_SRC, {
        /// `ParseIntError::kind()` reports the specific reason the parse
        /// failed, not just that it failed.
        #[kani::proof]
        fn verify_parse_int_error_reports_the_kind_of_the_failure() {
            let err = "not a number".parse::<i32>().expect_err("non-digit input must fail to parse");
            assert_eq!(
                err.kind(),
                &IntErrorKind::InvalidDigit,
                "ParseIntError::kind() reports the specific parse failure reason"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<ParseFloatError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_parse_float_error_occurs_only_for_unparseable_input".to_owned(),
            VERIFY_PARSE_FLOAT_ERROR_OCCURS_ONLY_FOR_UNPARSEABLE_INPUT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<ParseFloatError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::num::ParseFloatError>",
        "kani",
        || <RustStdStandard<ParseFloatError> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_PARSE_FLOAT_ERROR_OCCURS_ONLY_FOR_UNPARSEABLE_INPUT_SRC, {
        /// A non-numeric string fails to parse as `f64` with
        /// `ParseFloatError`, while a valid numeric string succeeds.
        /// `ParseFloatError`'s public API is Display/Debug/Error only (no
        /// `.kind()`, unlike `ParseIntError`), so there's no further
        /// structure to check beyond success/failure itself.
        #[kani::proof]
        fn verify_parse_float_error_occurs_only_for_unparseable_input() {
            assert!(
                FallibleOperationReportsFailure::ensures("not a float".parse::<f64>().is_err()),
                "a non-numeric string fails to parse as f64"
            );
            assert!(
                "3.14".parse::<f64>().is_ok(),
                "a valid numeric string parses as f64 successfully"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<FpCategory> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_fp_category_matches_the_value_it_classifies".to_owned(),
            VERIFY_FP_CATEGORY_MATCHES_THE_VALUE_IT_CLASSIFIES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<FpCategory>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::num::FpCategory>",
        "kani",
        || <RustStdStandard<FpCategory> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_FP_CATEGORY_MATCHES_THE_VALUE_IT_CLASSIFIES_SRC, {
        /// Each representative floating-point value classifies into the
        /// `FpCategory` variant matching its own `is_*` predicates.
        #[kani::proof]
        fn verify_fp_category_matches_the_value_it_classifies() {
            assert_eq!(f64::NAN.classify(), FpCategory::Nan, "NaN classifies as FpCategory::Nan");
            assert_eq!(
                f64::INFINITY.classify(),
                FpCategory::Infinite,
                "infinity classifies as FpCategory::Infinite"
            );
            assert_eq!(0.0f64.classify(), FpCategory::Zero, "zero classifies as FpCategory::Zero");
            assert_eq!(
                f64::MIN_POSITIVE.classify(),
                FpCategory::Normal,
                "the smallest positive normal value classifies as FpCategory::Normal"
            );
            let subnormal = f64::MIN_POSITIVE / 2.0;
            assert_eq!(
                subnormal.classify(),
                FpCategory::Subnormal,
                "a value smaller than the smallest normal value classifies as FpCategory::Subnormal"
            );
        }
    }
}
