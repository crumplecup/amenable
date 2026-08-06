#![cfg(feature = "verus")]

use amenable_core::Witness;
use amenable_std::{RustStdStandard, RustStdType, VerusVerifier};

#[expect(
    deprecated,
    reason = "LinesAny is stable, only deprecated in favor of Lines; covering it is a coverage-completeness question, not a call to use it"
)]
type LinesAnyStatic = std::str::LinesAny<'static>;

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
fn parse_int_error_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<core::num::ParseIntError> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_parse_int_error_model_reports_the_kind_of_the_failure"
    );
    assert_eq!(
        proof.provenance,
        <core::num::ParseIntError as RustStdType>::provenance()
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
fn fmt_error_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<std::fmt::Error> as Witness<VerusVerifier>>::proof(),
        <std::fmt::Error as RustStdType>::provenance()
    );
}

#[test]
fn phantom_data_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<std::marker::PhantomData<i32>> as Witness<VerusVerifier>>::proof(),
        <std::marker::PhantomData<i32> as RustStdType>::provenance()
    );
}

#[test]
fn phantom_pinned_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<std::marker::PhantomPinned> as Witness<VerusVerifier>>::proof(),
        <std::marker::PhantomPinned as RustStdType>::provenance()
    );
}

#[test]
fn addr_parse_error_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<std::net::AddrParseError> as Witness<VerusVerifier>>::proof(),
        <std::net::AddrParseError as RustStdType>::provenance()
    );
}

#[test]
fn parse_bool_error_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<std::str::ParseBoolError> as Witness<VerusVerifier>>::proof(),
        <std::str::ParseBoolError as RustStdType>::provenance()
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
fn iter_chain_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::iter::Chain<std::ops::Range<i32>, std::ops::Range<i32>>> as Witness<
        VerusVerifier,
    >>::proof();

    assert_eq!(
        proof.harness,
        "verify_chain_model_sequences_two_iterators_end_to_end"
    );
    assert_eq!(
        proof.provenance,
        <std::iter::Chain<std::ops::Range<i32>, std::ops::Range<i32>> as RustStdType>::provenance()
    );
}

#[test]
fn iter_zip_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::iter::Zip<std::ops::Range<i32>, std::ops::Range<i32>>> as Witness<
            VerusVerifier,
        >>::proof();

    assert_eq!(
        proof.harness,
        "verify_zip_model_pairs_items_from_two_iterators"
    );
    assert_eq!(
        proof.provenance,
        <std::iter::Zip<std::ops::Range<i32>, std::ops::Range<i32>> as RustStdType>::provenance()
    );
}

#[test]
fn iter_enumerate_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::iter::Enumerate<std::ops::Range<i32>>> as Witness<
        VerusVerifier,
    >>::proof();

    assert_eq!(
        proof.harness,
        "verify_enumerate_model_pairs_each_item_with_its_index"
    );
    assert_eq!(
        proof.provenance,
        <std::iter::Enumerate<std::ops::Range<i32>> as RustStdType>::provenance()
    );
}

#[test]
fn iter_rev_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::iter::Rev<std::ops::Range<i32>>> as Witness<VerusVerifier>>::proof();

    assert_eq!(proof.harness, "verify_rev_model_reverses_iteration_order");
    assert_eq!(
        proof.provenance,
        <std::iter::Rev<std::ops::Range<i32>> as RustStdType>::provenance()
    );
}

#[test]
fn iter_skip_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::iter::Skip<std::ops::Range<i32>>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_skip_model_discards_the_first_n_items"
    );
    assert_eq!(
        proof.provenance,
        <std::iter::Skip<std::ops::Range<i32>> as RustStdType>::provenance()
    );
}

#[test]
fn iter_skip_while_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<
        std::iter::SkipWhile<std::ops::Range<i32>, fn(&i32) -> bool>,
    > as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_skip_while_model_discards_items_while_the_predicate_holds"
    );
    assert_eq!(
        proof.provenance,
        <std::iter::SkipWhile<std::ops::Range<i32>, fn(&i32) -> bool> as RustStdType>::provenance()
    );
}

#[test]
fn iter_step_by_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::iter::StepBy<std::ops::Range<i32>>> as Witness<
        VerusVerifier,
    >>::proof();

    assert_eq!(proof.harness, "verify_step_by_model_yields_every_nth_item");
    assert_eq!(
        proof.provenance,
        <std::iter::StepBy<std::ops::Range<i32>> as RustStdType>::provenance()
    );
}

#[test]
fn iter_take_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::iter::Take<std::ops::Range<i32>>> as Witness<VerusVerifier>>::proof();

    assert_eq!(proof.harness, "verify_take_model_yields_at_most_n_items");
    assert_eq!(
        proof.provenance,
        <std::iter::Take<std::ops::Range<i32>> as RustStdType>::provenance()
    );
}

#[test]
fn iter_take_while_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<
        std::iter::TakeWhile<std::ops::Range<i32>, fn(&i32) -> bool>,
    > as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_take_while_model_yields_items_while_the_predicate_holds"
    );
    assert_eq!(
        proof.provenance,
        <std::iter::TakeWhile<std::ops::Range<i32>, fn(&i32) -> bool> as RustStdType>::provenance()
    );
}

#[test]
fn iter_once_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::iter::Once<i32>> as Witness<VerusVerifier>>::proof();

    assert_eq!(proof.harness, "verify_once_model_yields_exactly_one_value");
    assert_eq!(
        proof.provenance,
        <std::iter::Once<i32> as RustStdType>::provenance()
    );
}

#[test]
fn iter_once_with_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::iter::OnceWith<fn() -> i32>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_once_with_model_calls_its_closure_exactly_once"
    );
    assert_eq!(
        proof.provenance,
        <std::iter::OnceWith<fn() -> i32> as RustStdType>::provenance()
    );
}

#[test]
fn iter_repeat_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::iter::Repeat<i32>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_repeat_model_yields_the_same_value_forever"
    );
    assert_eq!(
        proof.provenance,
        <std::iter::Repeat<i32> as RustStdType>::provenance()
    );
}

#[test]
fn iter_repeat_with_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::iter::RepeatWith<fn() -> i32>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_repeat_with_model_calls_its_closure_once_per_item"
    );
    assert_eq!(
        proof.provenance,
        <std::iter::RepeatWith<fn() -> i32> as RustStdType>::provenance()
    );
}

#[test]
fn iter_repeat_n_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::iter::RepeatN<i32>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_repeat_n_model_yields_the_value_exactly_n_times"
    );
    assert_eq!(
        proof.provenance,
        <std::iter::RepeatN<i32> as RustStdType>::provenance()
    );
}

#[test]
fn iter_empty_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::iter::Empty<i32>> as Witness<VerusVerifier>>::proof();

    assert_eq!(proof.harness, "verify_empty_model_yields_nothing");
    assert_eq!(
        proof.provenance,
        <std::iter::Empty<i32> as RustStdType>::provenance()
    );
}

