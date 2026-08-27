//! `KaniWitness` impls for `std::fs`.
//!
//! The direct `std::fs` tempdir path crosses real OS-backed filesystem state
//! that Kani times out on today. Production proofs are therefore being
//! migrated incrementally onto an Amenable-owned filesystem model; the direct
//! real-filesystem boundary remains preserved in the gallery.

use std::fs::{
    DirBuilder, DirEntry, File, FileTimes, FileType, Metadata, OpenOptions, Permissions, ReadDir,
};

use amenable_core::{Establish, Evidence, ProofToken};
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::bridge_kani_witness;
use crate::{
    KaniCreateNewObservation, KaniDirEntryObservation, KaniFileContentObservation,
    KaniFileLenObservation, KaniFileTimesObservation, KaniFileTypeObservation, KaniLockObservation,
    KaniPermissionsObservation, KaniReadDirObservation, KaniRecursiveDirObservation, KaniVerifier,
};

impl KaniWitness for RustStdStandard<DirBuilder> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_dir_builder_creates_nested_directories_recursively".to_owned(),
            VERIFY_DIR_BUILDER_CREATES_NESTED_DIRECTORIES_RECURSIVELY_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<DirBuilder>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<DirBuilder>",
        "kani",
        || <RustStdStandard<DirBuilder> as KaniWitness>::proof().to_string(),
    )
}

/// Witness that a `KaniRecursiveDirObservation` instance actually
/// demonstrated the ancestor-preserving join law, minted only by
/// [`KaniRecursiveDirObservation::demonstrate_ancestor_preservation`].
pub struct KaniRecursiveDirWitnessToken(());

impl ProofToken for KaniRecursiveDirWitnessToken {
    type Proposition = KaniRecursiveDirObservation;
}

impl KaniRecursiveDirObservation {
    /// Assert the observation's ancestors and leaf match repeated joins
    /// from `base`. Consumes `self`: the only way to obtain the token is
    /// to have run this check against a real observation instance, not to
    /// assert it independently.
    #[cfg_attr(
        not(kani),
        tracing::instrument(level = "debug", skip(self, base, first, second, leaf))
    )]
    #[must_use]
    pub fn demonstrate_ancestor_preservation(
        self,
        base: crate::KaniFsPath,
        first: crate::KaniFsLabel,
        second: crate::KaniFsLabel,
        leaf: crate::KaniFsLabel,
    ) -> KaniRecursiveDirWitnessToken {
        assert_eq!(
            self.first_ancestor(),
            base.join(first),
            "recursive creation preserves the first ancestor"
        );
        assert_eq!(
            self.second_ancestor(),
            base.join(first).join(second),
            "recursive creation preserves the second ancestor"
        );
        assert_eq!(
            self.leaf(),
            base.join(first).join(second).join(leaf),
            "recursive creation preserves the leaf"
        );
        KaniRecursiveDirWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<DirBuilder>`'s recursive
/// directory-creation claim has been established from a
/// `KaniRecursiveDirObservation` that has itself demonstrated the
/// ancestor-preserving join law.
pub struct RustStdDirBuilderRecursiveToken(());

impl ProofToken for RustStdDirBuilderRecursiveToken {
    type Proposition = RustStdStandard<DirBuilder>;
}

impl Establish<KaniRecursiveDirWitnessToken, KaniVerifier> for RustStdStandard<DirBuilder> {
    type Token = RustStdDirBuilderRecursiveToken;

    fn establish(_credential: KaniRecursiveDirWitnessToken) -> Self::Token {
        RustStdDirBuilderRecursiveToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_DIR_BUILDER_CREATES_NESTED_DIRECTORIES_RECURSIVELY_SRC, {
        /// `.recursive(true)` creates every missing ancestor directory,
        /// not just the leaf.
        /// This proof uses the Amenable-owned filesystem model: if the real
        /// `DirBuilder` recursive path refines these directory-creation laws,
        /// the Rust-facing claim follows. The claim is established through
        /// `Establish<KaniRecursiveDirObservation, KaniVerifier> for
        /// RustStdStandard<DirBuilder>` from the observation instance that
        /// actually demonstrated the ancestor-preserving join law, rather
        /// than asserted independently of it.
        #[kani::proof]
        fn verify_dir_builder_creates_nested_directories_recursively() {
            let base = crate::KaniFsPath::root();
            let a = crate::KaniFsLabel::new('a');
            let b = crate::KaniFsLabel::new('b');
            let c = crate::KaniFsLabel::new('c');
            let observation = crate::KaniRecursiveDirObservation::new(base, a, b, c);
            let demonstration = observation.demonstrate_ancestor_preservation(base, a, b, c);

            let _token = RustStdStandard::<DirBuilder>::establish(demonstration);
        }
    }
}

