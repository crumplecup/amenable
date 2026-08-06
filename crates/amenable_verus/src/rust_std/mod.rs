//! One file per std carrier, each holding exactly the `verus! { ... }`
//! spec function(s) that carrier's real invariant needs — matching how
//! `elicitation_verus` groups its own proof files (`chars.rs`,
//! `durations.rs`, one file per type cluster), so `amenable_std::
//! verus_witness` can `include_str!` a single file as one type's whole
//! `claim`, the same one-claim-per-carrier granularity `amenable_kani`/
//! `amenable_creusot` get from `amenable_derive::harness!` capturing one
//! function at a time.

pub mod arc_carrier;
pub mod array_into_iter_carrier;
pub mod ascii_escape_carrier;
pub mod binary_heap_carrier;
pub mod box_carrier;
pub mod btree_carrier;
pub mod cell_carrier;
pub mod char_carrier;
pub mod char_transform_carrier;
pub mod char_try_from_carrier;
pub mod chars_carrier;
pub mod cow_carrier;
pub mod cstr_carrier;
pub mod cstring_carrier;
pub mod decode_utf16_carrier;
pub mod fp_category_carrier;
pub mod from_utf16_error_carrier;
pub mod from_utf8_error_carrier;
pub mod from_vec_with_nul_carrier;
pub mod hash_carrier;
pub mod int_error_kind_carrier;
pub mod into_string_error_carrier;
pub mod iter_generator_carrier;
pub mod iter_sequence_carrier;
pub mod iter_stateful_carrier;
pub mod iter_transform_carrier;
pub mod iter_window_carrier;
pub mod layout_carrier;
pub mod lazy_cell_carrier;
pub mod linked_list_carrier;
pub mod manually_drop_carrier;
pub mod once_cell_carrier;
pub mod option_carrier;
pub mod ordered_pair_into_iter_carrier;
pub mod ordered_pair_iter_mut_carrier;
pub mod ordering_carrier;
pub mod parse_char_error_carrier;
pub mod parse_float_error_carrier;
pub mod rc_carrier;
pub mod ref_carrier;
pub mod ref_cell_carrier;
pub mod ref_mut_carrier;
pub mod result_carrier;
pub mod reverse_carrier;
pub mod saturating_carrier;
pub mod sip_hasher_carrier;
pub mod string_carrier;
pub mod try_from_int_error_carrier;
pub mod try_from_slice_carrier;
pub mod try_reserve_error_carrier;
pub mod type_id_carrier;
pub mod unordered_pair_carrier;
pub mod unsafe_cell_carrier;
pub mod vec_carrier;
pub mod vec_deque_carrier;
pub mod vec_deque_iter_carrier;
pub mod vec_extract_if_carrier;
pub mod vec_into_iter_carrier;
pub mod vec_splice_carrier;
pub mod weak_carrier;
pub mod wrapping_carrier;
