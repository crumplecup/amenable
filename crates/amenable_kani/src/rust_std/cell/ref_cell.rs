//! `RefCell<i32>`, `Ref`, and `RefMut`'s `KaniWitness` impls and harnesses
//! -- the dynamically checked borrow rule and the two guard types' `Deref`
//! behavior -- plus the trusted `BorrowError` / `BorrowMutError` carriers.

use std::cell::{BorrowError, BorrowMutError, RefCell};

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use crate::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::{bridge_kani_witness, impl_kani_witness_trusted};
#[cfg(kani)]
use crate::{
    DerefReflectsTheStoredValue, FallibleOperationReportsFailure, FallibleOperationReportsSuccess,
};

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
                    FallibleOperationReportsFailure::ensures(cell.try_borrow_mut().is_err()),
                    "mutable borrow rejected while a shared borrow is live"
                );
            }
            assert!(
                FallibleOperationReportsSuccess::ensures(cell.try_borrow_mut().is_ok()),
                "mutable borrow allowed once the shared borrow is dropped"
            );

            let updated: i32 = kani::any();
            {
                let mut borrow = cell.borrow_mut();
                assert!(
                    FallibleOperationReportsFailure::ensures(cell.try_borrow().is_err()),
                    "shared borrow rejected while a mutable borrow is live"
                );
                assert!(
                    FallibleOperationReportsFailure::ensures(cell.try_borrow_mut().is_err()),
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

impl_kani_witness_trusted!(BorrowError, BorrowMutError);
