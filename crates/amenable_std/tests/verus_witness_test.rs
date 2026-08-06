#![cfg(feature = "verus")]

use amenable_core::Witness;
use amenable_std::{RustStdStandard, RustStdType, VerusVerifier};

#[test]
fn bool_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<bool> as Witness<VerusVerifier>>::proof(),
        <bool as RustStdType>::provenance()
    );
}

#[test]
fn char_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<char> as Witness<VerusVerifier>>::proof();

    assert_eq!(proof.harness, "verify_char_roundtrip");
    assert_eq!(proof.provenance, <char as RustStdType>::provenance());
}

#[test]
fn string_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<String> as Witness<VerusVerifier>>::proof();

    assert_eq!(proof.harness, "verify_string_roundtrip");
    assert_eq!(proof.provenance, <String as RustStdType>::provenance());
}

#[test]
fn ordering_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::cmp::Ordering> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_ordering_reverse_swaps_less_and_greater"
    );
    assert_eq!(
        proof.provenance,
        <std::cmp::Ordering as RustStdType>::provenance()
    );
}

#[test]
fn option_i32_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<Option<i32>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_option_unwrap_returns_the_wrapped_value"
    );
    assert_eq!(proof.provenance, <Option<i32> as RustStdType>::provenance());
}

#[test]
fn result_i32_i32_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<Result<i32, i32>> as Witness<VerusVerifier>>::proof();

    assert_eq!(proof.harness, "verify_result_unwrap_returns_the_ok_value");
    assert_eq!(
        proof.provenance,
        <Result<i32, i32> as RustStdType>::provenance()
    );
}

#[test]
fn wrapping_i32_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::num::Wrapping<i32>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_wrapping_field_roundtrips_the_constructed_value"
    );
    assert_eq!(
        proof.provenance,
        <std::num::Wrapping<i32> as RustStdType>::provenance()
    );
}

#[test]
fn saturating_i32_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::num::Saturating<i32>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_saturating_field_roundtrips_the_constructed_value"
    );
    assert_eq!(
        proof.provenance,
        <std::num::Saturating<i32> as RustStdType>::provenance()
    );
}

#[test]
fn reverse_i32_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::cmp::Reverse<i32>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_reverse_field_roundtrips_the_constructed_value"
    );
    assert_eq!(
        proof.provenance,
        <std::cmp::Reverse<i32> as RustStdType>::provenance()
    );
}

#[test]
fn manually_drop_i32_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::mem::ManuallyDrop<i32>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_manually_drop_derefs_and_into_inner_round_trip"
    );
    assert_eq!(
        proof.provenance,
        <std::mem::ManuallyDrop<i32> as RustStdType>::provenance()
    );
}

#[test]
fn fp_category_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<core::num::FpCategory> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_fp_category_matches_the_value_it_classifies"
    );
    assert_eq!(
        proof.provenance,
        <core::num::FpCategory as RustStdType>::provenance()
    );
}

#[test]
fn int_error_kind_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<core::num::IntErrorKind> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_int_error_kind_classifies_parse_failures"
    );
    assert_eq!(
        proof.provenance,
        <core::num::IntErrorKind as RustStdType>::provenance()
    );
}

#[test]
fn parse_float_error_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<core::num::ParseFloatError> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_parse_float_error_occurs_only_for_unparseable_input"
    );
    assert_eq!(
        proof.provenance,
        <core::num::ParseFloatError as RustStdType>::provenance()
    );
}

#[test]
fn try_from_int_error_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<core::num::TryFromIntError> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_try_from_int_error_occurs_exactly_when_out_of_range"
    );
    assert_eq!(
        proof.provenance,
        <core::num::TryFromIntError as RustStdType>::provenance()
    );
}

#[test]
fn box_i32_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<Box<i32>> as Witness<VerusVerifier>>::proof();

    assert_eq!(proof.harness, "verify_box_derefs_and_writes_through");
    assert_eq!(proof.provenance, <Box<i32> as RustStdType>::provenance());
}

