//! `Pending`'s own trivial witness. Unlike `stoplight::Green`, which
//! gets its `Witness<KaniVerifier>` impl "for free" from the `Red ->
//! Green` cycle-back edge (`Green` is also an edge *target* in that
//! cycle), nothing in this worked example's initial scope ever targets
//! `Pending` — a transfer only ever starts there, never returns to it.
//! So nothing auto-generates one via `#[amenable_derive::exchange]`;
//! this is hand-written, and honestly trivial: there is no computation
//! to prove about the fact that a new transfer starts `Pending`, the
//! same way there's none for `Green`'s own power-on claim.

use amenable_core::Witness;
use amenable_gaap::Pending;

use crate::KaniVerifier;

impl Witness<KaniVerifier> for Pending {
    type SupportingEvidence = Self;
    type ProofArtifact = ();

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {}
}
