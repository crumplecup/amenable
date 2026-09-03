//! `KaniWitness` impls and load/store harnesses for the unsigned atomic
//! integers (`AtomicU8`/`U16`/`U32`/`U64`/`Usize`).
//!
//! Each type is written out literally rather than generated through a
//! wrapping `macro_rules!`, for the reason given in the `signed` submodule.

use std::sync::atomic::{AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicUsize};

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_std::RustStdStandard;

#[cfg(kani)]
use crate::AtomicLoadReflectsTheLastWrite;
use crate::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::bridge_kani_witness;

impl KaniWitness for RustStdStandard<AtomicU8> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_atomic_u8".to_owned(),
            VERIFY_ATOMIC_U8_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<AtomicU8>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<AtomicU8>",
        "kani",
        || <RustStdStandard<AtomicU8> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_ATOMIC_U8_SRC, {
        /// `AtomicU8::new` sets the value observable via `load`, and
        /// `store` overwrites it, both under `SeqCst` ordering.
        #[kani::proof]
        fn verify_atomic_u8() {
            let initial: u8 = kani::any();
            let atomic = AtomicU8::new(initial);
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    atomic.load(std::sync::atomic::Ordering::SeqCst),
                    initial
                )),
                "AtomicU8::new sets the value observable via load"
            );

            let next: u8 = kani::any();
            atomic.store(next, std::sync::atomic::Ordering::SeqCst);
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    atomic.load(std::sync::atomic::Ordering::SeqCst),
                    next
                )),
                "AtomicU8::store overwrites the value observable via load"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<AtomicU16> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_atomic_u16".to_owned(),
            VERIFY_ATOMIC_U16_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<AtomicU16>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<AtomicU16>",
        "kani",
        || <RustStdStandard<AtomicU16> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_ATOMIC_U16_SRC, {
        /// `AtomicU16::new` sets the value observable via `load`, and
        /// `store` overwrites it, both under `SeqCst` ordering.
        #[kani::proof]
        fn verify_atomic_u16() {
            let initial: u16 = kani::any();
            let atomic = AtomicU16::new(initial);
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    atomic.load(std::sync::atomic::Ordering::SeqCst),
                    initial
                )),
                "AtomicU16::new sets the value observable via load"
            );

            let next: u16 = kani::any();
            atomic.store(next, std::sync::atomic::Ordering::SeqCst);
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    atomic.load(std::sync::atomic::Ordering::SeqCst),
                    next
                )),
                "AtomicU16::store overwrites the value observable via load"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<AtomicU32> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_atomic_u32".to_owned(),
            VERIFY_ATOMIC_U32_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<AtomicU32>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<AtomicU32>",
        "kani",
        || <RustStdStandard<AtomicU32> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_ATOMIC_U32_SRC, {
        /// `AtomicU32::new` sets the value observable via `load`, and
        /// `store` overwrites it, both under `SeqCst` ordering.
        #[kani::proof]
        fn verify_atomic_u32() {
            let initial: u32 = kani::any();
            let atomic = AtomicU32::new(initial);
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    atomic.load(std::sync::atomic::Ordering::SeqCst),
                    initial
                )),
                "AtomicU32::new sets the value observable via load"
            );

            let next: u32 = kani::any();
            atomic.store(next, std::sync::atomic::Ordering::SeqCst);
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    atomic.load(std::sync::atomic::Ordering::SeqCst),
                    next
                )),
                "AtomicU32::store overwrites the value observable via load"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<AtomicU64> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_atomic_u64".to_owned(),
            VERIFY_ATOMIC_U64_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<AtomicU64>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<AtomicU64>",
        "kani",
        || <RustStdStandard<AtomicU64> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_ATOMIC_U64_SRC, {
        /// `AtomicU64::new` sets the value observable via `load`, and
        /// `store` overwrites it, both under `SeqCst` ordering.
        #[kani::proof]
        fn verify_atomic_u64() {
            let initial: u64 = kani::any();
            let atomic = AtomicU64::new(initial);
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    atomic.load(std::sync::atomic::Ordering::SeqCst),
                    initial
                )),
                "AtomicU64::new sets the value observable via load"
            );

            let next: u64 = kani::any();
            atomic.store(next, std::sync::atomic::Ordering::SeqCst);
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    atomic.load(std::sync::atomic::Ordering::SeqCst),
                    next
                )),
                "AtomicU64::store overwrites the value observable via load"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<AtomicUsize> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_atomic_usize".to_owned(),
            VERIFY_ATOMIC_USIZE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<AtomicUsize>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<AtomicUsize>",
        "kani",
        || <RustStdStandard<AtomicUsize> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_ATOMIC_USIZE_SRC, {
        /// `AtomicUsize::new` sets the value observable via `load`, and
        /// `store` overwrites it, both under `SeqCst` ordering.
        #[kani::proof]
        fn verify_atomic_usize() {
            let initial: usize = kani::any();
            let atomic = AtomicUsize::new(initial);
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    atomic.load(std::sync::atomic::Ordering::SeqCst),
                    initial
                )),
                "AtomicUsize::new sets the value observable via load"
            );

            let next: usize = kani::any();
            atomic.store(next, std::sync::atomic::Ordering::SeqCst);
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    atomic.load(std::sync::atomic::Ordering::SeqCst),
                    next
                )),
                "AtomicUsize::store overwrites the value observable via load"
            );
        }
    }
}