#[test]
fn infallible_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<std::convert::Infallible> as Witness<VerusVerifier>>::proof(),
        <std::convert::Infallible as RustStdType>::provenance()
    );
}

#[test]
fn layout_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<core::alloc::Layout> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_layout_from_size_align_rejects_a_non_power_of_two_alignment"
    );
    assert_eq!(
        proof.provenance,
        <core::alloc::Layout as RustStdType>::provenance()
    );
}

#[test]
fn layout_error_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<core::alloc::LayoutError> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_layout_from_size_align_rejects_a_non_power_of_two_alignment"
    );
    assert_eq!(
        proof.provenance,
        <core::alloc::LayoutError as RustStdType>::provenance()
    );
}

#[test]
fn vec_i32_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<Vec<i32>> as Witness<VerusVerifier>>::proof();

    assert_eq!(proof.harness, "verify_vec_push_pop_round_trips");
    assert_eq!(proof.provenance, <Vec<i32> as RustStdType>::provenance());
}

#[test]
fn char_try_from_error_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<core::char::CharTryFromError> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_char_try_from_fails_exactly_for_surrogates_and_out_of_range"
    );
    assert_eq!(
        proof.provenance,
        <core::char::CharTryFromError as RustStdType>::provenance()
    );
}

#[test]
fn try_from_char_error_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<core::char::TryFromCharError> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_try_from_char_error_occurs_exactly_when_out_of_range"
    );
    assert_eq!(
        proof.provenance,
        <core::char::TryFromCharError as RustStdType>::provenance()
    );
}

#[test]
fn c_void_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<core::ffi::c_void> as Witness<VerusVerifier>>::proof(),
        <core::ffi::c_void as RustStdType>::provenance()
    );
}

#[test]
fn borrow_error_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<std::cell::BorrowError> as Witness<VerusVerifier>>::proof(),
        <std::cell::BorrowError as RustStdType>::provenance()
    );
}

#[test]
fn borrow_mut_error_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<std::cell::BorrowMutError> as Witness<VerusVerifier>>::proof(),
        <std::cell::BorrowMutError as RustStdType>::provenance()
    );
}

#[test]
fn type_id_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<core::any::TypeId> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_type_id_is_reflexive_and_distinguishes_distinct_types"
    );
    assert_eq!(
        proof.provenance,
        <core::any::TypeId as RustStdType>::provenance()
    );
}

#[test]
fn try_from_slice_error_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::array::TryFromSliceError> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_try_from_slice_rejects_a_length_mismatch"
    );
    assert_eq!(
        proof.provenance,
        <std::array::TryFromSliceError as RustStdType>::provenance()
    );
}

#[test]
fn from_utf16_error_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::string::FromUtf16Error> as Witness<VerusVerifier>>::proof();

    assert_eq!(proof.harness, "verify_from_utf16_rejects_a_lone_surrogate");
    assert_eq!(
        proof.provenance,
        <std::string::FromUtf16Error as RustStdType>::provenance()
    );
}

#[test]
fn cstring_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::ffi::CString> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_cstring_excludes_the_terminator_and_rejects_interior_nul"
    );
    assert_eq!(
        proof.provenance,
        <std::ffi::CString as RustStdType>::provenance()
    );
}

#[test]
fn nul_error_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::ffi::NulError> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_cstring_excludes_the_terminator_and_rejects_interior_nul"
    );
    assert_eq!(
        proof.provenance,
        <std::ffi::NulError as RustStdType>::provenance()
    );
}

#[test]
fn from_vec_with_nul_error_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::ffi::FromVecWithNulError> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_from_vec_with_nul_requires_the_nul_only_at_the_end"
    );
    assert_eq!(
        proof.provenance,
        <std::ffi::FromVecWithNulError as RustStdType>::provenance()
    );
}

#[test]
fn parse_char_error_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<core::char::ParseCharError> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_parse_char_error_occurs_for_empty_or_multi_character_strings"
    );
    assert_eq!(
        proof.provenance,
        <core::char::ParseCharError as RustStdType>::provenance()
    );
}

