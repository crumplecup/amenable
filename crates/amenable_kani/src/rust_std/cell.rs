//! `KaniWitness` impls for `core::cell`.
//!
//! `UnsafeCell`'s only raw accessor (`.get()`, returning `*mut T`) needs an
//! `unsafe` block to dereference, and this crate forbids unsafe code
//! (`#![forbid(unsafe_code)]` in `lib.rs`) — so its harness sticks to the
//! safe accessors (`get_mut`, `into_inner`) instead.

use std::cell::{BorrowError, BorrowMutError, Cell, LazyCell, OnceCell, RefCell, UnsafeCell};

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
#[cfg(kani)]
use crate::DerefReflectsTheStoredValue;
#[cfg(kani)]
use crate::IteratorYieldsNoneWhenExhausted;
use crate::KaniWitness;
use crate::rust_std::macros::{bridge_kani_witness, impl_kani_witness_trusted, kani_ensures};

impl KaniWitness for RustStdStandard<Cell<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_cell_get_set_replace_take_round_trip".to_owned(),
            VERIFY_CELL_GET_SET_REPLACE_TAKE_ROUND_TRIP_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Cell<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Cell<i32>>",
        "kani",
        || <RustStdStandard<Cell<i32>> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<Cell<i32>>,
    "amenable_std::rust_std::RustStdStandard<Cell<i32>>",
    (i32, i32),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_CELL_GET_SET_REPLACE_TAKE_ROUND_TRIP_SRC, {
        /// `Cell`'s whole interface is get/set-by-value: `new` stores the
        /// initial value, `set` overwrites it, `replace` overwrites it and
        /// hands back the old value, and `take` (needs `T: Default`) does
        /// the same against `T::default()`. The first, second, and last
        /// assertions call `RustStdStandard::<Cell<i32>>::ensures`
        /// directly rather than restating the comparison.
        #[kani::proof]
        fn verify_cell_get_set_replace_take_round_trip() {
            let initial: i32 = kani::any();
            let cell = Cell::new(initial);
            assert!(
                <RustStdStandard<Cell<i32>> as Ensures<crate::KaniVerifier>>::ensures((cell.get(), initial)),
                "new stores the initial value"
            );

            let updated: i32 = kani::any();
            cell.set(updated);
            assert!(
                <RustStdStandard<Cell<i32>> as Ensures<crate::KaniVerifier>>::ensures((cell.get(), updated)),
                "set overwrites the stored value"
            );

            let replacement: i32 = kani::any();
            let old = cell.replace(replacement);
            assert!(
                <RustStdStandard<Cell<i32>> as Ensures<crate::KaniVerifier>>::ensures((old, updated)),
                "replace returns the previous value"
            );
            assert!(
                <RustStdStandard<Cell<i32>> as Ensures<crate::KaniVerifier>>::ensures((cell.get(), replacement)),
                "replace stores the new value"
            );

            let taken = cell.take();
            assert!(
                <RustStdStandard<Cell<i32>> as Ensures<crate::KaniVerifier>>::ensures((taken, replacement)),
                "take returns the stored value"
            );
            assert_eq!(
                cell.get(),
                i32::default(),
                "take leaves the default value behind"
            );
        }
    }
}

/// [`RustStdStandard<Cell<u32>>`] reuses [`RustStdStandard<Cell<i32>>`]'s
/// harness rather than adding a new Kani proof — every proof across this
/// crate that counts drops via a `Cell<u32>` witness relies on exactly
/// the get/set round-trip `verify_cell_get_set_replace_take_round_trip`
/// already checks for `i32`, just at a different scalar width.
impl KaniWitness for RustStdStandard<Cell<u32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_cell_get_set_replace_take_round_trip".to_owned(),
            VERIFY_CELL_GET_SET_REPLACE_TAKE_ROUND_TRIP_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Cell<u32>>);

kani_ensures!(
    RustStdStandard<Cell<u32>>,
    "amenable_std::rust_std::RustStdStandard<Cell<u32>>",
    (u32, u32),
    |(actual, expected)| actual == expected
);

/// [`RustStdStandard<Cell<usize>>`] reuses the same harness for the same
/// reason [`RustStdStandard<Cell<u32>>`] does — a call-counter witness
/// relies on the identical get/set round-trip, at `usize` width.
impl KaniWitness for RustStdStandard<Cell<usize>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_cell_get_set_replace_take_round_trip".to_owned(),
            VERIFY_CELL_GET_SET_REPLACE_TAKE_ROUND_TRIP_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Cell<usize>>);

kani_ensures!(
    RustStdStandard<Cell<usize>>,
    "amenable_std::rust_std::RustStdStandard<Cell<usize>>",
    (usize, usize),
    |(actual, expected)| actual == expected
);

