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
///
/// Bounding `C` on [`ProofToken`] is the actual obligation gate: the only
/// way to hold a value of a `ProofToken` type is to have obtained one from
/// an earlier lawful `establish()` call (every `ProofToken` implementor's
/// fields are private), so a caller can never mint a token from a bare
/// domain value that never demonstrated anything. Taking `credential` by
/// value (rather than by reference) means minting also consumes the prior
/// token, so it cannot be replayed against a second `establish` call.
pub trait Establish<C, V: Verifier>: Evidence + Witness<V> + Sized
where
    C: ProofToken,
{
    /// Concrete proof token minted for this evidence.
    type Token: ProofToken<Proposition = Self>;

    /// Mint a proof token from a lawful credential.
    #[track_caller]
    fn establish(credential: C) -> Self::Token;
}

/// Lawful proof-bearing exchange from one sidecar state to another.
///
/// `Input` and `Output` are each required to be a [`Sidecar`]: a value that
/// couples an action's payload to its own proof token. That coupling is the
/// shape amenable to formal verification — an exchange only ever consumes
/// an input whose precondition is already proven (carried by the input's
/// own `SidecarToken`, via [`Sidecar::sidecar`]) and only ever produces an
/// output already bundled with the proof of its postcondition (the
/// output's own `SidecarToken`). There is no precondition or postcondition
/// token threaded through `exchange`'s signature separately: both are
/// exactly `Input::Proposition`/`Input::SidecarToken` and
/// `Output::Proposition`/`Output::SidecarToken`, so declaring them again
/// here would just be restating what `Sidecar` already guarantees.
pub trait Exchange<Input, Output>
where
    Input: Sidecar,
    Output: Sidecar,
{
    /// Error surface for failed exchanges.
    type Error;

    /// Perform the exchange, consuming a proven input sidecar and
    /// producing a proven output sidecar.
    fn exchange(&self, input: Input) -> Result<Output, Self::Error>;
}
