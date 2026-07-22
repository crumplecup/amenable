//! `KaniWitness` impls for `alloc::rc`.

use std::rc::Rc;

use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::bridge_kani_witness;

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
            assert_eq!(*rc, value, "deref exposes the wrapped value");
            assert_eq!(Rc::strong_count(&rc), 1, "a fresh Rc has strong_count 1");

            let rc2 = Rc::clone(&rc);
            assert_eq!(Rc::strong_count(&rc), 2, "clone increments strong_count");
            drop(rc2);
            assert_eq!(
                Rc::strong_count(&rc),
                1,
                "dropping the clone decrements strong_count back"
            );

            struct DropWitness {
                drop_count: std::rc::Rc<std::cell::Cell<u32>>,
            }
            impl Drop for DropWitness {
                fn drop(&mut self) {
                    self.drop_count.set(self.drop_count.get() + 1);
                }
            }

            let drop_count = std::rc::Rc::new(std::cell::Cell::new(0));
            let witness = Rc::new(DropWitness { drop_count: drop_count.clone() });
            let witness2 = Rc::clone(&witness);
            drop(witness2);
            assert_eq!(drop_count.get(), 0, "the value survives dropping one of two strong refs");
            drop(witness);
            assert_eq!(drop_count.get(), 1, "the value drops exactly once, when the last strong ref drops");
        }
    }
}

// Written as the fully-qualified `std::rc::Weak<i32>` throughout, not a
// bare/aliased `Weak`: `alloc::sync::Weak` shares the same bare name, and
// this qualification is what lets `amenable_std`'s matching evidence
// string (see `alloc_rc.rs`'s own registration comment) disambiguate the
// two for tooling reading the registry (e.g. `elicit_doc`'s coverage
// report).
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
            assert_eq!(*upgraded, value, "an upgraded Weak exposes the original value");
            drop(upgraded);
            assert_eq!(
                Rc::strong_count(&rc),
                1,
                "dropping the upgraded Rc restores the original strong count"
            );

            drop(rc);
            assert!(
                weak.upgrade().is_none(),
                "upgrade fails once all strong references are dropped"
            );

            struct DropWitness {
                drop_count: std::rc::Rc<std::cell::Cell<u32>>,
            }
            impl Drop for DropWitness {
                fn drop(&mut self) {
                    self.drop_count.set(self.drop_count.get() + 1);
                }
            }

            let drop_count = std::rc::Rc::new(std::cell::Cell::new(0));
            let witness = Rc::new(DropWitness { drop_count: drop_count.clone() });
            let weak_witness = Rc::downgrade(&witness);
            drop(witness);
            assert_eq!(
                drop_count.get(),
                1,
                "the value drops once the last strong ref drops, though a Weak to it still exists"
            );
            assert!(weak_witness.upgrade().is_none());
        }
    }
}