impl KaniWitness for RustStdStandard<DirEntry> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_dir_entry_reports_the_created_files_name_and_path".to_owned(),
            VERIFY_DIR_ENTRY_REPORTS_THE_CREATED_FILES_NAME_AND_PATH_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<DirEntry>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<DirEntry>",
        "kani",
        || <RustStdStandard<DirEntry> as KaniWitness>::proof().to_string(),
    )
}

/// Witness that a `KaniDirEntryObservation` instance actually demonstrated
/// entry identity is preserved exactly, minted only by
/// [`KaniDirEntryObservation::demonstrate_identity`].
pub struct KaniDirEntryWitnessToken(());

impl ProofToken for KaniDirEntryWitnessToken {
    type Proposition = KaniDirEntryObservation;
}

impl KaniDirEntryObservation {
    /// Assert the entry reports the expected name and path. Consumes
    /// `self` for the same reason
    /// [`KaniRecursiveDirObservation::demonstrate_ancestor_preservation`]
    /// does.
    #[cfg_attr(
        not(kani),
        tracing::instrument(level = "debug", skip(self, expected_name, expected_path))
    )]
    #[must_use]
    pub fn demonstrate_identity(
        self,
        expected_name: crate::KaniFsLabel,
        expected_path: crate::KaniFsPath,
    ) -> KaniDirEntryWitnessToken {
        let entry = self.entry();
        assert_eq!(entry.file_name(), Some(expected_name));
        assert_eq!(entry.path(), expected_path);
        KaniDirEntryWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<DirEntry>`'s name/path
/// reporting claim has been established from a `KaniDirEntryObservation`
/// that has itself demonstrated entry identity is preserved exactly.
pub struct RustStdDirEntryToken(());

impl ProofToken for RustStdDirEntryToken {
    type Proposition = RustStdStandard<DirEntry>;
}

impl Establish<KaniDirEntryWitnessToken, KaniVerifier> for RustStdStandard<DirEntry> {
    type Token = RustStdDirEntryToken;

    fn establish(_credential: KaniDirEntryWitnessToken) -> Self::Token {
        RustStdDirEntryToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_DIR_ENTRY_REPORTS_THE_CREATED_FILES_NAME_AND_PATH_SRC, {
        /// A `DirEntry` yielded for a created file reports that file's
        /// own name and full path.
        /// This proof uses the Amenable-owned filesystem model: if the real
        /// `read_dir` / `DirEntry` path preserves entry identity this way, the
        /// Rust-facing claim follows. The claim is established through
        /// `Establish<KaniDirEntryObservation, KaniVerifier> for
        /// RustStdStandard<DirEntry>` from the observation instance that
        /// actually demonstrated that identity, rather than asserted
        /// independently of it.
        #[kani::proof]
        fn verify_dir_entry_reports_the_created_files_name_and_path() {
            let base = crate::KaniFsPath::root().join(crate::KaniFsLabel::new('b'));
            let name = crate::KaniFsLabel::new('f');
            let path = base.join(name);
            let observation = crate::KaniDirEntryObservation::new(base, name);
            let demonstration = observation.demonstrate_identity(name, path);

            let _token = RustStdStandard::<DirEntry>::establish(demonstration);
        }
    }
}

impl KaniWitness for RustStdStandard<File> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_file_write_then_read_round_trips_the_bytes".to_owned(),
            VERIFY_FILE_WRITE_THEN_READ_ROUND_TRIPS_THE_BYTES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<File>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<File>",
        "kani",
        || <RustStdStandard<File> as KaniWitness>::proof().to_string(),
    )
}

/// Witness that a `KaniFileContentObservation` instance actually
/// demonstrated its byte-preserving round trip, minted only by
/// [`KaniFileContentObservation::demonstrate_round_trip`].
pub struct KaniFileContentWitnessToken(());

impl ProofToken for KaniFileContentWitnessToken {
    type Proposition = KaniFileContentObservation;
}

