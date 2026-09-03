//! `AtomicBool`'s `KaniWitness` impl, plus the generic
//! [`AtomicLoadReflectsTheLastWrite`] contract type every `Atomic*` harness
//! in this module reuses. It lives here because its own `proof()` cites
//! `verify_atomic_bool` as its representative harness.

use std::sync::atomic::AtomicBool;

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use crate::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::bridge_kani_witness;

impl KaniWitness for RustStdStandard<AtomicBool> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_atomic_bool".to_owned(),
            VERIFY_ATOMIC_BOOL_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<AtomicBool>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<AtomicBool>",
        "kani",
        || <RustStdStandard<AtomicBool> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_ATOMIC_BOOL_SRC, {
        /// `AtomicBool::new` sets the value observable via `load`, and
        /// `store` overwrites it, both under `SeqCst` ordering. Both
        /// assertions call `AtomicLoadReflectsTheLastWrite::ensures`
        /// directly rather than restating the comparison -- see that
        /// type for why this is the one harness its registration reuses
        /// as a witness.
        #[kani::proof]
        fn verify_atomic_bool() {
            let initial: bool = kani::any();
            let atomic = AtomicBool::new(initial);
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    atomic.load(std::sync::atomic::Ordering::SeqCst),
                    initial
                )),
                "AtomicBool::new sets the value observable via load"
            );

            let next: bool = kani::any();
            atomic.store(next, std::sync::atomic::Ordering::SeqCst);
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    atomic.load(std::sync::atomic::Ordering::SeqCst),
                    next
                )),
                "AtomicBool::store overwrites the value observable via load"
            );
        }
    }
}

/// A `(loaded, expected)` pair known to agree: an atomic's `.load()`
/// reflects the value most recently established for it, by `new`,
/// `store`, `swap`, `compare_exchange`, or `fetch_add`.
///
/// Independently hand-written as `assert_eq!(atomic.load(Ordering::SeqCst),
/// expected, ...)` at 29 real sites across every `Atomic*` integer/bool
/// type in this module plus two call-counter sites elsewhere
/// (`rust_std::iter::verify_repeat_with_calls_its_closure_once_per_item`,
/// `rust_std::sync_lock::verify_once_runs_its_closure_exactly_once`) --
/// the identical claim regardless of the atomic's value type. Generic
/// over that value type rather than one registration per `Atomic*` type,
/// the same reasoning (and the same reason it needs a hand-written
/// `Witness`/`Ensures` impl instead of the
/// `bridge_kani_witness!`/`kani_ensures!` macros) as
/// `IteratorYieldsNoneWhenExhausted` in `rust_std::iter`.
///
/// `AtomicPtr<i32>`'s own `RustStdStandard<AtomicPtr<i32>>` carrier
/// already has a *different* `Ensures<KaniVerifier>` bound occupying its
/// slot (`.swap()` returning the previous value, in the `pointer`
/// submodule) -- a second, distinct claim about the same type needs its
/// own type regardless, per the associated-type-uniqueness rule, so
/// `AtomicPtr`'s four `.load()` sites could never have used a per-carrier
/// registration even if every other `Atomic*` type had.
pub struct AtomicLoadReflectsTheLastWrite<T>(std::marker::PhantomData<T>);

impl<T> amenable_core::Standard for AtomicLoadReflectsTheLastWrite<T> {
    type Provenance = amenable_std::RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn provenance(&self) -> Self::Provenance {
        <i32 as amenable_std::RustStdType>::provenance()
    }
}

impl<T> Evidence for AtomicLoadReflectsTheLastWrite<T> {
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

impl<T> KaniWitness for AtomicLoadReflectsTheLastWrite<T> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_atomic_bool".to_owned(),
            VERIFY_ATOMIC_BOOL_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

impl<T> amenable_core::Witness<crate::KaniVerifier> for AtomicLoadReflectsTheLastWrite<T> {
    type SupportingEvidence = <Self as KaniWitness>::SupportingEvidence;
    type ProofArtifact = <Self as KaniWitness>::ProofArtifact;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        <Self as KaniWitness>::proof()
    }
}

impl<T: PartialEq> amenable_core::Ensures<crate::KaniVerifier>
    for AtomicLoadReflectsTheLastWrite<T>
{
    type Input = (T, T);
    type Bound = bool;

    fn ensures((loaded, expected): (T, T)) -> bool {
        loaded == expected
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_kani::AtomicLoadReflectsTheLastWrite",
        "kani",
        "ensures",
        || stringify!(loaded == expected),
    )
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::AtomicLoadReflectsTheLastWrite",
        "kani",
        || <AtomicLoadReflectsTheLastWrite<i32> as KaniWitness>::proof().to_string(),
    )
}
