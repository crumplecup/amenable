//! The cross-verifier catalog of proofs a reviewer could assess.

use crate::{AmenableError, AmenableResult, KaniProofRegistration, ProofRecord};
use std::collections::BTreeSet;
use tracing::instrument;

/// One entry in the cross-verifier assessable-proof catalog -- something a
/// reviewer could meaningfully score, whether it's a Kani harness, a
/// Creusot contract, or a Verus spec function. Deliberately thinner than
/// [`KaniProof`]: `queue`/`ensure_registered` only ever need the ID.
#[derive(derive_getters::Getters)]
pub(super) struct RegisteredProof {
    /// The fully-qualified registered proof ID.
    id: String,
}

#[instrument(level = "debug")]
pub(super) fn registered_proofs() -> Vec<RegisteredProof> {
    let mut proofs: Vec<RegisteredProof> = inventory::iter::<KaniProofRegistration>()
        .map(|registration| RegisteredProof {
            id: (registration.proof())().id().clone(),
        })
        .collect();
    proofs.extend(registered_checked_proofs("creusot"));
    proofs.extend(registered_checked_proofs("verus"));
    proofs.sort_unstable_by(|left, right| left.id.cmp(&right.id));
    proofs
}

/// Creusot/Verus proofs are registered as one [`ProofRecord`] per evidence
/// type, not one per harness -- several evidence entries can share the
/// identical harness (an accommodation model backing a whole family of
/// types, say). This collapses that down to one catalog entry per
/// distinct `(verifier, harness)` pair, matching Kani's own one-entry-
/// per-harness granularity, by parsing the `harness: ` line every
/// `CheckedProof`/`VerusCheckedProof::Display` impl renders. Entries with
/// no harness line (a `Standard`-style trusted citation, nothing to
/// independently review) are excluded -- there is no proof there to
/// assess, the same reason Kani's own registry only ever contains real
/// `#[kani::proof]` harnesses.
#[instrument(level = "debug")]
fn registered_checked_proofs(verifier: &str) -> Vec<RegisteredProof> {
    let harnesses: BTreeSet<String> = inventory::iter::<ProofRecord>()
        .filter(|record| record.verifier() == verifier)
        .filter_map(|record| {
            let description = (record.describe())();
            description
                .lines()
                .find_map(|line| line.strip_prefix("harness: "))
                .map(str::to_owned)
        })
        .collect();

    harnesses
        .into_iter()
        .map(|harness| RegisteredProof {
            id: format!("amenable_{verifier}::{harness}"),
        })
        .collect()
}

#[instrument(level = "debug")]
pub(super) fn ensure_registered(proof_id: &str) -> AmenableResult<()> {
    registered_proofs()
        .into_iter()
        .any(|proof| proof.id == proof_id)
        .then_some(())
        .ok_or_else(|| AmenableError::invariant(format!("unknown registered proof ID: {proof_id}")))
}
