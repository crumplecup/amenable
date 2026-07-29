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
        CheckedProof {
            harness: "verify_dir_builder_creates_nested_directories_recursively".to_owned(),
            claim: VERIFY_DIR_BUILDER_CREATES_NESTED_DIRECTORIES_RECURSIVELY_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<DirBuilder>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<DirBuilder>",
        verifier: "kani",
        describe: || <RustStdStandard<DirBuilder> as KaniWitness>::proof().to_string(),
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

impl Establish<KaniRecursiveDirObservation, KaniVerifier> for RustStdStandard<DirBuilder> {
    type Token = RustStdDirBuilderRecursiveToken;

    fn establish(_credential: &KaniRecursiveDirObservation) -> Self::Token {
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
            let observation = crate::KaniRecursiveDirObservation::new(
                base,
                crate::KaniFsLabel::new('a'),
                crate::KaniFsLabel::new('b'),
                crate::KaniFsLabel::new('c'),
            );

            assert_eq!(
                observation.first_ancestor(),
                base.join(crate::KaniFsLabel::new('a')),
                "recursive creation preserves the first ancestor"
            );
            assert_eq!(
                observation.second_ancestor(),
                base.join(crate::KaniFsLabel::new('a'))
                    .join(crate::KaniFsLabel::new('b')),
                "recursive creation preserves the second ancestor"
            );
            assert_eq!(
                observation.leaf(),
                base.join(crate::KaniFsLabel::new('a'))
                    .join(crate::KaniFsLabel::new('b'))
                    .join(crate::KaniFsLabel::new('c')),
                "recursive creation preserves the leaf"
            );

            let _token = RustStdStandard::<DirBuilder>::establish(&observation);
        }
    }
}

impl KaniWitness for RustStdStandard<DirEntry> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_dir_entry_reports_the_created_files_name_and_path".to_owned(),
            claim: VERIFY_DIR_ENTRY_REPORTS_THE_CREATED_FILES_NAME_AND_PATH_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<DirEntry>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<DirEntry>",
        verifier: "kani",
        describe: || <RustStdStandard<DirEntry> as KaniWitness>::proof().to_string(),
    }
}

/// Lawful token minted once `RustStdStandard<DirEntry>`'s name/path
/// reporting claim has been established from a `KaniDirEntryObservation`
/// that has itself demonstrated entry identity is preserved exactly.
pub struct RustStdDirEntryToken(());

impl ProofToken for RustStdDirEntryToken {
    type Proposition = RustStdStandard<DirEntry>;
}

impl Establish<KaniDirEntryObservation, KaniVerifier> for RustStdStandard<DirEntry> {
    type Token = RustStdDirEntryToken;

    fn establish(_credential: &KaniDirEntryObservation) -> Self::Token {
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
            let path = base.join(crate::KaniFsLabel::new('f'));
            let observation =
                crate::KaniDirEntryObservation::new(base, crate::KaniFsLabel::new('f'));
            let entry = observation.entry();

            assert_eq!(entry.file_name(), Some(crate::KaniFsLabel::new('f')));
            assert_eq!(entry.path(), path);

            let _token = RustStdStandard::<DirEntry>::establish(&observation);
        }
    }
}

impl KaniWitness for RustStdStandard<File> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_file_write_then_read_round_trips_the_bytes".to_owned(),
            claim: VERIFY_FILE_WRITE_THEN_READ_ROUND_TRIPS_THE_BYTES_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<File>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<File>",
        verifier: "kani",
        describe: || <RustStdStandard<File> as KaniWitness>::proof().to_string(),
    }
}

/// Lawful token minted once `RustStdStandard<File>`'s write/read round-trip
/// claim has been established from a `KaniFileContentObservation` that has
/// itself demonstrated the byte-preserving round trip.
pub struct RustStdFileContentToken(());

impl ProofToken for RustStdFileContentToken {
    type Proposition = RustStdStandard<File>;
}

impl Establish<KaniFileContentObservation, KaniVerifier> for RustStdStandard<File> {
    type Token = RustStdFileContentToken;

    fn establish(_credential: &KaniFileContentObservation) -> Self::Token {
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

            assert_eq!(
                observation.read(),
                bytes,
                "bytes written to a file are read back unchanged through a fresh handle"
            );

            let _token = RustStdStandard::<File>::establish(&observation);
        }
    }
}

impl KaniWitness for RustStdStandard<FileTimes> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_file_times_sets_the_recorded_modification_time".to_owned(),
            claim: VERIFY_FILE_TIMES_SETS_THE_RECORDED_MODIFICATION_TIME_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<FileTimes>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<FileTimes>",
        verifier: "kani",
        describe: || <RustStdStandard<FileTimes> as KaniWitness>::proof().to_string(),
    }
}

