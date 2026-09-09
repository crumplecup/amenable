//! `exchange` — spot-checks the exchange-surface *shape* `amenable_time`
//! owns: `RawInput` input sidecar, the boundary token, the output sidecar
//! `ParsedCalendarDate`, its `Establish` edge, and the `TemporalParser<V>`
//! bundle (whose contract is "be the parse exchanges"). The `Exchange`
//! impls live in the backend crate, so running an exchange is not
//! exercised here.

use amenable_core::{Establish, Evidence, Exchange, ProofToken, Verifier, Witness};
use amenable_time::{
    CalendarDateValid, CalendarDateValidToken, ParsedCalendarDate, RawInput, RawTemporalText,
    TemporalError, TemporalInputReceived, TemporalInputToken, TemporalParser,
};

#[test]
fn raw_input_carries_the_boundary_text_and_marker() {
    amenable_core::init_tracing();

    let input = RawInput::received("2026-09-09");
    assert_eq!(input.as_str(), "2026-09-09");

    assert!(<RawTemporalText as Evidence>::is_root());
    assert_eq!(RawTemporalText::basis(), RawTemporalText::default());
    assert!(<TemporalInputReceived as Evidence>::is_root());
}

#[test]
fn the_input_token_is_a_freely_minted_root() {
    amenable_core::init_tracing();

    assert_eq!(TemporalInputToken::new(), TemporalInputToken::default());
}

#[test]
fn the_output_token_justifies_its_composite_proposition() {
    amenable_core::init_tracing();

    fn assert_token<Tok: ProofToken<Proposition = CalendarDateValid>>() {}
    assert_token::<CalendarDateValidToken>();
}

/// The `Establish` edge and the `TemporalParser<V>` bundle line up: given
/// a verifier that witnesses `CalendarDateValid`, its output token is
/// establishable from the input token, and any type that is the calendar
/// parse exchange is a `TemporalParser<V>`.
#[test]
fn the_exchange_wiring_type_checks() {
    fn _assert_establish<V>()
    where
        V: Verifier,
        CalendarDateValid:
            Witness<V> + Establish<TemporalInputToken, V, Token = CalendarDateValidToken>,
    {
    }

    fn _assert_bundle<T, V>()
    where
        V: Verifier,
        CalendarDateValid: Witness<V>,
        T: Exchange<RawInput, ParsedCalendarDate, V, Error = TemporalError> + Send + Sync,
        T: TemporalParser<V>,
    {
    }
}
