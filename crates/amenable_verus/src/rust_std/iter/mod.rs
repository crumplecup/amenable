mod array_into_iter_carrier;
pub use array_into_iter_carrier::{
    VerusArrayIntoIterModel, verify_array_into_iter_model_yields_elements_in_order,
};
#[cfg(verus_keep_ghost)]
pub use array_into_iter_carrier::{
    array_into_iter_advance_matches_position, array_into_iter_model_starts_at_first_position,
    yields_three_values_in_order_then_ends,
};
mod iter_generator_carrier;
pub use iter_generator_carrier::{
    verify_empty_model_yields_nothing, verify_once_model_yields_exactly_one_value,
    verify_once_with_model_calls_its_closure_exactly_once,
    verify_repeat_model_yields_the_same_value_forever,
    verify_repeat_n_model_yields_the_value_exactly_n_times,
    verify_repeat_with_model_calls_its_closure_once_per_item,
};
mod iter_sequence_carrier;
#[cfg(verus_keep_ghost)]
pub use iter_sequence_carrier::{
    four_increment_headroom_holds, increment_headroom_holds, single_increment_headroom_holds,
    ten_increment_headroom_holds, two_increment_headroom_holds,
};
pub use iter_sequence_carrier::{
    verify_chain_model_sequences_two_iterators_end_to_end,
    verify_enumerate_model_pairs_each_item_with_its_index,
    verify_rev_model_reverses_iteration_order, verify_zip_model_pairs_items_from_two_iterators,
};
mod iter_stateful_carrier;
#[cfg(verus_keep_ghost)]
pub use iter_stateful_carrier::is_within_scan_sum_headroom;
pub use iter_stateful_carrier::{
    verify_cycle_model_repeats_its_sequence_forever,
    verify_flat_map_model_flattens_each_generated_iterator,
    verify_flatten_model_concatenates_the_inner_iterators,
    verify_from_fn_model_yields_until_the_closure_returns_none,
    verify_fuse_model_keeps_returning_none_once_exhausted,
    verify_inspect_model_calls_once_per_item_without_changing_values,
    verify_peekable_model_peek_does_not_consume,
    verify_scan_model_threads_state_through_its_closure,
    verify_successors_model_generates_from_the_previous_item,
};
mod iter_transform_carrier;
#[cfg(verus_keep_ghost)]
pub use iter_transform_carrier::{
    is_within_map_while_doubling_headroom, map_while_closure_result_matches,
    nonzero_item_survives_filtering,
};
pub use iter_transform_carrier::{
    verify_cloned_model_clones_each_referenced_item,
    verify_copied_model_copies_each_referenced_item,
    verify_filter_map_model_applies_and_filters_in_one_step,
    verify_filter_model_yields_only_items_matching_the_predicate,
    verify_map_model_applies_its_closure_to_each_item,
    verify_map_while_model_maps_items_while_the_closure_returns_some,
};
mod iter_window_carrier;
pub use iter_window_carrier::{
    verify_skip_model_discards_the_first_n_items,
    verify_skip_while_model_discards_items_while_the_predicate_holds,
    verify_step_by_model_yields_every_nth_item, verify_take_model_yields_at_most_n_items,
    verify_take_while_model_yields_items_while_the_predicate_holds,
};
mod option_result_iter_carrier;
#[cfg(verus_keep_ghost)]
pub use option_result_iter_carrier::into_iter_yields_zero_or_one_owned_value;
pub use option_result_iter_carrier::{
    verify_into_iter_model_yields_zero_or_one_owned_value,
    verify_iter_model_yields_zero_or_one_reference, verify_iter_mut_model_writes_through,
};
mod ordered_pair_into_iter_carrier;
pub use ordered_pair_into_iter_carrier::{
    VerusOrderedPairIntoIterModel, verify_ordered_pair_into_iter_model_yields_owned_values_in_order,
};
#[cfg(verus_keep_ghost)]
pub use ordered_pair_into_iter_carrier::{
    ordered_pair_into_iter_advance_result_matches,
    ordered_pair_into_iter_model_starts_at_position_zero,
};
mod ordered_pair_iter_mut_carrier;
pub use ordered_pair_iter_mut_carrier::{
    VerusOrderedPairIterMutModel, verify_ordered_pair_iter_mut_model_writes_through_in_order,
};
mod slice_chunk_by_carrier;
#[cfg(verus_keep_ghost)]
pub use slice_chunk_by_carrier::chunk_by_result_matches_grouping;
pub use slice_chunk_by_carrier::verify_chunk_by_model_groups_adjacent_elements_matching_the_predicate;
mod slice_chunks_carrier;
#[cfg(verus_keep_ghost)]
pub use slice_chunks_carrier::ten_increment_write_through;
pub use slice_chunks_carrier::{
    verify_chunks_exact_model_discards_a_short_remainder,
    verify_chunks_exact_mut_model_leaves_the_remainder_untouched,
    verify_chunks_model_yields_non_overlapping_groups_with_a_short_last_chunk,
    verify_chunks_mut_model_writes_through_every_chunk,
    verify_rchunks_exact_model_discards_a_short_remainder_at_the_front,
    verify_rchunks_exact_mut_model_leaves_the_front_remainder_untouched,
    verify_rchunks_model_groups_from_the_back, verify_rchunks_mut_model_writes_through_every_chunk,
    verify_windows_model_yields_overlapping_slices,
};
mod slice_iter_carrier;
pub use slice_iter_carrier::{
    verify_iter_model_yields_shared_references_in_order,
    verify_iter_mut_model_yields_mutable_references_that_write_through,
};
mod slice_split_carrier;
pub use slice_split_carrier::{
    verify_rsplit_model_yields_subslices_from_the_back,
    verify_rsplit_mut_model_writes_through_the_rearmost_piece,
    verify_rsplit_n_model_caps_the_number_of_pieces_from_the_back,
    verify_split_inclusive_model_keeps_the_match_at_the_end_of_each_piece,
    verify_split_inclusive_mut_model_keeps_the_match_at_the_end_of_each_piece,
    verify_split_model_yields_subslices_between_matches,
    verify_split_mut_model_writes_through_the_first_piece,
    verify_split_n_model_caps_the_number_of_pieces,
};
mod unordered_pair_carrier;
#[cfg(verus_keep_ghost)]
pub use unordered_pair_carrier::drain_result_matches_order;
pub use unordered_pair_carrier::{
    VerusUnorderedPairModel, verify_unordered_pair_model_yields_every_element_once,
};
