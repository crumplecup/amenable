//! Lawful proof-token exchange machinery.

use crate::{Evidence, Verifier, Witness};

/// Opaque proof token carried through lawful exchanges.
pub trait ProofToken {
    /// Evidence proposition justified by this token.
    type Proposition: Evidence;
}

/// Canonical pairing between a payload and an explicit proof sidecar.
pub trait Sidecar {
    /// The primary payload carried through an exchange.
    type Primary: Evidence;

    /// The proposition carried as the proof sidecar.
    type Proposition: Evidence;

    /// Concrete proof token sidecar.
    type SidecarToken: ProofToken<Proposition = Self::Proposition>;

    /// Borrow the primary payload.
    fn primary(&self) -> &Self::Primary;

    /// Copy out the proof sidecar token.
    fn sidecar(&self) -> Self::SidecarToken;
}

/// Constitutional alias for lawful proof minting from an existing credential.
///
/// Requiring `Witness<V>` here is a compile-time obligation, not a runtime
/// call: establishing a token never invokes a verifier (proving is a
/// different mode than doing), but it does mean an `Establish` impl cannot
/// exist unless a matching `Witness<V>` impl — naming which proof backs
/// it — exists alongside it.
pub trait Establish<C, V: Verifier>: Evidence + Witness<V> + Sized {
    /// Concrete proof token minted for this evidence.
    type Token: ProofToken<Proposition = Self>;

    /// Mint a proof token from a lawful credential.
    #[track_caller]
    fn establish(credential: &C) -> Self::Token;
}

/// Lawful proof-bearing exchange from one sidecar state to another.
pub trait Exchange<Input, Output> {
    /// Proposition required before the exchange may proceed.
    type Precondition: Evidence;

    /// Proposition established by a successful exchange.
    type Postcondition: Evidence;

    /// Concrete proof token required for the precondition.
    type PreconditionToken: ProofToken<Proposition = Self::Precondition>;

    /// Concrete proof token minted for the postcondition.
    type PostconditionToken: ProofToken<Proposition = Self::Postcondition>;

    /// Error surface for failed exchanges.
    type Error;

    /// Perform the exchange, consuming the input sidecar and minting a new one.
    fn exchange(
        &self,
        input: Input,
        proof: Self::PreconditionToken,
    ) -> Result<(Output, Self::PostconditionToken), Self::Error>;
}
