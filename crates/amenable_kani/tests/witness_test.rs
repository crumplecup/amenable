use amenable_core::Witness;
use amenable_kani::KaniVerifier;
use amenable_std::{RustStdStandard, RustStdType};

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
fn string_witness_is_checked_and_still_carries_chain_derived_provenance() {
    let proof = <RustStdStandard<String> as Witness<KaniVerifier>>::proof();

    assert_eq!(proof.harness, "verify_string_utf8_valid");
    assert_eq!(proof.provenance, <String as RustStdType>::provenance());
}
