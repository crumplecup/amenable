//! `exchange` — spot-checks the generic sidecar carriers: `RawInput`
//! wraps a raw string at the boundary, `Proven<D, P>` couples a parsed
//! descriptor to a proof token for `P`, and `Proven::prove` is the only
//! external mint path (it consumes the input's boundary credential).

use amenable_core::{Evidence, ProofToken};
use amenable_time::{
    CalendarDateDescriptor, CalendarDateValid, Proven, ProvenToken, RawInput, RawTemporalText,
    TemporalInputReceived, TemporalInputToken,
};

#[test]
fn raw_input_carries_the_boundary_text_and_marker() {
    amenable_core::init_tracing();

    let input = RawInput::received("2026-09-09");
    assert_eq!(input.as_str(), "2026-09-09");

    // The primary payload is `Evidence`; the proposition is the trivial
    // "input received" marker.
    assert!(<RawTemporalText as Evidence>::is_root());
    assert_eq!(RawTemporalText::basis(), RawTemporalText::default());
    assert!(<TemporalInputReceived as Evidence>::is_root());
}

#[test]
fn proven_couples_a_descriptor_to_a_proof_token() {
    amenable_core::init_tracing();

    let descriptor = CalendarDateDescriptor::new(2026, 9, 9);
    let proven: Proven<CalendarDateDescriptor, CalendarDateValid> =
        Proven::prove(descriptor, TemporalInputToken::new());

    assert_eq!(proven.descriptor().year(), 2026);
    assert_eq!(proven.descriptor().day(), 9);

    // `ProvenToken<CalendarDateValid>` justifies exactly that proposition.
    fn assert_proposition<T: ProofToken<Proposition = CalendarDateValid>>() {}
    assert_proposition::<ProvenToken<CalendarDateValid>>();
}

#[test]
fn the_input_token_is_a_freely_minted_root() {
    amenable_core::init_tracing();

    // "Input received" is asserted at the boundary — the token needs no
    // prior credential.
    let a = TemporalInputToken::new();
    let b = TemporalInputToken::default();
    assert_eq!(a, b);
}
