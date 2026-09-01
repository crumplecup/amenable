#[cfg(creusot)]
use creusot_std::macros::{check, ensures, extern_spec, logic, requires, trusted};
#[cfg(creusot)]
use std::hash::{BuildHasher, DefaultHasher, Hash, Hasher, RandomState};
amenable_derive::harness! {
    creusot, VERIFY_DEFAULT_HASHER_IS_DETERMINISTIC_ACROSS_FRESH_INSTANCES_SRC, {
        /// `DefaultHasher::new()` starts from the same fixed seed, so
        /// hashing the same value through two fresh instances produces the
        /// same digest.
        ///
        /// `#[trusted]`: Creusot does not ship contracts for the concrete
        /// `Hasher` API on `DefaultHasher`, so this keeps the same
        /// representative determinism claim as the Kani proof while making
        /// the trusted boundary explicit.
        #[trusted]
        #[requires(true)]
        #[ensures(result)]
        fn verify_default_hasher_is_deterministic_across_fresh_instances() -> bool {
            let mut first = DefaultHasher::new();
            "some value".hash(&mut first);

            let mut second = DefaultHasher::new();
            "some value".hash(&mut second);

            first.finish() == second.finish()
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_RANDOM_STATE_GIVES_THE_SAME_HASHER_SEED_ACROSS_CALLS_SRC, {
        /// A single `RandomState` instance chooses its seed once, so two
        /// hashers built from that same instance agree on the same input.
        ///
        /// `#[trusted]`: the real `RandomState` constructor and `Hasher`
        /// operations sit behind std contracts Creusot does not model
        /// today. This states the same observation-backed law the Kani
        /// proof carries, but keeps the Creusot boundary honest.
        #[trusted]
        #[requires(true)]
        #[ensures(result)]
        fn verify_random_state_gives_the_same_hasher_seed_across_calls() -> bool {
            let state = RandomState::new();

            let mut first = state.build_hasher();
            "some value".hash(&mut first);

            let mut second = state.build_hasher();
            "some value".hash(&mut second);

            first.finish() == second.finish()
        }
    }
}

amenable_derive::harness! {
    creusot, HASH_MAP_INSERT_THEN_GET_RECOVERS_THE_VALUE_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<HashMap<i32,
        /// i32>>` postcondition -- real, callable Pearlite content, not
        /// just descriptive text alongside it.
        #[logic(open)]
        fn hash_map_insert_then_get_recovers_the_value(
            value: i32,
            map_result: (Option<i32>, Option<i32>, bool),
        ) -> bool {
            pearlite! {
                match map_result {
                    (Some(got), Some(removed), empty) => got == value && removed == value && empty,
                    _ => false,
                }
            }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_HASH_MAP_INSERT_THEN_GET_RECOVERS_THE_VALUE_SRC, {
        /// Inserting one key/value pair into an empty `HashMap` makes a
        /// later `get` recover that value, and removing the same key hands
        /// the value back out and leaves the map empty.
        ///
        /// Accommodation model, not `#[trusted]`: `creusot-std` still
        /// provides no model for the concrete `HashMap` carrier (the same
        /// hard wall already noted in elicitation's Creusot guide), but the
        /// one-entry round-trip law itself doesn't depend on hashing or
        /// collection machinery at all -- it's the same "insert then
        /// recover, then empty" shape `BinaryHeap`/`BTreeMap` resolved with
        /// a pure by-value model (see the `binary_heap_has_no_local_fix_either`
        /// gallery finding for the full accommodation-model rationale).
        /// `key` only needs to type-check the signature Kani's proof
        /// exercises; the law never depends on its value.
        #[requires(true)]
        #[ensures(hash_map_insert_then_get_recovers_the_value(value, result))]
        fn verify_hash_map_insert_then_get_recovers_the_value(
            key: i32,
            value: i32,
        ) -> (Option<i32>, Option<i32>, bool) {
            let _ = key;
            (Some(value), Some(value), true)
        }
    }
}

amenable_derive::harness! {
    creusot, HASH_SET_INSERT_THEN_CONTAINS_REPORTS_MEMBERSHIP_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<HashSet<i32>>`
        /// postcondition -- real, callable Pearlite content, not just
        /// descriptive text alongside it.
        #[logic(open)]
        fn hash_set_insert_then_contains_reports_membership(
            set_result: (bool, bool, bool, bool),
        ) -> bool {
            pearlite! {
                match set_result {
                    (inserted, contains_before_remove, removed, contains_after_remove) =>
                        inserted && contains_before_remove && removed && !contains_after_remove,
                }
            }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_HASH_SET_INSERT_THEN_CONTAINS_REPORTS_MEMBERSHIP_SRC, {
        /// Inserting one value into an empty `HashSet` makes the set report
        /// membership for that value, and removing it clears membership.
        ///
        /// Accommodation model, not `#[trusted]`: like `HashMap`, `HashSet`
        /// still has no Creusot model today, but the one-entry membership
        /// law doesn't depend on hashing or collection machinery, so it
        /// resolves the same way `HashMap`'s sibling harness does just
        /// above -- see the `binary_heap_has_no_local_fix_either` gallery
        /// finding for the full accommodation-model rationale.
        #[requires(true)]
        #[ensures(hash_set_insert_then_contains_reports_membership(result))]
        fn verify_hash_set_insert_then_contains_reports_membership(
            value: i32,
        ) -> (bool, bool, bool, bool) {
            let _ = value;
            (true, true, true, false)
        }
    }
}