#[test]
fn rc_i32_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::rc::Rc<i32>> as Witness<VerusVerifier>>::proof();

    assert_eq!(proof.harness, "verify_rc_derefs_to_the_wrapped_value");
    assert_eq!(
        proof.provenance,
        <std::rc::Rc<i32> as RustStdType>::provenance()
    );
}

#[test]
fn arc_i32_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::sync::Arc<i32>> as Witness<VerusVerifier>>::proof();

    assert_eq!(proof.harness, "verify_arc_derefs_to_the_wrapped_value");
    assert_eq!(
        proof.provenance,
        <std::sync::Arc<i32> as RustStdType>::provenance()
    );
}

#[test]
fn into_string_error_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::ffi::IntoStringError> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_into_string_error_recovers_the_original_cstring"
    );
    assert_eq!(
        proof.provenance,
        <std::ffi::IntoStringError as RustStdType>::provenance()
    );
}

#[test]
fn from_bytes_until_nul_error_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<core::ffi::FromBytesUntilNulError> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_from_bytes_until_nul_requires_a_nul_byte_somewhere"
    );
    assert_eq!(
        proof.provenance,
        <core::ffi::FromBytesUntilNulError as RustStdType>::provenance()
    );
}

#[test]
fn from_bytes_with_nul_error_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<core::ffi::FromBytesWithNulError> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_from_bytes_with_nul_requires_the_nul_only_at_the_end"
    );
    assert_eq!(
        proof.provenance,
        <core::ffi::FromBytesWithNulError as RustStdType>::provenance()
    );
}

#[test]
fn build_hasher_default_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<
        std::hash::BuildHasherDefault<std::collections::hash_map::DefaultHasher>,
    > as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_build_hasher_default_produces_consistent_hashers"
    );
    assert_eq!(
        proof.provenance,
        <std::hash::BuildHasherDefault<
            std::collections::hash_map::DefaultHasher,
        > as RustStdType>::provenance()
    );
}

#[test]
#[expect(
    deprecated,
    reason = "SipHasher itself is stable (only deprecated as a recommendation to use DefaultHasher instead); covering it is a coverage-completeness question, not a call to use it"
)]
fn sip_hasher_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::hash::SipHasher> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_sip_hasher_produces_consistent_hashes"
    );
    assert_eq!(
        proof.provenance,
        <std::hash::SipHasher as RustStdType>::provenance()
    );
}

#[test]
fn cow_i32_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::borrow::Cow<'static, i32>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_cow_borrowed_and_owned_agree_on_their_value"
    );
    assert_eq!(
        proof.provenance,
        <std::borrow::Cow<'static, i32> as RustStdType>::provenance()
    );
}

#[test]
fn btree_map_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::collections::BTreeMap<i32, i32>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_btree_map_insert_get_remove_round_trips"
    );
    assert_eq!(
        proof.provenance,
        <std::collections::BTreeMap<i32, i32> as RustStdType>::provenance()
    );
}

#[test]
fn btree_set_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::collections::BTreeSet<i32>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_btree_set_insert_contains_remove_round_trips"
    );
    assert_eq!(
        proof.provenance,
        <std::collections::BTreeSet<i32> as RustStdType>::provenance()
    );
}

#[test]
fn vec_deque_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::collections::VecDeque<i32>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_vec_deque_pushes_and_pops_from_both_ends"
    );
    assert_eq!(
        proof.provenance,
        <std::collections::VecDeque<i32> as RustStdType>::provenance()
    );
}

#[test]
fn try_reserve_error_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::collections::TryReserveError> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_try_reserve_preserves_vec_contents_regardless_of_outcome"
    );
    assert_eq!(
        proof.provenance,
        <std::collections::TryReserveError as RustStdType>::provenance()
    );
}

