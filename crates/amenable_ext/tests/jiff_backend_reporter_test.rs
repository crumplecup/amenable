//! `JiffTimeBackend`'s real jiff-backed `Exchange` bodies — Phase 6 of
//! `docs/AMENABLE_TIME_JIFF_BACKEND_PLAN.md`: `TemporalReporter`'s real
//! capability declaration, every flag checked against jiff's own real
//! source or docs.

#![cfg(feature = "jiff")]

use amenable_ext::JiffTimeBackend;
use amenable_time::TemporalReporter;

#[test]
fn reports_nanosecond_fractional_second_precision() {
    let backend = JiffTimeBackend;
    assert_eq!(backend.max_fractional_second_digits(), Some(9));
}

#[test]
fn does_not_support_leap_seconds() {
    let backend = JiffTimeBackend;
    assert!(!backend.supports_leap_seconds());
}

#[test]
fn does_not_support_the_unknown_local_offset_convention() {
    let backend = JiffTimeBackend;
    assert!(!backend.supports_unknown_local_offset());
}

#[test]
fn supports_named_zone_round_trip_via_the_real_iana_tzdb() {
    let backend = JiffTimeBackend;
    assert!(backend.supports_named_zone_round_trip());
}

#[test]
fn does_not_support_the_end_of_day_twenty_four_form() {
    let backend = JiffTimeBackend;
    assert!(!backend.supports_end_of_day_twenty_four());
}

#[test]
fn reports_no_queryable_tzdb_revision() {
    let backend = JiffTimeBackend;
    assert_eq!(backend.current_tzdb_revision(), None);
}

#[test]
fn declares_no_serialization_profile_until_a_parser_formatter_exists() {
    let backend = JiffTimeBackend;
    assert!(backend.supported_serialization_profiles().is_empty());
}

#[test]
fn capabilities_render_as_a_readable_block() {
    let backend = JiffTimeBackend;
    let rendered = backend.capabilities().to_string();
    assert!(rendered.contains("9"));
}