#[test]
fn iter_cycle_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::iter::Cycle<std::ops::Range<i32>>> as Witness<
        VerusVerifier,
    >>::proof();

    assert_eq!(
        proof.harness,
        "verify_cycle_model_repeats_its_sequence_forever"
    );
    assert_eq!(
        proof.provenance,
        <std::iter::Cycle<std::ops::Range<i32>> as RustStdType>::provenance()
    );
}

#[test]
fn iter_fuse_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::iter::Fuse<std::ops::Range<i32>>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_fuse_model_keeps_returning_none_once_exhausted"
    );
    assert_eq!(
        proof.provenance,
        <std::iter::Fuse<std::ops::Range<i32>> as RustStdType>::provenance()
    );
}

#[test]
fn iter_inspect_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::iter::Inspect<std::ops::Range<i32>, fn(&i32)>> as Witness<
        VerusVerifier,
    >>::proof();

    assert_eq!(
        proof.harness,
        "verify_inspect_model_calls_once_per_item_without_changing_values"
    );
    assert_eq!(
        proof.provenance,
        <std::iter::Inspect<std::ops::Range<i32>, fn(&i32)> as RustStdType>::provenance()
    );
}

#[test]
fn iter_peekable_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::iter::Peekable<std::ops::Range<i32>>> as Witness<
        VerusVerifier,
    >>::proof();

    assert_eq!(proof.harness, "verify_peekable_model_peek_does_not_consume");
    assert_eq!(
        proof.provenance,
        <std::iter::Peekable<std::ops::Range<i32>> as RustStdType>::provenance()
    );
}

#[test]
fn iter_scan_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<
        std::iter::Scan<std::ops::Range<i32>, i32, fn(&mut i32, i32) -> Option<i32>>,
    > as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_scan_model_threads_state_through_its_closure"
    );
    assert_eq!(
        proof.provenance,
        <std::iter::Scan<std::ops::Range<i32>, i32, fn(&mut i32, i32) -> Option<i32>> as RustStdType>::provenance()
    );
}

#[test]
fn iter_flat_map_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<
        std::iter::FlatMap<
            std::array::IntoIter<i32, 1>,
            std::ops::Range<i32>,
            fn(i32) -> std::ops::Range<i32>,
        >,
    > as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_flat_map_model_flattens_each_generated_iterator"
    );
    assert_eq!(
        proof.provenance,
        <std::iter::FlatMap<
            std::array::IntoIter<i32, 1>,
            std::ops::Range<i32>,
            fn(i32) -> std::ops::Range<i32>,
        > as RustStdType>::provenance()
    );
}

#[test]
fn iter_flatten_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::iter::Flatten<std::vec::IntoIter<std::ops::Range<i32>>>> as Witness<
        VerusVerifier,
    >>::proof();

    assert_eq!(
        proof.harness,
        "verify_flatten_model_concatenates_the_inner_iterators"
    );
    assert_eq!(
        proof.provenance,
        <std::iter::Flatten<std::vec::IntoIter<std::ops::Range<i32>>> as RustStdType>::provenance()
    );
}

#[test]
fn iter_successors_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::iter::Successors<i32, fn(&i32) -> Option<i32>>> as Witness<
        VerusVerifier,
    >>::proof();

    assert_eq!(
        proof.harness,
        "verify_successors_model_generates_from_the_previous_item"
    );
    assert_eq!(
        proof.provenance,
        <std::iter::Successors<i32, fn(&i32) -> Option<i32>> as RustStdType>::provenance()
    );
}

#[test]
fn iter_from_fn_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::iter::FromFn<fn() -> Option<i32>>> as Witness<
        VerusVerifier,
    >>::proof();

    assert_eq!(
        proof.harness,
        "verify_from_fn_model_yields_until_the_closure_returns_none"
    );
    assert_eq!(
        proof.provenance,
        <std::iter::FromFn<fn() -> Option<i32>> as RustStdType>::provenance()
    );
}

#[test]
fn fmt_alignment_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::fmt::Alignment> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_alignment_model_reaches_the_formatter_from_the_format_spec"
    );
    assert_eq!(
        proof.provenance,
        <std::fmt::Alignment as RustStdType>::provenance()
    );
}

#[test]
fn fmt_formatter_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::fmt::Formatter<'static>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_formatter_model_exposes_the_parsed_width_and_precision"
    );
    assert_eq!(
        proof.provenance,
        <std::fmt::Formatter<'static> as RustStdType>::provenance()
    );
}

#[test]
fn fmt_arguments_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::fmt::Arguments<'static>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_arguments_model_renders_the_same_as_the_value_itself"
    );
    assert_eq!(
        proof.provenance,
        <std::fmt::Arguments<'static> as RustStdType>::provenance()
    );
}

#[test]
fn fmt_from_fn_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<
        std::fmt::FromFn<fn(&mut std::fmt::Formatter<'_>) -> std::fmt::Result>,
    > as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_from_fn_model_forwards_display_to_the_supplied_closure"
    );
    assert_eq!(
        proof.provenance,
        <std::fmt::FromFn<fn(&mut std::fmt::Formatter<'_>) -> std::fmt::Result> as RustStdType>::provenance()
    );
}

#[test]
fn fmt_debug_struct_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::fmt::DebugStruct<'static, 'static>> as Witness<
        VerusVerifier,
    >>::proof();

    assert_eq!(
        proof.harness,
        "verify_debug_struct_model_renders_named_fields"
    );
    assert_eq!(
        proof.provenance,
        <std::fmt::DebugStruct<'static, 'static> as RustStdType>::provenance()
    );
}

#[test]
fn fmt_debug_tuple_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::fmt::DebugTuple<'static, 'static>> as Witness<
        VerusVerifier,
    >>::proof();

    assert_eq!(
        proof.harness,
        "verify_debug_tuple_model_renders_positional_fields"
    );
    assert_eq!(
        proof.provenance,
        <std::fmt::DebugTuple<'static, 'static> as RustStdType>::provenance()
    );
}

#[test]
fn fmt_debug_list_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::fmt::DebugList<'static, 'static>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_debug_list_model_renders_entries_in_brackets"
    );
    assert_eq!(
        proof.provenance,
        <std::fmt::DebugList<'static, 'static> as RustStdType>::provenance()
    );
}

#[test]
fn fmt_debug_set_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::fmt::DebugSet<'static, 'static>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_debug_set_model_renders_entries_in_braces"
    );
    assert_eq!(
        proof.provenance,
        <std::fmt::DebugSet<'static, 'static> as RustStdType>::provenance()
    );
}

#[test]
fn fmt_debug_map_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::fmt::DebugMap<'static, 'static>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_debug_map_model_renders_key_value_pairs"
    );
    assert_eq!(
        proof.provenance,
        <std::fmt::DebugMap<'static, 'static> as RustStdType>::provenance()
    );
}