#[test]
fn vec_into_iter_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::vec::IntoIter<i32>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_vec_into_iter_round_trips_via_collect"
    );
    assert_eq!(
        proof.provenance,
        <std::vec::IntoIter<i32> as RustStdType>::provenance()
    );
}

#[test]
fn vec_deque_iter_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::collections::vec_deque::Iter<'static, i32>> as Witness<
        VerusVerifier,
    >>::proof();

    assert_eq!(
        proof.harness,
        "verify_vec_deque_iter_round_trips_via_collect"
    );
    assert_eq!(
        proof.provenance,
        <std::collections::vec_deque::Iter<'static, i32> as RustStdType>::provenance()
    );
}

#[test]
fn chars_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::str::Chars<'static>> as Witness<VerusVerifier>>::proof();

    assert_eq!(proof.harness, "verify_chars_yields_characters_in_order");
    assert_eq!(
        proof.provenance,
        <std::str::Chars<'static> as RustStdType>::provenance()
    );
}

#[test]
fn binary_heap_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::collections::BinaryHeap<i32>> as Witness<VerusVerifier>>::proof();

    assert_eq!(proof.harness, "verify_max_heap_pair_pops_the_maximum_first");
    assert_eq!(
        proof.provenance,
        <std::collections::BinaryHeap<i32> as RustStdType>::provenance()
    );
}

#[test]
fn iter_map_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::iter::Map<std::ops::Range<i32>, fn(i32) -> i32>> as Witness<
            VerusVerifier,
        >>::proof();

    assert_eq!(
        proof.harness,
        "verify_map_model_applies_its_closure_to_each_item"
    );
    assert_eq!(
        proof.provenance,
        <std::iter::Map<std::ops::Range<i32>, fn(i32) -> i32> as RustStdType>::provenance()
    );
}

#[test]
fn iter_filter_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<
        std::iter::Filter<std::array::IntoIter<i32, 1>, fn(&i32) -> bool>,
    > as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_filter_model_yields_only_items_matching_the_predicate"
    );
    assert_eq!(
        proof.provenance,
        <std::iter::Filter<std::array::IntoIter<i32, 1>, fn(&i32) -> bool> as RustStdType>::provenance()
    );
}

#[test]
fn iter_filter_map_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<
        std::iter::FilterMap<std::array::IntoIter<i32, 1>, fn(i32) -> Option<i32>>,
    > as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_filter_map_model_applies_and_filters_in_one_step"
    );
    assert_eq!(
        proof.provenance,
        <std::iter::FilterMap<std::array::IntoIter<i32, 1>, fn(i32) -> Option<i32>> as RustStdType>::provenance()
    );
}

#[test]
fn iter_map_while_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<
        std::iter::MapWhile<std::ops::Range<i32>, fn(i32) -> Option<i32>>,
    > as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_map_while_model_maps_items_while_the_closure_returns_some"
    );
    assert_eq!(
        proof.provenance,
        <std::iter::MapWhile<std::ops::Range<i32>, fn(i32) -> Option<i32>> as RustStdType>::provenance()
    );
}

#[test]
fn iter_cloned_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::iter::Cloned<std::slice::Iter<'static, i32>>> as Witness<
        VerusVerifier,
    >>::proof();

    assert_eq!(
        proof.harness,
        "verify_cloned_model_clones_each_referenced_item"
    );
    assert_eq!(
        proof.provenance,
        <std::iter::Cloned<std::slice::Iter<'static, i32>> as RustStdType>::provenance()
    );
}

#[test]
fn iter_copied_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::iter::Copied<std::slice::Iter<'static, i32>>> as Witness<
        VerusVerifier,
    >>::proof();

    assert_eq!(
        proof.harness,
        "verify_copied_model_copies_each_referenced_item"
    );
    assert_eq!(
        proof.provenance,
        <std::iter::Copied<std::slice::Iter<'static, i32>> as RustStdType>::provenance()
    );
}

