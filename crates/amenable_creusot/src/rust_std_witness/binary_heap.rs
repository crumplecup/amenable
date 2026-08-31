use super::CheckedProof;

use std::collections::BinaryHeap;
use std::collections::binary_heap::{
    Drain as BinaryHeapDrain, IntoIter as BinaryHeapIntoIter, Iter as BinaryHeapIter,
    PeekMut as BinaryHeapPeekMut,
};

use crate::{
    A_LESS_THAN_B_HOLDS_SRC, BINARY_HEAP_DRAIN_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_HOLDS_SRC,
    BINARY_HEAP_INTO_ITER_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_HOLDS_SRC,
    BINARY_HEAP_ITER_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_HOLDS_SRC,
    BINARY_HEAP_PEEK_MUT_EXPOSES_THE_MAXIMUM_HOLDS_SRC,
    BINARY_HEAP_POP_YIELDS_THE_MAXIMUM_FIRST_HOLDS_SRC, CreusotVerifier, CreusotWitness,
    VERIFY_BINARY_HEAP_DRAIN_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_SRC,
    VERIFY_BINARY_HEAP_INTO_ITER_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_SRC,
    VERIFY_BINARY_HEAP_ITER_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_SRC,
    VERIFY_BINARY_HEAP_PEEK_MUT_EXPOSES_THE_MAXIMUM_SRC,
    VERIFY_BINARY_HEAP_POP_YIELDS_THE_MAXIMUM_FIRST_SRC,
};
use amenable_core::{Ensures, Evidence, Requires, Witness};

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
impl CreusotWitness for RustStdStandard<BinaryHeap<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_binary_heap_pop_yields_the_maximum_first".to_string(),
            VERIFY_BINARY_HEAP_POP_YIELDS_THE_MAXIMUM_FIRST_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<BinaryHeap<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<BinaryHeap<i32>>",
        "creusot",
        || <RustStdStandard<BinaryHeap<i32>> as CreusotWitness>::proof().to_string(),
    )
}

/// Returns
/// `amenable_creusot::BINARY_HEAP_POP_YIELDS_THE_MAXIMUM_FIRST_HOLDS_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn binary_heap_pop_yields_the_maximum_first_holds`
/// the real site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<BinaryHeap<i32>> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        BINARY_HEAP_POP_YIELDS_THE_MAXIMUM_FIRST_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<BinaryHeap<i32>>",
        "creusot",
        "ensures",
        || <RustStdStandard<BinaryHeap<i32>> as Ensures<CreusotVerifier>>::ensures(()),
    )
}

impl CreusotWitness for RustStdStandard<BinaryHeapDrain<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_binary_heap_drain_yields_every_pushed_element_once".to_string(),
            VERIFY_BINARY_HEAP_DRAIN_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<BinaryHeapDrain<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::binary_heap::Drain<'static, i32>>",
        "creusot",
        || <RustStdStandard<BinaryHeapDrain<'static, i32>> as CreusotWitness>::proof().to_string(),
    )
}

/// Returns
/// `amenable_creusot::BINARY_HEAP_DRAIN_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_HOLDS_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn
/// binary_heap_drain_yields_every_pushed_element_once_holds` the real
/// site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<BinaryHeapDrain<'static, i32>> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        BINARY_HEAP_DRAIN_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<BinaryHeapDrain<'static, i32>>",
        "creusot",
        "ensures",
        || <RustStdStandard<BinaryHeapDrain<'static, i32>> as Ensures<CreusotVerifier>>::ensures(()),
    )
}

impl CreusotWitness for RustStdStandard<BinaryHeapIntoIter<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_binary_heap_into_iter_yields_every_pushed_element_once".to_string(),
            VERIFY_BINARY_HEAP_INTO_ITER_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<BinaryHeapIntoIter<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::binary_heap::IntoIter<i32>>",
        "creusot",
        || <RustStdStandard<BinaryHeapIntoIter<i32>> as CreusotWitness>::proof().to_string(),
    )
}

/// Returns
/// `amenable_creusot::BINARY_HEAP_INTO_ITER_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_HOLDS_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn
/// binary_heap_into_iter_yields_every_pushed_element_once_holds` the
/// real site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<BinaryHeapIntoIter<i32>> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        BINARY_HEAP_INTO_ITER_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<BinaryHeapIntoIter<i32>>",
        "creusot",
        "ensures",
        || <RustStdStandard<BinaryHeapIntoIter<i32>> as Ensures<CreusotVerifier>>::ensures(()),
    )
}

impl CreusotWitness for RustStdStandard<BinaryHeapIter<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_binary_heap_iter_yields_every_pushed_element_once".to_string(),
            VERIFY_BINARY_HEAP_ITER_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<BinaryHeapIter<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::binary_heap::Iter<'static, i32>>",
        "creusot",
        || <RustStdStandard<BinaryHeapIter<'static, i32>> as CreusotWitness>::proof().to_string(),
    )
}

/// Returns
/// `amenable_creusot::BINARY_HEAP_ITER_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_HOLDS_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn
/// binary_heap_iter_yields_every_pushed_element_once_holds` the real
/// site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<BinaryHeapIter<'static, i32>> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        BINARY_HEAP_ITER_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<BinaryHeapIter<'static, i32>>",
        "creusot",
        "ensures",
        || <RustStdStandard<BinaryHeapIter<'static, i32>> as Ensures<CreusotVerifier>>::ensures(()),
    )
}

impl CreusotWitness for RustStdStandard<BinaryHeapPeekMut<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_binary_heap_peek_mut_exposes_the_maximum".to_string(),
            VERIFY_BINARY_HEAP_PEEK_MUT_EXPOSES_THE_MAXIMUM_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<BinaryHeapPeekMut<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::binary_heap::PeekMut<'static, i32>>",
        "creusot",
        || <RustStdStandard<BinaryHeapPeekMut<'static, i32>> as CreusotWitness>::proof().to_string(),
    )
}

/// Returns `amenable_creusot::A_LESS_THAN_B_HOLDS_SRC` /
/// `BINARY_HEAP_PEEK_MUT_EXPOSES_THE_MAXIMUM_HOLDS_SRC` directly -- the
/// verbatim, `harness!`-captured source of the real `#[logic(open)]`
/// fns the real site calls, not a hand-retyped copy of their
/// expressions. `A_LESS_THAN_B_HOLDS_SRC` is the same shared
/// precondition `RustStdStandard<BTreeSet<i32>>`'s own `Requires` impl
/// above already names.
impl Requires<CreusotVerifier> for RustStdStandard<BinaryHeapPeekMut<'static, i32>> {
    type Input = ();
    type Bound = &'static str;

    fn requires(_: ()) -> &'static str {
        A_LESS_THAN_B_HOLDS_SRC
    }
}

impl Ensures<CreusotVerifier> for RustStdStandard<BinaryHeapPeekMut<'static, i32>> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        BINARY_HEAP_PEEK_MUT_EXPOSES_THE_MAXIMUM_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<BinaryHeapPeekMut<'static, i32>>",
        "creusot",
        "requires",
        ||
            <RustStdStandard<BinaryHeapPeekMut<'static, i32>> as Requires<CreusotVerifier>>::requires(()),
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<BinaryHeapPeekMut<'static, i32>>",
        "creusot",
        "ensures",
        ||
            <RustStdStandard<BinaryHeapPeekMut<'static, i32>> as Ensures<CreusotVerifier>>::ensures(()),
    )
}
