use std::fs::{FileType, Metadata};

use amenable_core::{Establish, Evidence, ProofToken};
use amenable_std::RustStdStandard;

use crate::KaniWitness;
use crate::rust_std::CheckedProof;
use crate::rust_std::macros::bridge_kani_witness;
use crate::{KaniFileLenObservation, KaniFileTypeObservation, KaniVerifier};

impl KaniWitness for RustStdStandard<FileType> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_file_type_distinguishes_files_from_directories".to_owned(),
            VERIFY_FILE_TYPE_DISTINGUISHES_FILES_FROM_DIRECTORIES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<FileType>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<FileType>",
        "kani",
        || <RustStdStandard<FileType> as KaniWitness>::proof().to_string(),
    )
}

/// Witness that a `KaniFileTypeObservation` instance actually demonstrated
/// the file/directory mutual exclusion, minted only by
/// [`KaniFileTypeObservation::demonstrate_distinction`].
pub struct KaniFileTypeWitnessToken(());

impl ProofToken for KaniFileTypeWitnessToken {
    type Proposition = KaniFileTypeObservation;
}

impl KaniFileTypeObservation {
    /// Assert a file reports `is_file()` but not `is_dir()`, and a
    /// directory the reverse. Consumes `self` for the same reason
    /// [`KaniRecursiveDirObservation::demonstrate_ancestor_preservation`]
    /// does.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    #[must_use]
    pub fn demonstrate_distinction(self) -> KaniFileTypeWitnessToken {
        assert!(self.file_is_file());
        assert!(!self.file_is_dir());

        assert!(self.directory_is_dir());
        assert!(!self.directory_is_file());
        KaniFileTypeWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<FileType>`'s file/directory
/// distinction claim has been established from a `KaniFileTypeObservation`
/// that has itself demonstrated the mutual exclusion.
pub struct RustStdFileTypeToken(());

impl ProofToken for RustStdFileTypeToken {
    type Proposition = RustStdStandard<FileType>;
}

impl Establish<KaniFileTypeWitnessToken, KaniVerifier> for RustStdStandard<FileType> {
    type Token = RustStdFileTypeToken;

    fn establish(_credential: KaniFileTypeWitnessToken) -> Self::Token {
        RustStdFileTypeToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_FILE_TYPE_DISTINGUISHES_FILES_FROM_DIRECTORIES_SRC, {
        /// A regular file's `FileType` reports `is_file()`, and a
        /// directory's reports `is_dir()` — never both.
        /// This proof uses the Amenable-owned filesystem model: if the real
        /// `metadata().file_type()` path preserves this file/directory
        /// distinction, the Rust-facing claim follows. The claim is
        /// established through `Establish<KaniFileTypeObservation,
        /// KaniVerifier> for RustStdStandard<FileType>` from the
        /// observation instance that actually demonstrated the mutual
        /// exclusion, rather than asserted independently of it.
        #[kani::proof]
        fn verify_file_type_distinguishes_files_from_directories() {
            let observation = crate::KaniFileTypeObservation::new();
            let demonstration = observation.demonstrate_distinction();

            let _token = RustStdStandard::<FileType>::establish(demonstration);
        }
    }
}

impl KaniWitness for RustStdStandard<Metadata> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_metadata_reports_the_written_length".to_owned(),
            VERIFY_METADATA_REPORTS_THE_WRITTEN_LENGTH_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Metadata>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Metadata>",
        "kani",
        || <RustStdStandard<Metadata> as KaniWitness>::proof().to_string(),
    )
}

/// Witness that a `KaniFileLenObservation` instance actually demonstrated
/// the exact written byte count, minted only by
/// [`KaniFileLenObservation::demonstrate_length`].
pub struct KaniFileLenWitnessToken(());

impl ProofToken for KaniFileLenWitnessToken {
    type Proposition = KaniFileLenObservation;
}

impl KaniFileLenObservation {
    /// Assert `.len()`/`.is_empty()` report exactly the written byte
    /// count. Consumes `self` for the same reason
    /// [`KaniRecursiveDirObservation::demonstrate_ancestor_preservation`]
    /// does.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    #[must_use]
    pub fn demonstrate_length(self, byte_count: u8) -> KaniFileLenWitnessToken {
        assert_eq!(
            self.len(),
            u64::from(byte_count),
            "metadata reports exactly the number of bytes written"
        );
        assert_eq!(self.is_empty(), byte_count == 0);
        KaniFileLenWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<Metadata>`'s written-length
/// claim has been established from a `KaniFileLenObservation` that has
/// itself demonstrated the exact byte count.
pub struct RustStdMetadataLenToken(());

impl ProofToken for RustStdMetadataLenToken {
    type Proposition = RustStdStandard<Metadata>;
}

impl Establish<KaniFileLenWitnessToken, KaniVerifier> for RustStdStandard<Metadata> {
    type Token = RustStdMetadataLenToken;

    fn establish(_credential: KaniFileLenWitnessToken) -> Self::Token {
        RustStdMetadataLenToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_METADATA_REPORTS_THE_WRITTEN_LENGTH_SRC, {
        /// `.len()` reports exactly the number of bytes written to the
        /// file.
        /// This proof uses the Amenable-owned filesystem model: if the real
        /// `metadata().len()` path reports the written byte count this way,
        /// the Rust-facing claim follows. The claim is established through
        /// `Establish<KaniFileLenObservation, KaniVerifier> for
        /// RustStdStandard<Metadata>` from the observation instance that
        /// actually demonstrated the exact count, rather than asserted
        /// independently of it.
        #[kani::proof]
        fn verify_metadata_reports_the_written_length() {
            let byte_count: u8 = kani::any();
            let observation = crate::KaniFileLenObservation::write(byte_count);
            let demonstration = observation.demonstrate_length(byte_count);

            let _token = RustStdStandard::<Metadata>::establish(demonstration);
        }
    }
}