#[test]
fn vec_extract_if_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::vec::ExtractIf<'static, i32, fn(&mut i32) -> bool>> as Witness<
            VerusVerifier,
        >>::proof();

    assert_eq!(
        proof.harness,
        "verify_vec_extract_if_model_partitions_by_the_predicate"
    );
    assert_eq!(
        proof.provenance,
        <std::vec::ExtractIf<'static, i32, fn(&mut i32) -> bool> as RustStdType>::provenance()
    );
}

#[test]
fn linked_list_extract_if_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<
        std::collections::linked_list::ExtractIf<'static, i32, fn(&mut i32) -> bool>,
    > as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_vec_extract_if_model_partitions_by_the_predicate"
    );
    assert_eq!(
        proof.provenance,
        <std::collections::linked_list::ExtractIf<'static, i32, fn(&mut i32) -> bool> as RustStdType>::provenance()
    );
}

#[test]
fn vec_splice_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::vec::Splice<'static, std::vec::IntoIter<i32>>> as Witness<
        VerusVerifier,
    >>::proof();

    assert_eq!(
        proof.harness,
        "verify_splice_model_replaces_a_range_and_yields_what_it_removed"
    );
    assert_eq!(
        proof.provenance,
        <std::vec::Splice<'static, std::vec::IntoIter<i32>> as RustStdType>::provenance()
    );
}

#[test]
fn string_drain_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::string::Drain<'static>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_ordered_pair_into_iter_model_yields_owned_values_in_order"
    );
    assert_eq!(
        proof.provenance,
        <std::string::Drain<'static> as RustStdType>::provenance()
    );
}

#[test]
fn binary_heap_peek_mut_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::collections::binary_heap::PeekMut<'static, i32>> as Witness<
            VerusVerifier,
        >>::proof();

    assert_eq!(proof.harness, "verify_max_heap_pair_pops_the_maximum_first");
    assert_eq!(
        proof.provenance,
        <std::collections::binary_heap::PeekMut<'static, i32> as RustStdType>::provenance()
    );
}

#[test]
fn linked_list_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::collections::LinkedList<i32>> as Witness<VerusVerifier>>::proof();

    assert_eq!(proof.harness, "verify_fifo_queue_pair_pops_in_push_order");
    assert_eq!(
        proof.provenance,
        <std::collections::LinkedList<i32> as RustStdType>::provenance()
    );
}

#[test]
fn cell_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::cell::Cell<i32>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_cell_model_get_set_replace_round_trip"
    );
    assert_eq!(
        proof.provenance,
        <std::cell::Cell<i32> as RustStdType>::provenance()
    );
}

#[test]
fn array_into_iter_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::array::IntoIter<i32, 3>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_array_into_iter_model_yields_elements_in_order"
    );
    assert_eq!(
        proof.provenance,
        <std::array::IntoIter<i32, 3> as RustStdType>::provenance()
    );
}

#[test]
fn ref_cell_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::cell::RefCell<i32>> as Witness<VerusVerifier>>::proof();

    assert_eq!(proof.harness, "verify_ref_cell_model_dynamic_borrow_rules");
    assert_eq!(
        proof.provenance,
        <std::cell::RefCell<i32> as RustStdType>::provenance()
    );
}

#[test]
fn once_cell_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::cell::OnceCell<i32>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_once_cell_model_initializes_exactly_once"
    );
    assert_eq!(
        proof.provenance,
        <std::cell::OnceCell<i32> as RustStdType>::provenance()
    );
}

#[test]
fn unsafe_cell_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::cell::UnsafeCell<i32>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_unsafe_cell_model_get_mut_and_into_inner_round_trip"
    );
    assert_eq!(
        proof.provenance,
        <std::cell::UnsafeCell<i32> as RustStdType>::provenance()
    );
}

#[test]
fn lazy_cell_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::cell::LazyCell<i32, fn() -> i32>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_lazy_cell_model_caches_its_initializer_result"
    );
    assert_eq!(
        proof.provenance,
        <std::cell::LazyCell<i32, fn() -> i32> as RustStdType>::provenance()
    );
}

