//! `KaniWitness` registration for `amenable_ext::ExtStandard<T>` — the
//! third-party-crate counterpart of `rust_std::macros::impl_kani_witness_trusted!`.
//!
//! A new sibling module, not nested under `rust_std`: `rust_std` names
//! the Rust standard library specifically (see that module's own doc
//! comment), and jiff/chrono/etc. are not that (Open decision 2 in
//! `docs/AMENABLE_EXT_PLAN.md`).

use amenable_core::Metadata;

/// Register an `ExtStandard<$ty>` whose `KaniWitness::proof()` is
/// "trusted": every third-party carrier registered so far has no
/// invariant beyond what the type system already guarantees, so there
/// is nothing for Kani to check — the same reasoning
/// `impl_kani_witness_trusted!` documents for `RustStdStandard<T>`.
/// `checked` (a real harness) gets added per type only where one adds
/// value over trusting the target crate's own correctness.
macro_rules! impl_kani_witness_trusted_ext {
    ($($ty:ty),* $(,)?) => {
        $(
            impl crate::KaniWitness for amenable_ext::ExtStandard<$ty> {
                type SupportingEvidence = Self;
                type ProofArtifact = amenable_ext::ExtProvenance;

                fn proof() -> Self::ProofArtifact {
                    amenable_core::Evidence::audit(
                        &<Self::SupportingEvidence as amenable_core::Evidence>::basis(),
                    )
                }
            }

            crate::rust_std::bridge_kani_witness!(amenable_ext::ExtStandard<$ty>);

            ::inventory::submit! {
                ::amenable_core::ProofRecord::new(
                    concat!("amenable_ext::ExtStandard<", stringify!($ty), ">"),
                    "kani",
                    || amenable_core::Metadata::report(
                        &<amenable_ext::ExtStandard<$ty> as crate::KaniWitness>::proof(),
                    ).to_string(),
                )
            }
        )*
    };
}

pub(crate) use impl_kani_witness_trusted_ext;

/// The `Ensures` counterpart of [`crate::rust_std::kani_ensures`], for
/// `ExtStandard<T>` rather than `RustStdStandard<T>` — same one-source
/// guarantee (the proof site calls `$ty::ensures(x)` directly; that call
/// *is* the bound), just targeting the third-party-crate carrier family.
macro_rules! kani_ensures_ext {
    ($ty:ty, $evidence:literal, $param_ty:ty, |$param:pat_param| $expr:expr) => {
        impl amenable_core::Ensures<crate::KaniVerifier> for $ty {
            type Input = $param_ty;
            type Bound = bool;

            fn ensures($param: $param_ty) -> bool {
                $expr
            }
        }

        ::inventory::submit! {
            ::amenable_core::ContractRecord::new(
                $evidence,
                "kani",
                "ensures",
                || stringify!($expr),
            )
        }
    };
}

pub(crate) use kani_ensures_ext;

/// Proof artifact for an `ExtStandard<T>` carrier with a real,
/// machine-checked Kani harness — the third-party-crate counterpart of
/// `rust_std::macros::CheckedProof`, holding an `ExtProvenance` instead
/// of a `RustStdProvenance`.
#[derive(Debug, Clone, PartialEq, Eq, derive_getters::Getters, derive_new::new)]
pub struct ExtCheckedProof {
    /// The Kani harness that checks this carrier's invariant.
    harness: String,
    /// The harness's own source — what it actually asserts, verbatim.
    claim: String,
    /// The chain-derived provenance this claim still rests on.
    provenance: amenable_ext::ExtProvenance,
}

impl std::fmt::Display for ExtCheckedProof {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, f)))]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "harness: {}", self.harness)?;
        writeln!(f, "claim: {}", self.claim)?;
        write!(f, "{}", self.provenance.report())
    }
}
