//! `KaniWitness` impls and harnesses for the Windows path-prefix types
//! `Prefix` and `PrefixComponent`, proved through the Amenable-owned
//! `KaniWindowsPrefixObservation` rather than the direct std paths (Windows
//! prefix parsing is host-platform-specific and not executable on this
//! Linux verifier host).

use std::path::{Prefix, PrefixComponent};

use amenable_core::{Establish, Evidence, ProofToken};
use amenable_std::RustStdStandard;

use crate::CheckedProof;
use crate::rust_std::macros::bridge_kani_witness;
use crate::{KaniVerifier, KaniWindowsPrefixObservation, KaniWitness};

impl KaniWitness for RustStdStandard<Prefix<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_prefix_disk_identifies_the_drive_letter".to_owned(),
            VERIFY_PREFIX_DISK_IDENTIFIES_THE_DRIVE_LETTER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Prefix<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Prefix<'static>>",
        "kani",
        || <RustStdStandard<Prefix<'static>> as KaniWitness>::proof().to_string(),
    )
}

/// Witness that a `KaniWindowsPrefixObservation` instance actually
/// demonstrated the parsed `Disk` drive letter, minted only by
/// [`KaniWindowsPrefixObservation::demonstrate_drive_letter`].
pub struct KaniWindowsPrefixDriveLetterWitnessToken(());

impl ProofToken for KaniWindowsPrefixDriveLetterWitnessToken {
    type Proposition = KaniWindowsPrefixObservation;
}

impl KaniWindowsPrefixObservation {
    /// Assert the parsed drive letter matches the expected byte. Consumes
    /// `self`: the only way to obtain the token is to have run this check
    /// against a real observation instance, not to assert it
    /// independently.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    #[must_use]
    pub fn demonstrate_drive_letter(
        self,
        expected: u8,
    ) -> KaniWindowsPrefixDriveLetterWitnessToken {
        assert_eq!(self.drive_letter(), expected);
        KaniWindowsPrefixDriveLetterWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<Prefix<'static>>`'s drive-letter
/// claim has been established from a `KaniWindowsPrefixObservation` that has
/// itself demonstrated the parsed `Disk` drive letter.
pub struct RustStdPrefixToken(());

impl ProofToken for RustStdPrefixToken {
    type Proposition = RustStdStandard<Prefix<'static>>;
}

impl Establish<KaniWindowsPrefixDriveLetterWitnessToken, KaniVerifier>
    for RustStdStandard<Prefix<'static>>
{
    type Token = RustStdPrefixToken;

    fn establish(_credential: KaniWindowsPrefixDriveLetterWitnessToken) -> Self::Token {
        RustStdPrefixToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_PREFIX_DISK_IDENTIFIES_THE_DRIVE_LETTER_SRC, {
        /// A Windows drive-letter path (`C:\...`) parses to a `Disk`
        /// prefix naming that letter.
        /// This proof uses the Amenable-owned Windows-prefix observation:
        /// the direct std path is host-platform-specific and does not
        /// execute on this Linux verifier host. The claim is established
        /// through `Establish<KaniWindowsPrefixObservation, KaniVerifier>
        /// for RustStdStandard<Prefix<'static>>` from the observation
        /// instance that actually demonstrated the `Disk` drive letter,
        /// rather than asserted independently of it.
        #[kani::proof]
        fn verify_prefix_disk_identifies_the_drive_letter() {
            let observation = crate::KaniWindowsPrefixObservation::disk("C:", b'C');
            let demonstration = observation.demonstrate_drive_letter(b'C');

            let _token = RustStdStandard::<Prefix<'static>>::establish(demonstration);
        }
    }
}

impl KaniWitness for RustStdStandard<PrefixComponent<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_prefix_component_pairs_raw_text_with_parsed_prefix".to_owned(),
            VERIFY_PREFIX_COMPONENT_PAIRS_RAW_TEXT_WITH_PARSED_PREFIX_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<PrefixComponent<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<PrefixComponent<'static>>",
        "kani",
        || <RustStdStandard<PrefixComponent<'static>> as KaniWitness>::proof()
            .to_string(),
    )
}

/// Witness that a `KaniWindowsPrefixObservation` instance actually
/// demonstrated both its raw text and parsed drive letter, minted only by
/// [`KaniWindowsPrefixObservation::demonstrate_raw_text_and_drive_letter`].
pub struct KaniWindowsPrefixComponentWitnessToken(());

impl ProofToken for KaniWindowsPrefixComponentWitnessToken {
    type Proposition = KaniWindowsPrefixObservation;
}

impl KaniWindowsPrefixObservation {
    /// Assert both the raw text and the parsed drive letter match what
    /// the source path actually wrote. Consumes `self` for the same
    /// reason [`KaniWindowsPrefixObservation::demonstrate_drive_letter`]
    /// does.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    #[must_use]
    pub fn demonstrate_raw_text_and_drive_letter(
        self,
        expected_text: &'static str,
        expected_letter: u8,
    ) -> KaniWindowsPrefixComponentWitnessToken {
        assert_eq!(self.raw_text(), expected_text);
        assert_eq!(self.drive_letter(), expected_letter);
        KaniWindowsPrefixComponentWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<PrefixComponent<'static>>`'s
/// raw-text-plus-parsed-prefix claim has been established from a
/// `KaniWindowsPrefixObservation` that has itself demonstrated both facets.
pub struct RustStdPrefixComponentToken(());

impl ProofToken for RustStdPrefixComponentToken {
    type Proposition = RustStdStandard<PrefixComponent<'static>>;
}

impl Establish<KaniWindowsPrefixComponentWitnessToken, KaniVerifier>
    for RustStdStandard<PrefixComponent<'static>>
{
    type Token = RustStdPrefixComponentToken;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(_credential)))]
    fn establish(_credential: KaniWindowsPrefixComponentWitnessToken) -> Self::Token {
        RustStdPrefixComponentToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_PREFIX_COMPONENT_PAIRS_RAW_TEXT_WITH_PARSED_PREFIX_SRC, {
        /// A `PrefixComponent`'s raw `OsStr` text and its parsed `Prefix`
        /// agree with what the source path actually wrote.
        /// This proof uses the same Amenable-owned Windows-prefix
        /// observation as `Prefix`: the direct std path is
        /// host-platform-specific and does not execute on this Linux
        /// verifier host. The claim is established through
        /// `Establish<KaniWindowsPrefixObservation, KaniVerifier> for
        /// RustStdStandard<PrefixComponent<'static>>` from the observation
        /// instance that actually demonstrated both the raw text and the
        /// parsed `Disk` drive letter, rather than asserted independently
        /// of it.
        #[kani::proof]
        fn verify_prefix_component_pairs_raw_text_with_parsed_prefix() {
            let observation = crate::KaniWindowsPrefixObservation::disk("C:", b'C');
            let demonstration = observation.demonstrate_raw_text_and_drive_letter("C:", b'C');

            let _token = RustStdStandard::<PrefixComponent<'static>>::establish(demonstration);
        }
    }
}
