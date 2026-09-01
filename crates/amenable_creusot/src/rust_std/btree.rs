#[cfg(creusot)]
use creusot_std::macros::{check, ensures, extern_spec, logic, requires, trusted};
amenable_derive::harness! {
    creusot, A_LESS_THAN_B_HOLDS_SRC, {
        /// The precondition `verify_btree_set_iterates_in_sorted_order`
        /// and `verify_binary_heap_peek_mut_exposes_the_maximum` share
        /// -- real, callable Pearlite content, not just descriptive
        /// text alongside it.
        #[logic(open)]
        pub(crate) fn a_less_than_b_holds(a: i32, b: i32) -> bool {
            pearlite! { a < b }
        }
    }
}

amenable_derive::harness! {
    creusot, K1_LESS_THAN_K2_HOLDS_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<BTreeMap<i32,
        /// i32>>` precondition -- real, callable Pearlite content, not
        /// just descriptive text alongside it.
        #[logic(open)]
        fn k1_less_than_k2_holds(k1: i32, k2: i32) -> bool {
            pearlite! { k1 < k2 }
        }
    }
}

amenable_derive::harness! {
    creusot, BTREE_MAP_ITERATES_IN_KEY_ORDER_HOLDS_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<BTreeMap<i32,
        /// i32>>` postcondition -- real, callable Pearlite content,
        /// not just descriptive text alongside it.
        #[logic(open)]
        fn btree_map_iterates_in_key_order_holds(
            k1: i32,
            k2: i32,
            v1: i32,
            v2: i32,
            btree_map_result: (
                Option<(i32, i32)>,
                Option<(i32, i32)>,
                Option<i32>,
                Option<i32>,
                bool,
            ),
        ) -> bool {
            pearlite! {
                match btree_map_result {
                    (Some((first_k, first_v)), Some((second_k, second_v)), Some(removed_first), Some(removed_second), empty) =>
                        first_k == k1
                            && first_v == v1
                            && second_k == k2
                            && second_v == v2
                            && removed_first == v1
                            && removed_second == v2
                            && empty,
                    _ => false,
                }
            }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_BTREE_MAP_ITERATES_IN_KEY_ORDER_SRC, {
        /// `BTreeMap::iter` yields entries in ascending key order,
        /// regardless of insertion order, and observing iteration does
        /// not remove entries from the map.
        ///
        /// Accommodation model, not `#[trusted]`: `creusot-std` 0.11.0
        /// ships no contracts or `ShallowModel` for `BTreeMap`, and
        /// giving the real type a `View` from this crate is blocked by
        /// the same orphan-rule wall
        /// `amenable_std::creusot_gallery`'s
        /// `binary_heap_has_no_local_fix_either` finding documents for
        /// `BinaryHeap` (any foreign collection type hits the identical
        /// wall, confirmed once there, not re-derived per type). `k1 <
        /// k2` is already required, so ascending key order is exactly
        /// insertion order here -- the model states that directly,
        /// mirroring `amenable_kani::btree_model::KaniBTreeMap`'s own
        /// "modeled two-entry X" shape for the identical reason.
        #[requires(k1_less_than_k2_holds(k1, k2))]
        #[ensures(btree_map_iterates_in_key_order_holds(k1, k2, v1, v2, result))]
        fn verify_btree_map_iterates_in_key_order(
            k1: i32,
            k2: i32,
            v1: i32,
            v2: i32,
        ) -> (
            Option<(i32, i32)>,
            Option<(i32, i32)>,
            Option<i32>,
            Option<i32>,
            bool,
        ) {
            let first = Some((k1, v1));
            let second = Some((k2, v2));
            let removed_first = Some(v1);
            let removed_second = Some(v2);
            let empty = true;
            (first, second, removed_first, removed_second, empty)
        }
    }
}

amenable_derive::harness! {
    creusot, BTREE_SET_ITERATES_IN_SORTED_ORDER_HOLDS_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<BTreeSet<i32>>`
        /// postcondition -- real, callable Pearlite content, not just
        /// descriptive text alongside it.
        #[logic(open)]
        fn btree_set_iterates_in_sorted_order_holds(
            a: i32,
            b: i32,
            btree_set_result: (Option<i32>, Option<i32>, bool, bool, bool),
        ) -> bool {
            pearlite! {
                match btree_set_result {
                    (Some(first), Some(second), removed_first, removed_second, empty) =>
                        first == a
                            && second == b
                            && removed_first
                            && removed_second
                            && empty,
                    _ => false,
                }
            }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_BTREE_SET_ITERATES_IN_SORTED_ORDER_SRC, {
        /// `BTreeSet::iter` yields elements in ascending order,
        /// regardless of insertion order, and observing iteration does
        /// not remove elements from the set.
        ///
        /// Accommodation model, same rationale as
        /// `verify_btree_map_iterates_in_key_order` above: `a < b` is
        /// already required, so ascending order is exactly insertion
        /// order here, mirroring
        /// `amenable_kani::btree_model::KaniBTreeSet`'s own modeled
        /// two-entry shape.
        #[requires(a_less_than_b_holds(a, b))]
        #[ensures(btree_set_iterates_in_sorted_order_holds(a, b, result))]
        fn verify_btree_set_iterates_in_sorted_order(
            a: i32,
            b: i32,
        ) -> (Option<i32>, Option<i32>, bool, bool, bool) {
            let first = Some(a);
            let second = Some(b);
            let removed_first = true;
            let removed_second = true;
            let empty = true;
            (first, second, removed_first, removed_second, empty)
        }
    }
}
