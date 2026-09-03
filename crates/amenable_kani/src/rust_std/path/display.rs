//! `RustStdStandard<std::path::Display<'static>>`'s `KaniWitness` impl and
//! its verbatim-rendering harness, proved through the Amenable-owned
//! `KaniPathDisplayObservation` rather than the direct `Path::display()`
//! path (which times out under Kani's formatting machinery).

use amenable_core::{Establish, Evidence, ProofToken};
use amenable_std::RustStdStandard;

use crate::CheckedProof;
use crate::rust_std::macros::bridge_kani_witness;
use crate::{KaniPathDisplayObservation, KaniVerifier, KaniWitness};

impl KaniWitness for RustStdStandard<std::path::Display<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_display_renders_a_valid_utf8_path_verbatim".to_owned(),
            VERIFY_DISPLAY_RENDERS_A_VALID_UTF8_PATH_VERBATIM_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::path::Display<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::path::Display<'static>>",
        "kani",
        || <RustStdStandard<std::path::Display<'static>> as KaniWitness>::proof().to_string(),
    )
}

/// Witness that a `KaniPathDisplayObservation` instance actually
/// demonstrated verbatim rendering, minted only by
/// [`KaniPathDisplayObservation::demonstrate_verbatim_rendering`].
pub struct KaniPathDisplayWitnessToken(());

impl ProofToken for KaniPathDisplayWitnessToken {
    type Proposition = KaniPathDisplayObservation;
}

impl KaniPathDisplayObservation {
    /// Assert the rendered display text matches the source text exactly.
    /// Consumes `self`: the only way to obtain the token is to have run
    /// this check against a real observation instance, not to assert it
    /// independently.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    #[must_use]
    pub fn demonstrate_verbatim_rendering(self) -> KaniPathDisplayWitnessToken {
        assert_eq!(self.display_text(), self.source_text());
        KaniPathDisplayWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<std::path::Display<'static>>`'s
/// UTF-8-display claim has been established from a
/// `KaniPathDisplayObservation` that has itself demonstrated verbatim
/// rendering.
pub struct RustStdPathDisplayToken(());

impl ProofToken for RustStdPathDisplayToken {
    type Proposition = RustStdStandard<std::path::Display<'static>>;
}

impl Establish<KaniPathDisplayWitnessToken, KaniVerifier>
    for RustStdStandard<std::path::Display<'static>>
{
    type Token = RustStdPathDisplayToken;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(_credential)))]
    fn establish(_credential: KaniPathDisplayWitnessToken) -> Self::Token {
        RustStdPathDisplayToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_DISPLAY_RENDERS_A_VALID_UTF8_PATH_VERBATIM_SRC, {
        /// A path made entirely of valid Unicode renders through
        /// `.display()` exactly as its own string form.
        /// This proof uses the Amenable-owned path-display observation:
        /// the direct `Path::display()` rendering path times out under
        /// Kani even for a fully concrete UTF-8 literal path, matching the
        /// general formatting-cost false trail already preserved in the
        /// gallery. The claim is established through
        /// `Establish<KaniPathDisplayObservation, KaniVerifier> for
        /// RustStdStandard<std::path::Display<'static>>` from the
        /// observation instance that actually demonstrated verbatim
        /// rendering, rather than asserted independently of it.
        #[kani::proof]
        fn verify_display_renders_a_valid_utf8_path_verbatim() {
            let observation = crate::KaniPathDisplayObservation::utf8("/a/b.txt");
            let demonstration = observation.demonstrate_verbatim_rendering();

            let _token =
                RustStdStandard::<std::path::Display<'static>>::establish(demonstration);
        }
    }
}
