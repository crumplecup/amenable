#![cfg(feature = "creusot")]

use std::alloc::{Layout, LayoutError};
use std::any::TypeId;
use std::array::{IntoIter as ArrayIntoIter, TryFromSliceError};
use std::borrow::Cow;
use std::boxed::Box;
use std::cell::{
    BorrowError, BorrowMutError, Cell, LazyCell, OnceCell, Ref, RefCell, RefMut, UnsafeCell,
};
use std::char::{
    CharTryFromError, DecodeUtf16, DecodeUtf16Error, ParseCharError, ToLowercase, ToUppercase,
    TryFromCharError,
};
use std::cmp::Reverse;
use std::collections::binary_heap::{Drain as BinaryHeapDrain, IntoIter as BinaryHeapIntoIter};
use std::collections::linked_list::IntoIter as LinkedListIntoIter;
use std::collections::vec_deque::{
    Drain as VecDequeDrain, IntoIter as VecDequeIntoIter, Iter as VecDequeIter,
    IterMut as VecDequeIterMut,
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, TryReserveError, VecDeque};
use std::convert::Infallible;
use std::ffi::{CStr, FromBytesUntilNulError, FromBytesWithNulError};
use std::ffi::{CString, FromVecWithNulError, IntoStringError, NulError};
use std::mem::ManuallyDrop;
use std::net::AddrParseError;
use std::num::{NonZero, Saturating, Wrapping};
use std::rc::Rc;
use std::string::{FromUtf8Error, FromUtf16Error};
use std::sync::Arc;
use std::time::Duration;
use std::vec::{
    Drain as VecDrain, ExtractIf as VecExtractIf, IntoIter as VecIntoIter, Splice as VecSplice, Vec,
};

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
fn vec_i32_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<Vec<i32>> as Witness<CreusotVerifier>>::proof(),
        <Vec<i32> as RustStdType>::provenance()
    );
}

#[test]
fn vec_drain_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<VecDrain<'static, i32>> as Witness<CreusotVerifier>>::proof(),
        <VecDrain<'static, i32> as RustStdType>::provenance()
    );
}

#[test]
fn vec_into_iter_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<VecIntoIter<i32>> as Witness<CreusotVerifier>>::proof(),
        <VecIntoIter<i32> as RustStdType>::provenance()
    );
}

#[test]
fn vec_extract_if_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<VecExtractIf<'static, i32, fn(&mut i32) -> bool>> as Witness<
            CreusotVerifier,
        >>::proof(),
        <VecExtractIf<'static, i32, fn(&mut i32) -> bool> as RustStdType>::provenance()
    );
}

#[test]
fn vec_splice_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<VecSplice<'static, VecIntoIter<i32>>> as Witness<CreusotVerifier>>::proof(
        ),
        <VecSplice<'static, VecIntoIter<i32>> as RustStdType>::provenance()
    );
}

#[test]
fn cell_i32_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<Cell<i32>> as Witness<CreusotVerifier>>::proof(),
        <Cell<i32> as RustStdType>::provenance()
    );
}

#[test]
fn ref_cell_i32_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<RefCell<i32>> as Witness<CreusotVerifier>>::proof(),
        <RefCell<i32> as RustStdType>::provenance()
    );
}

#[test]
fn ref_i32_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<Ref<'static, i32>> as Witness<CreusotVerifier>>::proof(),
        <Ref<'static, i32> as RustStdType>::provenance()
    );
}

#[test]
fn ref_mut_i32_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<RefMut<'static, i32>> as Witness<CreusotVerifier>>::proof(),
        <RefMut<'static, i32> as RustStdType>::provenance()
    );
}

#[test]
fn once_cell_i32_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<OnceCell<i32>> as Witness<CreusotVerifier>>::proof(),
        <OnceCell<i32> as RustStdType>::provenance()
    );
}

#[test]
fn unsafe_cell_i32_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<UnsafeCell<i32>> as Witness<CreusotVerifier>>::proof(),
        <UnsafeCell<i32> as RustStdType>::provenance()
    );
}

#[test]
fn lazy_cell_i32_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<LazyCell<i32, fn() -> i32>> as Witness<CreusotVerifier>>::proof(),
        <LazyCell<i32, fn() -> i32> as RustStdType>::provenance()
    );
}

#[test]
fn borrow_error_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<BorrowError> as Witness<CreusotVerifier>>::proof(),
        <BorrowError as RustStdType>::provenance()
    );
}

#[test]
fn borrow_mut_error_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<BorrowMutError> as Witness<CreusotVerifier>>::proof(),
        <BorrowMutError as RustStdType>::provenance()
    );
}

#[test]
fn char_try_from_error_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<CharTryFromError> as Witness<CreusotVerifier>>::proof(),
        <CharTryFromError as RustStdType>::provenance()
    );
}

#[test]
fn decode_utf16_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<DecodeUtf16<std::array::IntoIter<u16, 1>>> as Witness<
            CreusotVerifier,
        >>::proof(),
        <DecodeUtf16<std::array::IntoIter<u16, 1>> as RustStdType>::provenance()
    );
}

#[test]
fn decode_utf16_error_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<DecodeUtf16Error> as Witness<CreusotVerifier>>::proof(),
        <DecodeUtf16Error as RustStdType>::provenance()
    );
}

