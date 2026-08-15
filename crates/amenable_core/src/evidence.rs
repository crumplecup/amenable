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

impl_tuple_evidence!((A, 0), (B, 1));
impl_tuple_evidence!((A, 0), (B, 1), (C, 2));
impl_tuple_evidence!((A, 0), (B, 1), (C, 2), (D, 3));
