//! `KaniWitness` registration for `amenable_ext::ExtStandard<T>` — the
//! third-party-crate counterpart of `rust_std::macros::impl_kani_witness_trusted!`.
//!
//! A new sibling module, not nested under `rust_std`: `rust_std` names
//! the Rust standard library specifically (see that module's own doc
//! comment), and jiff/chrono/etc. are not that (Open decision 2 in
//! `docs/AMENABLE_EXT_PLAN.md`).

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
