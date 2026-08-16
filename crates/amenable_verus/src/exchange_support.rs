//! Companion `#[verifier::external_trait_specification]`s for
//! `amenable_core`'s `Verifier`/`Evidence`/`ProofToken`/`Witness<V>`
//! family, needed by any real (not merely diagnostic) `Sidecar<V>`-based
//! proof under Verus. Real, permanent infrastructure, not a gallery
//! case: `gallery::proof_token_external_trait_bound` is where this
//! exact fix was discovered and its rationale is fully documented (four
//! distinct, real reasons, each confirmed against actual Verus output —
//! see that module's own doc comment); this module is the same fix,
//! extracted so every future real `Exchange`-shaped proof doesn't have
//! to duplicate it.

use verus_builtin_macros::verus;

use crate::{Evidence, ProofToken, Verifier, Witness};

verus! {

#[verifier::external_trait_specification]
pub trait ExVerifier: 'static {
    type ExternalTraitSpecificationFor: Verifier;
}

#[verifier::external_trait_specification]
pub trait ExEvidence {
    type ExternalTraitSpecificationFor: Evidence;
}

#[verifier::external_trait_specification]
pub trait ExProofToken {
    type ExternalTraitSpecificationFor: ProofToken;
    type Proposition: Evidence;
}

#[verifier::external_trait_specification]
pub trait ExWitness<V: Verifier> {
    type ExternalTraitSpecificationFor: Witness<V>;
    type SupportingEvidence: Evidence;
    type ProofArtifact;
}

} // verus!
