#![cfg(feature = "creusot")]

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
