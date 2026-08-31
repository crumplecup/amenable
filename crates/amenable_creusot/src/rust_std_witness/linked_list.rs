use super::CheckedProof;

use std::collections::LinkedList;
use std::collections::linked_list::{
    ExtractIf as LinkedListExtractIf, IntoIter as LinkedListIntoIter, Iter as LinkedListIter,
    IterMut as LinkedListIterMut,
};

use crate::{
    CreusotVerifier, CreusotWitness, DRAINS_TWO_VALUES_IN_ORDER_AND_EMPTIES_SRC,
    LINKED_LIST_EXTRACT_IF_PARTITIONS_BY_THE_PREDICATE_HOLDS_SRC,
    LINKED_LIST_ITER_MUT_WRITES_THROUGH_HOLDS_SRC,
    LINKED_LIST_ITER_YIELDS_REFERENCES_IN_ORDER_HOLDS_SRC,
    VERIFY_LINKED_LIST_EXTRACT_IF_PARTITIONS_BY_THE_PREDICATE_SRC,
    VERIFY_LINKED_LIST_INTO_ITER_YIELDS_OWNED_VALUES_IN_ORDER_SRC,
    VERIFY_LINKED_LIST_IS_FIFO_THROUGH_BACK_AND_FRONT_SRC,
    VERIFY_LINKED_LIST_ITER_MUT_WRITES_THROUGH_SRC,
    VERIFY_LINKED_LIST_ITER_YIELDS_REFERENCES_IN_ORDER_SRC,
    YIELDS_TWO_VALUES_IN_ORDER_THEN_ENDS_SRC,
};
use amenable_core::{Ensures, Evidence, Witness};

use amenable_std::{
    DrainsTwoValuesInOrderAndEmpties, RustStdStandard, YieldsTwoValuesInOrderThenEnds,
};

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
impl CreusotWitness for RustStdStandard<LinkedList<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_linked_list_is_fifo_through_back_and_front".to_string(),
            VERIFY_LINKED_LIST_IS_FIFO_THROUGH_BACK_AND_FRONT_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<LinkedList<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<LinkedList<i32>>",
        "creusot",
        || <RustStdStandard<LinkedList<i32>> as CreusotWitness>::proof().to_string(),
    )
}

/// [`DrainsTwoValuesInOrderAndEmpties`] reuses the `LinkedList` FIFO
/// harness rather than adding a new Creusot proof — it names the
/// postcondition both the `LinkedList` FIFO and `VecDeque::drain`
/// proofs already share, it doesn't prove anything new.
impl CreusotWitness for DrainsTwoValuesInOrderAndEmpties {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_linked_list_is_fifo_through_back_and_front".to_string(),
            VERIFY_LINKED_LIST_IS_FIFO_THROUGH_BACK_AND_FRONT_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(DrainsTwoValuesInOrderAndEmpties);

/// Returns `amenable_creusot::DRAINS_TWO_VALUES_IN_ORDER_AND_EMPTIES_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn drains_two_values_in_order_and_empties` both real
/// sites call, not a hand-retyped copy of its expression. There is
/// exactly one place this postcondition's text exists in the whole
/// codebase.
impl Ensures<CreusotVerifier> for DrainsTwoValuesInOrderAndEmpties {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        DRAINS_TWO_VALUES_IN_ORDER_AND_EMPTIES_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::DrainsTwoValuesInOrderAndEmpties",
        "creusot",
        "ensures",
        || <DrainsTwoValuesInOrderAndEmpties as Ensures<CreusotVerifier>>::ensures(()),
    )
}

impl CreusotWitness for RustStdStandard<LinkedListIter<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_linked_list_iter_yields_references_in_order".to_string(),
            VERIFY_LINKED_LIST_ITER_YIELDS_REFERENCES_IN_ORDER_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<LinkedListIter<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::linked_list::Iter<'static, i32>>",
        "creusot",
        || <RustStdStandard<LinkedListIter<'static, i32>> as CreusotWitness>::proof().to_string(),
    )
}

/// Returns
/// `amenable_creusot::LINKED_LIST_ITER_YIELDS_REFERENCES_IN_ORDER_HOLDS_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn linked_list_iter_yields_references_in_order_holds`
/// the real site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<LinkedListIter<'static, i32>> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        LINKED_LIST_ITER_YIELDS_REFERENCES_IN_ORDER_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<LinkedListIter<'static, i32>>",
        "creusot",
        "ensures",
        || <RustStdStandard<LinkedListIter<'static, i32>> as Ensures<CreusotVerifier>>::ensures(()),
    )
}

