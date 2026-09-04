//! `AtomicPtr<i32>`'s `KaniWitness` impl and its load/store/swap/
//! compare-exchange harness.

use std::sync::atomic::AtomicPtr;

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use crate::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::{bridge_kani_witness, kani_ensures};
#[cfg(kani)]
use crate::{AccessorRecoversTheExpectedValue, AtomicLoadReflectsTheLastWrite};

impl KaniWitness for RustStdStandard<AtomicPtr<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_atomic_ptr_load_store_swap_and_compare_exchange".to_owned(),
            VERIFY_ATOMIC_PTR_LOAD_STORE_SWAP_AND_COMPARE_EXCHANGE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<AtomicPtr<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<AtomicPtr<i32>>",
        "kani",
        || <RustStdStandard<AtomicPtr<i32>> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<AtomicPtr<i32>>,
    "amenable_std::rust_std::RustStdStandard<AtomicPtr<i32>>",
    (*mut i32, *mut i32),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_ATOMIC_PTR_LOAD_STORE_SWAP_AND_COMPARE_EXCHANGE_SRC, {
        /// `AtomicPtr::new` sets the pointer value observable via `load`;
        /// `store` overwrites it; `swap` returns the previous pointer and
        /// installs the new one; `compare_exchange` updates on a matching
        /// current value and reports the previous value on success.
        /// Pointer values are the addresses of distinct local slots (each
        /// holding a symbolic `i32`), so every value is pairwise-distinct
        /// by construction and no pointer is ever dereferenced — the
        /// whole proof stays free of `unsafe`.
        ///
        /// `compare_exchange`'s *failure* branch (mismatched current,
        /// reporting the real current value unchanged) is deliberately
        /// not checked here: Kani's model of the underlying
        /// `atomic_compare_exchange` intrinsic does not correctly track
        /// the reported value on that branch, and that false trail remains
        /// preserved in `gallery::atomic_ptr_compare_exchange` — a minimal,
        /// isolated reproduction (a single `compare_exchange` call against
        /// a freshly constructed `AtomicPtr`, no `swap` or prior exchange
        /// involved) that fails the same way, confirming this is a genuine
        /// Kani/CBMC limitation rather than a logic error in this proof or
        /// in std.
        #[kani::proof]
        fn verify_atomic_ptr_load_store_swap_and_compare_exchange() {
            let mut initial_slot: i32 = kani::any();
            let mut stored_slot: i32 = kani::any();
            let mut swapped_in_slot: i32 = kani::any();
            let mut exchange_target_slot: i32 = kani::any();

            let initial: *mut i32 = &mut initial_slot;
            let atomic = AtomicPtr::new(initial);
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    atomic.load(std::sync::atomic::Ordering::SeqCst),
                    initial
                )),
                "AtomicPtr::new sets the value observable via load"
            );

            let stored: *mut i32 = &mut stored_slot;
            atomic.store(stored, std::sync::atomic::Ordering::SeqCst);
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    atomic.load(std::sync::atomic::Ordering::SeqCst),
                    stored
                )),
                "AtomicPtr::store overwrites the value observable via load"
            );

            let swapped_in: *mut i32 = &mut swapped_in_slot;
            let previous = atomic.swap(swapped_in, std::sync::atomic::Ordering::SeqCst);
            assert!(
                <RustStdStandard<AtomicPtr<i32>> as Ensures<crate::KaniVerifier>>::ensures((
                    previous, stored
                )),
                "AtomicPtr::swap returns the value that was there before"
            );
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    atomic.load(std::sync::atomic::Ordering::SeqCst),
                    swapped_in
                )),
                "AtomicPtr::swap installs the new value observable via load"
            );

            let exchange_target: *mut i32 = &mut exchange_target_slot;
            let success_result = atomic.compare_exchange(
                swapped_in,
                exchange_target,
                std::sync::atomic::Ordering::SeqCst,
                std::sync::atomic::Ordering::SeqCst,
            );
            assert!(
                AccessorRecoversTheExpectedValue::ensures((success_result, Ok(swapped_in))),
                "compare_exchange succeeds and returns the previous value when current matches"
            );
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    atomic.load(std::sync::atomic::Ordering::SeqCst),
                    exchange_target
                )),
                "compare_exchange installs the new value on success"
            );
        }
    }
}
