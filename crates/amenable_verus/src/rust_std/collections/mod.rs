mod binary_heap_carrier;
pub use binary_heap_carrier::{VerusMaxHeapPair, verify_max_heap_pair_pops_the_maximum_first};
mod btree_carrier;
pub use btree_carrier::{
    verify_btree_map_insert_get_remove_round_trips,
    verify_btree_set_insert_contains_remove_round_trips,
};
mod get_disjoint_mut_carrier;
pub use get_disjoint_mut_carrier::verify_get_disjoint_mut_model_rejects_overlap_and_out_of_bounds;
mod hash_carrier;
pub use hash_carrier::verify_build_hasher_default_produces_consistent_hashers;
mod linked_list_carrier;
pub use linked_list_carrier::{VerusFifoQueuePair, verify_fifo_queue_pair_pops_in_push_order};
mod sip_hasher_carrier;
pub use sip_hasher_carrier::verify_sip_hasher_produces_consistent_hashes;
mod std_collections_carrier;
pub use std_collections_carrier::{
    verify_hash_map_model_insert_then_get_recovers_the_value,
    verify_hash_set_model_insert_then_contains_reports_membership,
};
mod std_hash_carrier;
pub use std_hash_carrier::{
    verify_default_hasher_model_is_deterministic_across_fresh_instances,
    verify_random_state_model_gives_the_same_hasher_seed_across_calls,
};
mod try_reserve_error_carrier;
pub use try_reserve_error_carrier::verify_try_reserve_preserves_vec_contents_regardless_of_outcome;
mod vec_carrier;
pub use vec_carrier::verify_vec_push_pop_round_trips;
mod vec_deque_carrier;
pub use vec_deque_carrier::verify_vec_deque_pushes_and_pops_from_both_ends;
mod vec_deque_iter_carrier;
pub use vec_deque_iter_carrier::verify_vec_deque_iter_round_trips_via_collect;
mod vec_extract_if_carrier;
pub use vec_extract_if_carrier::{
    VerusExtractIfModel, verify_vec_extract_if_model_partitions_by_the_predicate,
};
mod vec_into_iter_carrier;
pub use vec_into_iter_carrier::verify_vec_into_iter_round_trips_via_collect;
mod vec_splice_carrier;
pub use vec_splice_carrier::verify_splice_model_replaces_a_range_and_yields_what_it_removed;

/// Ghost/spec-only re-exports, one `#[cfg(verus_keep_ghost)]` gate on this
/// `mod` instead of scattered per-carrier ones -- see `misc::mod`'s own
/// doc comment for the full rationale.
#[cfg(verus_keep_ghost)]
mod ghost_reexports {
    pub use super::binary_heap_carrier::{
        binary_heap_model_pop_returns_recorded_order,
        binary_heap_model_records_values_in_heap_order, binary_heap_pop_yields_the_maximum_first,
    };
    pub use super::btree_carrier::{
        btree_map_insert_get_remove_round_trip_holds, btree_map_round_trip_inputs_are_distinct,
        btree_set_insert_contains_remove_round_trip_holds,
        btree_set_round_trip_inputs_are_distinct,
    };
    pub use super::hash_carrier::{ExBuildHasherDefault, default_hasher_new_view_is_empty};
    pub use super::sip_hasher_carrier::{
        ExSipHasher, sip_hasher_finish_matches_spec, sip_hasher_new_view_is_empty,
        sip_hasher_write_appends_to_view,
    };
    pub use super::vec_carrier::vec_len_after_one_push_is_one;
    pub use super::vec_extract_if_carrier::partition_result_matches;
    pub use super::vec_splice_carrier::splice_result_matches;
}
#[cfg(verus_keep_ghost)]
pub use ghost_reexports::*;
