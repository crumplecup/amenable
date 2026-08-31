//! `KaniWitness` impls for `std::fs`.
//!
//! The direct `std::fs` tempdir path crosses real OS-backed filesystem state
//! that Kani times out on today. Production proofs are therefore being
//! migrated incrementally onto an Amenable-owned filesystem model; the direct
//! real-filesystem boundary remains preserved in the gallery.
//!
//! Split by the real API family each file covers: [`dir_builder_and_entry`]
//! (`DirBuilder`, `DirEntry`), [`file_and_times`] (`File`, `FileTimes`),
//! [`file_type_and_metadata`] (`FileType`, `Metadata`),
//! [`open_options_and_permissions`] (`OpenOptions`, `Permissions`), and
//! [`read_dir_and_lock`] (`ReadDir`, `TryLockError`).

mod dir_builder_and_entry;
mod file_and_times;
mod file_type_and_metadata;
mod open_options_and_permissions;
mod read_dir_and_lock;
