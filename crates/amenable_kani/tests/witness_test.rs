use amenable_core::{Ensures, Witness};
use amenable_kani::KaniVerifier;
#[cfg(unix)]
use amenable_kani::NonNegativeFd;
use amenable_std::{RustStdStandard, RustStdType, ValidUnicodeScalar};

#[test]
fn bool_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <RustStdStandard<bool> as Witness<KaniVerifier>>::proof(),
        <bool as RustStdType>::provenance()
    );
}

#[test]
fn char_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<char> as Witness<KaniVerifier>>::proof();

    assert_eq!(proof.harness, "verify_char_unicode_scalar");
    assert_eq!(proof.provenance, <char as RustStdType>::provenance());
}

#[test]
fn valid_unicode_scalar_reuses_the_char_try_from_harness_and_names_its_bound() {
    let proof = <ValidUnicodeScalar as Witness<KaniVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_char_try_from_fails_exactly_for_surrogates_and_out_of_range"
    );
    assert_eq!(proof.provenance, <char as RustStdType>::provenance());
    assert_eq!(
        <ValidUnicodeScalar as Ensures<KaniVerifier>>::ensures(),
        "value <= 0x0010_FFFF && !(0xD800..=0xDFFF).contains(&value)"
    );
}

#[test]
fn string_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<String> as Witness<KaniVerifier>>::proof();

    assert_eq!(proof.harness, "verify_string_utf8_valid");
    assert_eq!(proof.provenance, <String as RustStdType>::provenance());
}

#[cfg(unix)]
#[test]
fn non_negative_fd_reuses_the_borrowed_fd_harness_and_names_its_bound() {
    let proof = <NonNegativeFd as Witness<KaniVerifier>>::proof();

    assert_eq!(
        proof.harness,
        "verify_borrowed_fd_reports_the_same_raw_value_as_the_owner"
    );
    assert_eq!(proof.provenance, <i32 as RustStdType>::provenance());
    assert_eq!(
        <NonNegativeFd as Ensures<KaniVerifier>>::ensures(),
        "value >= 0"
    );
}