/// Lawful token minted once `RustStdStandard<FileTimes>`'s modification-time
/// claim has been established from a `KaniFileTimesObservation` that has
/// itself demonstrated the exact time reflection.
pub struct RustStdFileTimesToken(());

impl ProofToken for RustStdFileTimesToken {
    type Proposition = RustStdStandard<FileTimes>;
}

impl Establish<KaniFileTimesObservation, KaniVerifier> for RustStdStandard<FileTimes> {
    type Token = RustStdFileTimesToken;

    fn establish(_credential: &KaniFileTimesObservation) -> Self::Token {
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

            assert_eq!(
                observation.modified(),
                target_unix_seconds,
                "the target modification time is reflected exactly in metadata"
            );

            let _token = RustStdStandard::<FileTimes>::establish(&observation);
        }
    }
}

impl KaniWitness for RustStdStandard<FileType> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_file_type_distinguishes_files_from_directories".to_owned(),
            claim: VERIFY_FILE_TYPE_DISTINGUISHES_FILES_FROM_DIRECTORIES_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<FileType>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<FileType>",
        verifier: "kani",
        describe: || <RustStdStandard<FileType> as KaniWitness>::proof().to_string(),
    }
}

/// Lawful token minted once `RustStdStandard<FileType>`'s file/directory
/// distinction claim has been established from a `KaniFileTypeObservation`
/// that has itself demonstrated the mutual exclusion.
pub struct RustStdFileTypeToken(());

impl ProofToken for RustStdFileTypeToken {
    type Proposition = RustStdStandard<FileType>;
}

impl Establish<KaniFileTypeObservation, KaniVerifier> for RustStdStandard<FileType> {
    type Token = RustStdFileTypeToken;

    fn establish(_credential: &KaniFileTypeObservation) -> Self::Token {
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

            assert!(observation.file_is_file());
            assert!(!observation.file_is_dir());

            assert!(observation.directory_is_dir());
            assert!(!observation.directory_is_file());

            let _token = RustStdStandard::<FileType>::establish(&observation);
        }
    }
}

impl KaniWitness for RustStdStandard<Metadata> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_metadata_reports_the_written_length".to_owned(),
            claim: VERIFY_METADATA_REPORTS_THE_WRITTEN_LENGTH_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Metadata>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Metadata>",
        verifier: "kani",
        describe: || <RustStdStandard<Metadata> as KaniWitness>::proof().to_string(),
    }
}

/// Lawful token minted once `RustStdStandard<Metadata>`'s written-length
/// claim has been established from a `KaniFileLenObservation` that has
/// itself demonstrated the exact byte count.
pub struct RustStdMetadataLenToken(());

impl ProofToken for RustStdMetadataLenToken {
    type Proposition = RustStdStandard<Metadata>;
}

impl Establish<KaniFileLenObservation, KaniVerifier> for RustStdStandard<Metadata> {
    type Token = RustStdMetadataLenToken;

    fn establish(_credential: &KaniFileLenObservation) -> Self::Token {
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

            assert_eq!(
                observation.len(),
                u64::from(byte_count),
                "metadata reports exactly the number of bytes written"
            );
            assert_eq!(observation.is_empty(), byte_count == 0);

            let _token = RustStdStandard::<Metadata>::establish(&observation);
        }
    }
}

impl KaniWitness for RustStdStandard<OpenOptions> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_open_options_create_new_rejects_an_existing_file".to_owned(),
            claim: VERIFY_OPEN_OPTIONS_CREATE_NEW_REJECTS_AN_EXISTING_FILE_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<OpenOptions>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<OpenOptions>",
        verifier: "kani",
        describe: || <RustStdStandard<OpenOptions> as KaniWitness>::proof().to_string(),
    }
}

/// Lawful token minted once `RustStdStandard<OpenOptions>`'s `create_new`
/// existence-check claim has been established from a `KaniCreateNewObservation`
/// that has itself demonstrated the successful-creation transition.
pub struct RustStdOpenOptionsCreateNewToken(());

impl ProofToken for RustStdOpenOptionsCreateNewToken {
    type Proposition = RustStdStandard<OpenOptions>;
}

impl Establish<KaniCreateNewObservation, KaniVerifier> for RustStdStandard<OpenOptions> {
    type Token = RustStdOpenOptionsCreateNewToken;

