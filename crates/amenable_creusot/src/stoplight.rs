//! Creusot proof-function content for the `Stoplight`/`Exchange` worked
//! example (see `amenable_kani::stoplight` for the real Kani-side
//! implementation, and `EXCHANGE_PROOF_DERIVATION_PLAN.md` for the design
//! discussion this file is the outcome of).
//!
//! The state/token/sidecar types here are a minimal accommodation model,
//! not the real `amenable_kani::stoplight` types — this crate cannot
//! carry a Cargo dependency on `amenable_kani`/`amenable_std`/`amenable`
//! at all, and never will: verifier backend crates (`amenable_kani`,
//! `amenable_creusot`, `amenable_verus`) are independent and never link
//! to each other directly, full stop — any candidate edge would also
//! close a real dependency cycle (`amenable_std` already optionally
//! depends back on this crate; `amenable` depends on this crate too).
//! What is *not* off-limits, though, and an earlier version of this file
//! wrongly assumed was: `amenable_core`'s own trait family (`Evidence`,
//! `Witness<V>`, `Sidecar<V>`, `Establish<C, V>`, `ProofToken`) — this
//! crate already has a real, unconditional Cargo dependency on
//! `amenable_core`, and none of those trait *definitions* use the
//! specific patterns that actually caused real `creusot-rustc` translator
//! crashes in this exact codebase (`Provenance` impls returning `Box<dyn
//! Iterator<..>>`; a return-position `impl Trait` method; ungated
//! `inventory::collect!`/`inventory::submit!` — see `amenable_std::
//! creusot_witness`'s doc comment and `amenable_std::creusot_gallery`'s
//! own confirmed findings). So the state/token/sidecar types below
//! genuinely implement the real `amenable_core` traits, gated to
//! `CreusotVerifier`, the same way the Kani side implements them gated
//! to `KaniVerifier` — `Provenance` stays left out (this file has no use
//! for it), but `inventory::submit!` does not, below: `#[cfg(not(creusot))]`
//! -gating it precisely in place (confirmed to work in an isolated probe,
//! not assumed) is what lets this crate register its own edges' `Proof
//! Record`s for `amenable_kani::stoplight::creusot_surface()` to query,
//! with no Cargo dependency in either direction — the registry (`amenable_
//! core::ProofRecord`) is the shared mechanism, matching how `kani_
//! surface()` already queries `KaniProofRegistration` the same way.
//!
//! Because the sidecar/establish machinery is now the real thing, not a
//! flattened stand-in, the real exchange bodies below (`Yellow::
//! establish(input.sidecar()); Established::new(Yellow, token)`) use the
//! *same call shape* as `amenable_kani::stoplight::Stoplight::
//! green_to_yellow` et al., differing only in which concrete types they
//! close over (`CreusotVerifier`/local mirror types vs. `KaniVerifier`/
//! the real ones) — closer to a real extraction-with-substitution
//! candidate for a future Step 2 tool than the earlier, more flattened
//! version of this file was.
//!
//! Hand-built for now, to validate the shape for real before any
//! generation tooling exists (matching this project's own "one real
//! example by hand first" discipline).

#[cfg(creusot)]
use amenable_core::{Establish, Evidence, ProofToken, Sidecar, Witness};
#[cfg(creusot)]
use creusot_std::macros::{ensures, requires};

#[cfg(creusot)]
use crate::CreusotVerifier;

/// Sanitized mirror of `amenable_kani::stoplight::Green`.
#[cfg(creusot)]
pub struct Green;

#[cfg(creusot)]
impl Evidence for Green {
    type Basis = Self;
    type Audit = ();

    fn basis() -> Self::Basis {
        Self
    }

    fn audit(&self) -> Self::Audit {}
}

#[cfg(creusot)]
impl Witness<CreusotVerifier> for Green {
    type SupportingEvidence = Self;
    type ProofArtifact = ();

    fn proof() -> Self::ProofArtifact {}
}

