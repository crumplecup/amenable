//! Interval-factory (order-offset-endpoints) precondition/established establish tokens.
//!
//!
//! Output tokens and their [`Establish`](amenable_core::Establish)
//! edges for the zone / conversion / interval factories.
//!
//! Each transition mints two edges: a `*PreconditionsToken`
//! established from the received input (the caller's individual
//! precondition sidecars are folded into the `*Preconditions`
//! composite, whose `Witness<V>` carries the real content), and a
//! `*EstablishedToken` established from that preconditions token.

/// Lawful token: the folded [`OrderOffsetEndpointsPreconditions`](crate::OrderOffsetEndpointsPreconditions)
/// were established from a received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::OrderOffsetEndpointsPreconditions")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::OrderOffsetEndpointsPreconditions"
)]
pub struct OrderOffsetEndpointsPreconditionsToken(());
/// Lawful token: the folded [`OrderOffsetEndpointsEstablished`](crate::OrderOffsetEndpointsEstablished)
/// proofs were re-issued from a proven [`OrderOffsetEndpointsPreconditions`](crate::OrderOffsetEndpointsPreconditions).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::OrderOffsetEndpointsEstablished")]
#[amenable_derive::establish(
    credential = "crate::OrderOffsetEndpointsPreconditionsToken",
    proposition = "crate::OrderOffsetEndpointsEstablished"
)]
pub struct OrderOffsetEndpointsEstablishedToken(());
