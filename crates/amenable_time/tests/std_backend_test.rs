//! The `std::time` canary backend implements the slice of the temporal
//! trait surface that `std::time` can honestly back, and the interface
//! type-checks end to end with those concrete types.

use amenable_core::{Establish, Exchange};
use amenable_time::{
    CanaryVerifier, OrderOffsetEndpointsInput, OrderOffsetEndpointsOutput,
    OrderOffsetEndpointsPreconditions, OrderOffsetEndpointsPreconditionsToken,
    OrderOffsetEndpointsRequest, ParsedDuration, RawInput, StdSystemTime, StdTimeBackend,
    TemporalDurationProps, TemporalError, TemporalErrorKind, TemporalInputToken,
    TemporalInstantProps, TemporalIntervalFactory, TemporalReporter,
};

/// The canary is a `TemporalIntervalFactory` — the whole supertrait
/// bundle (four `Exchange`s + every `Witness<CanaryVerifier>` bound)
/// resolves with concrete types. This is the canary: it fails to compile
/// if the interface drifts.
fn _assert_is_interval_factory<T: TemporalIntervalFactory<CanaryVerifier>>() {}
const _: () = {
    let _ = _assert_is_interval_factory::<StdTimeBackend>;
};

#[test]
fn reporter_declares_the_std_time_capability_slice() {
    let backend = StdTimeBackend;
    assert!(backend.supported_serialization_profiles().is_empty());
    assert_eq!(backend.max_fractional_second_digits(), Some(9));
    assert!(!backend.supports_leap_seconds());
    assert!(!backend.supports_named_zone_round_trip());
    assert!(!backend.supports_end_of_day_twenty_four());
    assert!(!backend.supports_unknown_local_offset());
    assert_eq!(backend.current_tzdb_revision(), None);
}

#[test]
fn native_carriers_are_std_time_types() {
    // The associated types name real `std::time` carriers.
    let _instant: <StdTimeBackend as TemporalInstantProps>::Instant =
        StdSystemTime(std::time::SystemTime::UNIX_EPOCH);
    let _dur: <StdTimeBackend as TemporalDurationProps>::Duration =
        amenable_time::StdDuration(std::time::Duration::from_secs(1));
}

#[test]
fn text_parse_edges_report_unsupported() {
    let backend = StdTimeBackend;
    let out: Result<ParsedDuration, TemporalError> =
        backend.exchange(RawInput::received("P1Y2M3DT4H5M6S"));
    match out {
        Err(err) => assert!(matches!(err.kind(), TemporalErrorKind::Unsupported(_))),
        Ok(_) => panic!("std::time has no ISO 8601 parser"),
    }
}

#[test]
fn order_offset_endpoints_carries_the_proof_forward() {
    let backend = StdTimeBackend;

    let token: OrderOffsetEndpointsPreconditionsToken =
        <OrderOffsetEndpointsPreconditions as Establish<
            TemporalInputToken,
            CanaryVerifier,
        >>::establish(TemporalInputToken::new());
    let input = OrderOffsetEndpointsInput::new(OrderOffsetEndpointsRequest::default(), token);

    let output: OrderOffsetEndpointsOutput =
        backend.exchange(input).expect("canary always accepts");
    // The output carries the re-issued ordering proposition.
    let _ = output.established();
}