impl KaniFileContentObservation {
    /// Assert the bytes read back match what was written. Consumes
    /// `self` for the same reason
    /// [`KaniRecursiveDirObservation::demonstrate_ancestor_preservation`]
    /// does.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self, bytes)))]
    #[must_use]
    pub fn demonstrate_round_trip(self, bytes: [u8; 4]) -> KaniFileContentWitnessToken {
        assert_eq!(
            self.read(),
            bytes,
            "bytes written to a file are read back unchanged through a fresh handle"
        );
        KaniFileContentWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<File>`'s write/read round-trip
/// claim has been established from a `KaniFileContentObservation` that has
/// itself demonstrated the byte-preserving round trip.
pub struct RustStdFileContentToken(());

impl ProofToken for RustStdFileContentToken {
    type Proposition = RustStdStandard<File>;
}

impl Establish<KaniFileContentWitnessToken, KaniVerifier> for RustStdStandard<File> {
    type Token = RustStdFileContentToken;

    fn establish(_credential: KaniFileContentWitnessToken) -> Self::Token {
        RustStdFileContentToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_FILE_WRITE_THEN_READ_ROUND_TRIPS_THE_BYTES_SRC, {
        /// Bytes written to a file and flushed by `Drop` are read back
        /// unchanged through a fresh handle.
        /// This proof uses the Amenable-owned filesystem model: if the real
        /// `File::write_all` / re-open path preserves written bytes this
        /// way, the Rust-facing claim follows. The claim is established
        /// through `Establish<KaniFileContentObservation, KaniVerifier> for
        /// RustStdStandard<File>` from the observation instance that
        /// actually demonstrated the round trip, rather than asserted
        /// independently of it.
        #[kani::proof]
        fn verify_file_write_then_read_round_trips_the_bytes() {
            let bytes: [u8; 4] = kani::any();
            let observation = crate::KaniFileContentObservation::write(bytes);
            let demonstration = observation.demonstrate_round_trip(bytes);

            let _token = RustStdStandard::<File>::establish(demonstration);
        }
    }
}

impl KaniWitness for RustStdStandard<FileTimes> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_file_times_sets_the_recorded_modification_time".to_owned(),
            VERIFY_FILE_TIMES_SETS_THE_RECORDED_MODIFICATION_TIME_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<FileTimes>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<FileTimes>",
        "kani",
        || <RustStdStandard<FileTimes> as KaniWitness>::proof().to_string(),
    )
}

/// Witness that a `KaniFileTimesObservation` instance actually
/// demonstrated the exact time reflection, minted only by
/// [`KaniFileTimesObservation::demonstrate_modification_time`].
pub struct KaniFileTimesWitnessToken(());

impl ProofToken for KaniFileTimesWitnessToken {
    type Proposition = KaniFileTimesObservation;
}

impl KaniFileTimesObservation {
    /// Assert the recorded modification time matches the target exactly.
    /// Consumes `self` for the same reason
    /// [`KaniRecursiveDirObservation::demonstrate_ancestor_preservation`]
    /// does.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    #[must_use]
    pub fn demonstrate_modification_time(
        self,
        target_unix_seconds: u64,
    ) -> KaniFileTimesWitnessToken {
        assert_eq!(
            self.modified(),
            target_unix_seconds,
            "the target modification time is reflected exactly in metadata"
        );
        KaniFileTimesWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<FileTimes>`'s modification-time
/// claim has been established from a `KaniFileTimesObservation` that has
/// itself demonstrated the exact time reflection.
pub struct RustStdFileTimesToken(());

impl ProofToken for RustStdFileTimesToken {
    type Proposition = RustStdStandard<FileTimes>;
}

impl Establish<KaniFileTimesWitnessToken, KaniVerifier> for RustStdStandard<FileTimes> {
    type Token = RustStdFileTimesToken;

    fn establish(_credential: KaniFileTimesWitnessToken) -> Self::Token {
        RustStdFileTimesToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_FILE_TIMES_SETS_THE_RECORDED_MODIFICATION_TIME_SRC, {
        /// `.set_modified()` on a `FileTimes`, applied via
        /// `File::set_times()`, is reflected exactly in the file's
        /// metadata. The target time is whole-second-precision, since
        /// some filesystems truncate sub-second components.
        /// This proof uses the Amenable-owned filesystem model: if the real
        /// `set_times` / `metadata().modified()` path preserves the target
        /// time this way, the Rust-facing claim follows. The claim is
        /// established through `Establish<KaniFileTimesObservation,
        /// KaniVerifier> for RustStdStandard<FileTimes>` from the
        /// observation instance that actually demonstrated the reflection,
        /// rather than asserted independently of it.
        #[kani::proof]
        fn verify_file_times_sets_the_recorded_modification_time() {
            let target_unix_seconds: u64 = kani::any();
            let observation = crate::KaniFileTimesObservation::set_modified(target_unix_seconds);
            let demonstration = observation.demonstrate_modification_time(target_unix_seconds);

            let _token = RustStdStandard::<FileTimes>::establish(demonstration);
        }
    }
}

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

