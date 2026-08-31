//! Gallery case: can a real `Ensures<V>`/`Requires<V>` impl (the same
//! `amenable_core::contract` trait family `kani_ensures!`/`kani_requires!`
//! wire up for Kani, per `EXCHANGE_PROOF_DERIVATION_PLAN.md`'s Step 6) be
//! the single source of truth for a real Verus `ensures(...)` clause,
//! instead of the clause restating its bound inline the way `gallery::
//! stoplight_exchange` currently does?
//!
//! ## The mechanism: `#[verifier::when_used_as_spec]`
//!
//! `Ensures<V>::ensures(input: Self::Input) -> Self::Bound` is declared as
//! a plain, unattributed `fn` in `amenable_core::contract` — it has to be,
//! since that crate has no dependency on `verus_builtin_macros` and can't
//! write Verus-only syntax (`spec fn`, `#[verifier::..]`) at all. Under
//! Verus, an un-annotated `fn` outside `verus! {}` is exec-mode by
//! default, so calling it directly inside an `ensures(...)` clause (a
//! spec-mode position) is illegal on its own — confirmed by first
//! attempting exactly that (see "What was tried and rejected" below).
//!
//! `~/repos/verus/source/vstd/std_specs/result.rs` shows the real fix
//! `vstd` itself uses for the identical problem (`Result::is_ok()` is a
//! real, exec, `std`-defined method, yet appears directly inside this very
//! file's own `ensures(...)` clauses): `#[verifier::when_used_as_spec(is_ok)]`
//! on the exec function, naming a separate, pure `spec fn is_ok` with an
//! identical signature. Verus transparently substitutes the spec version
//! wherever the exec version is referenced from spec position (confirmed
//! directly in Verus's own test suite,
//! `rust_verify_test/tests/when_used_as_spec.rs`) — the exec call remains
//! real, callable code for ordinary (non-proof) callers; only spec
//! contexts see the substitution.
//!
//! `rust_verify_test/tests/when_used_as_spec.rs`'s own `test_traits` case
//! confirms the attribute is legal per-*impl*, not just per-trait-
//! declaration: `impl T for bool` gives `g()` a completely different
//! `when_used_as_spec` target (`h`) than the trait's own declared default
//! (`f`) — exactly the shape this case needs, since `Ensures<V>`'s trait
//! declaration itself can carry no Verus attribute at all.
//!
//! ## Disposition
//!
//! **Confirmed working.** Each concrete `Ensures<GalleryVerifier>` impl
//! below pairs its exec `ensures()` body with a private `spec fn`
//! companion of identical logic, linked via `#[verifier::
//! when_used_as_spec]` declared at the impl site (not the trait
//! declaration, which can't carry it). `Exchange::exchange`'s own real
//! `ensures(...)` clause then reads `Yellow::ensures(result)` — the exact
//! same call shape `gallery::stoplight_exchange` already uses for Kani's
//! `Yellow::ensures(*result)` — and Verus resolves it to the spec
//! companion, not the exec body, exactly as `vstd::std_specs::result::
//! is_ok` does for `Result::is_ok()`. Verified non-vacuous the same way
//! every other case in this gallery is: a real bug (`result.is_err()`
//! swapped in for `result.is_ok()` inside the spec companion only, exec
//! body left correct) produced a real `postcondition not satisfied`
//! failure, confirming Verus is checking the spec companion's logic, not
//! silently trusting the exec body or the call site's own claim.
//!
//! This closes the same gap `EXCHANGE_PROOF_DERIVATION_PLAN.md`'s Step 6
//! closed for Kani: the contract type carries the bound, and the proof
//! site calls through it rather than restating it — contrary to
//! `amenable_core::contract`'s own doc comment, which anticipated Verus
//! needing the weaker `Bound = &'static str` fallback (a *description* of
//! the bound, not a checked one). That fallback is not needed here:
//! `Bound = bool` works for Verus too, via `when_used_as_spec`, matching
//! Kani's own shape rather than falling back to the non-Rust-syntax case
//! the doc comment describes. Confirmed to generalize past this minimal
//! case, not just work here: `gallery::stoplight_exchange`'s real
//! three-edge `Stoplight` cycle now routes all three `ensures(...)`
//! clauses through this same pattern. `amenable_core::contract`'s own
//! doc comment still describes the `&'static str` fallback as Verus's
//! shape and should be corrected to match.
//!
//! ## What was tried and rejected
//!
//! Calling `Yellow::ensures(result)` directly inside `ensures(...)` with
//! no `when_used_as_spec` bridge at all: real rejection ("cannot call
//! function with mode exec"). Confirms the bridge is load-bearing, not
//! incidental — Verus does not treat a plain trait-impl method as
//! implicitly spec-safe just because its logic happens to be pure.

use verus_builtin_macros::verus;

use crate::{Ensures, Evidence, Witness};

use super::support::GalleryVerifier;

verus! {

pub struct EnsuresGreen;

impl Evidence for EnsuresGreen {
    type Basis = Self;
    type Audit = ();

    fn basis() -> Self::Basis {
        EnsuresGreen
    }

    fn audit(&self) -> Self::Audit {}

    fn is_root() -> bool {
        true
    }
}

impl Witness<GalleryVerifier> for EnsuresGreen {
    type SupportingEvidence = Self;
    type ProofArtifact = ();

    fn proof() -> Self::ProofArtifact {}
}

/// Pure companion to `ensures()` below — identical logic, spec-mode.
/// Must be at least as visible as `ensures()` itself (a public trait
/// method): a private spec companion triggers a real, confirmed
/// rejection ("when_used_as_spec refers to function which is more
/// private").
pub open spec fn ensures_green_ok_spec(result: Result<EnsuresGreen, ()>) -> bool {
    result.is_ok()
}

impl Ensures<GalleryVerifier> for EnsuresGreen {
    type Input = Result<EnsuresGreen, ()>;
    type Bound = bool;

    /// The real, callable check for ordinary (non-proof) callers —
    /// mirrors Kani's `Bound = bool` shape exactly. `#[verifier::
    /// when_used_as_spec]` makes this the same function Verus consults
    /// when this call appears in spec position (an `ensures(...)`
    /// clause), rather than trusting the call site to restate the bound.
    #[verifier::when_used_as_spec(ensures_green_ok_spec)]
    fn ensures(result: Result<EnsuresGreen, ()>) -> bool {
        result.is_ok()
    }
}

/// The real, load-bearing check: does a genuine `ensures(...)` clause
/// resolve `EnsuresGreen::ensures(..)` to the spec companion above? If
/// the bridge weren't wired, this wouldn't even compile (calling an exec
/// fn from spec position is a hard error, not a silent pass) — see this
/// file's own "What was tried and rejected" section for the confirmed
/// real rejection when the bridge was absent.
///
/// Returns the bare `EnsuresGreen`, not `Result<EnsuresGreen, ()>` —
/// unlike `gallery::stoplight_exchange`'s real `Exchange::exchange`
/// (a trait impl method, whose signature the trait dictates), a bare
/// `pub fn` returning `Result<_, ()>` trips a real, separate clippy lint
/// (`result_unit_err`) under plain `cargo clippy`. The `ensures(...)`
/// clause still exercises the identical `Ensures::ensures()` bridge by
/// wrapping the result in `Ok(..)` itself.
pub fn make_green() -> (result: EnsuresGreen)
    ensures
        EnsuresGreen::ensures(Ok(result)),
{
    EnsuresGreen
}

} // verus!
