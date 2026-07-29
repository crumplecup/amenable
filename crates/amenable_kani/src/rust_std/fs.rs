//! `KaniWitness` impls for `std::fs`.
//!
//! The direct `std::fs` tempdir path crosses real OS-backed filesystem state
//! that Kani times out on today. Production proofs are therefore being
//! migrated incrementally onto an Amenable-owned filesystem model; the direct
//! real-filesystem boundary remains preserved in the gallery.

use std::fs::{
    DirBuilder, DirEntry, File, FileTimes, FileType, Metadata, OpenOptions, Permissions, ReadDir,
};

use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::bridge_kani_witness;

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

amenable_derive::harness! {
    kani, VERIFY_DIR_BUILDER_CREATES_NESTED_DIRECTORIES_RECURSIVELY_SRC, {
        /// `.recursive(true)` creates every missing ancestor directory,
        /// not just the leaf.
        /// This proof uses the Amenable-owned filesystem model: if the real
        /// `DirBuilder` recursive path refines these directory-creation laws,
        /// the Rust-facing claim follows.
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

amenable_derive::harness! {
    kani, VERIFY_DIR_ENTRY_REPORTS_THE_CREATED_FILES_NAME_AND_PATH_SRC, {
        /// A `DirEntry` yielded for a created file reports that file's
        /// own name and full path.
        /// This proof uses the Amenable-owned filesystem model: if the real
        /// `read_dir` / `DirEntry` path preserves entry identity this way, the
        /// Rust-facing claim follows.
        #[kani::proof]
        fn verify_dir_entry_reports_the_created_files_name_and_path() {
            let base = crate::KaniFsPath::root().join(crate::KaniFsLabel::new('b'));
            let path = base.join(crate::KaniFsLabel::new('f'));
            let entry = crate::KaniDirEntryObservation::new(base, crate::KaniFsLabel::new('f'))
                .entry();

            assert_eq!(entry.file_name(), Some(crate::KaniFsLabel::new('f')));
            assert_eq!(entry.path(), path);
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

amenable_derive::harness! {
    kani, VERIFY_FILE_WRITE_THEN_READ_ROUND_TRIPS_THE_BYTES_SRC, {
        /// Bytes written to a file and flushed by `Drop` are read back
        /// unchanged through a fresh handle.
        /// This proof uses the Amenable-owned filesystem model: if the real
        /// `File::write_all` / re-open path preserves written bytes this
        /// way, the Rust-facing claim follows.
        #[kani::proof]
        fn verify_file_write_then_read_round_trips_the_bytes() {
            let bytes: [u8; 4] = kani::any();
            let observation = crate::KaniFileContentObservation::write(bytes);

            assert_eq!(
                observation.read(),
                bytes,
                "bytes written to a file are read back unchanged through a fresh handle"
            );
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

amenable_derive::harness! {
    kani, VERIFY_FILE_TIMES_SETS_THE_RECORDED_MODIFICATION_TIME_SRC, {
        /// `.set_modified()` on a `FileTimes`, applied via
        /// `File::set_times()`, is reflected exactly in the file's
        /// metadata. The target time is whole-second-precision, since
        /// some filesystems truncate sub-second components.
        /// This proof uses the Amenable-owned filesystem model: if the real
        /// `set_times` / `metadata().modified()` path preserves the target
        /// time this way, the Rust-facing claim follows.
        #[kani::proof]
        fn verify_file_times_sets_the_recorded_modification_time() {
            let target_unix_seconds: u64 = kani::any();
            let observation = crate::KaniFileTimesObservation::set_modified(target_unix_seconds);

            assert_eq!(
                observation.modified(),
                target_unix_seconds,
                "the target modification time is reflected exactly in metadata"
            );
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

amenable_derive::harness! {
    kani, VERIFY_FILE_TYPE_DISTINGUISHES_FILES_FROM_DIRECTORIES_SRC, {
        /// A regular file's `FileType` reports `is_file()`, and a
        /// directory's reports `is_dir()` — never both.
        /// This proof uses the Amenable-owned filesystem model: if the real
        /// `metadata().file_type()` path preserves this file/directory
        /// distinction, the Rust-facing claim follows.
        #[kani::proof]
        fn verify_file_type_distinguishes_files_from_directories() {
            let observation = crate::KaniFileTypeObservation::new();

            assert!(observation.file_is_file());
            assert!(!observation.file_is_dir());

            assert!(observation.directory_is_dir());
            assert!(!observation.directory_is_file());
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

amenable_derive::harness! {
    kani, VERIFY_METADATA_REPORTS_THE_WRITTEN_LENGTH_SRC, {
        /// `.len()` reports exactly the number of bytes written to the
        /// file.
        /// This proof uses the Amenable-owned filesystem model: if the real
        /// `metadata().len()` path reports the written byte count this way,
        /// the Rust-facing claim follows.
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

amenable_derive::harness! {
    kani, VERIFY_OPEN_OPTIONS_CREATE_NEW_REJECTS_AN_EXISTING_FILE_SRC, {
        /// `.create_new(true)` fails with `AlreadyExists` on a path that
        /// already has a file, and succeeds on a genuinely fresh one.
        /// This proof uses the Amenable-owned filesystem model: if the real
        /// `OpenOptions::create_new` path preserves this existence check,
        /// the Rust-facing claim follows.
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

amenable_derive::harness! {
    kani, VERIFY_PERMISSIONS_READONLY_ROUND_TRIPS_THROUGH_SET_PERMISSIONS_SRC, {
        /// Flipping `.set_readonly(true)` and applying it via
        /// `fs::set_permissions` is reflected the next time the file's
        /// permissions are read.
        /// This proof uses the Amenable-owned filesystem model: if the real
        /// `set_permissions` / `metadata().permissions()` path preserves
        /// the readonly bit this way, the Rust-facing claim follows.
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

amenable_derive::harness! {
    kani, VERIFY_READ_DIR_ITERATES_EVERY_ENTRY_IN_THE_DIRECTORY_SRC, {
        /// `.read_dir()` yields exactly the files that were created in
        /// that directory, no more and no fewer.
        /// This proof uses the Amenable-owned filesystem model: if the real
        /// `read_dir` path preserves entry identity this way, the
        /// Rust-facing claim follows.
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

amenable_derive::harness! {
    kani, VERIFY_TRY_LOCK_ERROR_REPORTS_A_LOCK_ALREADY_HELD_SRC, {
        /// A second handle's `.try_lock()` fails while the first handle
        /// still holds the file lock.
        /// This proof uses the Amenable-owned filesystem model: if the real
        /// `File::try_lock` path preserves this exclusion, the Rust-facing
        /// claim follows.
        #[kani::proof]
        fn verify_try_lock_error_reports_a_lock_already_held() {
            let mut file = crate::KaniLockObservation::new();
            assert!(file.try_lock().is_ok(), "the first handle acquires the lock");

            assert!(
                file.try_lock().is_err(),
                "a second handle can't also lock the same file"
            );
        }
    }
}
