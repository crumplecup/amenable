//! Lawful proof-token exchange machinery.

use crate::{Evidence, Verifier, Witness};

/// Opaque proof token carried through lawful exchanges.
pub trait ProofToken {
    /// Evidence proposition justified by this token.
    type Proposition: Evidence;
}

/// Canonical pairing between a payload and an explicit proof sidecar.
///
/// `V` is an explicit generic parameter, not an associated type: a shared
/// generic parameter makes verifier consistency a plain type error at
/// every composition site (an [`Exchange`] can't mix a `Sidecar<
/// KaniVerifier>` input with a `Sidecar<CreusotVerifier>` output — `V`
/// simply fails to unify), where an associated type would only be checked
/// wherever a caller remembered to write an explicit equality bound.
///
/// `Proposition: Evidence + Witness<V>` is the actual proof-bearing
/// obligation this trait exists for: the real point of a sidecar is that
/// its proposition is backed by a genuine `Witness<V>` proof, not merely
/// `Evidence`-shaped. `Primary` stays plain `Evidence` — the raw payload
/// doesn't itself need proving, only the proposition carried alongside it
/// does. (An earlier version of this design threaded `V` into `Evidence`
/// itself as a supertrait bound, `Evidence<V>: Witness<V>`; that broke
/// every legitimately verifier-agnostic use of `Evidence` elsewhere in the
/// tree — a generic provenance/audit report has no business requiring a
/// proof under some specific backend just to compile — and was reverted in
/// favor of this narrower, call-site-specific compound bound. See
/// `Evidence`'s own doc comment.)
pub trait Sidecar<V: Verifier> {
    /// The primary payload carried through an exchange.
    type Primary: Evidence;

    /// The proposition carried as the proof sidecar.
    type Proposition: Evidence + Witness<V>;

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
/// `Input` and `Output` are each required to be a [`Sidecar<V>`]: a value
/// that couples an action's payload to its own proof token. That coupling
/// is the shape amenable to formal verification — an exchange only ever
/// consumes an input whose precondition is already proven (carried by the
/// input's own `SidecarToken`, via [`Sidecar::sidecar`]) and only ever
/// produces an output already bundled with the proof of its postcondition
/// (the output's own `SidecarToken`). There is no precondition or
/// postcondition token threaded through `exchange`'s signature separately:
/// both are exactly `Input::Proposition`/`Input::SidecarToken` and
/// `Output::Proposition`/`Output::SidecarToken`, so declaring them again
/// here would just be restating what `Sidecar` already guarantees.
///
/// `Input`/`Output` share the same `V` by construction (both bound on
/// `Sidecar<V>`, one shared parameter) — an impl cannot mix a `Sidecar<
/// KaniVerifier>` input with a `Sidecar<CreusotVerifier>` output; `V`
/// simply fails to unify.
pub trait Exchange<Input, Output, V: Verifier>
where
    Input: Sidecar<V>,
    Output: Sidecar<V>,
{
    /// Error surface for failed exchanges.
    type Error;

    /// Perform the exchange, consuming a proven input sidecar and
    /// producing a proven output sidecar.
    fn exchange(&self, input: Input) -> Result<Output, Self::Error>;
}
