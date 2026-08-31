//! The `std::fs` types.

use super::machinery::{VerusCheckedProof, VerusWitness, bridge_verus_witness};
use super::path::{
    DIR_BUILDER_MODEL_CREATES_NESTED_DIRECTORIES_RECURSIVELY_VERUS_FRAGMENT,
    VERIFY_DIR_BUILDER_MODEL_CREATES_NESTED_DIRECTORIES_RECURSIVELY_SRC,
};
use crate::RustStdStandard;
use amenable_core::Evidence;

impl VerusWitness for RustStdStandard<std::fs::DirBuilder> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_dir_builder_model_creates_nested_directories_recursively".to_owned(),
            VERIFY_DIR_BUILDER_MODEL_CREATES_NESTED_DIRECTORIES_RECURSIVELY_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::fs::DirBuilder>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fs::DirBuilder>",
        "verus",
        || {
            <RustStdStandard<std::fs::DirBuilder> as VerusWitness>::proof().to_string()
        },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fs::DirBuilder>",
        "verus",
        "ensures",
        || DIR_BUILDER_MODEL_CREATES_NESTED_DIRECTORIES_RECURSIVELY_VERUS_FRAGMENT,
    )
}

const VERIFY_DIR_ENTRY_MODEL_REPORTS_THE_CREATED_FILES_NAME_AND_PATH_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/path_and_fs/fs_path_carrier.rs");

const DIR_ENTRY_MODEL_REPORTS_THE_CREATED_FILES_NAME_AND_PATH_VERUS_FRAGMENT: &str = r#"pub open spec fn dir_entry_model_reports_the_created_files_name_and_path(
    parent: char,
    name: char,
    result: (char, (char, char)),
) -> bool {
    &&& result.0 == name
    &&& result.1 == (parent, name)
}"#;

impl VerusWitness for RustStdStandard<std::fs::DirEntry> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_dir_entry_model_reports_the_created_files_name_and_path".to_owned(),
            VERIFY_DIR_ENTRY_MODEL_REPORTS_THE_CREATED_FILES_NAME_AND_PATH_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::fs::DirEntry>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fs::DirEntry>",
        "verus",
        || {
            <RustStdStandard<std::fs::DirEntry> as VerusWitness>::proof().to_string()
        },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fs::DirEntry>",
        "verus",
        "ensures",
        || DIR_ENTRY_MODEL_REPORTS_THE_CREATED_FILES_NAME_AND_PATH_VERUS_FRAGMENT,
    )
}

const VERIFY_READ_DIR_MODEL_ITERATES_EVERY_ENTRY_IN_THE_DIRECTORY_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/path_and_fs/fs_path_carrier.rs");

const READ_DIR_MODEL_ITERATES_EVERY_ENTRY_IN_THE_DIRECTORY_VERUS_FRAGMENT: &str = r#"pub open spec fn read_dir_model_iterates_every_entry_in_the_directory(
    first_name: char,
    second_name: char,
    result: (u32, char, char),
) -> bool {
    &&& result.0 == 2
    &&& result.1 == first_name
    &&& result.2 == second_name
}"#;

impl VerusWitness for RustStdStandard<std::fs::ReadDir> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_read_dir_model_iterates_every_entry_in_the_directory".to_owned(),
            VERIFY_READ_DIR_MODEL_ITERATES_EVERY_ENTRY_IN_THE_DIRECTORY_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::fs::ReadDir>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fs::ReadDir>",
        "verus",
        || {
            <RustStdStandard<std::fs::ReadDir> as VerusWitness>::proof().to_string()
        },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fs::ReadDir>",
        "verus",
        "ensures",
        || READ_DIR_MODEL_ITERATES_EVERY_ENTRY_IN_THE_DIRECTORY_VERUS_FRAGMENT,
    )
}

const VERIFY_FILE_MODEL_WRITE_THEN_READ_ROUND_TRIPS_THE_BYTES_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/path_and_fs/fs_content_carrier.rs");

const FILE_MODEL_WRITE_THEN_READ_ROUND_TRIPS_THE_BYTES_VERUS_FRAGMENT: &str = r#"pub open spec fn file_model_write_then_read_round_trips_the_bytes(
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    result: (u8, u8, u8, u8),
) -> bool {
    result == (a, b, c, d)
}"#;

impl VerusWitness for RustStdStandard<std::fs::File> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_file_model_write_then_read_round_trips_the_bytes".to_owned(),
            VERIFY_FILE_MODEL_WRITE_THEN_READ_ROUND_TRIPS_THE_BYTES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::fs::File>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fs::File>",
        "verus",
        || { <RustStdStandard<std::fs::File> as VerusWitness>::proof().to_string() },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fs::File>",
        "verus",
        "ensures",
        || FILE_MODEL_WRITE_THEN_READ_ROUND_TRIPS_THE_BYTES_VERUS_FRAGMENT,
    )
}