impl KaniWitness for RustStdStandard<RefCell<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_ref_cell_dynamic_borrow_rules".to_owned(),
            VERIFY_REF_CELL_DYNAMIC_BORROW_RULES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<RefCell<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<RefCell<i32>>",
        "kani",
        || <RustStdStandard<RefCell<i32>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_REF_CELL_DYNAMIC_BORROW_RULES_SRC, {
        /// `RefCell`'s defining behavior is the dynamically checked borrow
        /// rule it layers over `Cell`'s get/set semantics: a mutable
        /// borrow is rejected while a shared borrow is alive, and
        /// permitted again once that borrow is dropped. This is the
        /// invariant `Ref`/`RefMut`'s own harnesses take for granted, so
        /// it gets checked here rather than restated per guard type.
        #[kani::proof]
        fn verify_ref_cell_dynamic_borrow_rules() {
            let initial: i32 = kani::any();
            let cell = RefCell::new(initial);

            {
                let borrow = cell.borrow();
                assert!(
                    DerefReflectsTheStoredValue::ensures((*borrow, initial)),
                    "borrow reads the stored value"
                );
                assert!(
                    cell.try_borrow_mut().is_err(),
                    "mutable borrow rejected while a shared borrow is live"
                );
            }
            assert!(
                cell.try_borrow_mut().is_ok(),
                "mutable borrow allowed once the shared borrow is dropped"
            );

            let updated: i32 = kani::any();
            {
                let mut borrow = cell.borrow_mut();
                assert!(
                    cell.try_borrow().is_err(),
                    "shared borrow rejected while a mutable borrow is live"
                );
                assert!(
                    cell.try_borrow_mut().is_err(),
                    "a second mutable borrow is rejected while the first is live"
                );
                *borrow = updated;
            }
            assert!(
                DerefReflectsTheStoredValue::ensures((*cell.borrow(), updated)),
                "borrow_mut's write is visible to a later borrow"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<std::cell::Ref<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_ref_derefs_to_the_borrowed_value".to_owned(),
            VERIFY_REF_DEREFS_TO_THE_BORROWED_VALUE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::cell::Ref<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Ref<'static, i32>>",
        "kani",
        || <RustStdStandard<std::cell::Ref<'static, i32>> as KaniWitness>::proof()
            .to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_REF_DEREFS_TO_THE_BORROWED_VALUE_SRC, {
        /// `Ref`'s only job is `Deref` onto the `RefCell`'s contents. The
        /// borrow here is not `'static` — the claim holds uniformly over
        /// every lifetime, so a local `RefCell` is enough to check it.
        #[kani::proof]
        fn verify_ref_derefs_to_the_borrowed_value() {
            let value: i32 = kani::any();
            let cell = RefCell::new(value);
            let borrow = cell.borrow();
            assert!(
                DerefReflectsTheStoredValue::ensures((*borrow, value)),
                "Ref derefs to the RefCell's stored value"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<std::cell::RefMut<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_ref_mut_derefs_and_writes_through_to_the_cell".to_owned(),
            VERIFY_REF_MUT_DEREFS_AND_WRITES_THROUGH_TO_THE_CELL_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::cell::RefMut<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<RefMut<'static, i32>>",
        "kani",
        || <RustStdStandard<std::cell::RefMut<'static, i32>> as KaniWitness>::proof()
            .to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_REF_MUT_DEREFS_AND_WRITES_THROUGH_TO_THE_CELL_SRC, {
        /// `RefMut` derefs to the `RefCell`'s contents, and a write
        /// through that deref is visible once the guard is dropped and
        /// the cell is borrowed again.
        #[kani::proof]
        fn verify_ref_mut_derefs_and_writes_through_to_the_cell() {
            let initial: i32 = kani::any();
            let cell = RefCell::new(initial);
            let updated: i32 = kani::any();
            {
                let mut borrow = cell.borrow_mut();
                assert!(
                    DerefReflectsTheStoredValue::ensures((*borrow, initial)),
                    "RefMut derefs to the RefCell's stored value"
                );
                *borrow = updated;
            }
            assert!(
                DerefReflectsTheStoredValue::ensures((*cell.borrow(), updated)),
                "RefMut's write through deref_mut is visible after the borrow ends"
            );
        }
    }
}

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
            assert!(cell.set(value).is_ok(), "the first set succeeds");
            assert!(
                GetterRecoversTheStoredReference::ensures((cell.get(), Some(&value))),
                "get returns the set value"
            );

            let other: i32 = kani::any();
            assert!(cell.set(other).is_err(), "a second set is rejected");
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
            assert_eq!(*cell.get_mut(), initial, "get_mut exposes the stored value");

            let updated: i32 = kani::any();
            *cell.get_mut() = updated;
            assert_eq!(
                cell.into_inner(),
                updated,
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

impl_kani_witness_trusted!(BorrowError, BorrowMutError);
