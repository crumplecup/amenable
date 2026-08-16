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
//! "future real proof shouldn't have to duplicate this" reason. And
//! [`verus_exchange!`] — the Verus-side counterpart to `#[amenable_derive
//! ::exchange(..)]` (`EXCHANGE_PROOF_DERIVATION_PLAN.md`'s Step 3/7),
//! discovered and fully documented (including a real macro-hygiene fix)
//! in `gallery::exchange_macro_generated`.

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

/// Generates the mechanical `Witness<$verifier>` impl (always this exact
/// trivial shape under Verus — there is no separate harness/artifact to
/// report; the proof *is* the `ensures` clause the SMT solver checks, not
/// a captured harness the way Kani's `CalculationProof` is) and the
/// `Exchange<$input_ty, $output_ty, $verifier>` impl, wired to whatever
/// `Ensures<$verifier>` impl `$evidence` already carries (from a
/// preceding [`verus_ensures!`] call, exactly as `amenable_kani::
/// stoplight` keeps `kani_ensures!` adjacent to but separate from
/// `#[amenable_derive::exchange(..)]`). `$body` — the real transition
/// logic, and the only real content here — is spliced in verbatim, never
/// reconstructed.
///
/// `$input_param` (the generated method's parameter *name*, not just its
/// type) is a real, required argument, not a hardcoded `input` — found
/// the hard way in `gallery::exchange_macro_generated`: an identifier
/// written literally in the macro's own template is a different
/// syntactic identifier from anything the caller writes in `$body`, even
/// spelled identically (standard `macro_rules!` mixed-site hygiene), so
/// the parameter name has to come from the same call-site tokens as its
/// uses inside `$body` for both to refer to the same binding.
///
/// Invoked *outside* any enclosing `verus! {}` block, for the identical
/// structural reason [`verus_ensures!`] requires it (see that macro's
/// own doc comment): its expansion wraps its content in a fresh, nested
/// `verus! {}` invocation of its own, and a `macro_rules!` macro's plain
/// output can't itself contain `spec`/`open`/`ensures` syntax the way a
/// directly-authored `verus! {}` body can.
macro_rules! verus_exchange {
    ($self_ty:ty, $input_param:ident: $input_ty:ty, $output_ty:ty, $error_ty:ty, $evidence:ty, $verifier:ty, $body:block) => {
        verus_builtin_macros::verus! {
            impl crate::Witness<$verifier> for $evidence {
                type SupportingEvidence = Self;
                type ProofArtifact = ();

                fn proof() -> Self::ProofArtifact {}
            }

            impl crate::Exchange<$input_ty, $output_ty, $verifier> for $self_ty {
                type Error = $error_ty;

                fn exchange(&self, $input_param: $input_ty) -> (result: ::std::result::Result<$output_ty, $error_ty>)
                    ensures
                        <$evidence as crate::Ensures<$verifier>>::ensures(result),
                $body
            }
        }
    };
}

pub(crate) use verus_exchange;
