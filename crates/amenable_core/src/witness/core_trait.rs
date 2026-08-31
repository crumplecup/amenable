//! The constitutional `Witness`/`ClassifiedWitness` trait pair, and the
//! macro that registers concrete instantiations for a backend that
//! cannot discover them automatically.

use super::support::WitnessSupportSummary;
use crate::{Evidence, Verifier};

/// Compile-time destination contract for witness artifacts whose proof
/// content lives in a separately compiled backend module.
pub trait WitnessModulePath {
    /// Backend module path where the generated proof content belongs.
    const MODULE_PATH: &'static str;
}

/// Constitutional extraction of verifier-facing proof emission.
///
/// A witness names which proof (if any) backs a piece of evidence for a
/// given verifier — a descriptor, discoverable without running anything.
/// Proving is a separate mode from doing: `proof` never executes a
/// verifier, it identifies the harness/contract that a separate tool
/// invocation (`cargo kani`, etc.) would check. Like `Evidence::basis`,
/// this is a static fact about the type, true for every instance.
pub trait Witness<V: Verifier> {
    /// Evidence this witness backs.
    type SupportingEvidence: Evidence;

    /// Descriptor of the backend-facing proof for this verifier.
    type ProofArtifact;

    /// Identify the proof artifact relevant to this evidence, for this
    /// verifier.
    fn proof() -> Self::ProofArtifact;

    /// Describe what kind of support backs this witness.
    ///
    /// Backends should override this when they can distinguish checked,
    /// trusted, or trivial closure. The default stays explicit: the
    /// support surface is not classified yet.
    fn support() -> WitnessSupportSummary {
        WitnessSupportSummary::opaque_leaf()
    }

    /// Produce the basis behind this proof's supporting evidence.
    fn basis() -> <Self::SupportingEvidence as Evidence>::Basis {
        <Self::SupportingEvidence as Evidence>::basis()
    }
}

/// Marks a [`Witness`] whose support surface has actually been
/// classified — never blanket-implemented, and never implemented by
/// [`Witness`]'s own default `support()` (which stays `Opaque`).
///
/// Compositional structural closure (`#[derive(Witness)]`) propagates
/// this bound onto every field/variant it composes, the same way it
/// already propagates the base `Witness<V>` bound. A leaf that never
/// overrode `support()` — still `Opaque` — never implements this trait,
/// so a composite containing one fails to implement it too. Exporting a
/// witness to a backend (`register_witness_exports!`) requires this
/// bound, so an `Opaque` leaf anywhere in the composed tree turns into a
/// real `cargo check`-time trait-resolution error naming the exact
/// unclassified leaf — not a runtime failure, and not a `const`-eval
/// panic (confirmed empirically during design: `assert!` in a `const`
/// initializer does fail the build, but only via that one panic-shaped
/// channel, and doesn't generalize to code still generic over field
/// types — this trait-bound approach has neither limitation).
#[diagnostic::on_unimplemented(
    message = "`{Self}` has no registered classification for this verifier (its Witness::support() is still Opaque)",
    note = "implement `ClassifiedWitness<{V}>` for it (e.g. via `bridge_verus_witness!`, or by overriding `support()` directly) before it can be composed into an exported witness"
)]
pub trait ClassifiedWitness<V: Verifier>: Witness<V> {}

/// Register explicit witness exports for a verifier backend.
///
/// This is for backends such as Verus that compile proof content in a
/// separate source unit and therefore cannot discover every derived type
/// automatically. Callers provide the concrete instantiations they want to
/// export; the macro records their evidence type, destination module, and
/// rendered witness artifact for later tooling.
#[macro_export]
macro_rules! register_witness_exports {
    (verifier = $verifier:ty; $($ty:ty),* $(,)?) => {
        $(
            const _: fn() = || {
                fn assert_classified<T: $crate::ClassifiedWitness<$verifier>>() {}
                assert_classified::<$ty>();
            };

            $crate::__inventory::submit! {
                $crate::WitnessExportRecord::new(
                    || <$verifier as $crate::Verifier>::name(),
                    || ::std::any::type_name::<$ty>(),
                    || <<$ty as $crate::Witness<$verifier>>::ProofArtifact as $crate::WitnessModulePath>::MODULE_PATH,
                    || <$ty as $crate::Witness<$verifier>>::proof().to_string(),
                    || <$ty as $crate::Witness<$verifier>>::support(),
                    || {
                        let proof = <$ty as $crate::Witness<$verifier>>::proof();
                        $crate::WitnessArtifact::witness_artifact(&proof)
                    },
                )
            }
        )*
    };
}
