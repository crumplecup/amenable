//! The two generic sidecar carriers for the temporal exchange surface.
//!
//! [`RawInput`] is every temporal method's input: a raw string coupled to
//! the trivial "input received" token. [`Proven<D, P>`] is every method's
//! output: the neutral descriptor `D` coupled to a [`ProvenToken<P>`]
//! justifying the proposition `P` (a `proof_composition` composite, or a
//! small per-method conjunction of them). Both are verifier-less
//! `#[derive(Sidecar)]` — the crate is backend-neutral, so no per-backend
//! mirror is needed (`project_creusot_translator_dependency_scope`).

use amenable_core::Evidence;

use crate::{ProvenToken, RawTemporalText, TemporalInputToken};

/// A temporal exchange's input: raw text plus the boundary token.
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::TemporalInputReceived",
    constructor = "pub(crate)"
)]
pub struct RawInput {
    #[sidecar(primary)]
    text: RawTemporalText,
    #[sidecar(token)]
    token: TemporalInputToken,
}

impl RawInput {
    /// Wrap a caller's raw temporal string for an exchange.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(text)))]
    #[must_use]
    pub fn received(text: impl Into<String>) -> Self {
        Self::new(RawTemporalText::new(text), TemporalInputToken::new())
    }

    /// Borrow the raw input string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.text.value()
    }
}

/// A temporal exchange's output: the neutral descriptor `D` plus a
/// [`ProvenToken<P>`] for the proposition `P` it satisfies.
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "P", constructor = "pub(crate)")]
pub struct Proven<D: Evidence, P: Evidence> {
    #[sidecar(primary)]
    descriptor: D,
    #[sidecar(token)]
    token: ProvenToken<P>,
}

impl<D: Evidence, P: Evidence> Proven<D, P> {
    /// Bundle a parsed descriptor with a proof token for `P`, consuming
    /// the input's boundary credential.
    ///
    /// This is Phase 4's honest stand-in for
    /// [`Establish::establish`](amenable_core::Establish::establish):
    /// consuming `credential` by value threads the input token through
    /// (it cannot be replayed), but there is no real `Witness<V>` proof
    /// backing `P` yet. Phase 6 replaces the body with
    /// `P::establish(credential)` once per-`P` `Establish` impls and
    /// backend `Witness<V>` proofs exist.
    #[cfg_attr(
        not(kani),
        tracing::instrument(level = "trace", skip(descriptor, credential))
    )]
    #[must_use]
    pub fn prove(descriptor: D, credential: TemporalInputToken) -> Self {
        let _consumed = credential;
        Self::new(descriptor, ProvenToken::new())
    }

    /// Borrow the neutral descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &D {
        &self.descriptor
    }
}