macro_rules! non_zero_witness_test {
    ($name:ident, $ty:ty, $harness:literal) => {
        #[test]
        fn $name() {
            let proof =
                <RustStdStandard<std::num::NonZero<$ty>> as Witness<VerusVerifier>>::proof();

            assert_eq!(proof.harness, $harness);
            assert_eq!(
                proof.provenance,
                <std::num::NonZero<$ty> as RustStdType>::provenance()
            );
        }
    };
}

non_zero_witness_test!(
    non_zero_i8_witness_is_checked_and_still_carries_chain_derived_provenance,
    i8,
    "verify_non_zero_i8_model_round_trips_iff_nonzero"
);
non_zero_witness_test!(
    non_zero_i16_witness_is_checked_and_still_carries_chain_derived_provenance,
    i16,
    "verify_non_zero_i16_model_round_trips_iff_nonzero"
);
non_zero_witness_test!(
    non_zero_i32_witness_is_checked_and_still_carries_chain_derived_provenance,
    i32,
    "verify_non_zero_i32_model_round_trips_iff_nonzero"
);
non_zero_witness_test!(
    non_zero_i64_witness_is_checked_and_still_carries_chain_derived_provenance,
    i64,
    "verify_non_zero_i64_model_round_trips_iff_nonzero"
);
non_zero_witness_test!(
    non_zero_i128_witness_is_checked_and_still_carries_chain_derived_provenance,
    i128,
    "verify_non_zero_i128_model_round_trips_iff_nonzero"
);
non_zero_witness_test!(
    non_zero_isize_witness_is_checked_and_still_carries_chain_derived_provenance,
    isize,
    "verify_non_zero_isize_model_round_trips_iff_nonzero"
);
non_zero_witness_test!(
    non_zero_u8_witness_is_checked_and_still_carries_chain_derived_provenance,
    u8,
    "verify_non_zero_u8_model_round_trips_iff_nonzero"
);
non_zero_witness_test!(
    non_zero_u16_witness_is_checked_and_still_carries_chain_derived_provenance,
    u16,
    "verify_non_zero_u16_model_round_trips_iff_nonzero"
);
non_zero_witness_test!(
    non_zero_u32_witness_is_checked_and_still_carries_chain_derived_provenance,
    u32,
    "verify_non_zero_u32_model_round_trips_iff_nonzero"
);
non_zero_witness_test!(
    non_zero_u64_witness_is_checked_and_still_carries_chain_derived_provenance,
    u64,
    "verify_non_zero_u64_model_round_trips_iff_nonzero"
);
non_zero_witness_test!(
    non_zero_u128_witness_is_checked_and_still_carries_chain_derived_provenance,
    u128,
    "verify_non_zero_u128_model_round_trips_iff_nonzero"
);
non_zero_witness_test!(
    non_zero_usize_witness_is_checked_and_still_carries_chain_derived_provenance,
    usize,
    "verify_non_zero_usize_model_round_trips_iff_nonzero"
);

#[test]
fn slice_iter_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::slice::Iter<'static, i32>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_iter_model_yields_shared_references_in_order"
    );
    assert_eq!(
        proof.provenance,
        <std::slice::Iter<'static, i32> as RustStdType>::provenance()
    );
}

#[test]
fn slice_iter_mut_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::slice::IterMut<'static, i32>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_iter_mut_model_yields_mutable_references_that_write_through"
    );
    assert_eq!(
        proof.provenance,
        <std::slice::IterMut<'static, i32> as RustStdType>::provenance()
    );
}

#[test]
fn ipv4_addr_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::net::Ipv4Addr> as Witness<VerusVerifier>>::proof();

    assert_eq!(proof.harness, "verify_ipv4_addr_model_octets_round_trip");
    assert_eq!(
        proof.provenance,
        <std::net::Ipv4Addr as RustStdType>::provenance()
    );
}

#[test]
fn ipv6_addr_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::net::Ipv6Addr> as Witness<VerusVerifier>>::proof();

    assert_eq!(proof.harness, "verify_ipv6_addr_model_segments_round_trip");
    assert_eq!(
        proof.provenance,
        <std::net::Ipv6Addr as RustStdType>::provenance()
    );
}

#[test]
fn ip_addr_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::net::IpAddr> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_ip_addr_model_variant_matches_its_kind"
    );
    assert_eq!(
        proof.provenance,
        <std::net::IpAddr as RustStdType>::provenance()
    );
}

#[test]
fn socket_addr_v4_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::net::SocketAddrV4> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_socket_addr_v4_model_round_trips_ip_and_port"
    );
    assert_eq!(
        proof.provenance,
        <std::net::SocketAddrV4 as RustStdType>::provenance()
    );
}

#[test]
fn socket_addr_v6_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::net::SocketAddrV6> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_socket_addr_v6_model_round_trips_all_fields"
    );
    assert_eq!(
        proof.provenance,
        <std::net::SocketAddrV6 as RustStdType>::provenance()
    );
}

#[test]
fn socket_addr_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::net::SocketAddr> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_socket_addr_model_variant_matches_its_kind"
    );
    assert_eq!(
        proof.provenance,
        <std::net::SocketAddr as RustStdType>::provenance()
    );
}

#[test]
fn discriminant_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::mem::Discriminant<Option<i32>>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_discriminant_model_identifies_variant_not_payload"
    );
    assert_eq!(
        proof.provenance,
        <std::mem::Discriminant<Option<i32>> as RustStdType>::provenance()
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

#[test]
fn slice_chunks_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::slice::Chunks<'static, i32>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_chunks_model_yields_non_overlapping_groups_with_a_short_last_chunk"
    );
    assert_eq!(
        proof.provenance,
        <std::slice::Chunks<'static, i32> as RustStdType>::provenance()
    );
}

#[test]
fn slice_chunks_exact_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::slice::ChunksExact<'static, i32>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_chunks_exact_model_discards_a_short_remainder"
    );
    assert_eq!(
        proof.provenance,
        <std::slice::ChunksExact<'static, i32> as RustStdType>::provenance()
    );
}

#[test]
fn slice_chunks_mut_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::slice::ChunksMut<'static, i32>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_chunks_mut_model_writes_through_every_chunk"
    );
    assert_eq!(
        proof.provenance,
        <std::slice::ChunksMut<'static, i32> as RustStdType>::provenance()
    );
}

#[test]
fn slice_chunks_exact_mut_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::slice::ChunksExactMut<'static, i32>> as Witness<
        VerusVerifier,
    >>::proof();

    assert_eq!(
        proof.harness,
        "verify_chunks_exact_mut_model_leaves_the_remainder_untouched"
    );
    assert_eq!(
        proof.provenance,
        <std::slice::ChunksExactMut<'static, i32> as RustStdType>::provenance()
    );
}