#[test]
fn char_escape_debug_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<core::char::EscapeDebug> as Witness<CreusotVerifier>>::proof(),
        <core::char::EscapeDebug as RustStdType>::provenance()
    );
}

#[test]
fn char_escape_default_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<core::char::EscapeDefault> as Witness<CreusotVerifier>>::proof(),
        <core::char::EscapeDefault as RustStdType>::provenance()
    );
}

#[test]
fn char_escape_unicode_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<core::char::EscapeUnicode> as Witness<CreusotVerifier>>::proof(),
        <core::char::EscapeUnicode as RustStdType>::provenance()
    );
}

#[test]
fn parse_char_error_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<ParseCharError> as Witness<CreusotVerifier>>::proof(),
        <ParseCharError as RustStdType>::provenance()
    );
}

#[test]
fn to_lowercase_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<ToLowercase> as Witness<CreusotVerifier>>::proof(),
        <ToLowercase as RustStdType>::provenance()
    );
}

#[test]
fn to_uppercase_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<ToUppercase> as Witness<CreusotVerifier>>::proof(),
        <ToUppercase as RustStdType>::provenance()
    );
}

#[test]
fn try_from_char_error_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<TryFromCharError> as Witness<CreusotVerifier>>::proof(),
        <TryFromCharError as RustStdType>::provenance()
    );
}

#[test]
fn type_id_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<TypeId> as Witness<CreusotVerifier>>::proof(),
        <TypeId as RustStdType>::provenance()
    );
}

#[test]
fn layout_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<Layout> as Witness<CreusotVerifier>>::proof(),
        <Layout as RustStdType>::provenance()
    );
}

#[test]
fn try_from_slice_error_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<TryFromSliceError> as Witness<CreusotVerifier>>::proof(),
        <TryFromSliceError as RustStdType>::provenance()
    );
}

#[test]
fn array_into_iter_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<ArrayIntoIter<i32, 3>> as Witness<CreusotVerifier>>::proof(),
        <ArrayIntoIter<i32, 3> as RustStdType>::provenance()
    );
}

#[test]
fn ascii_escape_default_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<core::ascii::EscapeDefault> as Witness<CreusotVerifier>>::proof(),
        <core::ascii::EscapeDefault as RustStdType>::provenance()
    );
}

#[test]
fn rc_i32_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<Rc<i32>> as Witness<CreusotVerifier>>::proof(),
        <Rc<i32> as RustStdType>::provenance()
    );
}

#[test]
fn rc_weak_i32_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<std::rc::Weak<i32>> as Witness<CreusotVerifier>>::proof(),
        <std::rc::Weak<i32> as RustStdType>::provenance()
    );
}

#[test]
fn string_drain_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<std::string::Drain<'static>> as Witness<CreusotVerifier>>::proof(),
        <std::string::Drain<'static> as RustStdType>::provenance()
    );
}

#[test]
fn from_utf16_error_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<FromUtf16Error> as Witness<CreusotVerifier>>::proof(),
        <FromUtf16Error as RustStdType>::provenance()
    );
}

#[test]
fn from_utf8_error_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<FromUtf8Error> as Witness<CreusotVerifier>>::proof(),
        <FromUtf8Error as RustStdType>::provenance()
    );
}

#[test]
fn arc_i32_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<Arc<i32>> as Witness<CreusotVerifier>>::proof(),
        <Arc<i32> as RustStdType>::provenance()
    );
}

#[test]
fn arc_weak_i32_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<std::sync::Weak<i32>> as Witness<CreusotVerifier>>::proof(),
        <std::sync::Weak<i32> as RustStdType>::provenance()
    );
}

#[test]
fn infallible_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<Infallible> as Witness<CreusotVerifier>>::proof(),
        <Infallible as RustStdType>::provenance()
    );
}

#[test]
fn layout_error_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<LayoutError> as Witness<CreusotVerifier>>::proof(),
        <LayoutError as RustStdType>::provenance()
    );
}

#[test]
fn addr_parse_error_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<AddrParseError> as Witness<CreusotVerifier>>::proof(),
        <AddrParseError as RustStdType>::provenance()
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
fn cstr_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<CStr> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_cstr_excludes_the_terminating_nul_from_to_bytes"
    );
    assert_eq!(proof.provenance, <CStr as RustStdType>::provenance());
}

#[test]
fn from_bytes_until_nul_error_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<FromBytesUntilNulError> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_from_bytes_until_nul_requires_a_nul_byte_somewhere"
    );
    assert_eq!(
        proof.provenance,
        <FromBytesUntilNulError as RustStdType>::provenance()
    );
}

#[test]
fn from_bytes_with_nul_error_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<FromBytesWithNulError> as Witness<CreusotVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_from_bytes_with_nul_requires_the_nul_only_at_the_end"
    );
    assert_eq!(
        proof.provenance,
        <FromBytesWithNulError as RustStdType>::provenance()
    );
}

#[test]
fn c_void_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<core::ffi::c_void> as Witness<CreusotVerifier>>::proof(),
        <core::ffi::c_void as RustStdType>::provenance()
    );
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
