//! Proof tokens for the temporal exchange surface.
//!
//! [`TemporalInputToken`] is a root token — freely minted at the
//! boundary, the same way `amenable_gaap`'s `PendingToken::new()` is:
//! "input received" is asserted by construction, there is no prior
//! credential to present. [`ProvenToken`] is the sidecar token an
//! exchange's *output* carries; its constructor is `pub(crate)` so only
//! this crate's own `Exchange` impls can mint one. Phase 6 replaces those
//! mint sites with `Establish::establish` once per-proposition
//! `Establish` impls and backend `Witness<V>` proofs exist.

use core::marker::PhantomData;

use amenable_core::Evidence;

/// Root token asserting raw temporal input was received at the boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::TemporalInputReceived")]
pub struct TemporalInputToken(());

impl TemporalInputToken {
    /// Mint the root token. Public and ungated: receiving input is
    /// asserted by construction, not derived from a prior proof.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
    #[must_use]
    pub fn new() -> Self {
        Self(())
    }
}

/// The sidecar token an exchange output carries, parameterised by the
/// proposition `P` it justifies. `pub(crate)` mint only.
#[derive(amenable_derive::ProofToken)]
#[proof_token(proposition = "P")]
pub struct ProvenToken<P: Evidence> {
    proposition: PhantomData<fn() -> P>,
}

impl<P: Evidence> ProvenToken<P> {
    /// Mint an output token. `pub(crate)`: the only external mint path is
    /// [`Proven::prove`](crate::Proven::prove), so a caller cannot forge
    /// a bare token detached from a descriptor.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    pub(crate) fn new() -> Self {
        Self {
            proposition: PhantomData,
        }
    }
}

impl<P: Evidence> core::fmt::Debug for ProvenToken<P> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("ProvenToken")
    }
}

impl<P: Evidence> Clone for ProvenToken<P> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<P: Evidence> Copy for ProvenToken<P> {}
