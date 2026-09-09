//! [`RawInput`] — every temporal method's input sidecar: a raw string
//! coupled to the trivial "input received" token. Verifier-less
//! `#[derive(Sidecar)]` — the crate is backend-neutral, so no per-backend
//! mirror is needed (`project_creusot_translator_dependency_scope`).
//!
//! Output sidecars are per-method `#[derive(Sidecar)]` structs (one per
//! `elicit_temporal` return tuple) — see [`super::parse`].

use crate::{RawTemporalText, TemporalInputToken};

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
