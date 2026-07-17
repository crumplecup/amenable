//! `KaniWitness` impls for `core::num`.
//!
//! Each `NonZero<T>` instantiation is written out literally rather than
//! generated through a wrapping `macro_rules!`: `amenable_derive::harness!`
//! captures a harness's verbatim source via the group's span, and a span
//! produced by a `macro_rules!` expansion resolves back to the *defining*
//! macro's on-disk text — so a generator macro would capture its own
//! `$ty`/`$harness_fn` placeholders, unsubstituted, instead of each type's
//! real harness. Twelve literal blocks is the price of an honest `claim`.

use std::num::NonZero;

use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::bridge_kani_witness;

impl KaniWitness for RustStdStandard<NonZero<i8>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_nonzero_i8",
            claim: VERIFY_NONZERO_I8_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<NonZero<i8>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<NonZero<i8>>",
        verifier: "kani",
        describe: || <RustStdStandard<NonZero<i8>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_NONZERO_I8_SRC, {
        /// `NonZero::new` succeeds iff the input is nonzero, and `.get()`
        /// round-trips the wrapped value unchanged.
        #[kani::proof]
        fn verify_nonzero_i8() {
            let value: i8 = kani::any();
            match NonZero::<i8>::new(value) {
                Some(nz) => {
                    assert!(value != 0, "NonZero::new succeeds only for nonzero values");
                    assert_eq!(nz.get(), value, "NonZero round-trips its wrapped value");
                }
                None => {
                    assert_eq!(value, 0, "NonZero::new fails only for zero");
                }
            }
        }
    }
}

impl KaniWitness for RustStdStandard<NonZero<i16>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_nonzero_i16",
            claim: VERIFY_NONZERO_I16_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<NonZero<i16>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<NonZero<i16>>",
        verifier: "kani",
        describe: || <RustStdStandard<NonZero<i16>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_NONZERO_I16_SRC, {
        /// `NonZero::new` succeeds iff the input is nonzero, and `.get()`
        /// round-trips the wrapped value unchanged.
        #[kani::proof]
        fn verify_nonzero_i16() {
            let value: i16 = kani::any();
            match NonZero::<i16>::new(value) {
                Some(nz) => {
                    assert!(value != 0, "NonZero::new succeeds only for nonzero values");
                    assert_eq!(nz.get(), value, "NonZero round-trips its wrapped value");
                }
                None => {
                    assert_eq!(value, 0, "NonZero::new fails only for zero");
                }
            }
        }
    }
}

impl KaniWitness for RustStdStandard<NonZero<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_nonzero_i32",
            claim: VERIFY_NONZERO_I32_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<NonZero<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<NonZero<i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<NonZero<i32>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_NONZERO_I32_SRC, {
        /// `NonZero::new` succeeds iff the input is nonzero, and `.get()`
        /// round-trips the wrapped value unchanged.
        #[kani::proof]
        fn verify_nonzero_i32() {
            let value: i32 = kani::any();
            match NonZero::<i32>::new(value) {
                Some(nz) => {
                    assert!(value != 0, "NonZero::new succeeds only for nonzero values");
                    assert_eq!(nz.get(), value, "NonZero round-trips its wrapped value");
                }
                None => {
                    assert_eq!(value, 0, "NonZero::new fails only for zero");
                }
            }
        }
    }
}

impl KaniWitness for RustStdStandard<NonZero<i64>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_nonzero_i64",
            claim: VERIFY_NONZERO_I64_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<NonZero<i64>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<NonZero<i64>>",
        verifier: "kani",
        describe: || <RustStdStandard<NonZero<i64>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_NONZERO_I64_SRC, {
        /// `NonZero::new` succeeds iff the input is nonzero, and `.get()`
        /// round-trips the wrapped value unchanged.
        #[kani::proof]
        fn verify_nonzero_i64() {
            let value: i64 = kani::any();
            match NonZero::<i64>::new(value) {
                Some(nz) => {
                    assert!(value != 0, "NonZero::new succeeds only for nonzero values");
                    assert_eq!(nz.get(), value, "NonZero round-trips its wrapped value");
                }
                None => {
                    assert_eq!(value, 0, "NonZero::new fails only for zero");
                }
            }
        }
    }
}

impl KaniWitness for RustStdStandard<NonZero<i128>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_nonzero_i128",
            claim: VERIFY_NONZERO_I128_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<NonZero<i128>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<NonZero<i128>>",
        verifier: "kani",
        describe: || <RustStdStandard<NonZero<i128>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_NONZERO_I128_SRC, {
        /// `NonZero::new` succeeds iff the input is nonzero, and `.get()`
        /// round-trips the wrapped value unchanged.
        #[kani::proof]
        fn verify_nonzero_i128() {
            let value: i128 = kani::any();
            match NonZero::<i128>::new(value) {
                Some(nz) => {
                    assert!(value != 0, "NonZero::new succeeds only for nonzero values");
                    assert_eq!(nz.get(), value, "NonZero round-trips its wrapped value");
                }
                None => {
                    assert_eq!(value, 0, "NonZero::new fails only for zero");
                }
            }
        }
    }
}

impl KaniWitness for RustStdStandard<NonZero<isize>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_nonzero_isize",
            claim: VERIFY_NONZERO_ISIZE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<NonZero<isize>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<NonZero<isize>>",
        verifier: "kani",
        describe: || <RustStdStandard<NonZero<isize>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_NONZERO_ISIZE_SRC, {
        /// `NonZero::new` succeeds iff the input is nonzero, and `.get()`
        /// round-trips the wrapped value unchanged.
        #[kani::proof]
        fn verify_nonzero_isize() {
            let value: isize = kani::any();
            match NonZero::<isize>::new(value) {
                Some(nz) => {
                    assert!(value != 0, "NonZero::new succeeds only for nonzero values");
                    assert_eq!(nz.get(), value, "NonZero round-trips its wrapped value");
                }
                None => {
                    assert_eq!(value, 0, "NonZero::new fails only for zero");
                }
            }
        }
    }
}

