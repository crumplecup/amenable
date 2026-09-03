use amenable_core::{MetadataEntry, Provenance};
use amenable_derive::Standard;

use super::tree_primitives::KaniFsNodeKind;

/// Observable result of reading a file's own type against a sibling
/// directory's, mutually exclusive by construction.
///
/// The assumption this observation stands in for -- that a regular file's
/// `FileType` reports `is_file()` and a directory's reports `is_dir()`,
/// never both, and nothing else about the real OS-backed type distinction
/// -- is named explicitly as a `Standard` rather than left as prose: the
/// direct `std::fs` path crosses OS-backed state Kani cannot symbolically
/// execute well (see this module's own doc comment), so this bounded
/// observation is what the `FileType` proof actually rests on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Standard)]
#[standard(basis = "Self")]
pub struct KaniFileTypeObservation {
    file: KaniFsNodeKind,
    directory: KaniFsNodeKind,
}

impl Provenance for KaniFileTypeObservation {
    type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn metadata(&self) -> Self::MetadataIter {
        Box::new({
            vec![
            MetadataEntry::new(
                "assumed",
                "a regular file's FileType reports is_file() and a directory's reports is_dir(), never both, standing in for the real OS-backed type distinction",
            ),
            MetadataEntry::new(
                "rationale",
                "the direct std::fs path crosses OS-backed state Kani cannot symbolically execute well today",
            ),
            MetadataEntry::new("file", format!("{:?}", self.file)),
            MetadataEntry::new("directory", format!("{:?}", self.directory)),
        ]
        .into_iter()
        })
    }
}

impl KaniFileTypeObservation {
    /// Model one file node and one directory node.
    #[must_use]
    pub fn new() -> Self {
        Self {
            file: KaniFsNodeKind::File,
            directory: KaniFsNodeKind::Directory,
        }
    }

    /// Report whether the modeled file node reports as a file.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn file_is_file(&self) -> bool {
        self.file == KaniFsNodeKind::File
    }

    /// Report whether the modeled file node reports as a directory.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn file_is_dir(&self) -> bool {
        self.file == KaniFsNodeKind::Directory
    }

    /// Report whether the modeled directory node reports as a directory.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn directory_is_dir(&self) -> bool {
        self.directory == KaniFsNodeKind::Directory
    }

    /// Report whether the modeled directory node reports as a file.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn directory_is_file(&self) -> bool {
        self.directory == KaniFsNodeKind::File
    }
}

impl Default for KaniFileTypeObservation {
    #[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
    fn default() -> Self {
        Self::new()
    }
}

/// Observable result of writing a fixed-width byte sequence to a file and
/// reading it back through a fresh handle.
///
/// The assumption this observation stands in for -- that bytes written to
/// a file and flushed by `Drop` are read back unchanged through a fresh
/// handle, and nothing else about the real OS-backed write/read path -- is
/// named explicitly as a `Standard` rather than left as prose: the direct
/// `std::fs` path crosses OS-backed state Kani cannot symbolically execute
/// well (see this module's own doc comment), so this bounded observation is
/// what the `File` proof actually rests on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Standard)]
#[standard(basis = "Self", basis_ctor = "Self::write([0u8; 4])")]
pub struct KaniFileContentObservation {
    content: [u8; 4],
}

impl Provenance for KaniFileContentObservation {
    type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn metadata(&self) -> Self::MetadataIter {
        Box::new({
            vec![
            MetadataEntry::new(
                "assumed",
                "bytes written to a file and flushed by Drop are read back unchanged through a fresh handle, standing in for the real OS-backed write/read path",
            ),
            MetadataEntry::new(
                "rationale",
                "the direct std::fs path crosses OS-backed state Kani cannot symbolically execute well today",
            ),
            MetadataEntry::new("content", format!("{:?}", self.content)),
        ]
        .into_iter()
        })
    }
}

impl KaniFileContentObservation {
    /// Model writing `content` to a fresh file.
    #[must_use]
    pub fn write(content: [u8; 4]) -> Self {
        Self { content }
    }

    /// Model reading the file back through a fresh handle.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn read(&self) -> [u8; 4] {
        self.content
    }
}

/// Observable result of writing a byte sequence and reading its recorded
/// length back from metadata.
///
/// The assumption this observation stands in for -- that `.len()` reports
/// exactly the number of bytes written to the file, and nothing else about
/// the real OS-backed metadata query -- is named explicitly as a `Standard`
/// rather than left as prose: the direct `std::fs` path crosses OS-backed
/// state Kani cannot symbolically execute well (see this module's own doc
/// comment), so this bounded observation is what the `Metadata` proof
/// actually rests on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Standard)]
#[standard(basis = "Self")]
pub struct KaniFileLenObservation {
    len: u8,
}

impl Provenance for KaniFileLenObservation {
    type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn metadata(&self) -> Self::MetadataIter {
        Box::new({
            vec![
            MetadataEntry::new(
                "assumed",
                ".len() reports exactly the number of bytes written to the file, standing in for the real OS-backed metadata query",
            ),
            MetadataEntry::new(
                "rationale",
                "the direct std::fs path crosses OS-backed state Kani cannot symbolically execute well today",
            ),
            MetadataEntry::new("len", self.len.to_string()),
        ]
        .into_iter()
        })
    }
}

impl KaniFileLenObservation {
    /// Model writing `len` bytes to a fresh file.
    #[must_use]
    pub fn write(len: u8) -> Self {
        Self { len }
    }

    /// Model the file's recorded metadata length.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn len(&self) -> u64 {
        u64::from(self.len)
    }

    /// Report whether the modeled file is empty.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

/// Observable result of `.set_modified()` / metadata `.modified()`.
///
/// The assumption this observation stands in for -- that a target
/// modification time set via `.set_modified()`, applied through
/// `File::set_times()`, is reflected exactly in the file's metadata, and
/// nothing else about the real OS-backed filesystem clock -- is named
/// explicitly as a `Standard` rather than left as prose: the direct
/// `std::fs` path crosses OS-backed state Kani cannot symbolically execute
/// well (see this module's own doc comment), so this bounded observation is
/// what the `FileTimes` proof actually rests on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Standard)]
#[standard(basis = "Self")]
pub struct KaniFileTimesObservation {
    modified_unix_seconds: u64,
}

impl Provenance for KaniFileTimesObservation {
    type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn metadata(&self) -> Self::MetadataIter {
        Box::new({
            vec![
            MetadataEntry::new(
                "assumed",
                "a target modification time set via .set_modified() is reflected exactly in the file's metadata, standing in for the real OS-backed filesystem clock",
            ),
            MetadataEntry::new(
                "rationale",
                "the direct std::fs path crosses OS-backed state Kani cannot symbolically execute well today",
            ),
            MetadataEntry::new(
                "modified_unix_seconds",
                self.modified_unix_seconds.to_string(),
            ),
        ]
        .into_iter()
        })
    }
}

impl KaniFileTimesObservation {
    /// Model setting a file's modification time.
    #[must_use]
    pub fn set_modified(modified_unix_seconds: u64) -> Self {
        Self {
            modified_unix_seconds,
        }
    }

    /// Model reading the file's recorded modification time back.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn modified(&self) -> u64 {
        self.modified_unix_seconds
    }
}
