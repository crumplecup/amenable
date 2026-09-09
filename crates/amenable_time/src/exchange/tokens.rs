//! The boundary proof token for the temporal exchange surface.
//!
//! [`TemporalInputToken`] is a root token — freely minted at the seam,
//! the same way `amenable_gaap`'s `PendingToken::new()` is: "input
//! received" is asserted by construction, there is no prior credential to
//! present. It is the credential every exchange output's
//! [`Establish`](amenable_core::Establish) impl consumes (see
//! [`super::establish`]).

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