#[test]
fn lazy_lock_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::sync::LazyLock<i32, fn() -> i32>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_lazy_cell_model_caches_its_initializer_result"
    );
    assert_eq!(
        proof.provenance,
        <std::sync::LazyLock<i32, fn() -> i32> as RustStdType>::provenance()
    );
}

#[test]
fn vec_drain_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::vec::Drain<'static, i32>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_ordered_pair_into_iter_model_yields_owned_values_in_order"
    );
    assert_eq!(
        proof.provenance,
        <std::vec::Drain<'static, i32> as RustStdType>::provenance()
    );
}

#[test]
fn vec_deque_into_iter_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::collections::vec_deque::IntoIter<i32>> as Witness<
        VerusVerifier,
    >>::proof();

    assert_eq!(
        proof.harness,
        "verify_ordered_pair_into_iter_model_yields_owned_values_in_order"
    );
    assert_eq!(
        proof.provenance,
        <std::collections::vec_deque::IntoIter<i32> as RustStdType>::provenance()
    );
}

#[test]
fn linked_list_into_iter_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::collections::linked_list::IntoIter<i32>> as Witness<
        VerusVerifier,
    >>::proof();

    assert_eq!(
        proof.harness,
        "verify_ordered_pair_into_iter_model_yields_owned_values_in_order"
    );
    assert_eq!(
        proof.provenance,
        <std::collections::linked_list::IntoIter<i32> as RustStdType>::provenance()
    );
}

#[test]
fn binary_heap_drain_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::collections::binary_heap::Drain<'static, i32>> as Witness<
        VerusVerifier,
    >>::proof();

    assert_eq!(
        proof.harness,
        "verify_unordered_pair_model_yields_every_element_once"
    );
    assert_eq!(
        proof.provenance,
        <std::collections::binary_heap::Drain<'static, i32> as RustStdType>::provenance()
    );
}

#[test]
fn binary_heap_into_iter_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::collections::binary_heap::IntoIter<i32>> as Witness<
        VerusVerifier,
    >>::proof();

    assert_eq!(
        proof.harness,
        "verify_unordered_pair_model_yields_every_element_once"
    );
    assert_eq!(
        proof.provenance,
        <std::collections::binary_heap::IntoIter<i32> as RustStdType>::provenance()
    );
}

#[test]
fn binary_heap_iter_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::collections::binary_heap::Iter<'static, i32>> as Witness<
        VerusVerifier,
    >>::proof();

    assert_eq!(
        proof.harness,
        "verify_unordered_pair_model_yields_every_element_once"
    );
    assert_eq!(
        proof.provenance,
        <std::collections::binary_heap::Iter<'static, i32> as RustStdType>::provenance()
    );
}

#[test]
fn linked_list_iter_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::collections::linked_list::Iter<'static, i32>> as Witness<
        VerusVerifier,
    >>::proof();

    assert_eq!(
        proof.harness,
        "verify_ordered_pair_into_iter_model_yields_owned_values_in_order"
    );
    assert_eq!(
        proof.provenance,
        <std::collections::linked_list::Iter<'static, i32> as RustStdType>::provenance()
    );
}

#[test]
fn vec_deque_iter_mut_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::collections::vec_deque::IterMut<'static, i32>> as Witness<
        VerusVerifier,
    >>::proof();

    assert_eq!(
        proof.harness,
        "verify_ordered_pair_iter_mut_model_writes_through_in_order"
    );
    assert_eq!(
        proof.provenance,
        <std::collections::vec_deque::IterMut<'static, i32> as RustStdType>::provenance()
    );
}

#[test]
fn linked_list_iter_mut_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::collections::linked_list::IterMut<'static, i32>> as Witness<
            VerusVerifier,
        >>::proof();

    assert_eq!(
        proof.harness,
        "verify_ordered_pair_iter_mut_model_writes_through_in_order"
    );
    assert_eq!(
        proof.provenance,
        <std::collections::linked_list::IterMut<'static, i32> as RustStdType>::provenance()
    );
}

