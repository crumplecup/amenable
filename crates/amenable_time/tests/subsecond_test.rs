//! The Phase 0 `Exchange`-edge domain types (`amenable_time` side): the
//! `FractionalSecond` payload and the `Received` / `Preserved` state
//! markers. The carrier, tokens, edge, and proofs live in
//! `amenable_kani::time`.

use amenable_core::{Evidence, Standard};
use amenable_time::{FractionalSecond, Preserved, Received};

#[test]
fn fractional_second_carries_its_digits_and_is_evidence() {
    amenable_core::init_tracing();
    let subsecond = FractionalSecond::new("500");

    assert_eq!(subsecond.digits(), "500");
    assert!(<FractionalSecond as Evidence>::is_root());
}

#[test]
fn state_markers_are_asserted_standard_roots() {
    amenable_core::init_tracing();

    assert!(<Received as Evidence>::is_root());
    assert!(<Preserved as Evidence>::is_root());

    assert_eq!(
        Received.report().to_string(),
        "asserted: a fractional second entered a backend exchange, by construction"
    );
    assert!(
        Preserved
            .report()
            .to_string()
            .contains("reachable only via a proven Received -> Preserved exchange")
    );
}
