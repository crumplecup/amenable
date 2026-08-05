#![cfg(feature = "creusot")]

use std::borrow::Cow;
use std::boxed::Box;
use std::cmp::Reverse;
use std::collections::binary_heap::{Drain as BinaryHeapDrain, IntoIter as BinaryHeapIntoIter};
use std::collections::linked_list::IntoIter as LinkedListIntoIter;
use std::collections::vec_deque::{
    Drain as VecDequeDrain, IntoIter as VecDequeIntoIter, Iter as VecDequeIter,
    IterMut as VecDequeIterMut,
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, TryReserveError, VecDeque};
use std::ffi::{CString, FromVecWithNulError, IntoStringError, NulError};
use std::mem::ManuallyDrop;
use std::num::{NonZero, Saturating, Wrapping};
use std::time::Duration;

use amenable_core::Witness;
use amenable_creusot::CreusotVerifier;
use amenable_std::{RustStdStandard, RustStdType};

#[test]
fn bool_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<bool> as Witness<CreusotVerifier>>::proof(),
        <bool as RustStdType>::provenance()
    );
}

#[test]
fn char_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<char> as Witness<CreusotVerifier>>::proof();

    assert_eq!(proof.harness, "verify_char_roundtrip");
    assert_eq!(proof.provenance, <char as RustStdType>::provenance());
}

#[test]
fn string_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<String> as Witness<CreusotVerifier>>::proof();

    assert_eq!(proof.harness, "verify_string_roundtrip");
    assert_eq!(proof.provenance, <String as RustStdType>::provenance());
}

#[test]
fn cow_i32_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<Cow<'static, i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_cow_destructure_recovers_the_wrapped_value"
    );
    assert_eq!(
        proof.provenance,
        <Cow<'static, i32> as RustStdType>::provenance()
    );
}

#[test]
fn btree_map_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<BTreeMap<i32, i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(proof.harness, "verify_btree_map_iterates_in_key_order");
    assert_eq!(
        proof.provenance,
        <BTreeMap<i32, i32> as RustStdType>::provenance()
    );
}

#[test]
fn btree_set_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<BTreeSet<i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(proof.harness, "verify_btree_set_iterates_in_sorted_order");
    assert_eq!(
        proof.provenance,
        <BTreeSet<i32> as RustStdType>::provenance()
    );
}

#[test]
fn binary_heap_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<BinaryHeap<i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_binary_heap_pop_yields_the_maximum_first"
    );
    assert_eq!(
        proof.provenance,
        <BinaryHeap<i32> as RustStdType>::provenance()
    );
}

#[test]
fn binary_heap_drain_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<BinaryHeapDrain<'static, i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_binary_heap_drain_yields_every_pushed_element_once"
    );
    assert_eq!(
        proof.provenance,
        <BinaryHeapDrain<'static, i32> as RustStdType>::provenance()
    );
}

#[test]
fn binary_heap_into_iter_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<BinaryHeapIntoIter<i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_binary_heap_into_iter_yields_every_pushed_element_once"
    );
    assert_eq!(
        proof.provenance,
        <BinaryHeapIntoIter<i32> as RustStdType>::provenance()
    );
}

#[test]
fn linked_list_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<LinkedList<i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_linked_list_is_fifo_through_back_and_front"
    );
    assert_eq!(
        proof.provenance,
        <LinkedList<i32> as RustStdType>::provenance()
    );
}

#[test]
fn linked_list_into_iter_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<LinkedListIntoIter<i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_linked_list_into_iter_yields_owned_values_in_order"
    );
    assert_eq!(
        proof.provenance,
        <LinkedListIntoIter<i32> as RustStdType>::provenance()
    );
}

#[test]
fn try_reserve_error_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<TryReserveError> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_try_reserve_rejects_an_impossible_capacity"
    );
    assert_eq!(
        proof.provenance,
        <TryReserveError as RustStdType>::provenance()
    );
}

#[test]
fn vec_deque_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<VecDeque<i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_vec_deque_pushes_and_pops_from_both_ends"
    );
    assert_eq!(
        proof.provenance,
        <VecDeque<i32> as RustStdType>::provenance()
    );
}

#[test]
fn vec_deque_into_iter_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<VecDequeIntoIter<i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_vec_deque_into_iter_yields_owned_values_in_order"
    );
    assert_eq!(
        proof.provenance,
        <VecDequeIntoIter<i32> as RustStdType>::provenance()
    );
}

