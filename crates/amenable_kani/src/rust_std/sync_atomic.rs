//! `KaniWitness` impls for `core::sync::atomic`.
//!
//! Each `Atomic*` type is written out literally rather than generated
//! through a wrapping `macro_rules!`, for the same reason `num.rs`'s
//! `NonZero<T>` instantiations are: `amenable_derive::harness!` captures a
//! harness's verbatim source via the group's span, and a span produced by a
//! `macro_rules!` expansion resolves back to the *defining* macro's on-disk
//! text — so a generator macro would capture its own placeholders,
//! unsubstituted, instead of each type's real harness.

use std::sync::atomic::{
    AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize, AtomicU8, AtomicU16,
    AtomicU32, AtomicU64, AtomicUsize,
};

use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::bridge_kani_witness;

impl KaniWitness for RustStdStandard<AtomicBool> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_atomic_bool".to_owned(),
            claim: VERIFY_ATOMIC_BOOL_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<AtomicBool>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<AtomicBool>",
        verifier: "kani",
        describe: || <RustStdStandard<AtomicBool> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_ATOMIC_BOOL_SRC, {
        /// `AtomicBool::new` sets the value observable via `load`, and
        /// `store` overwrites it, both under `SeqCst` ordering.
        #[kani::proof]
        fn verify_atomic_bool() {
            let initial: bool = kani::any();
            let atomic = AtomicBool::new(initial);
            assert_eq!(
                atomic.load(std::sync::atomic::Ordering::SeqCst),
                initial,
                "AtomicBool::new sets the value observable via load"
            );

            let next: bool = kani::any();
            atomic.store(next, std::sync::atomic::Ordering::SeqCst);
            assert_eq!(
                atomic.load(std::sync::atomic::Ordering::SeqCst),
                next,
                "AtomicBool::store overwrites the value observable via load"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<AtomicI8> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_atomic_i8".to_owned(),
            claim: VERIFY_ATOMIC_I8_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<AtomicI8>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<AtomicI8>",
        verifier: "kani",
        describe: || <RustStdStandard<AtomicI8> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_ATOMIC_I8_SRC, {
        /// `AtomicI8::new` sets the value observable via `load`, and
        /// `store` overwrites it, both under `SeqCst` ordering.
        #[kani::proof]
        fn verify_atomic_i8() {
            let initial: i8 = kani::any();
            let atomic = AtomicI8::new(initial);
            assert_eq!(
                atomic.load(std::sync::atomic::Ordering::SeqCst),
                initial,
                "AtomicI8::new sets the value observable via load"
            );

            let next: i8 = kani::any();
            atomic.store(next, std::sync::atomic::Ordering::SeqCst);
            assert_eq!(
                atomic.load(std::sync::atomic::Ordering::SeqCst),
                next,
                "AtomicI8::store overwrites the value observable via load"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<AtomicI16> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_atomic_i16".to_owned(),
            claim: VERIFY_ATOMIC_I16_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<AtomicI16>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<AtomicI16>",
        verifier: "kani",
        describe: || <RustStdStandard<AtomicI16> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_ATOMIC_I16_SRC, {
        /// `AtomicI16::new` sets the value observable via `load`, and
        /// `store` overwrites it, both under `SeqCst` ordering.
        #[kani::proof]
        fn verify_atomic_i16() {
            let initial: i16 = kani::any();
            let atomic = AtomicI16::new(initial);
            assert_eq!(
                atomic.load(std::sync::atomic::Ordering::SeqCst),
                initial,
                "AtomicI16::new sets the value observable via load"
            );

            let next: i16 = kani::any();
            atomic.store(next, std::sync::atomic::Ordering::SeqCst);
            assert_eq!(
                atomic.load(std::sync::atomic::Ordering::SeqCst),
                next,
                "AtomicI16::store overwrites the value observable via load"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<AtomicI32> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_atomic_i32".to_owned(),
            claim: VERIFY_ATOMIC_I32_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<AtomicI32>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<AtomicI32>",
        verifier: "kani",
        describe: || <RustStdStandard<AtomicI32> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_ATOMIC_I32_SRC, {
        /// `AtomicI32::new` sets the value observable via `load`, and
        /// `store` overwrites it, both under `SeqCst` ordering.
        #[kani::proof]
        fn verify_atomic_i32() {
            let initial: i32 = kani::any();
            let atomic = AtomicI32::new(initial);
            assert_eq!(
                atomic.load(std::sync::atomic::Ordering::SeqCst),
                initial,
                "AtomicI32::new sets the value observable via load"
            );

            let next: i32 = kani::any();
            atomic.store(next, std::sync::atomic::Ordering::SeqCst);
            assert_eq!(
                atomic.load(std::sync::atomic::Ordering::SeqCst),
                next,
                "AtomicI32::store overwrites the value observable via load"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<AtomicI64> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_atomic_i64".to_owned(),
            claim: VERIFY_ATOMIC_I64_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<AtomicI64>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<AtomicI64>",
        verifier: "kani",
        describe: || <RustStdStandard<AtomicI64> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_ATOMIC_I64_SRC, {
        /// `AtomicI64::new` sets the value observable via `load`, and
        /// `store` overwrites it, both under `SeqCst` ordering.
        #[kani::proof]
        fn verify_atomic_i64() {
            let initial: i64 = kani::any();
            let atomic = AtomicI64::new(initial);
            assert_eq!(
                atomic.load(std::sync::atomic::Ordering::SeqCst),
                initial,
                "AtomicI64::new sets the value observable via load"
            );

            let next: i64 = kani::any();
            atomic.store(next, std::sync::atomic::Ordering::SeqCst);
            assert_eq!(
                atomic.load(std::sync::atomic::Ordering::SeqCst),
                next,
                "AtomicI64::store overwrites the value observable via load"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<AtomicIsize> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_atomic_isize".to_owned(),
            claim: VERIFY_ATOMIC_ISIZE_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<AtomicIsize>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<AtomicIsize>",
        verifier: "kani",
        describe: || <RustStdStandard<AtomicIsize> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_ATOMIC_ISIZE_SRC, {
        /// `AtomicIsize::new` sets the value observable via `load`, and
        /// `store` overwrites it, both under `SeqCst` ordering.
        #[kani::proof]
        fn verify_atomic_isize() {
            let initial: isize = kani::any();
            let atomic = AtomicIsize::new(initial);
            assert_eq!(
                atomic.load(std::sync::atomic::Ordering::SeqCst),
                initial,
                "AtomicIsize::new sets the value observable via load"
            );

            let next: isize = kani::any();
            atomic.store(next, std::sync::atomic::Ordering::SeqCst);
            assert_eq!(
                atomic.load(std::sync::atomic::Ordering::SeqCst),
                next,
                "AtomicIsize::store overwrites the value observable via load"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<AtomicU8> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_atomic_u8".to_owned(),
            claim: VERIFY_ATOMIC_U8_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<AtomicU8>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<AtomicU8>",
        verifier: "kani",
        describe: || <RustStdStandard<AtomicU8> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_ATOMIC_U8_SRC, {
        /// `AtomicU8::new` sets the value observable via `load`, and
        /// `store` overwrites it, both under `SeqCst` ordering.
        #[kani::proof]
        fn verify_atomic_u8() {
            let initial: u8 = kani::any();
            let atomic = AtomicU8::new(initial);
            assert_eq!(
                atomic.load(std::sync::atomic::Ordering::SeqCst),
                initial,
                "AtomicU8::new sets the value observable via load"
            );

            let next: u8 = kani::any();
            atomic.store(next, std::sync::atomic::Ordering::SeqCst);
            assert_eq!(
                atomic.load(std::sync::atomic::Ordering::SeqCst),
                next,
                "AtomicU8::store overwrites the value observable via load"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<AtomicU16> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_atomic_u16".to_owned(),
            claim: VERIFY_ATOMIC_U16_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<AtomicU16>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<AtomicU16>",
        verifier: "kani",
        describe: || <RustStdStandard<AtomicU16> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_ATOMIC_U16_SRC, {
        /// `AtomicU16::new` sets the value observable via `load`, and
        /// `store` overwrites it, both under `SeqCst` ordering.
        #[kani::proof]
        fn verify_atomic_u16() {
            let initial: u16 = kani::any();
            let atomic = AtomicU16::new(initial);
            assert_eq!(
                atomic.load(std::sync::atomic::Ordering::SeqCst),
                initial,
                "AtomicU16::new sets the value observable via load"
            );

            let next: u16 = kani::any();
            atomic.store(next, std::sync::atomic::Ordering::SeqCst);
            assert_eq!(
                atomic.load(std::sync::atomic::Ordering::SeqCst),
                next,
                "AtomicU16::store overwrites the value observable via load"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<AtomicU32> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_atomic_u32".to_owned(),
            claim: VERIFY_ATOMIC_U32_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<AtomicU32>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<AtomicU32>",
        verifier: "kani",
        describe: || <RustStdStandard<AtomicU32> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_ATOMIC_U32_SRC, {
        /// `AtomicU32::new` sets the value observable via `load`, and
        /// `store` overwrites it, both under `SeqCst` ordering.
        #[kani::proof]
        fn verify_atomic_u32() {
            let initial: u32 = kani::any();
            let atomic = AtomicU32::new(initial);
            assert_eq!(
                atomic.load(std::sync::atomic::Ordering::SeqCst),
                initial,
                "AtomicU32::new sets the value observable via load"
            );

            let next: u32 = kani::any();
            atomic.store(next, std::sync::atomic::Ordering::SeqCst);
            assert_eq!(
                atomic.load(std::sync::atomic::Ordering::SeqCst),
                next,
                "AtomicU32::store overwrites the value observable via load"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<AtomicU64> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_atomic_u64".to_owned(),
            claim: VERIFY_ATOMIC_U64_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<AtomicU64>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<AtomicU64>",
        verifier: "kani",
        describe: || <RustStdStandard<AtomicU64> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_ATOMIC_U64_SRC, {
        /// `AtomicU64::new` sets the value observable via `load`, and
        /// `store` overwrites it, both under `SeqCst` ordering.
        #[kani::proof]
        fn verify_atomic_u64() {
            let initial: u64 = kani::any();
            let atomic = AtomicU64::new(initial);
            assert_eq!(
                atomic.load(std::sync::atomic::Ordering::SeqCst),
                initial,
                "AtomicU64::new sets the value observable via load"
            );

            let next: u64 = kani::any();
            atomic.store(next, std::sync::atomic::Ordering::SeqCst);
            assert_eq!(
                atomic.load(std::sync::atomic::Ordering::SeqCst),
                next,
                "AtomicU64::store overwrites the value observable via load"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<AtomicUsize> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_atomic_usize".to_owned(),
            claim: VERIFY_ATOMIC_USIZE_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<AtomicUsize>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<AtomicUsize>",
        verifier: "kani",
        describe: || <RustStdStandard<AtomicUsize> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_ATOMIC_USIZE_SRC, {
        /// `AtomicUsize::new` sets the value observable via `load`, and
        /// `store` overwrites it, both under `SeqCst` ordering.
        #[kani::proof]
        fn verify_atomic_usize() {
            let initial: usize = kani::any();
            let atomic = AtomicUsize::new(initial);
            assert_eq!(
                atomic.load(std::sync::atomic::Ordering::SeqCst),
                initial,
                "AtomicUsize::new sets the value observable via load"
            );

            let next: usize = kani::any();
            atomic.store(next, std::sync::atomic::Ordering::SeqCst);
            assert_eq!(
                atomic.load(std::sync::atomic::Ordering::SeqCst),
                next,
                "AtomicUsize::store overwrites the value observable via load"
            );
        }
    }
}
