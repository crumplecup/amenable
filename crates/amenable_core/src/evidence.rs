//! Root evidentiary trait for constitutional propositions.

/// A proposition with explicit lineage and audit artifacts.
///
/// Evidence forms a chain: every claim points at its [`basis`](Evidence::basis),
/// the prior link it was built on — another piece of evidence such as a
/// calculation, or, at the root, a `Standard`, which is its own basis. A
/// given type has exactly one basis, true for every value of that type, so
/// `basis` is a static fact about the type rather than something read off a
/// particular instance. `audit`, by contrast, is what actually happened for
/// a given instance, and does depend on it. Evidence says nothing about
/// verifier backends or how proofs get derived; consuming a chain of
/// evidence to produce a backend-specific proof is `Witness`'s job.
///
/// Deliberately verifier-agnostic, not an oversight: plenty of legitimate
/// `Evidence` use has nothing to do with any specific backend's proof (a
/// generic provenance/audit report over an arbitrary `Standard`, for
/// instance) and must not be forced to name one just to compile. Where a
/// real proof genuinely is required — a value flowing through a `Sidecar`,
/// carried across a proof-bearing `Exchange` — that requirement is stated
/// explicitly there as a compound `Evidence + Witness<V>` bound, the same
/// pattern `Establish<C, V>: Evidence + Witness<V>` already used before
/// this trait was touched. An earlier version of this trait added `V:
/// Verifier` directly to `Evidence` itself as a supertrait bound
/// (`Evidence<V>: Witness<V>`); that broke every legitimately
/// verifier-agnostic use of `Evidence` in the tree (concretely:
/// `amenable_std::write_rust_std_certificate_artifacts`'s generic
/// provenance-dump helper, which never asks "is this proven," only "what
/// does this claim to be") and was reverted in favor of this narrower,
/// call-site-specific bound.
///
/// **Two cfg-exclusive declarations, not one.** Verus's own driver
/// unconditionally sets `--cfg verus_keep_ghost` (confirmed by reading
/// `rust_verify/src/driver.rs` in the real `verus-lang/verus` source, not
/// assumed), and no ordinary Rust toolchain (`cargo check`, `cargo kani`,
/// `cargo creusot`) ever sets it — so exactly one of the two declarations
/// below is ever compiled in a given build, with no naming conflict.
///
/// The root cause, confirmed empirically after real methodological
/// missteps (an initial `chain()`-only fix looked sufficient but was a
/// false positive — masked by Verus's own error reporting stopping at
/// the *first* cyclic definition found, which happened to be a different
/// one in the same test file; only a fully isolated, single-variable
/// test settled it): a self-referential root (`type Basis = Self`, this
/// trait's own deliberate "a root is its own basis" design, stated
/// above) trips Verus's static cyclic-self-reference check unconditionally
/// whenever `Basis` carries this trait's own bound (`type Basis:
/// Evidence`) — independent of `chain()`, independent of reachability,
/// with no per-item escape hatch found despite real testing (`#[verifier::
/// external_body]` doesn't apply to trait impls at all; `#[verifier::
/// external]` compiles but crashes Verus's own backend). Confirmed via a
/// clean, single-variable isolated probe: a side-by-side trait identical
/// in every respect except carrying no bound on `Basis` at all compiled
/// clean (`343 verified, 0 errors`) for the identical self-referential
/// root impl.
///
/// So the `#[cfg(verus_keep_ghost)]` declaration below drops the `Basis:
/// Evidence` bound (and, since nothing left needs it, `chain()`, which
/// only exists to walk that bound) — confirmed for real as the complete
/// fix, not just the bound in isolation: a self-referential root impl of
/// a trait shaped exactly like this cfg pair (bound + `chain()` under
/// `not(verus_keep_ghost)`, no bound under `verus_keep_ghost`) verified
/// clean end to end (`346 verified, 0 errors`). The `#[cfg(not(verus_keep_ghost))]`
/// declaration is byte-for-byte what this trait has always been — every
/// existing caller (`amenable_kani::tests::calculation_test`,
/// `amenable_derive::tests::standard_fixture_corpus_test`, both real and
/// already passing) sees no change at all.
#[cfg(not(verus_keep_ghost))]
pub trait Evidence {
    /// The prior link this evidence was built on top of.
    type Basis: Evidence;

    /// Rich audit artifact describing what was done to uphold this claim.
    type Audit;

    /// Produce the prior link in this evidence's chain — the same for every
    /// value of this type.
    fn basis() -> Self::Basis;

    /// Produce the audit artifact responsible for upholding this claim.
    fn audit(&self) -> Self::Audit;

    /// Whether this evidence is a root: its own basis. Hand-written
    /// `Standard` impls should use `#[amenable_derive::evidence]` on their
    /// `impl Evidence` block rather than overriding this directly — it
    /// computes the correct value from the `Basis` declaration at compile
    /// time. `#[calculation]`-generated evidence always leaves this at the
    /// default, since a calculation's basis is never itself.
    fn is_root() -> bool {
        false
    }

    /// Walk the chain of type names from this evidence back to its root,
    /// inclusive of both ends. A type-level operation: since `basis` is the
    /// same for every value of a type, so is its chain.
    fn chain() -> Vec<&'static str> {
        let mut names = vec![::std::any::type_name::<Self>()];

        if !Self::is_root() {
            names.extend(Self::Basis::chain());
        }

        names
    }
}

/// See the `#[cfg(not(verus_keep_ghost))]` declaration above for the full
/// rationale. This variant drops `Basis: Evidence` (so a self-referential
/// root no longer creates a same-trait cycle Verus's checker rejects) and
/// `chain()` (which only existed to walk that now-absent bound).
#[cfg(verus_keep_ghost)]
pub trait Evidence {
    /// The prior link this evidence was built on top of.
    type Basis;

    /// Rich audit artifact describing what was done to uphold this claim.
    type Audit;

    /// Produce the prior link in this evidence's chain — the same for every
    /// value of this type.
    fn basis() -> Self::Basis;

    /// Produce the audit artifact responsible for upholding this claim.
    fn audit(&self) -> Self::Audit;

    /// Whether this evidence is a root: its own basis.
    fn is_root() -> bool {
        false
    }
}

macro_rules! impl_tuple_evidence {
    ($(($member:ident, $index:tt)),+) => {
        impl<$($member),+> Evidence for ($($member,)+)
        where
            $($member: Evidence,)+
        {
            type Basis = ($($member::Basis,)+);
            type Audit = ($($member::Audit,)+);

            fn basis() -> Self::Basis {
                ($($member::basis(),)+)
            }

            fn audit(&self) -> Self::Audit {
                ($(self.$index.audit(),)+)
            }

            fn is_root() -> bool {
                $($member::is_root())&&+
            }
        }
    };
}

// `#[cfg(not(verus_keep_ghost))]`: matches the trait declaration above.
// Load-bearing for ordinary builds (`#[calculation]`'s multi-argument
// `Basis`, e.g. `AddEvidence`'s `(Debit, Credit)`, and `amenable_kani::
// tests::calculation_test`'s real `chain()`-walking coverage of it) but
// never needed under Verus, since nothing in scope for Verus today uses
// tuple-shaped `Evidence`.
#[cfg(not(verus_keep_ghost))]
impl_tuple_evidence!((A, 0), (B, 1));
#[cfg(not(verus_keep_ghost))]
impl_tuple_evidence!((A, 0), (B, 1), (C, 2));
#[cfg(not(verus_keep_ghost))]
impl_tuple_evidence!((A, 0), (B, 1), (C, 2), (D, 3));