#[test]
fn slice_rchunks_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::slice::RChunks<'static, i32>> as Witness<VerusVerifier>>::proof();

    assert_eq!(proof.harness, "verify_rchunks_model_groups_from_the_back");
    assert_eq!(
        proof.provenance,
        <std::slice::RChunks<'static, i32> as RustStdType>::provenance()
    );
}

#[test]
fn slice_rchunks_exact_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::slice::RChunksExact<'static, i32>> as Witness<
        VerusVerifier,
    >>::proof();

    assert_eq!(
        proof.harness,
        "verify_rchunks_exact_model_discards_a_short_remainder_at_the_front"
    );
    assert_eq!(
        proof.provenance,
        <std::slice::RChunksExact<'static, i32> as RustStdType>::provenance()
    );
}

#[test]
fn slice_rchunks_exact_mut_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::slice::RChunksExactMut<'static, i32>> as Witness<
        VerusVerifier,
    >>::proof();

    assert_eq!(
        proof.harness,
        "verify_rchunks_exact_mut_model_leaves_the_front_remainder_untouched"
    );
    assert_eq!(
        proof.provenance,
        <std::slice::RChunksExactMut<'static, i32> as RustStdType>::provenance()
    );
}

#[test]
fn slice_rchunks_mut_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::slice::RChunksMut<'static, i32>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_rchunks_mut_model_writes_through_every_chunk"
    );
    assert_eq!(
        proof.provenance,
        <std::slice::RChunksMut<'static, i32> as RustStdType>::provenance()
    );
}

#[test]
fn slice_windows_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::slice::Windows<'static, i32>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_windows_model_yields_overlapping_slices"
    );
    assert_eq!(
        proof.provenance,
        <std::slice::Windows<'static, i32> as RustStdType>::provenance()
    );
}

#[test]
fn slice_chunk_by_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::slice::ChunkBy<'static, i32, fn(&i32, &i32) -> bool>> as Witness<
            VerusVerifier,
        >>::proof();

    assert_eq!(
        proof.harness,
        "verify_chunk_by_model_groups_adjacent_elements_matching_the_predicate"
    );
    assert_eq!(
        proof.provenance,
        <std::slice::ChunkBy<'static, i32, fn(&i32, &i32) -> bool> as RustStdType>::provenance()
    );
}

#[test]
fn slice_chunk_by_mut_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<
        std::slice::ChunkByMut<'static, i32, fn(&i32, &i32) -> bool>,
    > as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_chunk_by_model_groups_adjacent_elements_matching_the_predicate"
    );
    assert_eq!(
        proof.provenance,
        <std::slice::ChunkByMut<'static, i32, fn(&i32, &i32) -> bool> as RustStdType>::provenance()
    );
}

#[test]
fn slice_split_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::slice::Split<'static, i32, fn(&i32) -> bool>> as Witness<
        VerusVerifier,
    >>::proof();

    assert_eq!(
        proof.harness,
        "verify_split_model_yields_subslices_between_matches"
    );
    assert_eq!(
        proof.provenance,
        <std::slice::Split<'static, i32, fn(&i32) -> bool> as RustStdType>::provenance()
    );
}

#[test]
fn slice_split_mut_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::slice::SplitMut<'static, i32, fn(&i32) -> bool>> as Witness<
            VerusVerifier,
        >>::proof();

    assert_eq!(
        proof.harness,
        "verify_split_mut_model_writes_through_the_first_piece"
    );
    assert_eq!(
        proof.provenance,
        <std::slice::SplitMut<'static, i32, fn(&i32) -> bool> as RustStdType>::provenance()
    );
}

#[test]
fn slice_split_inclusive_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::slice::SplitInclusive<'static, i32, fn(&i32) -> bool>> as Witness<
            VerusVerifier,
        >>::proof();

    assert_eq!(
        proof.harness,
        "verify_split_inclusive_model_keeps_the_match_at_the_end_of_each_piece"
    );
    assert_eq!(
        proof.provenance,
        <std::slice::SplitInclusive<'static, i32, fn(&i32) -> bool> as RustStdType>::provenance()
    );
}

#[test]
fn slice_split_inclusive_mut_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<
        std::slice::SplitInclusiveMut<'static, i32, fn(&i32) -> bool>,
    > as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_split_inclusive_mut_model_keeps_the_match_at_the_end_of_each_piece"
    );
    assert_eq!(
        proof.provenance,
        <std::slice::SplitInclusiveMut<'static, i32, fn(&i32) -> bool> as RustStdType>::provenance(
        )
    );
}

#[test]
fn slice_split_n_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::slice::SplitN<'static, i32, fn(&i32) -> bool>> as Witness<
        VerusVerifier,
    >>::proof();

    assert_eq!(
        proof.harness,
        "verify_split_n_model_caps_the_number_of_pieces"
    );
    assert_eq!(
        proof.provenance,
        <std::slice::SplitN<'static, i32, fn(&i32) -> bool> as RustStdType>::provenance()
    );
}

#[test]
fn slice_split_n_mut_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::slice::SplitNMut<'static, i32, fn(&i32) -> bool>> as Witness<
            VerusVerifier,
        >>::proof();

    assert_eq!(
        proof.harness,
        "verify_split_n_model_caps_the_number_of_pieces"
    );
    assert_eq!(
        proof.provenance,
        <std::slice::SplitNMut<'static, i32, fn(&i32) -> bool> as RustStdType>::provenance()
    );
}

#[test]
fn slice_rsplit_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::slice::RSplit<'static, i32, fn(&i32) -> bool>> as Witness<
        VerusVerifier,
    >>::proof();

    assert_eq!(
        proof.harness,
        "verify_rsplit_model_yields_subslices_from_the_back"
    );
    assert_eq!(
        proof.provenance,
        <std::slice::RSplit<'static, i32, fn(&i32) -> bool> as RustStdType>::provenance()
    );
}

#[test]
fn slice_rsplit_mut_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::slice::RSplitMut<'static, i32, fn(&i32) -> bool>> as Witness<
            VerusVerifier,
        >>::proof();

    assert_eq!(
        proof.harness,
        "verify_rsplit_mut_model_writes_through_the_rearmost_piece"
    );
    assert_eq!(
        proof.provenance,
        <std::slice::RSplitMut<'static, i32, fn(&i32) -> bool> as RustStdType>::provenance()
    );
}

#[test]
fn slice_rsplit_n_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::slice::RSplitN<'static, i32, fn(&i32) -> bool>> as Witness<
        VerusVerifier,
    >>::proof();

    assert_eq!(
        proof.harness,
        "verify_rsplit_n_model_caps_the_number_of_pieces_from_the_back"
    );
    assert_eq!(
        proof.provenance,
        <std::slice::RSplitN<'static, i32, fn(&i32) -> bool> as RustStdType>::provenance()
    );
}

