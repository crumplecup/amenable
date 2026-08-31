//! Gallery case: can a macro generate the *whole* mechanical `Exchange`
//! scaffold under Verus -- not just the `Ensures<V>` wiring `exchange_
//! support::verus_ensures!` already covers (`gallery::ensures_macro_
//! generated`), but the `Witness<V>` impl and the `Exchange<Input,
//! Output, V>` impl itself too, taking only the real transition body as
//! an argument -- the Verus-side counterpart to `EXCHANGE_PROOF_
//! DERIVATION_PLAN.md`'s Step 3/7 (`#[amenable_derive::exchange(..)]`,
//! which generates the identical set of items for Kani: `Witness<V>`,
//! the registry entry, the `Exchange` delegation, and (Step 7) the DFCC
//! contract attribute, leaving only the method body hand-authored)?
//!
//! `verus_ensures!` closing the contract-routing gap was real progress,
//! but left every other real edge's `Witness<GalleryVerifier>` impl (a
//! fixed, always-identical three lines: `type SupportingEvidence = Self;
//! type ProofArtifact = (); fn proof() -> Self::ProofArtifact {}`) and
//! `Exchange<..>` impl scaffold (`type Error = ..; fn exchange(&self,
//! input: ..) -> (result: ..) ensures .. { <the only real content> }`)
//! copy-pasted by hand per edge -- exactly the kind of mechanical,
//! drift-prone repetition this whole plan lineage exists to close.
//!
//! ## Disposition
//!
//! **Confirmed working, with one real hygiene fix along the way.**
//! `verus_exchange!` (defined below) takes the self/input/output/error/
//! evidence/verifier types plus the real transition body as a
//! `$body:block` argument, and expands to both the `Witness<V>` impl and
//! the `Exchange<Input, Output, V>` impl -- the body spliced in verbatim,
//! not reconstructed. `EnsuresBlue`'s own `Exchange` edge (`gallery::
//! ensures_macro_generated`) stays untouched; this file builds a second,
//! independent minimal state pair (`ExchangeRoot -> ExchangeNext`)
//! entirely through `verus_exchange!` plus a preceding `verus_ensures!`
//! call, mirroring the real split `amenable_kani::stoplight` uses
//! between `kani_ensures!` (the predicate, hand-authored) and `#[
//! amenable_derive::exchange(..)]` (the mechanical scaffold).
//!
//! **What was tried and rejected.** The first shape wrote the generated
//! method's parameter literally as `fn exchange(&self, input: $input_ty)`
//! inside the macro's own template, matching every real hand-written
//! edge's own naming. Real, immediate rejection when the caller's
//! `$body` referenced `input`: `` cannot find value `input` in this
//! scope ``, with rustc's own diagnostic naming the cause precisely --
//! "an identifier with the same name is defined here, but is not
//! accessible due to macro hygiene." Standard `macro_rules!` mixed-site
//! hygiene: an identifier written literally in the macro's own
//! definition (the `input` in `fn exchange(&self, input: ..)`) is a
//! *different* syntactic identifier from anything the caller writes,
//! even if spelled identically -- only tokens the caller actually
//! supplies (captured via a metavariable) carry the caller's own hygiene
//! context. Fixed by taking the parameter name itself as a macro
//! argument (`$input_param:ident: $input_ty:ty`, invoked as `input:
//! GalleryExchanged<..>` below) rather than hardcoding it -- now both the
//! generated parameter declaration and the caller's own uses of it
//! inside `$body` come from the same call-site tokens, so they refer to
//! the same identifier.
//!
//! Builds directly on `gallery::ensures_macro_generated`'s own finding
//! (a `macro_rules!` macro generating Verus-native items has to wrap its
//! own output in a *nested* `verus! { .. }` invocation, called from
//! *outside* any enclosing one) -- confirmed to extend cleanly to a
//! *full trait impl with a real, non-trivial body* referencing other
//! items (`Establish::establish`, `Sidecar::sidecar`) from the calling
//! module's own scope, not just a single-expression `spec fn`.
//!
//! **Verified non-vacuous**, same discipline as every other case: a real
//! bug (`Err(())` swapped in for the macro-generated edge's real `Ok(..)`
//! body) produced a real, precise `postcondition not satisfied` failure
//! at the macro-generated `ensures` clause's own call site, confirming
//! the generated `Exchange` impl's contract is a real, checked claim on
//! the spliced-in body -- not vacuous, and not silently dropped by the
//! macro. Reverted and re-verified clean.

use verus_builtin_macros::verus;

