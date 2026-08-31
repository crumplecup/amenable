use super::CheckedProof;

use std::collections::{BTreeMap, BTreeSet};

use crate::{
    A_LESS_THAN_B_HOLDS_SRC, BTREE_MAP_ITERATES_IN_KEY_ORDER_HOLDS_SRC,
    BTREE_SET_ITERATES_IN_SORTED_ORDER_HOLDS_SRC, CreusotVerifier, CreusotWitness,
    K1_LESS_THAN_K2_HOLDS_SRC, VERIFY_BTREE_MAP_ITERATES_IN_KEY_ORDER_SRC,
    VERIFY_BTREE_SET_ITERATES_IN_SORTED_ORDER_SRC,
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
impl CreusotWitness for RustStdStandard<BTreeMap<i32, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_btree_map_iterates_in_key_order".to_string(),
            VERIFY_BTREE_MAP_ITERATES_IN_KEY_ORDER_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<BTreeMap<i32, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<BTreeMap<i32, i32>>",
        "creusot",
        || <RustStdStandard<BTreeMap<i32, i32>> as CreusotWitness>::proof().to_string(),
    )
}

/// Returns `amenable_creusot::K1_LESS_THAN_K2_HOLDS_SRC` /
/// `BTREE_MAP_ITERATES_IN_KEY_ORDER_HOLDS_SRC` directly -- the verbatim,
/// `harness!`-captured source of the real `#[logic(open)]` fns the real
/// site calls, not a hand-retyped copy of their expressions.
impl Requires<CreusotVerifier> for RustStdStandard<BTreeMap<i32, i32>> {
    type Input = ();
    type Bound = &'static str;

    fn requires(_: ()) -> &'static str {
        K1_LESS_THAN_K2_HOLDS_SRC
    }
}

impl Ensures<CreusotVerifier> for RustStdStandard<BTreeMap<i32, i32>> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        BTREE_MAP_ITERATES_IN_KEY_ORDER_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<BTreeMap<i32, i32>>",
        "creusot",
        "requires",
        || <RustStdStandard<BTreeMap<i32, i32>> as Requires<CreusotVerifier>>::requires(
            (),
        ),
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<BTreeMap<i32, i32>>",
        "creusot",
        "ensures",
        || <RustStdStandard<BTreeMap<i32, i32>> as Ensures<CreusotVerifier>>::ensures(()),
    )
}

impl CreusotWitness for RustStdStandard<BTreeSet<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_btree_set_iterates_in_sorted_order".to_string(),
            VERIFY_BTREE_SET_ITERATES_IN_SORTED_ORDER_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<BTreeSet<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<BTreeSet<i32>>",
        "creusot",
        || <RustStdStandard<BTreeSet<i32>> as CreusotWitness>::proof().to_string(),
    )
}

/// Returns `amenable_creusot::A_LESS_THAN_B_HOLDS_SRC` /
/// `BTREE_SET_ITERATES_IN_SORTED_ORDER_HOLDS_SRC` directly -- the
/// verbatim, `harness!`-captured source of the real `#[logic(open)]`
/// fns the real site calls, not a hand-retyped copy of their
/// expressions. `A_LESS_THAN_B_HOLDS_SRC` is shared with
/// `RustStdStandard<BinaryHeapPeekMut<'static, i32>>`'s own `Requires`
/// impl below -- the identical precondition, named once.
impl Requires<CreusotVerifier> for RustStdStandard<BTreeSet<i32>> {
    type Input = ();
    type Bound = &'static str;

    fn requires(_: ()) -> &'static str {
        A_LESS_THAN_B_HOLDS_SRC
    }
}

impl Ensures<CreusotVerifier> for RustStdStandard<BTreeSet<i32>> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        BTREE_SET_ITERATES_IN_SORTED_ORDER_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<BTreeSet<i32>>",
        "creusot",
        "requires",
        || <RustStdStandard<BTreeSet<i32>> as Requires<CreusotVerifier>>::requires(()),
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<BTreeSet<i32>>",
        "creusot",
        "ensures",
        || <RustStdStandard<BTreeSet<i32>> as Ensures<CreusotVerifier>>::ensures(()),
    )
}
