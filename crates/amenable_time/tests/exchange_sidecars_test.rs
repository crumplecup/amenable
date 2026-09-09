//! `exchange` — spot-checks the exchange surface wiring: `RawInput` wraps
//! a raw string at the boundary, output tokens are `ProofToken`s for
//! their composite proposition, and the `TemporalExchange` blanket over
//! `TemporalParser` type-checks (its `exchange` body mints the output
//! token through `Establish`). Actually *running* an exchange needs a
//! backend `Witness<V>` proof for `CalendarDateValid`, so that is left to
//! the backend crates.

use amenable_core::{Establish, Evidence, ProofToken, Verifier, Witness};
use amenable_time::{
    CalendarDateDescriptor, CalendarDateValid, CalendarDateValidToken, ParsedCalendarDate,
    RawInput, RawTemporalText, TemporalExchange, TemporalInputReceived, TemporalInputToken,
    TemporalParser, TemporalResult,
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

    let a = TemporalInputToken::new();
    let b = TemporalInputToken::default();
    assert_eq!(a, b);
}

#[test]
fn output_tokens_justify_their_composite_proposition() {
    amenable_core::init_tracing();

    fn assert_token<T: ProofToken<Proposition = CalendarDateValid>>() {}
    assert_token::<CalendarDateValidToken>();
}

/// The `TemporalExchange` blanket over `TemporalParser` type-checks — its
/// body mints `CalendarDateValidToken` via
/// `<CalendarDateValid as Establish<TemporalInputToken, V>>::establish`,
/// gated on the backend's `Witness<V>` proof.
#[test]
fn the_parser_exchange_blanket_is_wired() {
    fn _assert_blanket<T, V>()
    where
        T: TemporalParser,
        V: Verifier,
        CalendarDateValid: Witness<V> + Establish<TemporalInputToken, V>,
        T: TemporalExchange<RawInput, ParsedCalendarDate, V, Error = amenable_time::TemporalError>,
    {
    }
}

/// A trivial in-test parser confirms the raw trait method is the only
/// thing a backend has to implement.
struct StubParser;

impl TemporalParser for StubParser {
    fn parse_calendar_date(&self, _input: &str) -> TemporalResult<CalendarDateDescriptor> {
        Ok(CalendarDateDescriptor::new(2026, 9, 9))
    }
}

#[test]
fn a_backend_only_implements_the_raw_method() {
    amenable_core::init_tracing();

    let parsed = StubParser
        .parse_calendar_date("2026-09-09")
        .expect("stub always parses");
    assert_eq!(parsed.year(), 2026);
}
