use std::collections::{BTreeMap, BTreeSet};

use amenable_core::Evidence;
#[cfg(kani)]
use amenable_core::{Ensures, Requires};
use amenable_derive::Standard;
use amenable_std::RustStdStandard;

#[cfg(kani)]
use crate::AccessorRecoversTheExpectedValue;
use crate::CheckedProof;
use crate::KaniWitness;
#[cfg(kani)]
use crate::PopRecoversTheStoredValue;
use crate::rust_std::macros::{bridge_kani_witness, kani_ensures, kani_requires};

/// A `(first, second)` pair known to satisfy `first < second`.
///
/// Independently hand-written as `kani::assume(a < b)` at 4 real sites:
/// this file's `BTreeMap`/`BTreeSet`/`BinaryHeap` harnesses (ordering
/// two fresh symbolic values before checking iteration/peek order) and
/// `rust_std::iter::verify_successors_generates_from_the_previous_item`
/// (bounding a generator seed below its model's fixed window, `seed <
/// 100`) -- the identical `<` relation regardless of whether the right
/// side is another symbolic value or a fixed literal, the same reason
/// `SplitOperandsAreDistinctFromThePattern` treats a literal and a
/// symbolic pattern uniformly. Registered as its own type rather than
/// on `RustStdStandard<i32>` directly: that carrier's
/// `Requires<KaniVerifier>` slot already holds the unrelated
/// `checked_add` precondition (`rust_std::primitives`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Standard)]
#[standard(
    basis = "RustStdStandard<i32>",
    basis_ctor = "RustStdStandard::<i32>::new()",
    provenance = "<i32 as amenable_std::RustStdType>::provenance()",
    provenance_type = "amenable_std::RustStdProvenance"
)]
pub struct FirstValueIsLessThanTheSecond;

impl KaniWitness for FirstValueIsLessThanTheSecond {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_btree_map_iterates_in_key_order".to_owned(),
            VERIFY_BTREE_MAP_ITERATES_IN_KEY_ORDER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(FirstValueIsLessThanTheSecond);

kani_requires!(
    FirstValueIsLessThanTheSecond,
    "amenable_kani::FirstValueIsLessThanTheSecond",
    (i32, i32),
    |(a, b)| a < b
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::FirstValueIsLessThanTheSecond",
        "kani",
        || <FirstValueIsLessThanTheSecond as KaniWitness>::proof().to_string(),
    )
}

impl KaniWitness for RustStdStandard<BTreeMap<i32, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_btree_map_iterates_in_key_order".to_owned(),
            VERIFY_BTREE_MAP_ITERATES_IN_KEY_ORDER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<BTreeMap<i32, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<BTreeMap<i32, i32>>",
        "kani",
        || <RustStdStandard<BTreeMap<i32, i32>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_BTREE_MAP_ITERATES_IN_KEY_ORDER_SRC, {
        /// Unlike a hash map, `BTreeMap::iter` always yields entries in
        /// ascending key order, regardless of insertion order — checked
        /// by inserting the larger key first. This proof uses an
        /// Amenable-owned ordered-map accommodation model: if the real
        /// `BTreeMap` path refines these modeled ordering and removal
        /// laws, the Rust-facing claim follows.
        #[kani::proof]
        fn verify_btree_map_iterates_in_key_order() {
            let k1: i32 = kani::any();
            let k2: i32 = kani::any();
            kani::assume(FirstValueIsLessThanTheSecond::requires((k1, k2)));
            let v1: i32 = kani::any();
            let v2: i32 = kani::any();

            let mut map = crate::KaniBTreeMap::new(k2, v2, k1, v1);
            assert!(
                AccessorRecoversTheExpectedValue::ensures((map.first_entry(), Some((&k1, &v1)))),
                "iteration is in ascending key order despite insertion order"
            );
            assert!(
                AccessorRecoversTheExpectedValue::ensures((map.second_entry(), Some((&k2, &v2)))),
                "iteration preserves the higher key and its value after the lower one"
            );
            assert!(
                PopRecoversTheStoredValue::ensures((map.remove(&k1), Some(v1))),
                "observing iteration leaves the lower key and its value in the map"
            );
            assert!(
                PopRecoversTheStoredValue::ensures((map.remove(&k2), Some(v2))),
                "iteration leaves the higher key and its value in the map"
            );
            assert!(
                EmptiedContainerReportsEmpty::ensures(map.is_empty()),
                "removing both entries after iteration empties the map"
            );
        }
    }
}

