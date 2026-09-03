//! `KaniWitness` impls for `alloc::sync`.

use std::sync::Arc;

use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::bridge_kani_witness;

/// The `#[cfg(kani)]` imports this file needs, consolidated into one gate
/// on this `mod` instead of one per item -- see
/// `amenable_creusot::stoplight::mirror`'s own doc comment for the
/// general rationale. Every name is re-exported: the `harness! { .. }`
/// blocks below need all of them, unqualified, at this file's own top
/// level.
#[cfg(kani)]
mod mirror {
    pub(super) use amenable_core::Ensures;
    pub(super) use std::cell::Cell;

    pub(super) use crate::DerefReflectsTheStoredValue;
    pub(super) use crate::StrongCountTracksLiveReferences;
    pub(super) use crate::WeakUpgradeReturnsNone;
}
#[cfg(kani)]
use mirror::{
    Cell, DerefReflectsTheStoredValue, Ensures, StrongCountTracksLiveReferences,
    WeakUpgradeReturnsNone,
};

impl KaniWitness for RustStdStandard<Arc<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_arc_strong_count_tracks_clones".to_owned(),
            VERIFY_ARC_STRONG_COUNT_TRACKS_CLONES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Arc<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Arc<i32>>",
        "kani",
        || <RustStdStandard<Arc<i32>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_ARC_STRONG_COUNT_TRACKS_CLONES_SRC, {
        /// Same reference-counting contract as `Rc`, for the
        /// thread-safe carrier: `Arc` derefs to its wrapped value, and
        /// `strong_count` increments on `clone` and decrements again
        /// once the clone is dropped. Checked with `i32` for the
        /// count/deref claim, and separately with a drop-instrumented,
        /// non-`Copy` witness type to confirm the wrapped value is
        /// dropped exactly once, only when the *last* strong reference
        /// drops.
        #[kani::proof]
        fn verify_arc_strong_count_tracks_clones() {
            let value: i32 = kani::any();
            let arc = Arc::new(value);
            assert!(
                DerefReflectsTheStoredValue::ensures((*arc, value)),
                "deref exposes the wrapped value"
            );
            assert!(
                StrongCountTracksLiveReferences::ensures((Arc::strong_count(&arc), 1)),
                "a fresh Arc has strong_count 1"
            );

            let arc2 = Arc::clone(&arc);
            assert!(
                StrongCountTracksLiveReferences::ensures((Arc::strong_count(&arc), 2)),
                "clone increments strong_count"
            );
            drop(arc2);
            assert!(
                StrongCountTracksLiveReferences::ensures((Arc::strong_count(&arc), 1)),
                "dropping the clone decrements strong_count back"
            );

            struct DropWitness {
                drop_count: std::rc::Rc<Cell<u32>>,
            }
            impl Drop for DropWitness {
                fn drop(&mut self) {
                    self.drop_count.set(self.drop_count.get() + 1);
                }
            }

            let drop_count = std::rc::Rc::new(Cell::new(0));
            let witness = Arc::new(DropWitness { drop_count: drop_count.clone() });
            let witness2 = Arc::clone(&witness);
            drop(witness2);
            assert!(
                RustStdStandard::<Cell<u32>>::ensures((drop_count.get(), 0)),
                "the value survives dropping one of two strong refs"
            );
            drop(witness);
            assert!(
                RustStdStandard::<Cell<u32>>::ensures((drop_count.get(), 1)),
                "the value drops exactly once, when the last strong ref drops"
            );
        }
    }
}

// Written as the fully-qualified `std::sync::Weak<i32>` throughout, not a
// bare/aliased `Weak`: `alloc::rc::Weak` shares the same bare name, and
// this qualification is what lets `amenable_std`'s matching evidence
// string (see `alloc_sync.rs`'s own registration comment) disambiguate the
// two for tooling reading the registry (e.g. `elicit_doc`'s coverage
// report).
impl KaniWitness for RustStdStandard<std::sync::Weak<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_arc_weak_upgrade_fails_once_the_strong_count_hits_zero".to_owned(),
            VERIFY_ARC_WEAK_UPGRADE_FAILS_ONCE_THE_STRONG_COUNT_HITS_ZERO_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::sync::Weak<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::sync::Weak<i32>>",
        "kani",
        || <RustStdStandard<std::sync::Weak<i32>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_ARC_WEAK_UPGRADE_FAILS_ONCE_THE_STRONG_COUNT_HITS_ZERO_SRC, {
        /// Same upgrade contract as `Rc`'s `Weak`: succeeds while a
        /// strong reference is alive, fails once the last one is
        /// dropped. Also
        /// checked with a drop-instrumented witness type: the wrapped
        /// value drops exactly once the last strong reference drops,
        /// even though a `Weak` to it still exists.
        #[kani::proof]
        fn verify_arc_weak_upgrade_fails_once_the_strong_count_hits_zero() {
            let value: i32 = kani::any();
            let arc = Arc::new(value);
            let weak = Arc::downgrade(&arc);
            assert!(
                RustStdStandard::<usize>::ensures((Arc::weak_count(&arc), 1)),
                "downgrade increments weak_count"
            );
            let upgraded = weak
                .upgrade()
                .expect("upgrade succeeds while a strong reference is alive");
            assert!(
                DerefReflectsTheStoredValue::ensures((*upgraded, value)),
                "an upgraded Weak exposes the original value"
            );
            drop(upgraded);
            assert!(
                StrongCountTracksLiveReferences::ensures((Arc::strong_count(&arc), 1)),
                "dropping the upgraded Arc restores the original strong count"
            );

            drop(arc);
            assert!(
                WeakUpgradeReturnsNone::ensures(weak.upgrade().is_none()),
                "upgrade fails once all strong references are dropped"
            );

            struct DropWitness {
                drop_count: std::rc::Rc<Cell<u32>>,
            }
            impl Drop for DropWitness {
                fn drop(&mut self) {
                    self.drop_count.set(self.drop_count.get() + 1);
                }
            }

            let drop_count = std::rc::Rc::new(Cell::new(0));
            let witness = Arc::new(DropWitness { drop_count: drop_count.clone() });
            let weak_witness = Arc::downgrade(&witness);
            drop(witness);
            assert!(
                RustStdStandard::<Cell<u32>>::ensures((drop_count.get(), 1)),
                "the value drops once the last strong ref drops, though a Weak to it still exists"
            );
            assert!(WeakUpgradeReturnsNone::ensures(weak_witness.upgrade().is_none()));
        }
    }
}
