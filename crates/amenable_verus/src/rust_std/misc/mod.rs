mod box_carrier;
#[cfg(verus_keep_ghost)]
pub use box_carrier::box_derefs_and_writes_through;
pub use box_carrier::verify_box_derefs_and_writes_through;
mod cow_carrier;
pub use cow_carrier::verify_cow_borrowed_and_owned_agree_on_their_value;
#[cfg(verus_keep_ghost)]
pub use cow_carrier::{cow_into_owned_preserves_variant_value, i32_to_owned_spec_is_identity};
mod discriminant_carrier;
pub use discriminant_carrier::verify_discriminant_model_identifies_variant_not_payload;
mod env_carrier;
#[cfg(verus_keep_ghost)]
pub use env_carrier::args_model_count_matches_program_plus_extra;
pub use env_carrier::{
    verify_args_model_reports_at_least_the_program_path,
    verify_join_paths_error_model_reports_an_unjoinable_path,
    verify_split_paths_model_recovers_paths_joined_by_join_paths,
};
mod fmt_carrier;
#[cfg(verus_keep_ghost)]
pub use fmt_carrier::{
    fmt_arguments_result_matches_display_token, fmt_debug_list_result_matches_entries_in_brackets,
    fmt_debug_map_result_matches_key_value_pair, fmt_debug_set_result_matches_entries_in_braces,
    fmt_debug_struct_result_matches_named_fields, fmt_debug_tuple_result_matches_positional_fields,
    fmt_from_fn_result_matches_display_token,
};
pub use fmt_carrier::{
    verify_alignment_model_reaches_the_formatter_from_the_format_spec,
    verify_arguments_model_renders_the_same_as_the_value_itself,
    verify_debug_list_model_renders_entries_in_brackets,
    verify_debug_map_model_renders_key_value_pairs,
    verify_debug_set_model_renders_entries_in_braces,
    verify_debug_struct_model_renders_named_fields,
    verify_debug_tuple_model_renders_positional_fields,
    verify_formatter_model_exposes_the_parsed_width_and_precision,
    verify_from_fn_model_forwards_display_to_the_supplied_closure,
};
mod layout_carrier;
pub use layout_carrier::verify_layout_from_size_align_rejects_a_non_power_of_two_alignment;
#[cfg(verus_keep_ghost)]
pub use layout_carrier::{
    ExLayout, ExLayoutError, from_size_align_rejects_a_non_power_of_two_alignment,
    is_power_of_two_spec,
};
mod manually_drop_carrier;
pub use manually_drop_carrier::verify_manually_drop_derefs_and_into_inner_round_trip;
mod misc_singletons_carrier;
pub use misc_singletons_carrier::{
    verify_backtrace_model_force_capture_always_actually_captures,
    verify_non_null_model_rejects_the_null_pointer,
    verify_panic_hook_info_model_reports_the_panics_own_message,
    verify_pin_model_derefs_and_get_mut_round_trip,
    verify_system_model_allocates_and_deallocates_a_layout,
    verify_vec_deque_drain_model_removes_and_yields_in_order,
};
mod ops_carrier;
pub use ops_carrier::{
    verify_bound_model_round_trips_its_endpoint,
    verify_control_flow_model_continue_and_break_are_disjoint,
    verify_range_full_model_contains_everything, verify_range_to_model_contains_matches_bound,
};
mod option_carrier;
#[cfg(verus_keep_ghost)]
pub use option_carrier::{option_is_none, option_wraps_the_given_value};
pub use option_carrier::{
    verify_option_unwrap_or_falls_back_to_the_default,
    verify_option_unwrap_returns_the_wrapped_value,
};
#[cfg(windows)]
mod os_windows_carrier;
#[cfg(all(windows, verus_keep_ghost))]
pub use os_windows_carrier::{
    ExBorrowedHandle, ExBorrowedSocket, ExEncodeWide, ExHandleOrInvalid, ExInvalidHandleError,
    ExOwnedHandle, ExOwnedSocket, as_raw_handle_addr_matches, as_raw_socket_matches,
    encode_wide_next_matches, handle_or_invalid_try_from_matches, owned_as_raw_handle_addr_matches,
    owned_as_raw_socket_matches,
};
mod panic_carrier;
pub use panic_carrier::{
    verify_assert_unwind_safe_model_derefs_transparently,
    verify_location_model_caller_reflects_the_immediate_call_site,
};
mod primitive_shapes_carrier;
#[cfg(verus_keep_ghost)]
pub use primitive_shapes_carrier::{
    does_not_have_length, has_length, invoked_exactly_once, is_ascii_byte,
    observed_option_matches_input, observed_pair_matches_input, observed_quad_matches_input,
    observed_triple_matches_input, observed_value_matches_input, text_view_matches_expected,
    value_unchanged, values_are_distinct, values_are_equal,
};
pub use primitive_shapes_carrier::{
    verify_array_model_indexing_and_length, verify_const_pointer_model_cast_is_reproducible,
    verify_fn_pointer_model_calls_the_underlying_function,
    verify_mut_pointer_model_cast_is_reproducible,
    verify_mutable_reference_model_dereferences_to_and_updates_the_referent,
    verify_shared_reference_model_dereferences_to_the_referent,
    verify_slice_model_indexing_and_length, verify_str_model_byte_length_and_content,
    verify_tuple_model_field_access,
};
mod result_carrier;
#[cfg(verus_keep_ghost)]
pub use result_carrier::{result_wraps_the_given_err_value, result_wraps_the_given_ok_value};
pub use result_carrier::{
    verify_result_unwrap_err_returns_the_err_value, verify_result_unwrap_returns_the_ok_value,
};
mod std_time_carrier;
#[cfg(verus_keep_ghost)]
pub use std_time_carrier::{duration_new_result_matches, duration_new_secs_headroom_holds};
pub use std_time_carrier::{
    verify_duration_model_new_normalizes_nanos_and_carries_into_secs,
    verify_instant_model_is_monotonically_nondecreasing,
    verify_system_time_error_model_recovers_how_far_backward_it_went,
    verify_system_time_model_duration_since_computes_the_elapsed_span,
};
mod type_id_carrier;
pub use type_id_carrier::verify_type_id_is_reflexive_and_distinguishes_distinct_types;
#[cfg(verus_keep_ghost)]
pub use type_id_carrier::{ExTypeId, type_id_eq_matches_identity, type_id_of_matches_spec};
