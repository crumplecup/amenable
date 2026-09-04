//! The specialized cells -- `OnceCell`, `UnsafeCell`, `LazyCell` -- and the
//! generic `GetterRecoversTheStoredReference` contract type (shared with
//! `sync_lock::once_and_lazy`'s `OnceLock`/`LazyLock` harnesses; its
//! `proof()` cites `verify_once_cell_initializes_exactly_once`).

use std::cell::{LazyCell, OnceCell, UnsafeCell};

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use crate::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::{bridge_kani_witness, kani_ensures};
#[cfg(kani)]
use crate::{
    AccessorRecoversTheExpectedValue, DerefReflectsTheStoredValue, FallibleOperationReportsFailure,
    FallibleOperationReportsSuccess, IteratorYieldsNoneWhenExhausted,
};

/// An `(actual, expected)` pair of `.get()` results known to agree: a
/// once-initialized cell's getter yields a reference to the exact
/// value it was set with.
///
/// Independently hand-written as `assert_eq!(cell.get(), Some(&value),
/// ...)` at 4 real sites split between `OnceCell` and `OnceLock` --
/// the identical claim regardless of the single-/multi-threaded
/// carrier. A distinct access pattern from
/// `IteratorYieldsAReferenceToTheStoredValue` even though the
/// `Ensures` impl body and the lifetime-generic design are identical:
/// that type's own name and doc comment are specifically about
/// iteration (`.next()`), not a plain getter -- same reasoning as
/// keeping `FieldAccessRecoversTheStoredValue` separate from
/// `IndexRecoversTheStoredElement` despite type-level overlap.
pub struct GetterRecoversTheStoredReference<T>(std::marker::PhantomData<T>);

impl<T> amenable_core::Standard for GetterRecoversTheStoredReference<T> {
    type Provenance = amenable_std::RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn provenance(&self) -> Self::Provenance {
        <i32 as amenable_std::RustStdType>::provenance()
    }
}

impl<T> Evidence for GetterRecoversTheStoredReference<T> {
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

impl<T> KaniWitness for GetterRecoversTheStoredReference<T> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_once_cell_initializes_exactly_once".to_owned(),
            VERIFY_ONCE_CELL_INITIALIZES_EXACTLY_ONCE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

impl<T> amenable_core::Witness<crate::KaniVerifier> for GetterRecoversTheStoredReference<T> {
    type SupportingEvidence = <Self as KaniWitness>::SupportingEvidence;
    type ProofArtifact = <Self as KaniWitness>::ProofArtifact;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        <Self as KaniWitness>::proof()
    }
}

impl<T: PartialEq> amenable_core::Ensures<crate::KaniVerifier>
    for GetterRecoversTheStoredReference<T>
{
    type Input = (T, T);
    type Bound = bool;

    fn ensures((actual, expected): (T, T)) -> bool {
        actual == expected
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_kani::GetterRecoversTheStoredReference",
        "kani",
        "ensures",
        || stringify!(actual == expected),
    )
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::GetterRecoversTheStoredReference",
        "kani",
        || <GetterRecoversTheStoredReference<i32> as KaniWitness>::proof().to_string(),
    )
}

impl KaniWitness for RustStdStandard<OnceCell<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_once_cell_initializes_exactly_once".to_owned(),
            VERIFY_ONCE_CELL_INITIALIZES_EXACTLY_ONCE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<OnceCell<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<OnceCell<i32>>",
        "kani",
        || <RustStdStandard<OnceCell<i32>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_ONCE_CELL_INITIALIZES_EXACTLY_ONCE_SRC, {
        /// `OnceCell` accepts exactly one `set`: a fresh cell has no
        /// value, the first `set` succeeds and is immediately visible
        /// through `get`, and a second `set` is rejected without
        /// disturbing the value the first one stored.
        #[kani::proof]
        fn verify_once_cell_initializes_exactly_once() {
            let cell: OnceCell<i32> = OnceCell::new();
            assert!(
                IteratorYieldsNoneWhenExhausted::ensures(cell.get()),
                "a fresh OnceCell has no value"
            );

            let value: i32 = kani::any();
            assert!(
                FallibleOperationReportsSuccess::ensures(cell.set(value).is_ok()),
                "the first set succeeds"
            );
            assert!(
                GetterRecoversTheStoredReference::ensures((cell.get(), Some(&value))),
                "get returns the set value"
            );

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

impl KaniWitness for RustStdStandard<UnsafeCell<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_unsafe_cell_get_mut_and_into_inner_round_trip".to_owned(),
            VERIFY_UNSAFE_CELL_GET_MUT_AND_INTO_INNER_ROUND_TRIP_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<UnsafeCell<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<UnsafeCell<i32>>",
        "kani",
        || <RustStdStandard<UnsafeCell<i32>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_UNSAFE_CELL_GET_MUT_AND_INTO_INNER_ROUND_TRIP_SRC, {
        /// `UnsafeCell`'s raw `.get()` returns `*mut T`, which needs
        /// `unsafe` to dereference — this crate forbids unsafe code, so
        /// this harness only exercises the safe accessors: `get_mut`
        /// (sound because `&mut self` already excludes aliasing) and
        /// `into_inner`.
        #[kani::proof]
        fn verify_unsafe_cell_get_mut_and_into_inner_round_trip() {
            let initial: i32 = kani::any();
            let mut cell = UnsafeCell::new(initial);
            assert!(
                DerefReflectsTheStoredValue::ensures((*cell.get_mut(), initial)),
                "get_mut exposes the stored value"
            );

            let updated: i32 = kani::any();
            *cell.get_mut() = updated;
            assert!(
                AccessorRecoversTheExpectedValue::ensures((cell.into_inner(), updated)),
                "into_inner returns the current value"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<LazyCell<i32, fn() -> i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_lazy_cell_caches_its_initializer_result".to_owned(),
            VERIFY_LAZY_CELL_CACHES_ITS_INITIALIZER_RESULT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<LazyCell<i32, fn() -> i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<LazyCell<i32, fn() -> i32>>",
        "kani",
        || <RustStdStandard<LazyCell<i32, fn() -> i32>> as KaniWitness>::proof()
            .to_string(),
    )
}

kani_ensures!(
    RustStdStandard<LazyCell<i32, fn() -> i32>>,
    "amenable_std::rust_std::RustStdStandard<LazyCell<i32, fn() -> i32>>",
    (i32, i32),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_LAZY_CELL_CACHES_ITS_INITIALIZER_RESULT_SRC, {
        /// `LazyCell` runs its initializer at most once. Rather than an
        /// explicit call counter, this exploits the nondeterminism of
        /// `kani::any()` inside the initializer: if `LazyCell` ever
        /// re-invoked it on a later deref, the two derefs would each
        /// force an independently arbitrary value, and Kani would find a
        /// counterexample where `first != second`. The equality holding
        /// for every possible input is exactly what "ran once, cached"
        /// means.
        #[kani::proof]
        fn verify_lazy_cell_caches_its_initializer_result() {
            fn init() -> i32 {
                kani::any()
            }
            let lazy: LazyCell<i32, fn() -> i32> = LazyCell::new(init);
            let first = *lazy;
            let second = *lazy;
            assert!(
                RustStdStandard::<LazyCell<i32, fn() -> i32>>::ensures((first, second)),
                "LazyCell caches its initializer's result across derefs"
            );
        }
    }
}
