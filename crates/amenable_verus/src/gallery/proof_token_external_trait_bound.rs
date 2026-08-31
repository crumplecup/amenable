//! Gallery case: `Sidecar<V>`'s generic impl for `Established<T, Token>`
//! bounds `Token` on `ProofToken<Proposition = T>` -- an associated-
//! type-equality bound on an external (mod-included, not `verus! {}`-
//! declared) trait.
//!
//! **Disposition: best practice, confirmed.** **Expected/actual outcome:
//! passes, for real -- `361 verified, 0 errors`, zero warnings.**
//!
//! ## The finding (confirmed, reproducible)
//!
//! Using `ProofToken` as a generic bound inside `verus! {}` -- without
//! anything more -- only *warns*: "cannot use external trait ...
//! ProofToken as a bound without declaring the trait (use
//! `#[verifier::external_trait_specification]` to declare the trait);
//! this is a warning for now but will eventually be an error." Taken at
//! face value that reads as "works today, deprecated path." It is worse
//! than that: attempting to actually *verify* a function whose signature
//! requires resolving `<Token as ProofToken>::Proposition` for a generic
//! `Token` crashes Verus's own backend outright --
//!
//! ```text
//! thread '<unnamed>' panicked at rust_verify/src/verifier.rs:663:17:
//! internal error: ill-typed AIR code: error 'use of undeclared function
//! proj%%lib!core_probe_exchange.ProofToken./Proposition' in expression
//! '(proj%%lib!core_probe_exchange.ProofToken./Proposition $
//! TYPE%lib!exchange_probe.GreenToken.)'
//! ```
//!
//! -- an internal AIR (Verus's SMT intermediate representation) panic,
//! not a clean rejection. Isolated to the generic `Sidecar<V>` impl's
//! own bound specifically: `Evidence`/`Witness`/`ProofToken`/`Establish`
//! together, with no `Sidecar` anywhere, verify clean (see
//! `evidence_self_referential_root`, `351 verified, 0 errors`). Adding
//! back *only* `impl<T, Token> Sidecar<V> for Established<T, Token>
//! where T: Evidence + Witness<V>, Token: ProofToken<Proposition = T> +
//! Copy { .. }` -- even completely uninstantiated, never called from any
//! function -- reproduces the crash. Removing `Sidecar` and testing the
//! real, un-instantiated `impl Establish<GreenToken, V> for Yellow`
//! alone does not crash (it's in `evidence_self_referential_root`
//! already); the crash is specific to the associated-type-*equality*
//! bound (`Proposition = T`) on an external trait used as a generic
//! bound, not to using an external trait as a bound in general (`T:
//! Evidence + Witness<V>` alone, with no equality constraint, is fine).
//!
//! ## The fix, confirmed
//!
//! Verus's own warning names the mechanism: `#[verifier::external_trait_
//! specification]`, a companion trait declaring `type
//! ExternalTraitSpecificationFor: ProofToken;` plus a mirrored `type
//! Proposition: Evidence;` (real syntax confirmed against `vstd::
//! std_specs::convert::ExFrom`/`ExInto`, a real shipped example, and
//! `rust_verify_test/tests/external_traits.rs`'s `test_trait5`/
//! `test_trait_extension` for the exact associated-type-mirroring shape
//! used here) -- but getting there took four real, sequential fixes, not
//! one:
//!
//! 1. `type Proposition;` alone (no bound) was rejected with a real,
//!    legible error ("Mismatched bounds on associated type (3 != 1)"):
//!    the mirror's bound has to match the real trait's own declaration
//!    exactly (`type Proposition: Evidence;`), not merely have the same
//!    name.
//! 2. Declaring `ExProofToken` alone still failed -- Verus's internal
//!    trait-conflict checker (which validates the specification against
//!    the real trait by generating synthetic Rust and re-checking it)
//!    needs `Evidence` to *also* have its own `external_trait_
//!    specification`, purely so that checker's own generated code can
//!    resolve the name -- not because `Evidence` was ever rejected as a
//!    bound on its own (it wasn't; see `evidence_self_referential_root`).
//! 3. The same requirement cascaded one level further once `Witness<V>`
//!    needed a specification too: `Witness<V: Verifier>`'s own bound on
//!    `V` meant `Verifier` also needed one, for the identical reason.
//! 4. `Verifier: 'static`'s supertrait bound has to be mirrored on the
//!    *specification trait itself* (`trait ExVerifier: 'static { .. }`),
//!    not attached to the `ExternalTraitSpecificationFor` associated
//!    type the way ordinary bounds are (`unexpected bound in
//!    ExternalTraitSpecificationFor`) -- confirmed against `vstd::
//!    std_specs::bits::ExBits`'s real `: Sized + 'static` shape.
//!
//! Four companion traits later (`ExVerifier`, `ExEvidence`,
//! `ExProofToken`, `ExWitness`), the full `Sidecar<V>` generic impl --
//! the exact real shape, including its `ProofToken<Proposition = T>`
//! bound -- verifies clean, with every "cannot use external trait as a
//! bound" warning gone too, not just the crash.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

