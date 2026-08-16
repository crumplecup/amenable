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
//! to each other directly, full stop. What is *not* off-limits: `amenable_
//! core`'s own trait family (`Evidence`, `Witness<V>`, `Sidecar<V>`,
//! `Establish<C, V>`, `ProofToken`) — this crate already has a real,
//! unconditional Cargo dependency on `amenable_core`, and none of those
//! trait *definitions* use the specific patterns that actually caused real
//! `creusot-rustc` translator crashes in this exact codebase (`Provenance`
//! impls returning `Box<dyn Iterator<..>>`; a return-position `impl
//! Trait` method; ungated `inventory::collect!`/`inventory::submit!` —
//! see `amenable_std::creusot_witness`'s doc comment and `amenable_std::
//! creusot_gallery`'s own confirmed findings). So the state/token/sidecar
//! types below genuinely implement the real `amenable_core` traits, gated
//! to `CreusotVerifier`, the same way the Kani side implements them gated
//! to `KaniVerifier` — `Provenance` stays left out (this file has no use
//! for it).
//!
//! **The three per-edge transition bodies are generated, not hand-written
//! or hand-kept-in-sync.** An earlier version of this file hand-copied
//! `amenable_kani::stoplight`'s real transition logic per edge, needing
//! `stoplight_mirror_consistency_test.rs` to catch drift between the two
//! copies — a real, standing risk this whole `EXCHANGE_PROOF_DERIVATION_
//! PLAN.md` lineage otherwise exists to close. `#[amenable_derive::
//! exchange(..)]` now captures each real edge's transition body verbatim
//! at macro-expansion time and registers it, alongside its real type
//! names, as an `amenable_core::ExchangeEdgeRecord` — safe to do from
//! `amenable_kani` (an ordinary, never-translated Cargo crate) even
//! though this crate could never do the same for itself. `amenable`'s own
//! `emit-creusot-companions` CLI command (`amenable::creusot_export`)
//! reads that registry and *writes* the three `include!`d files below —
//! real, checked-in, `inventory`-free source `cargo creusot` just
//! compiles as ordinary static code, the same "generate from a safe
//! registry query, never call `inventory` inside the translated crate"
//! pattern `amenable::verus_export`/`emit-verus-witnesses` already uses
//! for the witness-composition system. Regenerate with `just
//! generate-creusot` after changing a real Kani-side transition; do not
//! hand-edit `src/generated/*.rs`.
//!
//! The state/token/sidecar type definitions below stay hand-written:
//! stable, one-time accommodation-model infrastructure with far lower
//! drift risk than a transition body's own evolving logic, and (unlike
//! the transition bodies) not something a different backend's real source
//! can be captured verbatim *from* in the first place -- Kani's own
//! `Green`/`Yellow`/`Established<T, Token>` are a different, concrete
//! type from this file's own, even though both names and shapes match by
//! convention.

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

/// Sanitized mirror of `amenable_kani::stoplight::StoplightError` — a
/// real, ordinary, constructible type, matching the real one's own
/// justification exactly: an uninhabited error type (`std::convert::
/// Infallible`) is incompatible with the same class of reconstruction
/// concern (Kani's `stub_verified` needs `Arbitrary`; here, the
/// generated body's own `Ok(..)` still needs *some* concrete `Err` type
/// to name in its signature, even though no generated edge ever
/// constructs one). Needed only because the generated bodies below are
/// the real Kani bodies' own verbatim text, `Ok(..)` wrapper included —
/// an earlier, hand-written version of this file simplified the return
/// type to the bare `Ok` payload and dropped the wrapper entirely, which
/// a *generated, unmodified* body can no longer do without becoming a
/// silently different claim than the one actually captured.
#[cfg(creusot)]
#[derive(Debug, Clone, Copy)]
pub enum StoplightError {
    /// The one variant. Exists so `StoplightError` is an ordinary
    /// constructible type, not so any edge below constructs it.
    NotUsed,
}

// The three per-edge `harness! { .. }` blocks (transition body + verbatim
// source constant) plus their `#[cfg(not(creusot))]`-gated `ProofRecord`
// registrations -- generated by `amenable emit-creusot-companions` from
// `amenable_core::ExchangeEdgeRecord`, not hand-written. See this file's
// own doc comment for the full mechanism and why hand-copying was
// dropped. `include!`, not `mod`: these share this file's own scope
// directly (`Green`/`Yellow`/`Established`/`StoplightError` above,
// already in scope), no `use super::*;`/explicit imports needed in the
// generated files themselves. Regenerate with `just generate-creusot`
// after changing a real Kani-side transition; do not hand-edit the
// included files.
include!("generated/green_to_yellow.rs");
include!("generated/yellow_to_red.rs");
include!("generated/red_to_green.rs");
