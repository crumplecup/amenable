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
//!
//! Also carries [`verus_ensures!`] — the Verus-side counterpart to
//! `amenable_kani::rust_std::macros::kani_ensures!`, discovered and fully
//! documented (including a real, rejected first attempt) in
//! `gallery::ensures_macro_generated`; extracted here for the identical
//! "future real proof shouldn't have to duplicate this" reason.

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

/// Generates the mechanical `Ensures<$verifier>` wiring (spec companion +
/// exec body + `#[verifier::when_used_as_spec]` bridge) `gallery::
/// ensures_contract_bound`/`gallery::stoplight_exchange` first proved out
/// by hand, and `gallery::ensures_macro_generated` confirmed this shape
/// for — see that module's own doc comment for the real, rejected first
/// attempt (a `macro_rules!` macro invoked *inside* the surrounding
/// `verus! {}` block fails: `spec`/`open` are not plain Rust syntax, and
/// ordinary `rustc` macro expansion validates output against plain Rust
/// item grammar before Verus-specific processing is ever reachable) and
/// the real fix (wrap the macro's own output in a *nested* `verus! {}`
/// invocation, called from outside any enclosing one).
///
/// No `evidence`-id/registry argument (unlike `kani_ensures!`'s own
/// `$evidence:literal`) — `amenable_verus` has no `inventory`-backed
/// registry to feed at all yet (`VERUS_EXCHANGE_PROOF_DERIVATION_PLAN.md`'s
/// own open question), so there is nothing real to register today. Takes
/// an explicit `$verifier` type, unlike `kani_ensures!`'s hardcoded
/// `crate::KaniVerifier`: `amenable_kani` has one canonical verifier
/// marker, but this crate has none — every gallery case defines its own
/// local `GalleryVerifier` (see `gallery::evidence_self_referential_root`'s
/// own doc comment for why).
macro_rules! verus_ensures {
    ($ty:ty, $verifier:ty, $spec_fn:ident, $param_ty:ty, |$param:pat_param| $expr:expr) => {
        verus_builtin_macros::verus! {
            pub open spec fn $spec_fn($param: $param_ty) -> bool {
                $expr
            }

            impl crate::Ensures<$verifier> for $ty {
                type Input = $param_ty;
                type Bound = bool;

                #[verifier::when_used_as_spec($spec_fn)]
                fn ensures($param: $param_ty) -> bool {
                    $expr
                }
            }
        }
    };
}

pub(crate) use verus_ensures;
