//! `KaniWitness` impls for `std::fs`.
//!
//! Every harness exercises a genuine, temporary location under
//! `std::env::temp_dir()`, scoped by both a fixed per-harness label and
//! the running process's id so concurrent runs can't collide, and removes
//! whatever it created before returning.

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
        #[kani::proof]
        fn verify_dir_builder_creates_nested_directories_recursively() {
            let base = std::env::temp_dir()
                .join(format!("amenable_kani_fs_dirbuilder_{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&base);
            let nested = base.join("a").join("b").join("c");

            DirBuilder::new().recursive(true).create(&nested).unwrap();
            assert!(nested.is_dir());

            std::fs::remove_dir_all(&base).unwrap();
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
        #[kani::proof]
        fn verify_dir_entry_reports_the_created_files_name_and_path() {
            let base = std::env::temp_dir()
                .join(format!("amenable_kani_fs_direntry_{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&base);
            std::fs::create_dir_all(&base).unwrap();
            File::create(base.join("one.txt")).unwrap();

            let entry = std::fs::read_dir(&base)
                .unwrap()
                .map(|entry| entry.unwrap())
                .find(|entry| entry.file_name() == "one.txt")
                .unwrap();
            assert_eq!(entry.path(), base.join("one.txt"));

            std::fs::remove_dir_all(&base).unwrap();
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
        #[kani::proof]
        fn verify_file_write_then_read_round_trips_the_bytes() {
            use std::io::{Read, Write};

            let base = std::env::temp_dir()
                .join(format!("amenable_kani_fs_file_{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&base);
            std::fs::create_dir_all(&base).unwrap();
            let path = base.join("data.txt");

            {
                let mut file = File::create(&path).unwrap();
                file.write_all(b"hello, file").unwrap();
            }
            let mut read_back = String::new();
            File::open(&path).unwrap().read_to_string(&mut read_back).unwrap();
            assert_eq!(read_back, "hello, file");

            std::fs::remove_dir_all(&base).unwrap();
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
        #[kani::proof]
        fn verify_file_times_sets_the_recorded_modification_time() {
            use std::time::{Duration, SystemTime};

            let base = std::env::temp_dir()
                .join(format!("amenable_kani_fs_filetimes_{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&base);
            std::fs::create_dir_all(&base).unwrap();
            let path = base.join("data.txt");
            File::create(&path).unwrap();

            let target_time = SystemTime::UNIX_EPOCH + Duration::from_secs(1_700_000_000);
            let times = FileTimes::new().set_modified(target_time);
            OpenOptions::new()
                .write(true)
                .open(&path)
                .unwrap()
                .set_times(times)
                .unwrap();

            let observed = std::fs::metadata(&path).unwrap().modified().unwrap();
            assert_eq!(observed, target_time);

            std::fs::remove_dir_all(&base).unwrap();
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
        #[kani::proof]
        fn verify_file_type_distinguishes_files_from_directories() {
            let base = std::env::temp_dir()
                .join(format!("amenable_kani_fs_filetype_{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&base);
            std::fs::create_dir_all(&base).unwrap();
            let file_path = base.join("data.txt");
            File::create(&file_path).unwrap();

            let file_type = std::fs::metadata(&file_path).unwrap().file_type();
            assert!(file_type.is_file());
            assert!(!file_type.is_dir());

            let dir_type = std::fs::metadata(&base).unwrap().file_type();
            assert!(dir_type.is_dir());
            assert!(!dir_type.is_file());

            std::fs::remove_dir_all(&base).unwrap();
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
        #[kani::proof]
        fn verify_metadata_reports_the_written_length() {
            use std::io::Write;

            let base = std::env::temp_dir()
                .join(format!("amenable_kani_fs_metadata_{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&base);
            std::fs::create_dir_all(&base).unwrap();
            let path = base.join("data.txt");
            File::create(&path).unwrap().write_all(b"hello, file").unwrap();

            assert_eq!(
                std::fs::metadata(&path).unwrap().len(),
                b"hello, file".len() as u64
            );

            std::fs::remove_dir_all(&base).unwrap();
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
        #[kani::proof]
        fn verify_open_options_create_new_rejects_an_existing_file() {
            let base = std::env::temp_dir()
                .join(format!("amenable_kani_fs_openoptions_{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&base);
            std::fs::create_dir_all(&base).unwrap();
            let existing = base.join("existing.txt");
            File::create(&existing).unwrap();

            assert_eq!(
                OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(&existing)
                    .unwrap_err()
                    .kind(),
                std::io::ErrorKind::AlreadyExists
            );

            let fresh = base.join("fresh.txt");
            OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&fresh)
                .unwrap();
            assert!(fresh.exists());

            std::fs::remove_dir_all(&base).unwrap();
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
        #[kani::proof]
        fn verify_permissions_readonly_round_trips_through_set_permissions() {
            let base = std::env::temp_dir()
                .join(format!("amenable_kani_fs_permissions_{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&base);
            std::fs::create_dir_all(&base).unwrap();
            let path = base.join("data.txt");
            File::create(&path).unwrap();

            let mut perms = std::fs::metadata(&path).unwrap().permissions();
            assert!(!perms.readonly());

            perms.set_readonly(true);
            std::fs::set_permissions(&path, perms).unwrap();
            assert!(std::fs::metadata(&path).unwrap().permissions().readonly());

            // Reset so the directory can be cleaned up afterward.
            let mut perms = std::fs::metadata(&path).unwrap().permissions();
            perms.set_readonly(false);
            std::fs::set_permissions(&path, perms).unwrap();

            std::fs::remove_dir_all(&base).unwrap();
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
        #[kani::proof]
        fn verify_read_dir_iterates_every_entry_in_the_directory() {
            let base = std::env::temp_dir()
                .join(format!("amenable_kani_fs_readdir_{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&base);
            std::fs::create_dir_all(&base).unwrap();
            File::create(base.join("one.txt")).unwrap();
            File::create(base.join("two.txt")).unwrap();

            let mut names: Vec<String> = std::fs::read_dir(&base)
                .unwrap()
                .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
                .collect();
            names.sort();
            assert_eq!(names, vec!["one.txt".to_string(), "two.txt".to_string()]);

            std::fs::remove_dir_all(&base).unwrap();
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
        #[kani::proof]
        fn verify_try_lock_error_reports_a_lock_already_held() {
            let base = std::env::temp_dir()
                .join(format!("amenable_kani_fs_trylock_{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&base);
            std::fs::create_dir_all(&base).unwrap();
            let path = base.join("lock.txt");

            let first = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&path)
                .unwrap();
            first.try_lock().unwrap();

            let second = OpenOptions::new().write(true).open(&path).unwrap();
            assert!(
                second.try_lock().is_err(),
                "a second handle can't also lock the same file"
            );
            drop(first);

            std::fs::remove_dir_all(&base).unwrap();
        }
    }
}