/// An emptied container's `.is_empty()` known to report `true`: whatever
/// sequence of removals (`drain`, repeated `pop`/`remove`, iteration)
/// took every element out, `.is_empty()` reflects it afterward.
///
/// Independently hand-written as `assert!(container.is_empty(), ...)` at
/// 13 real sites across `BTreeMap`, `BTreeSet`, `LinkedList`,
/// `VecDeque`, `BinaryHeap`, and `Vec` -- the identical claim regardless
/// of container type. Unlike `IteratorYieldsNoneWhenExhausted`,
/// `AtomicLoadReflectsTheLastWrite`, and `DerefReflectsTheStoredValue`,
/// this bound needs no type parameter at all: every real site already
/// computes the `bool` before asserting it, so the predicate has nothing
/// container-type-specific left to be generic over, and the ordinary
/// `kani_ensures!`/`bridge_kani_witness!` macros work unmodified.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Standard)]
#[standard(
    basis = "RustStdStandard<i32>",
    basis_ctor = "RustStdStandard::<i32>::new()",
    provenance = "<i32 as amenable_std::RustStdType>::provenance()",
    provenance_type = "amenable_std::RustStdProvenance"
)]
pub struct EmptiedContainerReportsEmpty;

impl KaniWitness for EmptiedContainerReportsEmpty {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_btree_map_iterates_in_key_order".to_owned(),
            VERIFY_BTREE_MAP_ITERATES_IN_KEY_ORDER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(EmptiedContainerReportsEmpty);

kani_ensures!(
    EmptiedContainerReportsEmpty,
    "amenable_kani::EmptiedContainerReportsEmpty",
    bool,
    |is_empty| is_empty
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::EmptiedContainerReportsEmpty",
        "kani",
        || <EmptiedContainerReportsEmpty as KaniWitness>::proof().to_string(),
    )
}

impl KaniWitness for RustStdStandard<BTreeSet<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_btree_set_iterates_in_sorted_order".to_owned(),
            VERIFY_BTREE_SET_ITERATES_IN_SORTED_ORDER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<BTreeSet<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<BTreeSet<i32>>",
        "kani",
        || <RustStdStandard<BTreeSet<i32>> as KaniWitness>::proof().to_string(),
    )
}

/// A `bool` known to be the `true` a set's `.remove()` reports when the
/// element was actually present -- following
/// `EmptiedContainerReportsEmpty`'s established shape for a raw
/// boolean claim, but its own distinct claim: this is about the
/// remove *operation's own outcome*, not the container's emptiness
/// afterward (the same reasoning `PopRemovedASegment` already applies
/// to `PathBuf::pop()`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Standard)]
#[standard(
    basis = "RustStdStandard<i32>",
    basis_ctor = "RustStdStandard::<i32>::new()",
    provenance = "<i32 as amenable_std::RustStdType>::provenance()",
    provenance_type = "amenable_std::RustStdProvenance"
)]
pub struct RemoveReportsElementWasPresent;

impl KaniWitness for RemoveReportsElementWasPresent {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_btree_set_iterates_in_sorted_order".to_owned(),
            VERIFY_BTREE_SET_ITERATES_IN_SORTED_ORDER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RemoveReportsElementWasPresent);

kani_ensures!(
    RemoveReportsElementWasPresent,
    "amenable_kani::RemoveReportsElementWasPresent",
    bool,
    |was_present| was_present
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::RemoveReportsElementWasPresent",
        "kani",
        || <RemoveReportsElementWasPresent as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_BTREE_SET_ITERATES_IN_SORTED_ORDER_SRC, {
        /// Same ordering guarantee as `BTreeMap`, for a set: `iter`
        /// yields elements in ascending order regardless of insertion
        /// order. This proof uses an Amenable-owned ordered-set
        /// accommodation model: if the real `BTreeSet` path refines
        /// these modeled ordering and removal laws, the Rust-facing
        /// claim follows.
        #[kani::proof]
        fn verify_btree_set_iterates_in_sorted_order() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            kani::assume(FirstValueIsLessThanTheSecond::requires((a, b)));

            let mut set = crate::KaniBTreeSet::new(b, a);
            assert!(
                AccessorRecoversTheExpectedValue::ensures((set.first_item(), Some(&a))),
                "iteration is in ascending order despite insertion order"
            );
            assert!(
                AccessorRecoversTheExpectedValue::ensures((set.second_item(), Some(&b))),
                "iteration preserves the higher element after the lower one"
            );
            assert!(
                RemoveReportsElementWasPresent::ensures(set.remove(&a)),
                "iteration leaves the lower element in the set"
            );
            assert!(
                RemoveReportsElementWasPresent::ensures(set.remove(&b)),
                "iteration leaves the higher element in the set"
            );
            assert!(
                EmptiedContainerReportsEmpty::ensures(set.is_empty()),
                "removing both elements after iteration empties the set"
            );
        }
    }
}