use crate::{Establish, Evidence, ProofToken, Sidecar, Witness};

use super::support::GalleryVerifier;

/// Local copy of `exchange_support::verus_ensures!` -- see this crate's
/// `gallery::ensures_macro_generated` for where this shape was
/// discovered and its own doc comment for why each gallery investigation
/// owns local fixtures rather than importing shared ones.
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

/// Generates the mechanical `Witness<$verifier>` impl (always this exact
/// trivial shape under Verus -- there is no separate harness/artifact to
/// report; the proof *is* the `ensures` clause the SMT solver checks, not
/// a captured harness the way Kani's `CalculationProof` is) and the
/// `Exchange<$input_ty, $output_ty, $verifier>` impl, wired to whatever
/// `Ensures<$verifier>` impl `$evidence` already carries (from a
/// preceding `verus_ensures!` call, exactly as `amenable_kani::stoplight`
/// keeps `kani_ensures!` adjacent to but separate from `#[amenable_
/// derive::exchange(..)]`). `$body` -- the real transition logic, and
/// the only real content here -- is spliced in verbatim, never
/// reconstructed. Invoked *outside* any enclosing `verus! {}` block, for
/// the identical structural reason `verus_ensures!` requires it (see
/// that macro's own doc comment).
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

verus! {

pub struct ExchangeRoot;

impl Evidence for ExchangeRoot {
    type Basis = Self;
    type Audit = ();

    fn basis() -> Self::Basis {
        ExchangeRoot
    }

    fn audit(&self) -> Self::Audit {}

    fn is_root() -> bool {
        true
    }
}

impl Witness<GalleryVerifier> for ExchangeRoot {
    type SupportingEvidence = Self;
    type ProofArtifact = ();

    fn proof() -> Self::ProofArtifact {}
}

pub struct ExchangeNext;

impl Evidence for ExchangeNext {
    type Basis = ExchangeRoot;
    type Audit = ();

    fn basis() -> Self::Basis {
        ExchangeRoot
    }

    fn audit(&self) -> Self::Audit {}
}

#[derive(Clone, Copy)]
pub struct ExchangeRootToken;

impl ProofToken for ExchangeRootToken {
    type Proposition = ExchangeRoot;
}

#[derive(Clone, Copy)]
pub struct ExchangeNextToken;

impl ProofToken for ExchangeNextToken {
    type Proposition = ExchangeNext;
}

impl Establish<ExchangeRootToken, GalleryVerifier> for ExchangeNext {
    type Token = ExchangeNextToken;

    fn establish(_credential: ExchangeRootToken) -> Self::Token {
        ExchangeNextToken
    }
}

pub struct GalleryExchanged<T, Token> {
    pub primary: T,
    pub token: Token,
}

impl<T, Token> GalleryExchanged<T, Token> {
    pub fn new(primary: T, token: Token) -> Self {
        Self { primary, token }
    }
}

impl<T, Token> Sidecar<GalleryVerifier> for GalleryExchanged<T, Token>
where
    T: Evidence + Witness<GalleryVerifier>,
    Token: ProofToken<Proposition = T> + Copy,
{
    type Primary = T;
    type Proposition = T;
    type SidecarToken = Token;

    fn primary(&self) -> &Self::Primary {
        &self.primary
    }

    fn sidecar(&self) -> Self::SidecarToken {
        self.token
    }
}

pub struct ExchangeStation;

} // verus!

verus_ensures!(
    ExchangeNext,
    GalleryVerifier,
    exchange_next_ok_spec,
    Result<GalleryExchanged<ExchangeNext, ExchangeNextToken>, ()>,
    |result| result.is_ok()
);

// The real, load-bearing check: `verus_exchange!` generates both the
// `Witness<GalleryVerifier> for ExchangeNext` impl (no hand-written copy
// exists anywhere in this file, unlike `ExchangeRoot`'s above, which
// needed one by hand since it has no `Exchange` edge of its own to back
// it) and the full `Exchange<..> for ExchangeStation` impl, with only
// this real transition body -- the actual `Establish`/`Sidecar` calls --
// authored here.
verus_exchange!(
    ExchangeStation,
    input: GalleryExchanged<ExchangeRoot, ExchangeRootToken>,
    GalleryExchanged<ExchangeNext, ExchangeNextToken>,
    (),
    ExchangeNext,
    GalleryVerifier,
    {
        let token = ExchangeNext::establish(input.sidecar());
        Ok(GalleryExchanged::new(ExchangeNext, token))
    }
);