#[test]
fn slice_rsplit_n_mut_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::slice::RSplitNMut<'static, i32, fn(&i32) -> bool>> as Witness<
            VerusVerifier,
        >>::proof();

    assert_eq!(
        proof.harness,
        "verify_rsplit_n_model_caps_the_number_of_pieces_from_the_back"
    );
    assert_eq!(
        proof.provenance,
        <std::slice::RSplitNMut<'static, i32, fn(&i32) -> bool> as RustStdType>::provenance()
    );
}

#[test]
fn slice_escape_ascii_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::slice::EscapeAscii<'static>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_escape_ascii_model_leaves_printable_bytes_unescaped"
    );
    assert_eq!(
        proof.provenance,
        <std::slice::EscapeAscii<'static> as RustStdType>::provenance()
    );
}

#[test]
fn slice_get_disjoint_mut_error_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::slice::GetDisjointMutError> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_get_disjoint_mut_model_rejects_overlap_and_out_of_bounds"
    );
    assert_eq!(
        proof.provenance,
        <std::slice::GetDisjointMutError as RustStdType>::provenance()
    );
}

#[test]
fn str_bytes_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::str::Bytes<'static>> as Witness<VerusVerifier>>::proof();

    assert_eq!(proof.harness, "verify_bytes_model_yields_the_utf8_encoding");
    assert_eq!(
        proof.provenance,
        <std::str::Bytes<'static> as RustStdType>::provenance()
    );
}

#[test]
fn str_char_indices_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::str::CharIndices<'static>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_char_indices_model_pairs_each_char_with_its_byte_offset"
    );
    assert_eq!(
        proof.provenance,
        <std::str::CharIndices<'static> as RustStdType>::provenance()
    );
}

#[test]
fn str_encode_utf16_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::str::EncodeUtf16<'static>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_encode_utf16_model_yields_utf16_code_units"
    );
    assert_eq!(
        proof.provenance,
        <std::str::EncodeUtf16<'static> as RustStdType>::provenance()
    );
}

#[test]
fn str_escape_debug_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::str::EscapeDebug<'static>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_str_escape_debug_model_escapes_control_characters"
    );
    assert_eq!(
        proof.provenance,
        <std::str::EscapeDebug<'static> as RustStdType>::provenance()
    );
}

#[test]
fn str_escape_default_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::str::EscapeDefault<'static>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_str_escape_default_model_escapes_control_characters"
    );
    assert_eq!(
        proof.provenance,
        <std::str::EscapeDefault<'static> as RustStdType>::provenance()
    );
}

#[test]
fn str_escape_unicode_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::str::EscapeUnicode<'static>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_str_escape_unicode_model_renders_the_codepoint_escape"
    );
    assert_eq!(
        proof.provenance,
        <std::str::EscapeUnicode<'static> as RustStdType>::provenance()
    );
}

#[test]
fn str_lines_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::str::Lines<'static>> as Witness<VerusVerifier>>::proof();

    assert_eq!(proof.harness, "verify_lines_model_splits_on_line_endings");
    assert_eq!(
        proof.provenance,
        <std::str::Lines<'static> as RustStdType>::provenance()
    );
}

#[test]
fn str_lines_any_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<LinesAnyStatic> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_lines_any_model_splits_on_any_line_ending"
    );
    assert_eq!(
        proof.provenance,
        <LinesAnyStatic as RustStdType>::provenance()
    );
}

#[test]
fn str_split_ascii_whitespace_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::str::SplitAsciiWhitespace<'static>> as Witness<
        VerusVerifier,
    >>::proof();

    assert_eq!(
        proof.harness,
        "verify_split_ascii_whitespace_model_collapses_runs_of_whitespace"
    );
    assert_eq!(
        proof.provenance,
        <std::str::SplitAsciiWhitespace<'static> as RustStdType>::provenance()
    );
}

#[test]
fn str_split_whitespace_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::str::SplitWhitespace<'static>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_split_whitespace_model_collapses_runs_of_whitespace"
    );
    assert_eq!(
        proof.provenance,
        <std::str::SplitWhitespace<'static> as RustStdType>::provenance()
    );
}

#[test]
fn str_utf8_chunks_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::str::Utf8Chunks<'static>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_utf8_chunks_model_yields_one_chunk_for_wholly_valid_input"
    );
    assert_eq!(
        proof.provenance,
        <std::str::Utf8Chunks<'static> as RustStdType>::provenance()
    );
}

#[test]
fn str_utf8_chunk_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::str::Utf8Chunk<'static>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_utf8_chunk_model_separates_the_valid_prefix_from_invalid_bytes"
    );
    assert_eq!(
        proof.provenance,
        <std::str::Utf8Chunk<'static> as RustStdType>::provenance()
    );
}

#[test]
fn str_utf8_error_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::str::Utf8Error> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_utf8_error_model_reports_the_valid_prefix_length_and_error_span"
    );
    assert_eq!(
        proof.provenance,
        <std::str::Utf8Error as RustStdType>::provenance()
    );
}

#[test]
fn str_split_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::str::Split<'static, char>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_str_split_model_yields_substrings_between_pattern_matches"
    );
    assert_eq!(
        proof.provenance,
        <std::str::Split<'static, char> as RustStdType>::provenance()
    );
}

#[test]
fn str_splitn_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::str::SplitN<'static, char>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_str_splitn_model_limits_to_n_substrings"
    );
    assert_eq!(
        proof.provenance,
        <std::str::SplitN<'static, char> as RustStdType>::provenance()
    );
}

#[test]
fn str_split_inclusive_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::str::SplitInclusive<'static, char>> as Witness<
        VerusVerifier,
    >>::proof();

    assert_eq!(
        proof.harness,
        "verify_str_split_inclusive_model_keeps_the_delimiter_attached"
    );
    assert_eq!(
        proof.provenance,
        <std::str::SplitInclusive<'static, char> as RustStdType>::provenance()
    );
}

#[test]
fn str_rsplit_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::str::RSplit<'static, char>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_str_rsplit_model_yields_substrings_from_the_back"
    );
    assert_eq!(
        proof.provenance,
        <std::str::RSplit<'static, char> as RustStdType>::provenance()
    );
}

#[test]
fn str_rsplitn_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::str::RSplitN<'static, char>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_str_rsplitn_model_limits_to_n_substrings_from_the_back"
    );
    assert_eq!(
        proof.provenance,
        <std::str::RSplitN<'static, char> as RustStdType>::provenance()
    );
}

#[test]
fn str_split_terminator_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::str::SplitTerminator<'static, char>> as Witness<
        VerusVerifier,
    >>::proof();

    assert_eq!(
        proof.harness,
        "verify_str_split_terminator_model_suppresses_a_trailing_empty_substring"
    );
    assert_eq!(
        proof.provenance,
        <std::str::SplitTerminator<'static, char> as RustStdType>::provenance()
    );
}

