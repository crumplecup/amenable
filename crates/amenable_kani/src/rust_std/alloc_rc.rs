//! `KaniWitness` impls for `alloc::rc`.

use std::rc::Rc;

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_std::RustStdStandard;
#[cfg(kani)]
use std::cell::Cell;

use super::CheckedProof;
#[cfg(kani)]
use crate::DerefReflectsTheStoredValue;
use crate::KaniWitness;
use crate::rust_std::macros::{bridge_kani_witness, kani_ensures};

/// A live-strong-reference count known to match `strong_count()`'s
/// report: `Rc`/`Arc` start at 1, increment on `clone`, and decrement
/// again once a clone drops.
///
/// Independently hand-written as `assert_eq!(Rc::strong_count(&rc),
/// N, ...)` / `assert_eq!(Arc::strong_count(&arc), N, ...)` at 8 real
/// sites split between `rust_std::alloc_rc` and `rust_std::alloc_sync`
/// -- the identical claim regardless of which single-/multi-threaded
/// reference-counted pointer. Needs no type parameter, same "trust the
/// body, name the flag" shape as `EmptiedContainerReportsEmpty`
/// (`rust_std::alloc_collections`): `strong_count()` always returns a
/// plain `usize`, so there's nothing container-type-specific left to
/// be generic over.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, amenable_derive::Standard)]
#[standard(
    basis = "RustStdStandard<usize>",
    basis_ctor = "RustStdStandard::<usize>::new()",
    provenance = "<usize as amenable_std::RustStdType>::provenance()",
    provenance_type = "amenable_std::RustStdProvenance"
)]
pub struct StrongCountTracksLiveReferences;

impl KaniWitness for StrongCountTracksLiveReferences {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_rc_strong_count_tracks_clones".to_owned(),
            claim: VERIFY_RC_STRONG_COUNT_TRACKS_CLONES_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(StrongCountTracksLiveReferences);

kani_ensures!(
    StrongCountTracksLiveReferences,
    "amenable_kani::StrongCountTracksLiveReferences",
    (usize, usize),
    |(actual, expected)| actual == expected
);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_kani::StrongCountTracksLiveReferences",
        verifier: "kani",
        describe: || <StrongCountTracksLiveReferences as KaniWitness>::proof().to_string(),
    }
}

