//! `KaniWitness` impls and load/store harnesses for the signed atomic
//! integers (`AtomicI8`/`I16`/`I32`/`I64`/`Isize`).
//!
//! Each type is written out literally rather than generated through a
//! wrapping `macro_rules!`: `amenable_derive::harness!` captures a harness's
//! verbatim source via the group's span, and a span produced by a
//! `macro_rules!` expansion resolves back to the *defining* macro's on-disk
//! text -- so a generator macro would capture its own placeholders,
//! unsubstituted, instead of each type's real harness.

use std::sync::atomic::{AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize};

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_std::RustStdStandard;

#[cfg(kani)]
use crate::AtomicLoadReflectsTheLastWrite;
use crate::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::bridge_kani_witness;

impl KaniWitness for RustStdStandard<AtomicI8> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_atomic_i8".to_owned(),
            VERIFY_ATOMIC_I8_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<AtomicI8>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<AtomicI8>",
        "kani",
        || <RustStdStandard<AtomicI8> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_ATOMIC_I8_SRC, {
        /// `AtomicI8::new` sets the value observable via `load`, and
        /// `store` overwrites it, both under `SeqCst` ordering.
        #[kani::proof]
        fn verify_atomic_i8() {
            let initial: i8 = kani::any();
            let atomic = AtomicI8::new(initial);
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    atomic.load(std::sync::atomic::Ordering::SeqCst),
                    initial
                )),
                "AtomicI8::new sets the value observable via load"
            );

            let next: i8 = kani::any();
            atomic.store(next, std::sync::atomic::Ordering::SeqCst);
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    atomic.load(std::sync::atomic::Ordering::SeqCst),
                    next
                )),
                "AtomicI8::store overwrites the value observable via load"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<AtomicI16> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_atomic_i16".to_owned(),
            VERIFY_ATOMIC_I16_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<AtomicI16>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<AtomicI16>",
        "kani",
        || <RustStdStandard<AtomicI16> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_ATOMIC_I16_SRC, {
        /// `AtomicI16::new` sets the value observable via `load`, and
        /// `store` overwrites it, both under `SeqCst` ordering.
        #[kani::proof]
        fn verify_atomic_i16() {
            let initial: i16 = kani::any();
            let atomic = AtomicI16::new(initial);
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    atomic.load(std::sync::atomic::Ordering::SeqCst),
                    initial
                )),
                "AtomicI16::new sets the value observable via load"
            );

            let next: i16 = kani::any();
            atomic.store(next, std::sync::atomic::Ordering::SeqCst);
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    atomic.load(std::sync::atomic::Ordering::SeqCst),
                    next
                )),
                "AtomicI16::store overwrites the value observable via load"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<AtomicI32> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_atomic_i32".to_owned(),
            VERIFY_ATOMIC_I32_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<AtomicI32>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<AtomicI32>",
        "kani",
        || <RustStdStandard<AtomicI32> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_ATOMIC_I32_SRC, {
        /// `AtomicI32::new` sets the value observable via `load`, and
        /// `store` overwrites it, both under `SeqCst` ordering.
        #[kani::proof]
        fn verify_atomic_i32() {
            let initial: i32 = kani::any();
            let atomic = AtomicI32::new(initial);
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    atomic.load(std::sync::atomic::Ordering::SeqCst),
                    initial
                )),
                "AtomicI32::new sets the value observable via load"
            );

            let next: i32 = kani::any();
            atomic.store(next, std::sync::atomic::Ordering::SeqCst);
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    atomic.load(std::sync::atomic::Ordering::SeqCst),
                    next
                )),
                "AtomicI32::store overwrites the value observable via load"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<AtomicI64> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_atomic_i64".to_owned(),
            VERIFY_ATOMIC_I64_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<AtomicI64>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<AtomicI64>",
        "kani",
        || <RustStdStandard<AtomicI64> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_ATOMIC_I64_SRC, {
        /// `AtomicI64::new` sets the value observable via `load`, and
        /// `store` overwrites it, both under `SeqCst` ordering.
        #[kani::proof]
        fn verify_atomic_i64() {
            let initial: i64 = kani::any();
            let atomic = AtomicI64::new(initial);
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    atomic.load(std::sync::atomic::Ordering::SeqCst),
                    initial
                )),
                "AtomicI64::new sets the value observable via load"
            );

            let next: i64 = kani::any();
            atomic.store(next, std::sync::atomic::Ordering::SeqCst);
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    atomic.load(std::sync::atomic::Ordering::SeqCst),
                    next
                )),
                "AtomicI64::store overwrites the value observable via load"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<AtomicIsize> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_atomic_isize".to_owned(),
            VERIFY_ATOMIC_ISIZE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<AtomicIsize>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<AtomicIsize>",
        "kani",
        || <RustStdStandard<AtomicIsize> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_ATOMIC_ISIZE_SRC, {
        /// `AtomicIsize::new` sets the value observable via `load`, and
        /// `store` overwrites it, both under `SeqCst` ordering.
        #[kani::proof]
        fn verify_atomic_isize() {
            let initial: isize = kani::any();
            let atomic = AtomicIsize::new(initial);
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    atomic.load(std::sync::atomic::Ordering::SeqCst),
                    initial
                )),
                "AtomicIsize::new sets the value observable via load"
            );

            let next: isize = kani::any();
            atomic.store(next, std::sync::atomic::Ordering::SeqCst);
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    atomic.load(std::sync::atomic::Ordering::SeqCst),
                    next
                )),
                "AtomicIsize::store overwrites the value observable via load"
            );
        }
    }
}