#[test]
fn str_rsplit_terminator_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::str::RSplitTerminator<'static, char>> as Witness<
        VerusVerifier,
    >>::proof();

    assert_eq!(
        proof.harness,
        "verify_str_rsplit_terminator_model_suppresses_a_trailing_empty_substring_from_the_back"
    );
    assert_eq!(
        proof.provenance,
        <std::str::RSplitTerminator<'static, char> as RustStdType>::provenance()
    );
}

#[test]
fn str_matches_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::str::Matches<'static, char>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_str_matches_model_yields_every_non_overlapping_occurrence"
    );
    assert_eq!(
        proof.provenance,
        <std::str::Matches<'static, char> as RustStdType>::provenance()
    );
}

#[test]
fn str_rmatches_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::str::RMatches<'static, char>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_str_matches_model_yields_every_non_overlapping_occurrence"
    );
    assert_eq!(
        proof.provenance,
        <std::str::RMatches<'static, char> as RustStdType>::provenance()
    );
}

#[test]
fn str_match_indices_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::str::MatchIndices<'static, char>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_str_match_indices_model_pairs_each_match_with_its_byte_offset"
    );
    assert_eq!(
        proof.provenance,
        <std::str::MatchIndices<'static, char> as RustStdType>::provenance()
    );
}

#[test]
fn str_rmatch_indices_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::str::RMatchIndices<'static, char>> as Witness<
        VerusVerifier,
    >>::proof();

    assert_eq!(
        proof.harness,
        "verify_str_rmatch_indices_model_pairs_each_match_with_its_byte_offset_from_the_back"
    );
    assert_eq!(
        proof.provenance,
        <std::str::RMatchIndices<'static, char> as RustStdType>::provenance()
    );
}

#[test]
fn stderr_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<std::io::Stderr> as Witness<VerusVerifier>>::proof(),
        <std::io::Stderr as RustStdType>::provenance()
    );
}

#[test]
fn stderr_lock_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<std::io::StderrLock<'static>> as Witness<VerusVerifier>>::proof(),
        <std::io::StderrLock<'static> as RustStdType>::provenance()
    );
}

#[test]
fn stdin_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<std::io::Stdin> as Witness<VerusVerifier>>::proof(),
        <std::io::Stdin as RustStdType>::provenance()
    );
}

#[test]
fn stdin_lock_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<std::io::StdinLock<'static>> as Witness<VerusVerifier>>::proof(),
        <std::io::StdinLock<'static> as RustStdType>::provenance()
    );
}

#[test]
fn stdout_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<std::io::Stdout> as Witness<VerusVerifier>>::proof(),
        <std::io::Stdout as RustStdType>::provenance()
    );
}

#[test]
fn stdout_lock_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<std::io::StdoutLock<'static>> as Witness<VerusVerifier>>::proof(),
        <std::io::StdoutLock<'static> as RustStdType>::provenance()
    );
}

#[test]
fn buf_reader_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::io::BufReader<&'static [u8]>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_buf_reader_model_reads_the_underlying_bytes"
    );
    assert_eq!(
        proof.provenance,
        <std::io::BufReader<&'static [u8]> as RustStdType>::provenance()
    );
}

#[test]
fn buf_writer_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::io::BufWriter<Vec<u8>>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_buf_writer_model_flushes_to_the_underlying_writer"
    );
    assert_eq!(
        proof.provenance,
        <std::io::BufWriter<Vec<u8>> as RustStdType>::provenance()
    );
}

#[test]
fn io_bytes_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::io::Bytes<&'static [u8]>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_bytes_model_yields_one_byte_at_a_time"
    );
    assert_eq!(
        proof.provenance,
        <std::io::Bytes<&'static [u8]> as RustStdType>::provenance()
    );
}

#[test]
fn into_inner_error_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::io::IntoInnerError<std::io::BufWriter<Vec<u8>>>> as Witness<
            VerusVerifier,
        >>::proof();

    assert_eq!(
        proof.harness,
        "verify_into_inner_error_model_recovers_the_writer_and_the_flush_error"
    );
    assert_eq!(
        proof.provenance,
        <std::io::IntoInnerError<std::io::BufWriter<Vec<u8>>> as RustStdType>::provenance()
    );
}

#[test]
fn line_writer_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::io::LineWriter<Vec<u8>>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_line_writer_model_flushes_on_a_newline_but_not_before_one"
    );
    assert_eq!(
        proof.provenance,
        <std::io::LineWriter<Vec<u8>> as RustStdType>::provenance()
    );
}

#[test]
fn io_lines_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::io::Lines<&'static [u8]>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_lines_model_splits_on_newlines_and_drops_the_terminator"
    );
    assert_eq!(
        proof.provenance,
        <std::io::Lines<&'static [u8]> as RustStdType>::provenance()
    );
}

#[test]
fn pipe_reader_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::io::PipeReader> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_pipe_model_delivers_written_bytes_to_the_paired_reader"
    );
    assert_eq!(
        proof.provenance,
        <std::io::PipeReader as RustStdType>::provenance()
    );
}

#[test]
fn pipe_writer_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::io::PipeWriter> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_pipe_model_delivers_written_bytes_to_the_paired_reader"
    );
    assert_eq!(
        proof.provenance,
        <std::io::PipeWriter as RustStdType>::provenance()
    );
}

#[test]
fn io_split_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::io::Split<&'static [u8]>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_split_model_segments_on_the_given_byte_and_drops_it"
    );
    assert_eq!(
        proof.provenance,
        <std::io::Split<&'static [u8]> as RustStdType>::provenance()
    );
}

#[test]
fn writer_panicked_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::io::WriterPanicked> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_writer_panicked_model_recovers_the_buffered_data"
    );
    assert_eq!(
        proof.provenance,
        <std::io::WriterPanicked as RustStdType>::provenance()
    );
}

#[test]
fn io_empty_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::io::Empty> as Witness<VerusVerifier>>::proof();

    assert_eq!(proof.harness, "verify_empty_model_read_reports_end_of_file");
    assert_eq!(
        proof.provenance,
        <std::io::Empty as RustStdType>::provenance()
    );
}

#[test]
fn io_repeat_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::io::Repeat> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_repeat_model_fills_the_buffer_with_the_given_byte"
    );
    assert_eq!(
        proof.provenance,
        <std::io::Repeat as RustStdType>::provenance()
    );
}

#[test]
fn io_sink_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::io::Sink> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_sink_model_write_reports_full_length_and_discards_content"
    );
    assert_eq!(
        proof.provenance,
        <std::io::Sink as RustStdType>::provenance()
    );
}

#[test]
fn seek_from_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::io::SeekFrom> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_seek_from_model_round_trips_each_variants_offset"
    );
    assert_eq!(
        proof.provenance,
        <std::io::SeekFrom as RustStdType>::provenance()
    );
}

