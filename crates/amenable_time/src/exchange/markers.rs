//! Boundary text markers for the temporal exchange surface.
//!
//! [`RawTemporalText`] is the payload an input sidecar carries and
//! [`FormattedTemporalText`] the payload a formatter output sidecar
//! carries — both `Evidence` (every `Sidecar::Primary` must be), neither
//! a proof. [`TemporalInputReceived`] is the proposition a `RawInput`
//! sidecar asserts: "a caller handed us this text". Like a citation-only
//! contract, its `Witness<V>` is trivial *but backend-provided* — we
//! can't offer an unconditional guarantee, but the guarantee is trivial
//! to express once the backend is known.

use derive_getters::Getters;
use derive_new::new;

/// The raw input text an exchange consumes — a `Sidecar::Primary`
/// payload, not a proof.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, amenable_derive::Evidence, Getters, new)]
#[evidence(basis = "Self")]
pub struct RawTemporalText {
    /// The verbatim input string.
    #[new(into)]
    value: String,
}

/// The wire text a formatter exchange emits — a `Sidecar::Primary`
/// payload, not a proof (the conformance proof rides in the token).
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, amenable_derive::Evidence, Getters, new)]
#[evidence(basis = "Self")]
pub struct FormattedTemporalText {
    /// The emitted string.
    #[new(into)]
    value: String,
}

/// The proposition a `RawInput` sidecar asserts: raw text was received at
/// the exchange boundary. A root claim, asserted by construction — its
/// `Witness<V>` is a trivial backend-provided impl.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, amenable_derive::Evidence)]
#[evidence(basis = "Self")]
pub struct TemporalInputReceived;