use crate::{Evidence, ProofToken, Verifier, Witness};

use super::support::GalleryVerifier;

verus! {

/// Companion specification for the real, mod-included `Verifier` trait
/// -- needed for the same reason `ExEvidence` is: Verus's internal
/// trait-conflict checker resolves bound names (here, `Witness<V:
/// Verifier>`'s own bound on `V`) against declared specifications, not
/// against ordinary Rust name resolution.
#[verifier::external_trait_specification]
pub trait ExVerifier: 'static {
    /// The real trait this specification stands in for.
    type ExternalTraitSpecificationFor: Verifier;
}

/// Companion specification for the real, mod-included `Evidence` trait
/// -- needed so Verus's own internal trait-conflict checker can resolve
/// `Evidence` by name while validating `ExProofToken`'s own bound below
/// (`type Proposition: Evidence`), not because `Evidence` itself was
/// ever rejected as a bound on its own (it wasn't -- see
/// `evidence_self_referential_root`).
#[verifier::external_trait_specification]
pub trait ExEvidence {
    /// The real trait this specification stands in for.
    type ExternalTraitSpecificationFor: Evidence;
}

/// Companion specification for the real, mod-included `ProofToken`
/// trait -- mirrors its one associated type (`Proposition`, bound on
/// `Evidence` to match the real trait's own declaration exactly -- a
/// mismatched bound was rejected cleanly the first time this was tried)
/// so Verus has a real declaration to resolve `<Token as ProofToken>::
/// Proposition` against instead of crashing its own AIR backend.
#[verifier::external_trait_specification]
pub trait ExProofToken {
    /// The real trait this specification stands in for.
    type ExternalTraitSpecificationFor: ProofToken;
    /// Mirrors `ProofToken::Proposition`, bound on `Evidence` to match the
    /// real trait's own declaration exactly.
    type Proposition: Evidence;
}

/// Companion specification for the real, mod-included (accommodation-
/// mirrored -- see `witness_accommodation`'s own doc comment) `Witness<V>`
/// trait, closing out the last "cannot use external trait as a bound"
/// warning alongside `ExEvidence`/`ExProofToken` above.
#[verifier::external_trait_specification]
pub trait ExWitness<V: Verifier> {
    /// The real trait this specification stands in for.
    type ExternalTraitSpecificationFor: Witness<V>;
    /// Mirrors `Witness::SupportingEvidence`.
    type SupportingEvidence: Evidence;
    /// Mirrors `Witness::ProofArtifact`.
    type ProofArtifact;
}

/// The minimal self-referential root: `Basis = Self`.
pub struct Green;

impl Evidence for Green {
    type Basis = Self;
    type Audit = ();

    fn basis() -> Self::Basis {
        Green
    }

    fn audit(&self) -> Self::Audit {}

    fn is_root() -> bool {
        true
    }
}

impl Witness<GalleryVerifier> for Green {
    type SupportingEvidence = Self;
    type ProofArtifact = ();

    fn proof() -> Self::ProofArtifact {}
}

/// The proof token witnessing [`Green`].
#[derive(Clone, Copy)]
pub struct GreenToken;

impl ProofToken for GreenToken {
    type Proposition = Green;
}

/// The real, load-bearing check's own carrier: `Sidecar<V>`'s generic
/// shape, exercised via [`use_sidecar`].
pub struct Established<T, Token> {
    primary: T,
    token: Token,
}

/// The real, load-bearing check: `Sidecar<V>`'s exact generic shape,
/// including the `ProofToken<Proposition = T>` associated-type-equality
/// bound that crashed Verus without `ExProofToken` above.
impl<T, Token> crate::Sidecar<GalleryVerifier> for Established<T, Token>
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

/// The real, load-bearing check itself: `Sidecar<V>`'s generic impl,
/// including the `ProofToken<Proposition = T>` associated-type-equality
/// bound that crashed Verus without `ExProofToken` above -- see this
/// file's own doc comment.
pub fn use_sidecar(established: &Established<Green, GreenToken>) -> (token: GreenToken)
    ensures
        true,
{
    use crate::Sidecar;
    established.sidecar()
}

} // verus!