#[test]
fn io_chain_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::io::Chain<&'static [u8], &'static [u8]>> as Witness<
        VerusVerifier,
    >>::proof();

    assert_eq!(
        proof.harness,
        "verify_chain_model_reads_the_first_source_then_the_second"
    );
    assert_eq!(
        proof.provenance,
        <std::io::Chain<&'static [u8], &'static [u8]> as RustStdType>::provenance()
    );
}

#[test]
fn io_cursor_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::io::Cursor<&'static [u8]>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_cursor_model_read_advances_position_and_seek_repositions_it"
    );
    assert_eq!(
        proof.provenance,
        <std::io::Cursor<&'static [u8]> as RustStdType>::provenance()
    );
}

#[test]
fn io_error_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::io::Error> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_error_model_from_error_kind_preserves_the_kind"
    );
    assert_eq!(
        proof.provenance,
        <std::io::Error as RustStdType>::provenance()
    );
}

#[test]
fn io_slice_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::io::IoSlice<'static>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_io_slice_model_derefs_to_the_wrapped_bytes"
    );
    assert_eq!(
        proof.provenance,
        <std::io::IoSlice<'static> as RustStdType>::provenance()
    );
}

#[test]
fn io_slice_mut_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::io::IoSliceMut<'static>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_io_slice_mut_model_derefs_to_and_permits_mutating_the_wrapped_bytes"
    );
    assert_eq!(
        proof.provenance,
        <std::io::IoSliceMut<'static> as RustStdType>::provenance()
    );
}

#[test]
fn io_take_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::io::Take<&'static [u8]>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_take_model_caps_reads_at_the_remaining_limit"
    );
    assert_eq!(
        proof.provenance,
        <std::io::Take<&'static [u8]> as RustStdType>::provenance()
    );
}

#[test]
fn atomic_bool_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::sync::atomic::AtomicBool> as Witness<VerusVerifier>>::proof();

    assert_eq!(proof.harness, "verify_atomic_bool_model_load_store");
    assert_eq!(
        proof.provenance,
        <std::sync::atomic::AtomicBool as RustStdType>::provenance()
    );
}

#[test]
fn atomic_i8_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::sync::atomic::AtomicI8> as Witness<VerusVerifier>>::proof();

    assert_eq!(proof.harness, "verify_atomic_i8_model_load_store");
    assert_eq!(
        proof.provenance,
        <std::sync::atomic::AtomicI8 as RustStdType>::provenance()
    );
}

#[test]
fn atomic_i16_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::sync::atomic::AtomicI16> as Witness<VerusVerifier>>::proof();

    assert_eq!(proof.harness, "verify_atomic_i16_model_load_store");
    assert_eq!(
        proof.provenance,
        <std::sync::atomic::AtomicI16 as RustStdType>::provenance()
    );
}

#[test]
fn atomic_i32_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::sync::atomic::AtomicI32> as Witness<VerusVerifier>>::proof();

    assert_eq!(proof.harness, "verify_atomic_i32_model_load_store");
    assert_eq!(
        proof.provenance,
        <std::sync::atomic::AtomicI32 as RustStdType>::provenance()
    );
}

#[test]
fn atomic_i64_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::sync::atomic::AtomicI64> as Witness<VerusVerifier>>::proof();

    assert_eq!(proof.harness, "verify_atomic_i64_model_load_store");
    assert_eq!(
        proof.provenance,
        <std::sync::atomic::AtomicI64 as RustStdType>::provenance()
    );
}

#[test]
fn atomic_isize_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::sync::atomic::AtomicIsize> as Witness<VerusVerifier>>::proof();

    assert_eq!(proof.harness, "verify_atomic_isize_model_load_store");
    assert_eq!(
        proof.provenance,
        <std::sync::atomic::AtomicIsize as RustStdType>::provenance()
    );
}

#[test]
fn atomic_u8_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::sync::atomic::AtomicU8> as Witness<VerusVerifier>>::proof();

    assert_eq!(proof.harness, "verify_atomic_u8_model_load_store");
    assert_eq!(
        proof.provenance,
        <std::sync::atomic::AtomicU8 as RustStdType>::provenance()
    );
}

#[test]
fn atomic_u16_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::sync::atomic::AtomicU16> as Witness<VerusVerifier>>::proof();

    assert_eq!(proof.harness, "verify_atomic_u16_model_load_store");
    assert_eq!(
        proof.provenance,
        <std::sync::atomic::AtomicU16 as RustStdType>::provenance()
    );
}

#[test]
fn atomic_u32_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::sync::atomic::AtomicU32> as Witness<VerusVerifier>>::proof();

    assert_eq!(proof.harness, "verify_atomic_u32_model_load_store");
    assert_eq!(
        proof.provenance,
        <std::sync::atomic::AtomicU32 as RustStdType>::provenance()
    );
}

#[test]
fn atomic_u64_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::sync::atomic::AtomicU64> as Witness<VerusVerifier>>::proof();

    assert_eq!(proof.harness, "verify_atomic_u64_model_load_store");
    assert_eq!(
        proof.provenance,
        <std::sync::atomic::AtomicU64 as RustStdType>::provenance()
    );
}

#[test]
fn atomic_usize_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::sync::atomic::AtomicUsize> as Witness<VerusVerifier>>::proof();

    assert_eq!(proof.harness, "verify_atomic_usize_model_load_store");
    assert_eq!(
        proof.provenance,
        <std::sync::atomic::AtomicUsize as RustStdType>::provenance()
    );
}

#[test]
fn atomic_ptr_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::sync::atomic::AtomicPtr<i32>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_atomic_ptr_model_load_store_swap_and_compare_exchange"
    );
    assert_eq!(
        proof.provenance,
        <std::sync::atomic::AtomicPtr<i32> as RustStdType>::provenance()
    );
}

#[test]
fn atomic_ordering_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::sync::atomic::Ordering> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_atomic_ordering_model_relaxed_store_is_observable"
    );
    assert_eq!(
        proof.provenance,
        <std::sync::atomic::Ordering as RustStdType>::provenance()
    );
}

#[test]
fn exit_code_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<std::process::ExitCode> as Witness<VerusVerifier>>::proof(),
        <std::process::ExitCode as RustStdType>::provenance()
    );
}

#[test]
fn process_child_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::process::Child> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_child_model_has_a_process_id_and_can_be_waited_on"
    );
    assert_eq!(
        proof.provenance,
        <std::process::Child as RustStdType>::provenance()
    );
}

#[test]
fn child_stderr_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::process::ChildStderr> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_child_stderr_model_captures_what_the_child_wrote_to_stderr"
    );
    assert_eq!(
        proof.provenance,
        <std::process::ChildStderr as RustStdType>::provenance()
    );
}

#[test]
fn child_stdin_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::process::ChildStdin> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_child_stdin_model_is_readable_by_the_child_process"
    );
    assert_eq!(
        proof.provenance,
        <std::process::ChildStdin as RustStdType>::provenance()
    );
}

