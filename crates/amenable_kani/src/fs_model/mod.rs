//! Kani-only accommodation model for `std::fs`.
//!
//! The real `std::fs` path crosses OS-backed filesystem state that Kani cannot
//! symbolically execute well today. This model captures the bounded observable
//! laws the production proofs actually claim, starting with recursive
//! directory creation and directory entry reporting.
//!
//! Split by the real concept each file covers: [`tree_primitives`] (the
//! symbolic path/label/node vocabulary every other file builds on),
//! [`directory_observations`] (recursive directory creation, directory
//! entries, and the `KaniFileSystem` simulator itself),
//! [`file_metadata_observations`] (file type, content, length, and times),
//! [`create_and_permissions_observations`] (`create_new` and permissions),
//! and [`lock_observations`] (file locking).

mod create_and_permissions_observations;
mod directory_observations;
mod file_metadata_observations;
mod lock_observations;
mod tree_primitives;

pub use create_and_permissions_observations::{
    KaniAlreadyExists, KaniCreateNewObservation, KaniPermissionsObservation, KaniReadDirObservation,
};
pub use directory_observations::{
    KaniDirEntryObservation, KaniFileSystem, KaniRecursiveDirObservation,
};
pub use file_metadata_observations::{
    KaniFileContentObservation, KaniFileLenObservation, KaniFileTimesObservation,
    KaniFileTypeObservation,
};
pub use lock_observations::{KaniAlreadyLocked, KaniLockObservation};
pub use tree_primitives::{KaniFsDirEntry, KaniFsLabel, KaniFsNodeKind, KaniFsPath};