#[test]
fn vec_deque_drain_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<VecDequeDrain<'static, i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_vec_deque_drain_removes_and_yields_in_order"
    );
    assert_eq!(
        proof.provenance,
        <VecDequeDrain<'static, i32> as RustStdType>::provenance()
    );
}

#[test]
fn vec_deque_iter_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<VecDequeIter<'static, i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_vec_deque_iter_yields_references_in_order"
    );
    assert_eq!(
        proof.provenance,
        <VecDequeIter<'static, i32> as RustStdType>::provenance()
    );
}

#[test]
fn vec_deque_iter_mut_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof =
        <RustStdStandard<VecDequeIterMut<'static, i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(proof.harness, "verify_vec_deque_iter_mut_writes_through");
    assert_eq!(
        proof.provenance,
        <VecDequeIterMut<'static, i32> as RustStdType>::provenance()
    );
}

#[test]
fn cstring_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<CString> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_cstring_excludes_the_terminator_and_rejects_interior_nul"
    );
    assert_eq!(proof.provenance, <CString as RustStdType>::provenance());
}

#[test]
fn from_vec_with_nul_error_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<FromVecWithNulError> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_from_vec_with_nul_requires_the_nul_only_at_the_end"
    );
    assert_eq!(
        proof.provenance,
        <FromVecWithNulError as RustStdType>::provenance()
    );
}

#[test]
fn into_string_error_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<IntoStringError> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_into_string_error_recovers_the_original_cstring"
    );
    assert_eq!(
        proof.provenance,
        <IntoStringError as RustStdType>::provenance()
    );
}

#[test]
fn nul_error_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<NulError> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_nul_error_reports_the_interior_nuls_position"
    );
    assert_eq!(proof.provenance, <NulError as RustStdType>::provenance());
}

#[test]
fn box_i32_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<Box<i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(proof.harness, "verify_box_new_preserves_the_wrapped_value");
    assert_eq!(proof.provenance, <Box<i32> as RustStdType>::provenance());
}

#[test]
fn duration_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<Duration> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_duration_new_normalizes_nanos_and_carries_into_secs"
    );
    assert_eq!(proof.provenance, <Duration as RustStdType>::provenance());
}

#[test]
fn nonzero_i16_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<NonZero<i16>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(proof.harness, "verify_nonzero_i16_roundtrips");
    assert_eq!(
        proof.provenance,
        <NonZero<i16> as RustStdType>::provenance()
    );
}

#[test]
fn ordering_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<std::cmp::Ordering> as Witness<CreusotVerifier>>::proof();

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
fn wrapping_i32_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<Wrapping<i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(proof.harness, "verify_wrapping_i32_add_wraps");
    assert_eq!(
        proof.provenance,
        <Wrapping<i32> as RustStdType>::provenance()
    );
}

#[test]
fn saturating_i32_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<Saturating<i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(proof.harness, "verify_saturating_i32_add_clamps");
    assert_eq!(
        proof.provenance,
        <Saturating<i32> as RustStdType>::provenance()
    );
}

#[test]
fn int_error_kind_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<core::num::IntErrorKind> as Witness<CreusotVerifier>>::proof();

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
fn try_from_int_error_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<core::num::TryFromIntError> as Witness<CreusotVerifier>>::proof();

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
fn parse_int_error_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<core::num::ParseIntError> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_parse_int_error_reports_the_kind_of_the_failure"
    );
    assert_eq!(
        proof.provenance,
        <core::num::ParseIntError as RustStdType>::provenance()
    );
}

#[test]
fn fp_category_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<core::num::FpCategory> as Witness<CreusotVerifier>>::proof();

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
fn parse_float_error_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<core::num::ParseFloatError> as Witness<CreusotVerifier>>::proof();

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
fn reverse_i32_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<Reverse<i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(proof.harness, "verify_reverse_inverts_comparison");
    assert_eq!(
        proof.provenance,
        <Reverse<i32> as RustStdType>::provenance()
    );
}

#[test]
fn option_i32_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<Option<i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(proof.harness, "verify_option_some_and_none_are_disjoint");
    assert_eq!(proof.provenance, <Option<i32> as RustStdType>::provenance());
}

#[test]
fn result_i32_i32_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<Result<i32, i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(proof.harness, "verify_result_ok_and_err_are_disjoint");
    assert_eq!(
        proof.provenance,
        <Result<i32, i32> as RustStdType>::provenance()
    );
}

#[test]
fn manually_drop_i32_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<ManuallyDrop<i32>> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_manually_drop_derefs_and_into_inner_round_trip"
    );
    assert_eq!(
        proof.provenance,
        <ManuallyDrop<i32> as RustStdType>::provenance()
    );
}
