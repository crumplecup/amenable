#![cfg(all(feature = "jiff", feature = "verus"))]

use amenable_core::{ClassifiedWitness, VerusVerifier, Witness, WitnessSupportSummary};
use amenable_ext::{ExtStandard, ExtType};

/// Compiles only if `T` is a real `ClassifiedWitness<VerusVerifier>` —
/// same pattern the `temporal_composition_test`s use in every backend
/// crate.
fn assert_classified<T: ClassifiedWitness<VerusVerifier>>() {}

#[test]
fn timestamp_witness_is_trusted_and_carries_chain_derived_provenance() {
    amenable_core::init_tracing();
    assert_classified::<ExtStandard<jiff::Timestamp>>();
    assert_eq!(
        <ExtStandard<jiff::Timestamp> as Witness<VerusVerifier>>::proof(),
        <jiff::Timestamp as ExtType>::provenance()
    );
    assert_eq!(
        <ExtStandard<jiff::Timestamp> as Witness<VerusVerifier>>::support(),
        WitnessSupportSummary::trusted_leaf()
    );
}

#[test]
fn zoned_witness_is_trusted_and_carries_chain_derived_provenance() {
    amenable_core::init_tracing();
    assert_classified::<ExtStandard<jiff::Zoned>>();
    assert_eq!(
        <ExtStandard<jiff::Zoned> as Witness<VerusVerifier>>::proof(),
        <jiff::Zoned as ExtType>::provenance()
    );
    assert_eq!(
        <ExtStandard<jiff::Zoned> as Witness<VerusVerifier>>::support(),
        WitnessSupportSummary::trusted_leaf()
    );
}

#[test]
fn civil_datetime_witness_is_trusted_and_carries_chain_derived_provenance() {
    amenable_core::init_tracing();
    assert_classified::<ExtStandard<jiff::civil::DateTime>>();
    assert_eq!(
        <ExtStandard<jiff::civil::DateTime> as Witness<VerusVerifier>>::proof(),
        <jiff::civil::DateTime as ExtType>::provenance()
    );
    assert_eq!(
        <ExtStandard<jiff::civil::DateTime> as Witness<VerusVerifier>>::support(),
        WitnessSupportSummary::trusted_leaf()
    );
}
