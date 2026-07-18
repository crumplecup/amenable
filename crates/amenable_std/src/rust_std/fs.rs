//! `RustStdType` registrations for `std::fs`.

use std::fs::{
    DirBuilder, DirEntry, File, FileTimes, FileType, Metadata, OpenOptions, Permissions, ReadDir,
    TryLockError,
};

use crate::rust_std::macros::{impl_rust_std_type, register_rust_std_standard_evidence};

impl_rust_std_type!(
    DirBuilder,
    "std",
    "std::fs",
    "https://doc.rust-lang.org/std/fs/struct.DirBuilder.html",
    "The DirBuilder carrier configures how a directory (and optionally its parents) is created."
);

impl_rust_std_type!(
    DirEntry,
    "std",
    "std::fs",
    "https://doc.rust-lang.org/std/fs/struct.DirEntry.html",
    "The DirEntry carrier describes one entry yielded while reading a directory."
);

impl_rust_std_type!(
    File,
    "std",
    "std::fs",
    "https://doc.rust-lang.org/std/fs/struct.File.html",
    "The File carrier is a handle to an open file on the filesystem."
);

impl_rust_std_type!(
    FileTimes,
    "std",
    "std::fs",
    "https://doc.rust-lang.org/std/fs/struct.FileTimes.html",
    "The FileTimes carrier configures the access and modification timestamps to set on a file."
);

impl_rust_std_type!(
    FileType,
    "std",
    "std::fs",
    "https://doc.rust-lang.org/std/fs/struct.FileType.html",
    "The FileType carrier classifies a filesystem entry as a regular file, directory, or symlink."
);

impl_rust_std_type!(
    Metadata,
    "std",
    "std::fs",
    "https://doc.rust-lang.org/std/fs/struct.Metadata.html",
    "The Metadata carrier holds the filesystem metadata for a file: size, permissions, timestamps, and file type."
);

impl_rust_std_type!(
    OpenOptions,
    "std",
    "std::fs",
    "https://doc.rust-lang.org/std/fs/struct.OpenOptions.html",
    "The OpenOptions carrier configures how a file is opened: read, write, append, create, and truncate options."
);

impl_rust_std_type!(
    Permissions,
    "std",
    "std::fs",
    "https://doc.rust-lang.org/std/fs/struct.Permissions.html",
    "The Permissions carrier represents a filesystem entry's access permissions."
);

impl_rust_std_type!(
    ReadDir,
    "std",
    "std::fs",
    "https://doc.rust-lang.org/std/fs/struct.ReadDir.html",
    "The ReadDir carrier lazily yields the entries of a directory."
);

impl_rust_std_type!(
    TryLockError,
    "std",
    "std::fs",
    "https://doc.rust-lang.org/std/fs/enum.TryLockError.html",
    "The TryLockError carrier reports why a non-blocking file lock attempt failed."
);

// `TryLockError` is written fully-qualified: `std::sync::TryLockError`
// shares its bare name, and only the qualified path disambiguates which
// one a given registration means for tooling reading the registry (e.g.
// `elicit_doc`'s coverage report).
register_rust_std_standard_evidence!(
    DirBuilder,
    DirEntry,
    File,
    FileTimes,
    FileType,
    Metadata,
    OpenOptions,
    Permissions,
    ReadDir,
    std::fs::TryLockError,
);