impl KaniWitness for RustStdStandard<OpenOptions> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_open_options_create_new_rejects_an_existing_file".to_owned(),
            VERIFY_OPEN_OPTIONS_CREATE_NEW_REJECTS_AN_EXISTING_FILE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<OpenOptions>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<OpenOptions>",
        "kani",
        || <RustStdStandard<OpenOptions> as KaniWitness>::proof().to_string(),
    )
}

/// Witness that a `KaniCreateNewObservation` instance actually demonstrated
/// a successful creation against a fresh path, having also confirmed the
/// same operation rejects an already-occupied one, minted only by
/// [`KaniCreateNewObservation::demonstrate_creation_succeeds`].
pub struct KaniCreateNewWitnessToken(());

impl ProofToken for KaniCreateNewWitnessToken {
    type Proposition = KaniCreateNewObservation;
}

impl KaniCreateNewObservation {
    /// Assert `.create_new()` fails against `existing` and
    /// `existing_directory`, then assert it succeeds against `self` (a
    /// fresh path) and leaves a file there. Consumes all three: the only
    /// way to obtain the token is to have run this check against real
    /// observation instances, not to assert it independently.
    #[cfg_attr(
        not(kani),
        tracing::instrument(level = "trace", skip(self, existing, existing_directory))
    )]
    #[must_use]
    pub fn demonstrate_creation_succeeds(
        mut self,
        mut existing: KaniCreateNewObservation,
        mut existing_directory: KaniCreateNewObservation,
    ) -> KaniCreateNewWitnessToken {
        assert!(
            existing.create_new().is_err(),
            "create_new fails against a path that already has a file"
        );
        assert!(
            existing_directory.create_new().is_err(),
            "create_new also fails when the path already names a directory"
        );

        assert!(
            self.create_new().is_ok(),
            "create_new succeeds against a genuinely fresh path"
        );
        assert!(
            self.is_file(),
            "a successful create_new leaves a file at the created path"
        );
        KaniCreateNewWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<OpenOptions>`'s `create_new`
/// existence-check claim has been established from a `KaniCreateNewObservation`
/// that has itself demonstrated the successful-creation transition.
pub struct RustStdOpenOptionsCreateNewToken(());

impl ProofToken for RustStdOpenOptionsCreateNewToken {
    type Proposition = RustStdStandard<OpenOptions>;
}

impl Establish<KaniCreateNewWitnessToken, KaniVerifier> for RustStdStandard<OpenOptions> {
    type Token = RustStdOpenOptionsCreateNewToken;

    fn establish(_credential: KaniCreateNewWitnessToken) -> Self::Token {
        RustStdOpenOptionsCreateNewToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_OPEN_OPTIONS_CREATE_NEW_REJECTS_AN_EXISTING_FILE_SRC, {
        /// `.create_new(true)` fails with `AlreadyExists` on a path that
        /// already has a file, and succeeds on a genuinely fresh one.
        /// This proof uses the Amenable-owned filesystem model: if the real
        /// `OpenOptions::create_new` path preserves this existence check,
        /// the Rust-facing claim follows. The claim is established through
        /// `Establish<KaniCreateNewObservation, KaniVerifier> for
        /// RustStdStandard<OpenOptions>` from the observation instance that
        /// actually demonstrated a successful creation, rather than
        /// asserted independently of it.
        #[kani::proof]
        fn verify_open_options_create_new_rejects_an_existing_file() {
            let existing = crate::KaniCreateNewObservation::existing_file();
            let existing_directory = crate::KaniCreateNewObservation::existing_directory();
            let fresh = crate::KaniCreateNewObservation::missing();
            let demonstration = fresh.demonstrate_creation_succeeds(existing, existing_directory);

            let _token = RustStdStandard::<OpenOptions>::establish(demonstration);
        }
    }
}

impl KaniWitness for RustStdStandard<Permissions> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_permissions_readonly_round_trips_through_set_permissions".to_owned(),
            VERIFY_PERMISSIONS_READONLY_ROUND_TRIPS_THROUGH_SET_PERMISSIONS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Permissions>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Permissions>",
        "kani",
        || <RustStdStandard<Permissions> as KaniWitness>::proof().to_string(),
    )
}

