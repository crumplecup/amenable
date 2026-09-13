#![cfg(feature = "jiff")]

use amenable_core::{EvidenceLink, Metadata, Standard};
use amenable_ext::{ExtStandard, ExtType};

#[test]
fn jiff_timestamp_emits_derived_provenance_records() {
    amenable_core::init_tracing();
    let provenance = <jiff::Timestamp as ExtType>::provenance();

    // `std::any::type_name::<jiff::Timestamp>()` resolves through the
    // crate's internal defining module, not its public re-export path
    // (`jiff::Timestamp`) -- `ExtType::provenance()` uses it verbatim,
    // same as `RustStdType::provenance()` does for std types.
    assert_eq!(
        provenance.report().to_string(),
        "language.authority_kind: external_standard\n\
language.authority: jiff contributors\n\
language.source_crate: jiff\n\
language.source_module: jiff\n\
source_url: https://docs.rs/jiff/latest/jiff/struct.Timestamp.html\n\
type_name: jiff::timestamp::Timestamp\n\
semantic_summary: A Timestamp is an instant in time represented as a signed count of nanoseconds since the Unix epoch, always in the Unix timescale at a UTC offset of zero."
    );
}

#[test]
fn jiff_zoned_emits_derived_provenance_records() {
    amenable_core::init_tracing();
    let provenance = <jiff::Zoned as ExtType>::provenance();

    assert_eq!(
        provenance.report().to_string(),
        "language.authority_kind: external_standard\n\
language.authority: jiff contributors\n\
language.source_crate: jiff\n\
language.source_module: jiff\n\
source_url: https://docs.rs/jiff/latest/jiff/struct.Zoned.html\n\
type_name: jiff::zoned::Zoned\n\
semantic_summary: A Zoned is a time zone aware instant in time, combining a Timestamp for the precise instant, a civil DateTime for the calendar date and clock time, and a TimeZone for how to apply time zone transitions during arithmetic."
    );
}

#[test]
fn jiff_civil_datetime_emits_derived_provenance_records() {
    amenable_core::init_tracing();
    let provenance = <jiff::civil::DateTime as ExtType>::provenance();

    assert_eq!(
        provenance.report().to_string(),
        "language.authority_kind: external_standard\n\
language.authority: jiff contributors\n\
language.source_crate: jiff\n\
language.source_module: jiff::civil\n\
source_url: https://docs.rs/jiff/latest/jiff/civil/struct.DateTime.html\n\
type_name: jiff::civil::datetime::DateTime\n\
semantic_summary: A civil DateTime is a representation of a datetime in the Gregorian calendar as a pair of a Date and a Time, guaranteed to be valid, and behaves without regard to daylight saving time or time zones."
    );
}

#[test]
fn ext_standard_wrapper_implements_standard() {
    amenable_core::init_tracing();
    let standard = ExtStandard::<jiff::Timestamp>::new();

    assert_eq!(
        standard.report().to_string(),
        Standard::report(&standard).to_string()
    );
    assert!(
        standard
            .report()
            .to_string()
            .contains("type_name: jiff::timestamp::Timestamp")
    );
}

#[test]
fn ext_standard_evidence_is_registered_for_every_jiff_type() {
    amenable_core::init_tracing();
    for name in [
        "amenable_ext::ExtStandard<jiff::Timestamp>",
        "amenable_ext::ExtStandard<jiff::Zoned>",
        "amenable_ext::ExtStandard<jiff::civil::DateTime>",
    ] {
        let found = inventory::iter::<EvidenceLink>().any(|link| link.name() == name);
        assert!(found, "expected an EvidenceLink named `{name}`");
    }
}