/// Sanitized mirror of `amenable_kani::stoplight::Yellow`.
#[cfg(creusot)]
pub struct Yellow;

#[cfg(creusot)]
impl Evidence for Yellow {
    type Basis = Self;
    type Audit = ();

    fn basis() -> Self::Basis {
        Self
    }

    fn audit(&self) -> Self::Audit {}
}

#[cfg(creusot)]
impl Witness<CreusotVerifier> for Yellow {
    type SupportingEvidence = Self;
    type ProofArtifact = ();

    fn proof() -> Self::ProofArtifact {}
}

/// Sanitized mirror of `amenable_kani::stoplight::Red`.
#[cfg(creusot)]
pub struct Red;

#[cfg(creusot)]
impl Evidence for Red {
    type Basis = Self;
    type Audit = ();

    fn basis() -> Self::Basis {
        Self
    }

    fn audit(&self) -> Self::Audit {}
}

#[cfg(creusot)]
impl Witness<CreusotVerifier> for Red {
    type SupportingEvidence = Self;
    type ProofArtifact = ();

    fn proof() -> Self::ProofArtifact {}
}

/// Sanitized mirror of `amenable_kani::stoplight::GreenToken`.
#[cfg(creusot)]
pub struct GreenToken(());

#[cfg(creusot)]
impl Clone for GreenToken {
    fn clone(&self) -> Self {
        GreenToken(())
    }
}

#[cfg(creusot)]
impl Copy for GreenToken {}

#[cfg(creusot)]
impl ProofToken for GreenToken {
    type Proposition = Green;
}

/// Sanitized mirror of `amenable_kani::stoplight::YellowToken`.
#[cfg(creusot)]
pub struct YellowToken(());

#[cfg(creusot)]
impl Clone for YellowToken {
    fn clone(&self) -> Self {
        YellowToken(())
    }
}

#[cfg(creusot)]
impl Copy for YellowToken {}

#[cfg(creusot)]
impl ProofToken for YellowToken {
    type Proposition = Yellow;
}

/// Sanitized mirror of `amenable_kani::stoplight::RedToken`.
#[cfg(creusot)]
pub struct RedToken(());

#[cfg(creusot)]
impl Clone for RedToken {
    fn clone(&self) -> Self {
        RedToken(())
    }
}

#[cfg(creusot)]
impl Copy for RedToken {}

#[cfg(creusot)]
impl ProofToken for RedToken {
    type Proposition = Red;
}

/// Sanitized mirror of `amenable_kani::stoplight::Established<T, Token>`
/// — genuinely generic and genuinely implements `Sidecar<CreusotVerifier>`,
/// unlike an earlier, flattened version of this file.
#[cfg(creusot)]
pub struct Established<T, Token> {
    primary: T,
    token: Token,
}

#[cfg(creusot)]
impl<T, Token> Established<T, Token> {
    fn new(primary: T, token: Token) -> Self {
        Self { primary, token }
    }
}

#[cfg(creusot)]
impl<T, Token> Sidecar<CreusotVerifier> for Established<T, Token>
where
    T: Evidence + Witness<CreusotVerifier>,
    Token: ProofToken<Proposition = T> + Clone,
{
    type Primary = T;
    type Proposition = T;
    type SidecarToken = Token;

    fn primary(&self) -> &Self::Primary {
        &self.primary
    }

    fn sidecar(&self) -> Self::SidecarToken {
        self.token.clone()
    }
}

#[cfg(creusot)]
impl Establish<GreenToken, CreusotVerifier> for Yellow {
    type Token = YellowToken;

    fn establish(_credential: GreenToken) -> Self::Token {
        YellowToken(())
    }
}

#[cfg(creusot)]
impl Establish<YellowToken, CreusotVerifier> for Red {
    type Token = RedToken;

    fn establish(_credential: YellowToken) -> Self::Token {
        RedToken(())
    }
}

