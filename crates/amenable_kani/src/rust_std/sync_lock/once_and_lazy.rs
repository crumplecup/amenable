use std::sync::{LazyLock, OnceLock, OnceState};

use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use crate::KaniWitness;
use crate::rust_std::CheckedProof;
use crate::rust_std::macros::{bridge_kani_witness, kani_ensures};

/// The `#[cfg(kani)]` imports this file needs, consolidated into one gate
/// on this `mod` instead of one per item -- see
/// `amenable_creusot::stoplight::mirror`'s own doc comment for the
/// general rationale. Every name is re-exported: the `harness! { .. }`
/// blocks below need all of them, unqualified, at this file's own top
/// level.
#[cfg(kani)]
mod mirror {
    pub(super) use amenable_core::Ensures;

    pub(super) use crate::AtomicLoadReflectsTheLastWrite;
    pub(super) use crate::FallibleOperationReportsFailure;
    pub(super) use crate::FallibleOperationReportsSuccess;
    pub(super) use crate::GetterRecoversTheStoredReference;
    pub(super) use crate::IteratorYieldsNoneWhenExhausted;
}
#[cfg(kani)]
use mirror::{
    AtomicLoadReflectsTheLastWrite, Ensures, FallibleOperationReportsFailure,
    FallibleOperationReportsSuccess, GetterRecoversTheStoredReference,
    IteratorYieldsNoneWhenExhausted,
};

impl KaniWitness for RustStdStandard<std::sync::Once> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_once_runs_its_closure_exactly_once".to_owned(),
            VERIFY_ONCE_RUNS_ITS_CLOSURE_EXACTLY_ONCE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::sync::Once>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::sync::Once>",
        "kani",
        || <RustStdStandard<std::sync::Once> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_ONCE_RUNS_ITS_CLOSURE_EXACTLY_ONCE_SRC, {
        /// `.call_once()` runs its closure the first time; a second
        /// call is a no-op, observed through a shared counter.
        #[kani::proof]
        fn verify_once_runs_its_closure_exactly_once() {
            static CALLS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
            let once = std::sync::Once::new();
            once.call_once(|| {
                CALLS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            });
            once.call_once(|| {
                CALLS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            });
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    CALLS.load(std::sync::atomic::Ordering::SeqCst),
                    1
                )),
                "call_once runs its closure exactly once"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<OnceState> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_once_state_reports_not_poisoned_on_a_clean_run".to_owned(),
            VERIFY_ONCE_STATE_REPORTS_NOT_POISONED_ON_A_CLEAN_RUN_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<OnceState>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<OnceState>",
        "kani",
        || <RustStdStandard<OnceState> as KaniWitness>::proof().to_string(),
    )
}

/// A `bool` known to be the `true` `OnceState::is_poisoned()` reports
/// when the `Once` really was poisoned by a panicking closure --
/// following `EmptiedContainerReportsEmpty`'s established shape for a
/// raw boolean claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, amenable_derive::Standard)]
#[standard(
    basis = "RustStdStandard<i32>",
    basis_ctor = "RustStdStandard::<i32>::new()",
    provenance = "<i32 as amenable_std::RustStdType>::provenance()",
    provenance_type = "amenable_std::RustStdProvenance"
)]
pub struct OnceStateIsPoisonedReportsTrue;

impl KaniWitness for OnceStateIsPoisonedReportsTrue {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_once_state_reports_not_poisoned_on_a_clean_run".to_owned(),
            VERIFY_ONCE_STATE_REPORTS_NOT_POISONED_ON_A_CLEAN_RUN_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(OnceStateIsPoisonedReportsTrue);

kani_ensures!(
    OnceStateIsPoisonedReportsTrue,
    "amenable_kani::OnceStateIsPoisonedReportsTrue",
    bool,
    |is_poisoned| is_poisoned
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::OnceStateIsPoisonedReportsTrue",
        "kani",
        || <OnceStateIsPoisonedReportsTrue as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_ONCE_STATE_REPORTS_NOT_POISONED_ON_A_CLEAN_RUN_SRC, {
        /// `.call_once_force()` hands its closure an `OnceState`
        /// reporting `is_poisoned() == false` on a clean (never-
        /// panicked) `Once`.
        #[kani::proof]
        fn verify_once_state_reports_not_poisoned_on_a_clean_run() {
            let once = std::sync::Once::new();
            once.call_once_force(|state| {
                assert!(
                    !OnceStateIsPoisonedReportsTrue::ensures(state.is_poisoned()),
                    "a clean Once reports not poisoned"
                );
            });
        }
    }
}

impl KaniWitness for RustStdStandard<OnceLock<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_once_lock_initializes_exactly_once".to_owned(),
            VERIFY_ONCE_LOCK_INITIALIZES_EXACTLY_ONCE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<OnceLock<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<OnceLock<i32>>",
        "kani",
        || <RustStdStandard<OnceLock<i32>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_ONCE_LOCK_INITIALIZES_EXACTLY_ONCE_SRC, {
        /// Same exactly-once contract as `core::cell::OnceCell`, for
        /// the thread-safe carrier: empty, first `set` succeeds, a
        /// second `set` is rejected without disturbing the value.
        #[kani::proof]
        fn verify_once_lock_initializes_exactly_once() {
            let cell: OnceLock<i32> = OnceLock::new();
            assert!(IteratorYieldsNoneWhenExhausted::ensures(cell.get()));

            let value: i32 = kani::any();
            assert!(
                FallibleOperationReportsSuccess::ensures(cell.set(value).is_ok()),
                "the first set succeeds"
            );
            assert!(GetterRecoversTheStoredReference::ensures((cell.get(), Some(&value))));

            let other: i32 = kani::any();
            assert!(
                FallibleOperationReportsFailure::ensures(cell.set(other).is_err()),
                "a second set is rejected"
            );
            assert!(
                GetterRecoversTheStoredReference::ensures((cell.get(), Some(&value))),
                "the original value survives a rejected second set"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<LazyLock<i32, fn() -> i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_lazy_lock_caches_its_initializer_result".to_owned(),
            VERIFY_LAZY_LOCK_CACHES_ITS_INITIALIZER_RESULT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<LazyLock<i32, fn() -> i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<LazyLock<i32, fn() -> i32>>",
        "kani",
        || <RustStdStandard<LazyLock<i32, fn() -> i32>> as KaniWitness>::proof()
            .to_string(),
    )
}

kani_ensures!(
    RustStdStandard<LazyLock<i32, fn() -> i32>>,
    "amenable_std::rust_std::RustStdStandard<LazyLock<i32, fn() -> i32>>",
    (i32, i32),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_LAZY_LOCK_CACHES_ITS_INITIALIZER_RESULT_SRC, {
        /// Same caching proof technique as `core::cell::LazyCell`:
        /// `kani::any()` inside the initializer means a re-invoked
        /// closure would force two independently arbitrary results, so
        /// the two derefs being forced equal is exactly what "ran once,
        /// cached" means.
        #[kani::proof]
        fn verify_lazy_lock_caches_its_initializer_result() {
            fn init() -> i32 {
                kani::any()
            }
            let lazy: LazyLock<i32, fn() -> i32> = LazyLock::new(init);
            let first = *lazy;
            let second = *lazy;
            assert!(
                RustStdStandard::<LazyLock<i32, fn() -> i32>>::ensures((first, second)),
                "LazyLock caches its initializer's result across derefs"
            );
        }
    }
}