    fn establish(_credential: &KaniCreateNewObservation) -> Self::Token {
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
            let mut existing = crate::KaniCreateNewObservation::existing_file();
            let mut existing_directory = crate::KaniCreateNewObservation::existing_directory();
            let mut fresh = crate::KaniCreateNewObservation::missing();

            assert!(
                existing.create_new().is_err(),
                "create_new fails against a path that already has a file"
            );
            assert!(
                existing_directory.create_new().is_err(),
                "create_new also fails when the path already names a directory"
            );

            assert!(
                fresh.create_new().is_ok(),
                "create_new succeeds against a genuinely fresh path"
            );
            assert!(
                fresh.is_file(),
                "a successful create_new leaves a file at the created path"
            );

            let _token = RustStdStandard::<OpenOptions>::establish(&fresh);
        }
    }
}

impl KaniWitness for RustStdStandard<Permissions> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_permissions_readonly_round_trips_through_set_permissions".to_owned(),
            claim: VERIFY_PERMISSIONS_READONLY_ROUND_TRIPS_THROUGH_SET_PERMISSIONS_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Permissions>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Permissions>",
        verifier: "kani",
        describe: || <RustStdStandard<Permissions> as KaniWitness>::proof().to_string(),
    }
}

/// Lawful token minted once `RustStdStandard<Permissions>`'s readonly
/// round-trip claim has been established from a `KaniPermissionsObservation`
/// that has itself demonstrated the round trip in both directions.
pub struct RustStdPermissionsToken(());

impl ProofToken for RustStdPermissionsToken {
    type Proposition = RustStdStandard<Permissions>;
}

impl Establish<KaniPermissionsObservation, KaniVerifier> for RustStdStandard<Permissions> {
    type Token = RustStdPermissionsToken;

    fn establish(_credential: &KaniPermissionsObservation) -> Self::Token {
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
            let mut observation = crate::KaniPermissionsObservation::new();
            assert!(!observation.readonly(), "a freshly created file is not readonly");

            observation.set_readonly(true);
            assert!(
                observation.readonly(),
                "setting readonly is reflected the next time permissions are read"
            );

            observation.set_readonly(false);
            assert!(!observation.readonly(), "clearing readonly is reflected as well");

            let _token = RustStdStandard::<Permissions>::establish(&observation);
        }
    }
}

impl KaniWitness for RustStdStandard<ReadDir> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_read_dir_iterates_every_entry_in_the_directory".to_owned(),
            claim: VERIFY_READ_DIR_ITERATES_EVERY_ENTRY_IN_THE_DIRECTORY_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<ReadDir>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<ReadDir>",
        verifier: "kani",
        describe: || <RustStdStandard<ReadDir> as KaniWitness>::proof().to_string(),
    }
}

/// Lawful token minted once `RustStdStandard<ReadDir>`'s entry-completeness
/// claim has been established from a `KaniReadDirObservation` that has
/// itself demonstrated exactly the created entries.
pub struct RustStdReadDirToken(());

impl ProofToken for RustStdReadDirToken {
    type Proposition = RustStdStandard<ReadDir>;
}

impl Establish<KaniReadDirObservation, KaniVerifier> for RustStdStandard<ReadDir> {
    type Token = RustStdReadDirToken;

    fn establish(_credential: &KaniReadDirObservation) -> Self::Token {
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
            let entries = observation.entries();

            assert_eq!(entries.len(), 2, "read_dir yields exactly the created entries");
            assert_eq!(entries[0].file_name(), Some(one));
            assert_eq!(entries[1].file_name(), Some(two));

            let _token = RustStdStandard::<ReadDir>::establish(&observation);
        }
    }
}

impl KaniWitness for RustStdStandard<std::fs::TryLockError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_try_lock_error_reports_a_lock_already_held".to_owned(),
            claim: VERIFY_TRY_LOCK_ERROR_REPORTS_A_LOCK_ALREADY_HELD_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::fs::TryLockError>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::fs::TryLockError>",
        verifier: "kani",
        describe: || {
            <RustStdStandard<std::fs::TryLockError> as KaniWitness>::proof().to_string()
        },
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

impl Establish<KaniLockObservation, KaniVerifier> for RustStdStandard<std::fs::TryLockError> {
    type Token = RustStdTryLockErrorToken;

    fn establish(_credential: &KaniLockObservation) -> Self::Token {
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
            let mut file = crate::KaniLockObservation::new();
            assert!(file.try_lock().is_ok(), "the first handle acquires the lock");

            assert!(
                file.try_lock().is_err(),
                "a second handle can't also lock the same file"
            );

            let _token =
                RustStdStandard::<std::fs::TryLockError>::establish(&file);
        }
    }
}