#[test]
fn child_stdout_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::process::ChildStdout> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_child_stdout_model_captures_what_the_child_wrote_to_stdout"
    );
    assert_eq!(
        proof.provenance,
        <std::process::ChildStdout as RustStdType>::provenance()
    );
}

#[test]
fn process_command_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::process::Command> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_command_model_env_override_is_visible_to_the_spawned_process"
    );
    assert_eq!(
        proof.provenance,
        <std::process::Command as RustStdType>::provenance()
    );
}

#[test]
fn command_args_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::process::CommandArgs<'static>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_command_args_model_reports_the_configured_arguments"
    );
    assert_eq!(
        proof.provenance,
        <std::process::CommandArgs<'static> as RustStdType>::provenance()
    );
}

#[test]
fn command_envs_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::process::CommandEnvs<'static>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_command_envs_model_reports_the_configured_overrides"
    );
    assert_eq!(
        proof.provenance,
        <std::process::CommandEnvs<'static> as RustStdType>::provenance()
    );
}

#[test]
fn exit_status_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::process::ExitStatus> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_exit_status_model_reports_a_nonzero_exit_code"
    );
    assert_eq!(
        proof.provenance,
        <std::process::ExitStatus as RustStdType>::provenance()
    );
}

#[test]
fn process_output_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::process::Output> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_output_model_captures_stdout_and_the_exit_status"
    );
    assert_eq!(
        proof.provenance,
        <std::process::Output as RustStdType>::provenance()
    );
}

#[test]
fn stdio_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::process::Stdio> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_stdio_model_null_discards_the_childs_output_handle"
    );
    assert_eq!(
        proof.provenance,
        <std::process::Stdio as RustStdType>::provenance()
    );
}

#[test]
fn ancestors_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::path::Ancestors<'static>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_ancestors_model_yields_self_then_each_parent_up_to_root"
    );
    assert_eq!(
        proof.provenance,
        <std::path::Ancestors<'static> as RustStdType>::provenance()
    );
}

#[test]
fn path_component_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::path::Component<'static>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_component_model_distinguishes_root_from_normal_segments"
    );
    assert_eq!(
        proof.provenance,
        <std::path::Component<'static> as RustStdType>::provenance()
    );
}

#[test]
fn path_components_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::path::Components<'static>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_components_model_yields_root_then_named_segments_in_order"
    );
    assert_eq!(
        proof.provenance,
        <std::path::Components<'static> as RustStdType>::provenance()
    );
}

#[test]
fn path_iter_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::path::Iter<'static>> as Witness<VerusVerifier>>::proof();

    assert_eq!(proof.harness, "verify_iter_model_yields_the_named_segments");
    assert_eq!(
        proof.provenance,
        <std::path::Iter<'static> as RustStdType>::provenance()
    );
}

#[test]
fn path_display_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::path::Display<'static>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_display_model_renders_a_valid_utf8_path_verbatim"
    );
    assert_eq!(
        proof.provenance,
        <std::path::Display<'static> as RustStdType>::provenance()
    );
}

#[test]
fn path_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::path::Path> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_path_model_derives_extension_file_name_and_parent"
    );
    assert_eq!(
        proof.provenance,
        <std::path::Path as RustStdType>::provenance()
    );
}

#[test]
fn path_buf_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::path::PathBuf> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_path_buf_model_push_pop_and_join_build_the_expected_path"
    );
    assert_eq!(
        proof.provenance,
        <std::path::PathBuf as RustStdType>::provenance()
    );
}

#[test]
fn prefix_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::path::Prefix<'static>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_prefix_model_disk_identifies_the_drive_letter"
    );
    assert_eq!(
        proof.provenance,
        <std::path::Prefix<'static> as RustStdType>::provenance()
    );
}

#[test]
fn prefix_component_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<std::path::PrefixComponent<'static>> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_prefix_component_model_pairs_raw_text_with_parsed_prefix"
    );
    assert_eq!(
        proof.provenance,
        <std::path::PrefixComponent<'static> as RustStdType>::provenance()
    );
}

#[test]
fn strip_prefix_error_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::path::StripPrefixError> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_strip_prefix_error_model_reports_a_non_matching_prefix"
    );
    assert_eq!(
        proof.provenance,
        <std::path::StripPrefixError as RustStdType>::provenance()
    );
}

#[test]
fn dir_builder_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::fs::DirBuilder> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_dir_builder_model_creates_nested_directories_recursively"
    );
    assert_eq!(
        proof.provenance,
        <std::fs::DirBuilder as RustStdType>::provenance()
    );
}

#[test]
fn dir_entry_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::fs::DirEntry> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_dir_entry_model_reports_the_created_files_name_and_path"
    );
    assert_eq!(
        proof.provenance,
        <std::fs::DirEntry as RustStdType>::provenance()
    );
}

#[test]
fn read_dir_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::fs::ReadDir> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_read_dir_model_iterates_every_entry_in_the_directory"
    );
    assert_eq!(
        proof.provenance,
        <std::fs::ReadDir as RustStdType>::provenance()
    );
}

#[test]
fn fs_file_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::fs::File> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_file_model_write_then_read_round_trips_the_bytes"
    );
    assert_eq!(
        proof.provenance,
        <std::fs::File as RustStdType>::provenance()
    );
}

#[test]
fn file_times_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::fs::FileTimes> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_file_times_model_sets_the_recorded_modification_time"
    );
    assert_eq!(
        proof.provenance,
        <std::fs::FileTimes as RustStdType>::provenance()
    );
}

#[test]
fn fs_metadata_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::fs::Metadata> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_metadata_model_reports_the_written_length"
    );
    assert_eq!(
        proof.provenance,
        <std::fs::Metadata as RustStdType>::provenance()
    );
}

#[test]
fn file_type_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::fs::FileType> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_file_type_model_distinguishes_files_from_directories"
    );
    assert_eq!(
        proof.provenance,
        <std::fs::FileType as RustStdType>::provenance()
    );
}

#[test]
fn open_options_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::fs::OpenOptions> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_open_options_model_create_new_rejects_an_existing_file"
    );
    assert_eq!(
        proof.provenance,
        <std::fs::OpenOptions as RustStdType>::provenance()
    );
}

#[test]
fn fs_permissions_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::fs::Permissions> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_permissions_model_readonly_round_trips_through_set_permissions"
    );
    assert_eq!(
        proof.provenance,
        <std::fs::Permissions as RustStdType>::provenance()
    );
}

#[test]
fn fs_try_lock_error_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::fs::TryLockError> as Witness<VerusVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_try_lock_error_model_reports_a_lock_already_held"
    );
    assert_eq!(
        proof.provenance,
        <std::fs::TryLockError as RustStdType>::provenance()
    );
}