const VERIFY_FILE_TIMES_MODEL_SETS_THE_RECORDED_MODIFICATION_TIME_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/path_and_fs/fs_content_carrier.rs");

const FILE_TIMES_MODEL_SETS_THE_RECORDED_MODIFICATION_TIME_VERUS_FRAGMENT: &str = r#"pub open spec fn file_times_model_sets_the_recorded_modification_time(
    target_unix_seconds: u64,
    result: u64,
) -> bool {
    result == target_unix_seconds
}"#;

impl VerusWitness for RustStdStandard<std::fs::FileTimes> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_file_times_model_sets_the_recorded_modification_time".to_owned(),
            VERIFY_FILE_TIMES_MODEL_SETS_THE_RECORDED_MODIFICATION_TIME_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::fs::FileTimes>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fs::FileTimes>",
        "verus",
        || {
            <RustStdStandard<std::fs::FileTimes> as VerusWitness>::proof().to_string()
        },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fs::FileTimes>",
        "verus",
        "ensures",
        || FILE_TIMES_MODEL_SETS_THE_RECORDED_MODIFICATION_TIME_VERUS_FRAGMENT,
    )
}

const VERIFY_METADATA_MODEL_REPORTS_THE_WRITTEN_LENGTH_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/path_and_fs/fs_content_carrier.rs");

const METADATA_MODEL_REPORTS_THE_WRITTEN_LENGTH_VERUS_FRAGMENT: &str = r#"pub open spec fn metadata_model_reports_the_written_length(
    byte_count: u8,
    result: (u64, bool),
) -> bool {
    &&& result.0 == byte_count as u64
    &&& result.1 == (byte_count == 0)
}"#;

impl VerusWitness for RustStdStandard<std::fs::Metadata> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_metadata_model_reports_the_written_length".to_owned(),
            VERIFY_METADATA_MODEL_REPORTS_THE_WRITTEN_LENGTH_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::fs::Metadata>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fs::Metadata>",
        "verus",
        || {
            <RustStdStandard<std::fs::Metadata> as VerusWitness>::proof().to_string()
        },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fs::Metadata>",
        "verus",
        "ensures",
        || METADATA_MODEL_REPORTS_THE_WRITTEN_LENGTH_VERUS_FRAGMENT,
    )
}

const VERIFY_FILE_TYPE_MODEL_DISTINGUISHES_FILES_FROM_DIRECTORIES_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/path_and_fs/fs_boolean_laws_carrier.rs");

impl VerusWitness for RustStdStandard<std::fs::FileType> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_file_type_model_distinguishes_files_from_directories".to_owned(),
            VERIFY_FILE_TYPE_MODEL_DISTINGUISHES_FILES_FROM_DIRECTORIES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::fs::FileType>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fs::FileType>",
        "verus",
        || {
            <RustStdStandard<std::fs::FileType> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_OPEN_OPTIONS_MODEL_CREATE_NEW_REJECTS_AN_EXISTING_FILE_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/path_and_fs/fs_boolean_laws_carrier.rs");

impl VerusWitness for RustStdStandard<std::fs::OpenOptions> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_open_options_model_create_new_rejects_an_existing_file".to_owned(),
            VERIFY_OPEN_OPTIONS_MODEL_CREATE_NEW_REJECTS_AN_EXISTING_FILE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::fs::OpenOptions>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fs::OpenOptions>",
        "verus",
        || {
            <RustStdStandard<std::fs::OpenOptions> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_PERMISSIONS_MODEL_READONLY_ROUND_TRIPS_THROUGH_SET_PERMISSIONS_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/path_and_fs/fs_boolean_laws_carrier.rs");

impl VerusWitness for RustStdStandard<std::fs::Permissions> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_permissions_model_readonly_round_trips_through_set_permissions".to_owned(),
            VERIFY_PERMISSIONS_MODEL_READONLY_ROUND_TRIPS_THROUGH_SET_PERMISSIONS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::fs::Permissions>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fs::Permissions>",
        "verus",
        || {
            <RustStdStandard<std::fs::Permissions> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_TRY_LOCK_ERROR_MODEL_REPORTS_A_LOCK_ALREADY_HELD_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/path_and_fs/fs_boolean_laws_carrier.rs");

impl VerusWitness for RustStdStandard<std::fs::TryLockError> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_try_lock_error_model_reports_a_lock_already_held".to_owned(),
            VERIFY_TRY_LOCK_ERROR_MODEL_REPORTS_A_LOCK_ALREADY_HELD_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::fs::TryLockError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fs::TryLockError>",
        "verus",
        || {
            <RustStdStandard<std::fs::TryLockError> as VerusWitness>::proof().to_string()
        },
    )
}

pub(super) const VERIFY_LOCAL_KEY_MODEL_WITH_READS_THE_INITIALIZED_VALUE_SRC: &str = include_str!(
    "../../../amenable_verus/src/rust_std/task_and_thread/thread_local_key_carrier.rs"
);
