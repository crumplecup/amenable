#![cfg(feature = "jiff")]

use amenable_core::Witness;
use amenable_ext::{ExtStandard, ExtType};
use amenable_kani::KaniVerifier;

#[test]
fn timestamp_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <ExtStandard<jiff::Timestamp> as Witness<KaniVerifier>>::proof(),
        <jiff::Timestamp as ExtType>::provenance()
    );
}

#[test]
fn zoned_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <ExtStandard<jiff::Zoned> as Witness<KaniVerifier>>::proof(),
        <jiff::Zoned as ExtType>::provenance()
    );
}

#[test]
fn civil_datetime_witness_is_trusted_and_carries_chain_derived_provenance() {
    assert_eq!(
        <ExtStandard<jiff::civil::DateTime> as Witness<KaniVerifier>>::proof(),
        <jiff::civil::DateTime as ExtType>::provenance()
    );
}

#[test]
fn a_proof_record_is_registered_for_every_jiff_type() {
    for name in [
        "amenable_ext::ExtStandard<jiff::Timestamp>",
        "amenable_ext::ExtStandard<jiff::Zoned>",
        "amenable_ext::ExtStandard<jiff::civil::DateTime>",
    ] {
        let found = inventory::iter::<amenable_core::ProofRecord>()
            .any(|record| record.evidence() == name && record.verifier() == "kani");
        assert!(found, "expected a kani ProofRecord for `{name}`");
    }
}