#[test]
fn cstr_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<core::ffi::CStr> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_cstr_excludes_the_terminating_nul_from_to_bytes"
    );
    assert_eq!(
        proof.provenance,
        <core::ffi::CStr as RustStdType>::provenance()
    );
}

#[test]
fn ascii_escape_default_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<core::ascii::EscapeDefault> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_escape_default_model_escapes_a_control_byte"
    );
    assert_eq!(
        proof.provenance,
        <core::ascii::EscapeDefault as RustStdType>::provenance()
    );
}

#[test]
fn from_utf8_error_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::string::FromUtf8Error> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_from_utf8_error_model_recovers_the_original_bytes"
    );
    assert_eq!(
        proof.provenance,
        <std::string::FromUtf8Error as RustStdType>::provenance()
    );
}

#[test]
fn rc_weak_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::rc::Weak<i32>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_weak_model_upgrade_fails_once_the_strong_count_hits_zero"
    );
    assert_eq!(
        proof.provenance,
        <std::rc::Weak<i32> as RustStdType>::provenance()
    );
}

#[test]
fn sync_weak_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::sync::Weak<i32>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_weak_model_upgrade_fails_once_the_strong_count_hits_zero"
    );
    assert_eq!(
        proof.provenance,
        <std::sync::Weak<i32> as RustStdType>::provenance()
    );
}

#[test]
fn ref_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::cell::Ref<'static, i32>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_ref_model_derefs_to_the_borrowed_value"
    );
    assert_eq!(
        proof.provenance,
        <std::cell::Ref<'static, i32> as RustStdType>::provenance()
    );
}

#[test]
fn ref_mut_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::cell::RefMut<'static, i32>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_ref_mut_model_derefs_and_writes_through_to_the_cell"
    );
    assert_eq!(
        proof.provenance,
        <std::cell::RefMut<'static, i32> as RustStdType>::provenance()
    );
}

#[test]
fn decode_utf16_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::char::DecodeUtf16<std::array::IntoIter<u16, 1>>> as Witness<
            VerusVerifier,
        >>::proof();

    assert_eq!(
        proof.harness,
        "verify_decode_utf16_model_round_trips_and_reports_lone_surrogates"
    );
    assert_eq!(
        proof.provenance,
        <std::char::DecodeUtf16<std::array::IntoIter<u16, 1>> as RustStdType>::provenance()
    );
}

#[test]
fn decode_utf16_error_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::char::DecodeUtf16Error> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_decode_utf16_model_round_trips_and_reports_lone_surrogates"
    );
    assert_eq!(
        proof.provenance,
        <std::char::DecodeUtf16Error as RustStdType>::provenance()
    );
}

#[test]
fn to_lowercase_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::char::ToLowercase> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_to_lowercase_model_maps_an_uppercase_ascii_letter"
    );
    assert_eq!(
        proof.provenance,
        <std::char::ToLowercase as RustStdType>::provenance()
    );
}

#[test]
fn to_uppercase_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::char::ToUppercase> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_to_uppercase_model_maps_a_lowercase_ascii_letter"
    );
    assert_eq!(
        proof.provenance,
        <std::char::ToUppercase as RustStdType>::provenance()
    );
}

#[test]
fn escape_debug_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::char::EscapeDebug> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_char_escape_debug_model_escapes_a_newline"
    );
    assert_eq!(
        proof.provenance,
        <std::char::EscapeDebug as RustStdType>::provenance()
    );
}

#[test]
fn escape_default_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::char::EscapeDefault> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_char_escape_default_model_escapes_a_newline"
    );
    assert_eq!(
        proof.provenance,
        <std::char::EscapeDefault as RustStdType>::provenance()
    );
}

#[test]
fn escape_unicode_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::char::EscapeUnicode> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_char_escape_unicode_model_renders_the_codepoint_escape"
    );
    assert_eq!(
        proof.provenance,
        <std::char::EscapeUnicode as RustStdType>::provenance()
    );
}
