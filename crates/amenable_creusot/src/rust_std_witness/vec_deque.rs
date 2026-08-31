use super::CheckedProof;

use std::collections::vec_deque::{
    Drain as VecDequeDrain, IntoIter as VecDequeIntoIter, Iter as VecDequeIter,
    IterMut as VecDequeIterMut,
};
use std::collections::{TryReserveError, VecDeque};

use crate::{
    CreusotVerifier, CreusotWitness, TRY_RESERVE_REJECTS_AN_IMPOSSIBLE_CAPACITY_HOLDS_SRC,
    VEC_DEQUE_ITER_MUT_WRITES_THROUGH_HOLDS_SRC,
    VEC_DEQUE_ITER_YIELDS_REFERENCES_IN_ORDER_HOLDS_SRC,
    VEC_DEQUE_PUSHES_AND_POPS_FROM_BOTH_ENDS_HOLDS_SRC,
    VERIFY_TRY_RESERVE_REJECTS_AN_IMPOSSIBLE_CAPACITY_SRC,
    VERIFY_VEC_DEQUE_DRAIN_REMOVES_AND_YIELDS_IN_ORDER_SRC,
    VERIFY_VEC_DEQUE_INTO_ITER_YIELDS_OWNED_VALUES_IN_ORDER_SRC,
    VERIFY_VEC_DEQUE_ITER_MUT_WRITES_THROUGH_SRC,
    VERIFY_VEC_DEQUE_ITER_YIELDS_REFERENCES_IN_ORDER_SRC,
    VERIFY_VEC_DEQUE_PUSHES_AND_POPS_FROM_BOTH_ENDS_SRC,
};
use amenable_core::{Ensures, Evidence, Witness};

use amenable_std::RustStdStandard;

macro_rules! bridge_creusot_witness {
    ($ty:ty) => {
        impl Witness<CreusotVerifier> for $ty {
            type SupportingEvidence = <$ty as CreusotWitness>::SupportingEvidence;
            type ProofArtifact = <$ty as CreusotWitness>::ProofArtifact;

            fn proof() -> Self::ProofArtifact {
                <$ty as CreusotWitness>::proof()
            }
        }
    };
}
impl CreusotWitness for RustStdStandard<TryReserveError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_try_reserve_rejects_an_impossible_capacity".to_string(),
            VERIFY_TRY_RESERVE_REJECTS_AN_IMPOSSIBLE_CAPACITY_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<TryReserveError>);

/// Returns
/// `amenable_creusot::TRY_RESERVE_REJECTS_AN_IMPOSSIBLE_CAPACITY_HOLDS_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn try_reserve_rejects_an_impossible_capacity_holds`
/// the real site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<TryReserveError> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        TRY_RESERVE_REJECTS_AN_IMPOSSIBLE_CAPACITY_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<TryReserveError>",
        "creusot",
        "ensures",
        || <RustStdStandard<TryReserveError> as Ensures<CreusotVerifier>>::ensures(()),
    )
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<TryReserveError>",
        "creusot",
        || <RustStdStandard<TryReserveError> as CreusotWitness>::proof().to_string(),
    )
}

impl CreusotWitness for RustStdStandard<VecDeque<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_vec_deque_pushes_and_pops_from_both_ends".to_string(),
            VERIFY_VEC_DEQUE_PUSHES_AND_POPS_FROM_BOTH_ENDS_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<VecDeque<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<VecDeque<i32>>",
        "creusot",
        || <RustStdStandard<VecDeque<i32>> as CreusotWitness>::proof().to_string(),
    )
}

/// Returns
/// `amenable_creusot::VEC_DEQUE_PUSHES_AND_POPS_FROM_BOTH_ENDS_HOLDS_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn vec_deque_pushes_and_pops_from_both_ends_holds`
/// the real site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<VecDeque<i32>> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        VEC_DEQUE_PUSHES_AND_POPS_FROM_BOTH_ENDS_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<VecDeque<i32>>",
        "creusot",
        "ensures",
        || <RustStdStandard<VecDeque<i32>> as Ensures<CreusotVerifier>>::ensures(()),
    )
}

impl CreusotWitness for RustStdStandard<VecDequeIntoIter<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_vec_deque_into_iter_yields_owned_values_in_order".to_string(),
            VERIFY_VEC_DEQUE_INTO_ITER_YIELDS_OWNED_VALUES_IN_ORDER_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<VecDequeIntoIter<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::vec_deque::IntoIter<i32>>",
        "creusot",
        || <RustStdStandard<VecDequeIntoIter<i32>> as CreusotWitness>::proof().to_string(),
    )
}

impl CreusotWitness for RustStdStandard<VecDequeDrain<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_vec_deque_drain_removes_and_yields_in_order".to_string(),
            VERIFY_VEC_DEQUE_DRAIN_REMOVES_AND_YIELDS_IN_ORDER_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<VecDequeDrain<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::vec_deque::Drain<'static, i32>>",
        "creusot",
        || <RustStdStandard<VecDequeDrain<'static, i32>> as CreusotWitness>::proof().to_string(),
    )
}

impl CreusotWitness for RustStdStandard<VecDequeIter<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_vec_deque_iter_yields_references_in_order".to_string(),
            VERIFY_VEC_DEQUE_ITER_YIELDS_REFERENCES_IN_ORDER_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<VecDequeIter<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::vec_deque::Iter<'static, i32>>",
        "creusot",
        || <RustStdStandard<VecDequeIter<'static, i32>> as CreusotWitness>::proof().to_string(),
    )
}

/// Returns
/// `amenable_creusot::VEC_DEQUE_ITER_YIELDS_REFERENCES_IN_ORDER_HOLDS_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn vec_deque_iter_yields_references_in_order_holds`
/// the real site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<VecDequeIter<'static, i32>> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        VEC_DEQUE_ITER_YIELDS_REFERENCES_IN_ORDER_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<VecDequeIter<'static, i32>>",
        "creusot",
        "ensures",
        || <RustStdStandard<VecDequeIter<'static, i32>> as Ensures<CreusotVerifier>>::ensures(()),
    )
}

impl CreusotWitness for RustStdStandard<VecDequeIterMut<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_vec_deque_iter_mut_writes_through".to_string(),
            VERIFY_VEC_DEQUE_ITER_MUT_WRITES_THROUGH_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<VecDequeIterMut<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::vec_deque::IterMut<'static, i32>>",
        "creusot",
        || <RustStdStandard<VecDequeIterMut<'static, i32>> as CreusotWitness>::proof().to_string(),
    )
}

/// Returns
/// `amenable_creusot::VEC_DEQUE_ITER_MUT_WRITES_THROUGH_HOLDS_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn vec_deque_iter_mut_writes_through_holds` the
/// real site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<VecDequeIterMut<'static, i32>> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        VEC_DEQUE_ITER_MUT_WRITES_THROUGH_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<VecDequeIterMut<'static, i32>>",
        "creusot",
        "ensures",
        || <RustStdStandard<VecDequeIterMut<'static, i32>> as Ensures<CreusotVerifier>>::ensures(()),
    )
}