impl KaniWitness for RustStdStandard<Rc<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_rc_strong_count_tracks_clones".to_owned(),
            claim: VERIFY_RC_STRONG_COUNT_TRACKS_CLONES_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Rc<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Rc<i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<Rc<i32>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_RC_STRONG_COUNT_TRACKS_CLONES_SRC, {
        /// `Rc` derefs to its wrapped value, and `strong_count`
        /// increments on `clone` and decrements again once the clone
        /// is dropped. Checked with `i32` for the count/deref claim,
        /// and separately with a drop-instrumented, non-`Copy` witness
        /// type to confirm the wrapped value is dropped exactly once,
        /// only when the *last* strong reference drops — `i32` alone
        /// can't distinguish "dropped once, at the right time" from
        /// "dropped early, late, twice, or leaked".
        #[kani::proof]
        fn verify_rc_strong_count_tracks_clones() {
            let value: i32 = kani::any();
            let rc = Rc::new(value);
            assert!(
                DerefReflectsTheStoredValue::ensures((*rc, value)),
                "deref exposes the wrapped value"
            );
            assert!(
                StrongCountTracksLiveReferences::ensures((Rc::strong_count(&rc), 1)),
                "a fresh Rc has strong_count 1"
            );

            let rc2 = Rc::clone(&rc);
            assert!(
                StrongCountTracksLiveReferences::ensures((Rc::strong_count(&rc), 2)),
                "clone increments strong_count"
            );
            drop(rc2);
            assert!(
                StrongCountTracksLiveReferences::ensures((Rc::strong_count(&rc), 1)),
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
            let witness = Rc::new(DropWitness { drop_count: drop_count.clone() });
            let witness2 = Rc::clone(&witness);
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

// Written as the fully-qualified `std::rc::Weak<i32>` throughout, not a
// bare/aliased `Weak`: `alloc::sync::Weak` shares the same bare name, and
// this qualification is what lets `amenable_std`'s matching evidence
// string (see `alloc_rc.rs`'s own registration comment) disambiguate the
// two for tooling reading the registry (e.g. `elicit_doc`'s coverage
// report).
/// A `Weak::upgrade()` outcome, once computed, known to report `None`:
/// once every strong reference has dropped, upgrading a `Weak` to it
/// can no longer succeed.
///
/// Independently hand-written as `assert!(weak.upgrade().is_none(),
/// ...)` at 4 real sites split between `Rc`'s and `Arc`'s `Weak` --
/// the identical claim regardless of the single-/multi-threaded
/// carrier. Same "trust the body, name the flag" shape as
/// `EmptiedContainerReportsEmpty`/`FallibleOperationReportsFailure`,
/// but a distinct type from the latter: `Option::is_none()` and
/// `Result::is_err()` are different outcome shapes, not the same
/// claim restated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, amenable_derive::Standard)]
#[standard(
    basis = "RustStdStandard<i32>",
    basis_ctor = "RustStdStandard::<i32>::new()",
    provenance = "<i32 as amenable_std::RustStdType>::provenance()",
    provenance_type = "amenable_std::RustStdProvenance"
)]
pub struct WeakUpgradeReturnsNone;

impl KaniWitness for WeakUpgradeReturnsNone {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_rc_weak_upgrade_fails_once_the_strong_count_hits_zero".to_owned(),
            claim: VERIFY_RC_WEAK_UPGRADE_FAILS_ONCE_THE_STRONG_COUNT_HITS_ZERO_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(WeakUpgradeReturnsNone);

kani_ensures!(
    WeakUpgradeReturnsNone,
    "amenable_kani::WeakUpgradeReturnsNone",
    bool,
    |is_none| is_none
);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_kani::WeakUpgradeReturnsNone",
        verifier: "kani",
        describe: || <WeakUpgradeReturnsNone as KaniWitness>::proof().to_string(),
    }
}

impl KaniWitness for RustStdStandard<std::rc::Weak<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_rc_weak_upgrade_fails_once_the_strong_count_hits_zero".to_owned(),
            claim: VERIFY_RC_WEAK_UPGRADE_FAILS_ONCE_THE_STRONG_COUNT_HITS_ZERO_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::rc::Weak<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::rc::Weak<i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<std::rc::Weak<i32>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_RC_WEAK_UPGRADE_FAILS_ONCE_THE_STRONG_COUNT_HITS_ZERO_SRC, {
        /// A `Weak` upgrades successfully while a strong reference is
        /// alive, and fails once the last one is dropped — the
        /// defining behavior that distinguishes it from a strong `Rc`.
        /// Also checked with a drop-instrumented witness type: the
        /// wrapped value drops exactly once the last strong reference
        /// drops, even though a `Weak` to it still exists — a `Weak`
        /// keeps the allocation's control block alive, not the value.
        #[kani::proof]
        fn verify_rc_weak_upgrade_fails_once_the_strong_count_hits_zero() {
            let value: i32 = kani::any();
            let rc = Rc::new(value);
            let weak = Rc::downgrade(&rc);
            assert_eq!(Rc::weak_count(&rc), 1, "downgrade increments weak_count");
            let upgraded = weak
                .upgrade()
                .expect("upgrade succeeds while a strong reference is alive");
            assert!(
                DerefReflectsTheStoredValue::ensures((*upgraded, value)),
                "an upgraded Weak exposes the original value"
            );
            drop(upgraded);
            assert!(
                StrongCountTracksLiveReferences::ensures((Rc::strong_count(&rc), 1)),
                "dropping the upgraded Rc restores the original strong count"
            );

            drop(rc);
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
            let witness = Rc::new(DropWitness { drop_count: drop_count.clone() });
            let weak_witness = Rc::downgrade(&witness);
            drop(witness);
            assert!(
                RustStdStandard::<Cell<u32>>::ensures((drop_count.get(), 1)),
                "the value drops once the last strong ref drops, though a Weak to it still exists"
            );
            assert!(WeakUpgradeReturnsNone::ensures(weak_witness.upgrade().is_none()));
        }
    }
}
