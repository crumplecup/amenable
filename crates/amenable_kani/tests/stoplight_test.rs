//! Exercises the first real `StateMachine`/`Exchange` implementation in
//! the workspace end to end: minting a root state's token has no
//! computational barrier, but performing a transition still goes through
//! `Exchange::exchange` for real.

use amenable_core::Exchange;
use amenable_kani::{Green, GreenToken, Red, Stoplight, Yellow};

#[test]
fn green_to_yellow_exchange_succeeds() -> Result<(), std::convert::Infallible> {
    let stoplight = Stoplight;
    let token = GreenToken::new(Green);

    let (state, _proof) = stoplight.exchange(Green, token)?;

    assert_eq!(state, Yellow);
    Ok(())
}

#[test]
fn full_cycle_returns_to_green() -> Result<(), std::convert::Infallible> {
    let stoplight = Stoplight;

    let (yellow, yellow_token) = stoplight.exchange(Green, GreenToken::new(Green))?;
    assert_eq!(yellow, Yellow);

    let (red, red_token) = stoplight.exchange(Yellow, yellow_token)?;
    assert_eq!(red, Red);

    let (green, _green_token) = stoplight.exchange(Red, red_token)?;
    assert_eq!(green, Green);

    Ok(())
}

// Illegal transitions have no Exchange impl to call at all -- there is no
// runtime check to exercise, only a fact about which impls exist to
// confirm by their absence. A test attempting
// `stoplight.exchange(Yellow, some_token_that_would_have_to_be_a_GreenToken)`
// for Exchange<Yellow, Green> would not compile, which is the actual
// claim: illegal transitions are uncompilable, not merely unlikely.
