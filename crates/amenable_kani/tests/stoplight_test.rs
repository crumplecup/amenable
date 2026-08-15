//! Exercises the first real `StateMachine`/`Exchange` implementation in
//! the workspace end to end, through the `Sidecar`-coupled `Established`
//! shape: every `exchange` call consumes an already-proven input and
//! returns an already-proven output, carrying the specific token
//! `Establish` minted for that transition rather than a token
//! independently re-minted afterward.

use amenable_core::{Exchange, Sidecar};
use amenable_kani::{
    Established, Green, GreenToken, Red, RedToken, Stoplight, StoplightError, Yellow, YellowToken,
};

#[test]
fn green_to_yellow_exchange_succeeds() -> Result<(), StoplightError> {
    let stoplight = Stoplight;

    let yellow: Established<Yellow, YellowToken> =
        stoplight.exchange(Established::<Green, GreenToken>::root())?;

    assert_eq!(yellow.primary(), &Yellow);
    Ok(())
}

#[test]
fn full_cycle_returns_to_green() -> Result<(), StoplightError> {
    let stoplight = Stoplight;

    let yellow: Established<Yellow, YellowToken> =
        stoplight.exchange(Established::<Green, GreenToken>::root())?;
    assert_eq!(yellow.primary(), &Yellow);

    let red: Established<Red, RedToken> = stoplight.exchange(yellow)?;
    assert_eq!(red.primary(), &Red);

    let green: Established<Green, GreenToken> = stoplight.exchange(red)?;
    assert_eq!(green.primary(), &Green);

    Ok(())
}

// Illegal transitions have no Exchange impl to call at all -- there is no
// runtime check to exercise, only a fact about which impls exist to
// confirm by their absence. A test attempting
// `stoplight.exchange(some_established_yellow)` where an
// `Exchange<Established<Yellow, YellowToken>, Established<Green,
// GreenToken>>` impl doesn't exist would not compile, which is the actual
// claim: illegal transitions are uncompilable, not merely unlikely.

// `Established::new` is private, so a test attempting to hand-construct
// `Established { primary: Yellow, token: YellowToken }` from outside the
// `stoplight` module -- minting a token disconnected from any real
// `Establish` call -- would also not compile. The only way to obtain a
// non-root `Established` value is a lawful `Stoplight::exchange` call.