impl CreusotWitness for RustStdStandard<LinkedListIterMut<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_linked_list_iter_mut_writes_through".to_string(),
            VERIFY_LINKED_LIST_ITER_MUT_WRITES_THROUGH_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<LinkedListIterMut<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::linked_list::IterMut<'static, i32>>",
        "creusot",
        || <RustStdStandard<LinkedListIterMut<'static, i32>> as CreusotWitness>::proof().to_string(),
    )
}

/// Returns
/// `amenable_creusot::LINKED_LIST_ITER_MUT_WRITES_THROUGH_HOLDS_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn linked_list_iter_mut_writes_through_holds` the
/// real site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<LinkedListIterMut<'static, i32>> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        LINKED_LIST_ITER_MUT_WRITES_THROUGH_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<LinkedListIterMut<'static, i32>>",
        "creusot",
        "ensures",
        || <RustStdStandard<LinkedListIterMut<'static, i32>> as Ensures<CreusotVerifier>>::ensures(()),
    )
}

impl CreusotWitness for RustStdStandard<LinkedListIntoIter<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_linked_list_into_iter_yields_owned_values_in_order".to_string(),
            VERIFY_LINKED_LIST_INTO_ITER_YIELDS_OWNED_VALUES_IN_ORDER_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<LinkedListIntoIter<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::linked_list::IntoIter<i32>>",
        "creusot",
        || <RustStdStandard<LinkedListIntoIter<i32>> as CreusotWitness>::proof().to_string(),
    )
}

/// [`YieldsTwoValuesInOrderThenEnds`] reuses the `LinkedList::into_iter`
/// harness rather than adding a new Creusot proof — it names the
/// postcondition both the `LinkedList` and `VecDeque` `into_iter`
/// proofs already share, it doesn't prove anything new.
impl CreusotWitness for YieldsTwoValuesInOrderThenEnds {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_linked_list_into_iter_yields_owned_values_in_order".to_string(),
            VERIFY_LINKED_LIST_INTO_ITER_YIELDS_OWNED_VALUES_IN_ORDER_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(YieldsTwoValuesInOrderThenEnds);

/// Returns `amenable_creusot::YIELDS_TWO_VALUES_IN_ORDER_THEN_ENDS_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn yields_two_values_in_order_then_ends` both real
/// sites call, not a hand-retyped copy of its expression. There is
/// exactly one place this postcondition's text exists in the whole
/// codebase.
impl Ensures<CreusotVerifier> for YieldsTwoValuesInOrderThenEnds {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        YIELDS_TWO_VALUES_IN_ORDER_THEN_ENDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::YieldsTwoValuesInOrderThenEnds",
        "creusot",
        "ensures",
        || <YieldsTwoValuesInOrderThenEnds as Ensures<CreusotVerifier>>::ensures(()),
    )
}

impl CreusotWitness for RustStdStandard<LinkedListExtractIf<'static, i32, fn(&mut i32) -> bool>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_linked_list_extract_if_partitions_by_the_predicate".to_string(),
            VERIFY_LINKED_LIST_EXTRACT_IF_PARTITIONS_BY_THE_PREDICATE_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<LinkedListExtractIf<'static, i32, fn(&mut i32) -> bool>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::linked_list::ExtractIf<'static, i32, fn(&mut i32) -> bool>>",
        "creusot",
        || {
            <RustStdStandard<LinkedListExtractIf<'static, i32, fn(&mut i32) -> bool>> as CreusotWitness>::proof()
                .to_string()
        },
    )
}

/// Returns
/// `amenable_creusot::LINKED_LIST_EXTRACT_IF_PARTITIONS_BY_THE_PREDICATE_HOLDS_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn
/// linked_list_extract_if_partitions_by_the_predicate_holds` the real
/// site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier>
    for RustStdStandard<LinkedListExtractIf<'static, i32, fn(&mut i32) -> bool>>
{
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        LINKED_LIST_EXTRACT_IF_PARTITIONS_BY_THE_PREDICATE_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<LinkedListExtractIf<'static, i32, fn(&mut i32) -> bool>>",
        "creusot",
        "ensures",
        ||
            <RustStdStandard<LinkedListExtractIf<'static, i32, fn(&mut i32) -> bool>> as Ensures<CreusotVerifier>>::ensures(()),
    )
}