#[cfg(creusot)]
impl Establish<RedToken, CreusotVerifier> for Green {
    type Token = GreenToken;

    fn establish(_credential: RedToken) -> Self::Token {
        GreenToken(())
    }
}

amenable_derive::harness! {
    creusot, VERIFY_GREEN_TO_YELLOW_EXCHANGE_SRC, {
        /// Same call shape as `amenable_kani::stoplight::Stoplight::
        /// green_to_yellow`'s real body — the real claim is legitimately
        /// trivial, the same as the Kani-side contract: this function
        /// never panics and always produces a well-formed `Established<
        /// Yellow, YellowToken>`, because there is no branching to falsify
        /// once the type system already enforces that Green's only lawful
        /// successor is Yellow.
        #[requires(true)]
        #[ensures(true)]
        fn green_to_yellow(
            input: Established<Green, GreenToken>,
        ) -> Established<Yellow, YellowToken> {
            let token = Yellow::establish(input.sidecar());
            Established::new(Yellow, token)
        }
    }
}

// Registered here, in this crate, rather than read cross-crate by
// `amenable_kani::stoplight::creusot_surface()` -- verifier backend
// crates never depend on each other; the registry (`amenable_core::
// ProofRecord`, queried the same way `kani_surface()` already queries
// `KaniProofRegistration`) is the shared mechanism, not a direct import.
// `#[cfg(not(creusot))]`, not unconditional: a real, confirmed-working
// alternative to this crate's earlier "move every inventory call to a
// different crate" fix -- gated precisely in place, `cargo creusot`'s
// translator never sees the `static` item `inventory::submit!` expands
// to at all (see `amenable_std::creusot_gallery`'s own confirmed finding
// for the isolated probe this was checked against before applying it
// here for real). `VERIFY_GREEN_TO_YELLOW_EXCHANGE_SRC` itself stays
// ungated (see `amenable_derive::harness!`'s own expansion), so it's
// still available to reference from inside this `#[cfg(not(creusot))]`
// block even though `Green`/`Yellow`/the harness body above are not.
#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_creusot::stoplight::green_to_yellow",
        verifier: "creusot",
        describe: || VERIFY_GREEN_TO_YELLOW_EXCHANGE_SRC.to_owned(),
    }
}

amenable_derive::harness! {
    creusot, VERIFY_YELLOW_TO_RED_EXCHANGE_SRC, {
        /// Same call shape as `amenable_kani::stoplight::Stoplight::
        /// yellow_to_red`'s real body — same legitimately trivial claim as
        /// `green_to_yellow`.
        #[requires(true)]
        #[ensures(true)]
        fn yellow_to_red(
            input: Established<Yellow, YellowToken>,
        ) -> Established<Red, RedToken> {
            let token = Red::establish(input.sidecar());
            Established::new(Red, token)
        }
    }
}

// See `green_to_yellow`'s own registration above for the pattern and
// rationale.
#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_creusot::stoplight::yellow_to_red",
        verifier: "creusot",
        describe: || VERIFY_YELLOW_TO_RED_EXCHANGE_SRC.to_owned(),
    }
}

amenable_derive::harness! {
    creusot, VERIFY_RED_TO_GREEN_EXCHANGE_SRC, {
        /// Same call shape as `amenable_kani::stoplight::Stoplight::
        /// red_to_green`'s real body — the cycle-closing edge, same
        /// legitimately trivial claim.
        #[requires(true)]
        #[ensures(true)]
        fn red_to_green(
            input: Established<Red, RedToken>,
        ) -> Established<Green, GreenToken> {
            let token = Green::establish(input.sidecar());
            Established::new(Green, token)
        }
    }
}

// See `green_to_yellow`'s own registration above for the pattern and
// rationale.
#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_creusot::stoplight::red_to_green",
        verifier: "creusot",
        describe: || VERIFY_RED_TO_GREEN_EXCHANGE_SRC.to_owned(),
    }
}