impl KaniWitness for RustStdStandard<NonZero<u8>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_nonzero_u8",
            claim: VERIFY_NONZERO_U8_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<NonZero<u8>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<NonZero<u8>>",
        verifier: "kani",
        describe: || <RustStdStandard<NonZero<u8>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_NONZERO_U8_SRC, {
        /// `NonZero::new` succeeds iff the input is nonzero, and `.get()`
        /// round-trips the wrapped value unchanged.
        #[kani::proof]
        fn verify_nonzero_u8() {
            let value: u8 = kani::any();
            match NonZero::<u8>::new(value) {
                Some(nz) => {
                    assert!(value != 0, "NonZero::new succeeds only for nonzero values");
                    assert_eq!(nz.get(), value, "NonZero round-trips its wrapped value");
                }
                None => {
                    assert_eq!(value, 0, "NonZero::new fails only for zero");
                }
            }
        }
    }
}

impl KaniWitness for RustStdStandard<NonZero<u16>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_nonzero_u16",
            claim: VERIFY_NONZERO_U16_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<NonZero<u16>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<NonZero<u16>>",
        verifier: "kani",
        describe: || <RustStdStandard<NonZero<u16>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_NONZERO_U16_SRC, {
        /// `NonZero::new` succeeds iff the input is nonzero, and `.get()`
        /// round-trips the wrapped value unchanged.
        #[kani::proof]
        fn verify_nonzero_u16() {
            let value: u16 = kani::any();
            match NonZero::<u16>::new(value) {
                Some(nz) => {
                    assert!(value != 0, "NonZero::new succeeds only for nonzero values");
                    assert_eq!(nz.get(), value, "NonZero round-trips its wrapped value");
                }
                None => {
                    assert_eq!(value, 0, "NonZero::new fails only for zero");
                }
            }
        }
    }
}

impl KaniWitness for RustStdStandard<NonZero<u32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_nonzero_u32",
            claim: VERIFY_NONZERO_U32_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<NonZero<u32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<NonZero<u32>>",
        verifier: "kani",
        describe: || <RustStdStandard<NonZero<u32>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_NONZERO_U32_SRC, {
        /// `NonZero::new` succeeds iff the input is nonzero, and `.get()`
        /// round-trips the wrapped value unchanged.
        #[kani::proof]
        fn verify_nonzero_u32() {
            let value: u32 = kani::any();
            match NonZero::<u32>::new(value) {
                Some(nz) => {
                    assert!(value != 0, "NonZero::new succeeds only for nonzero values");
                    assert_eq!(nz.get(), value, "NonZero round-trips its wrapped value");
                }
                None => {
                    assert_eq!(value, 0, "NonZero::new fails only for zero");
                }
            }
        }
    }
}

impl KaniWitness for RustStdStandard<NonZero<u64>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_nonzero_u64",
            claim: VERIFY_NONZERO_U64_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<NonZero<u64>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<NonZero<u64>>",
        verifier: "kani",
        describe: || <RustStdStandard<NonZero<u64>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_NONZERO_U64_SRC, {
        /// `NonZero::new` succeeds iff the input is nonzero, and `.get()`
        /// round-trips the wrapped value unchanged.
        #[kani::proof]
        fn verify_nonzero_u64() {
            let value: u64 = kani::any();
            match NonZero::<u64>::new(value) {
                Some(nz) => {
                    assert!(value != 0, "NonZero::new succeeds only for nonzero values");
                    assert_eq!(nz.get(), value, "NonZero round-trips its wrapped value");
                }
                None => {
                    assert_eq!(value, 0, "NonZero::new fails only for zero");
                }
            }
        }
    }
}

impl KaniWitness for RustStdStandard<NonZero<u128>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_nonzero_u128",
            claim: VERIFY_NONZERO_U128_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<NonZero<u128>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<NonZero<u128>>",
        verifier: "kani",
        describe: || <RustStdStandard<NonZero<u128>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_NONZERO_U128_SRC, {
        /// `NonZero::new` succeeds iff the input is nonzero, and `.get()`
        /// round-trips the wrapped value unchanged.
        #[kani::proof]
        fn verify_nonzero_u128() {
            let value: u128 = kani::any();
            match NonZero::<u128>::new(value) {
                Some(nz) => {
                    assert!(value != 0, "NonZero::new succeeds only for nonzero values");
                    assert_eq!(nz.get(), value, "NonZero round-trips its wrapped value");
                }
                None => {
                    assert_eq!(value, 0, "NonZero::new fails only for zero");
                }
            }
        }
    }
}

impl KaniWitness for RustStdStandard<NonZero<usize>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_nonzero_usize",
            claim: VERIFY_NONZERO_USIZE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<NonZero<usize>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<NonZero<usize>>",
        verifier: "kani",
        describe: || <RustStdStandard<NonZero<usize>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_NONZERO_USIZE_SRC, {
        /// `NonZero::new` succeeds iff the input is nonzero, and `.get()`
        /// round-trips the wrapped value unchanged.
        #[kani::proof]
        fn verify_nonzero_usize() {
            let value: usize = kani::any();
            match NonZero::<usize>::new(value) {
                Some(nz) => {
                    assert!(value != 0, "NonZero::new succeeds only for nonzero values");
                    assert_eq!(nz.get(), value, "NonZero round-trips its wrapped value");
                }
                None => {
                    assert_eq!(value, 0, "NonZero::new fails only for zero");
                }
            }
        }
    }
}