/// Witness that a `KaniPermissionsObservation` instance actually
/// demonstrated the readonly round trip in both directions, minted only by
/// [`KaniPermissionsObservation::demonstrate_readonly_round_trip`].
pub struct KaniPermissionsWitnessToken(());

impl ProofToken for KaniPermissionsWitnessToken {
    type Proposition = KaniPermissionsObservation;
}

impl KaniPermissionsObservation {
    /// Assert a fresh file isn't readonly, then assert setting and
    /// clearing readonly are each reflected the next time permissions are
    /// read. Consumes `self` for the same reason
    /// [`KaniRecursiveDirObservation::demonstrate_ancestor_preservation`]
    /// does.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    #[must_use]
    pub fn demonstrate_readonly_round_trip(mut self) -> KaniPermissionsWitnessToken {
        assert!(!self.readonly(), "a freshly created file is not readonly");

        self = self.with_readonly(true);
        assert!(
            self.readonly(),
            "setting readonly is reflected the next time permissions are read"
        );

        self = self.with_readonly(false);
        assert!(!self.readonly(), "clearing readonly is reflected as well");
        KaniPermissionsWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<Permissions>`'s readonly
/// round-trip claim has been established from a `KaniPermissionsObservation`
/// that has itself demonstrated the round trip in both directions.
pub struct RustStdPermissionsToken(());

impl ProofToken for RustStdPermissionsToken {
    type Proposition = RustStdStandard<Permissions>;
}

impl Establish<KaniPermissionsWitnessToken, KaniVerifier> for RustStdStandard<Permissions> {
    type Token = RustStdPermissionsToken;

    fn establish(_credential: KaniPermissionsWitnessToken) -> Self::Token {
        RustStdPermissionsToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_PERMISSIONS_READONLY_ROUND_TRIPS_THROUGH_SET_PERMISSIONS_SRC, {
        /// Flipping `.set_readonly(true)` and applying it via
        /// `fs::set_permissions` is reflected the next time the file's
        /// permissions are read.
        /// This proof uses the Amenable-owned filesystem model: if the real
        /// `set_permissions` / `metadata().permissions()` path preserves
        /// the readonly bit this way, the Rust-facing claim follows. The
        /// claim is established through `Establish<KaniPermissionsObservation,
        /// KaniVerifier> for RustStdStandard<Permissions>` from the
        /// observation instance that actually demonstrated the round trip
        /// in both directions, rather than asserted independently of it.
        #[kani::proof]
        fn verify_permissions_readonly_round_trips_through_set_permissions() {
            let observation = crate::KaniPermissionsObservation::new();
            let demonstration = observation.demonstrate_readonly_round_trip();

            let _token = RustStdStandard::<Permissions>::establish(demonstration);
        }
    }
}

impl KaniWitness for RustStdStandard<ReadDir> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_read_dir_iterates_every_entry_in_the_directory".to_owned(),
            VERIFY_READ_DIR_ITERATES_EVERY_ENTRY_IN_THE_DIRECTORY_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<ReadDir>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<ReadDir>",
        "kani",
        || <RustStdStandard<ReadDir> as KaniWitness>::proof().to_string(),
    )
}

/// Witness that a `KaniReadDirObservation` instance actually demonstrated
/// exactly the created entries, minted only by
/// [`KaniReadDirObservation::demonstrate_completeness`].
pub struct KaniReadDirWitnessToken(());

impl ProofToken for KaniReadDirWitnessToken {
    type Proposition = KaniReadDirObservation;
}

impl KaniReadDirObservation {
    /// Assert `.entries()` yields exactly the two created entries, in
    /// order. Consumes `self` for the same reason
    /// [`KaniRecursiveDirObservation::demonstrate_ancestor_preservation`]
    /// does.
    #[cfg_attr(
        not(kani),
        tracing::instrument(level = "debug", skip(self, first_name, second_name))
    )]
    #[must_use]
    pub fn demonstrate_completeness(
        self,
        first_name: crate::KaniFsLabel,
        second_name: crate::KaniFsLabel,
    ) -> KaniReadDirWitnessToken {
        let entries = self.entries();
        assert_eq!(
            entries.len(),
            2,
            "read_dir yields exactly the created entries"
        );
        assert_eq!(entries[0].file_name(), Some(first_name));
        assert_eq!(entries[1].file_name(), Some(second_name));
        KaniReadDirWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<ReadDir>`'s entry-completeness
/// claim has been established from a `KaniReadDirObservation` that has
/// itself demonstrated exactly the created entries.
pub struct RustStdReadDirToken(());

impl ProofToken for RustStdReadDirToken {
    type Proposition = RustStdStandard<ReadDir>;
}

impl Establish<KaniReadDirWitnessToken, KaniVerifier> for RustStdStandard<ReadDir> {
    type Token = RustStdReadDirToken;

    fn establish(_credential: KaniReadDirWitnessToken) -> Self::Token {
        RustStdReadDirToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_READ_DIR_ITERATES_EVERY_ENTRY_IN_THE_DIRECTORY_SRC, {
        /// `.read_dir()` yields exactly the files that were created in
        /// that directory, no more and no fewer.
        /// This proof uses the Amenable-owned filesystem model: if the real
        /// `read_dir` path preserves entry identity this way, the
        /// Rust-facing claim follows. The claim is established through
        /// `Establish<KaniReadDirObservation, KaniVerifier> for
        /// RustStdStandard<ReadDir>` from the observation instance that
        /// actually demonstrated the entry completeness, rather than
        /// asserted independently of it.
        #[kani::proof]
        fn verify_read_dir_iterates_every_entry_in_the_directory() {
            let base = crate::KaniFsPath::root().join(crate::KaniFsLabel::new('d'));
            let one = crate::KaniFsLabel::new('1');
            let two = crate::KaniFsLabel::new('2');
            let observation = crate::KaniReadDirObservation::new(base, one, two);
            let demonstration = observation.demonstrate_completeness(one, two);

            let _token = RustStdStandard::<ReadDir>::establish(demonstration);
        }
    }
}

impl KaniWitness for RustStdStandard<std::fs::TryLockError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_try_lock_error_reports_a_lock_already_held".to_owned(),
            VERIFY_TRY_LOCK_ERROR_REPORTS_A_LOCK_ALREADY_HELD_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::fs::TryLockError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fs::TryLockError>",
        "kani",
        || {
            <RustStdStandard<std::fs::TryLockError> as KaniWitness>::proof().to_string()
        },
    )
}

/// Witness that a `KaniLockObservation` instance actually demonstrated a
/// second handle failing to acquire an already-held lock, minted only by
/// [`KaniLockObservation::demonstrate_exclusion`].
pub struct KaniLockWitnessToken(());

impl ProofToken for KaniLockWitnessToken {
    type Proposition = KaniLockObservation;
}

impl KaniLockObservation {
    /// Assert the first `.try_lock()` succeeds and a second fails while
    /// the lock is still held. Consumes `self` for the same reason
    /// [`KaniRecursiveDirObservation::demonstrate_ancestor_preservation`]
    /// does.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    #[must_use]
    pub fn demonstrate_exclusion(mut self) -> KaniLockWitnessToken {
        assert!(
            self.try_lock().is_ok(),
            "the first handle acquires the lock"
        );

        assert!(
            self.try_lock().is_err(),
            "a second handle can't also lock the same file"
        );
        KaniLockWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<std::fs::TryLockError>`'s
/// mutual-exclusion claim has been established from a
/// `KaniLockObservation` that has itself demonstrated a second handle
/// failing to acquire an already-held lock.
pub struct RustStdTryLockErrorToken(());

impl ProofToken for RustStdTryLockErrorToken {
    type Proposition = RustStdStandard<std::fs::TryLockError>;
}

impl Establish<KaniLockWitnessToken, KaniVerifier> for RustStdStandard<std::fs::TryLockError> {
    type Token = RustStdTryLockErrorToken;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(_credential)))]
    fn establish(_credential: KaniLockWitnessToken) -> Self::Token {
        RustStdTryLockErrorToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_TRY_LOCK_ERROR_REPORTS_A_LOCK_ALREADY_HELD_SRC, {
        /// A second handle's `.try_lock()` fails while the first handle
        /// still holds the file lock.
        /// This proof uses the Amenable-owned filesystem model: if the real
        /// `File::try_lock` path preserves this exclusion, the Rust-facing
        /// claim follows. The claim is established through
        /// `Establish<KaniLockObservation, KaniVerifier> for
        /// RustStdStandard<std::fs::TryLockError>` from the observation
        /// instance that actually demonstrated the exclusion, rather than
        /// asserted independently of it.
        #[kani::proof]
        fn verify_try_lock_error_reports_a_lock_already_held() {
            let file = crate::KaniLockObservation::new();
            let demonstration = file.demonstrate_exclusion();

            let _token = RustStdStandard::<std::fs::TryLockError>::establish(demonstration);
        }
    }
}
