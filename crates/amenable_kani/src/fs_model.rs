//! Kani-only accommodation model for `std::fs`.
//!
//! The real `std::fs` path crosses OS-backed filesystem state that Kani cannot
//! symbolically execute well today. This model captures the bounded observable
//! laws the production proofs actually claim, starting with recursive
//! directory creation and directory entry reporting.

use amenable_core::{MetadataEntry, Provenance};
use amenable_derive::Standard;

const KANI_FS_MAX_DEPTH: usize = 4;

/// Symbolic path-segment label for filesystem proofs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KaniFsLabel(char);

impl KaniFsLabel {
    /// Create a modeled path label.
    #[must_use]
    pub fn new(label: char) -> Self {
        Self(label)
    }
}

/// Bounded, fixed-depth path model for filesystem proofs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KaniFsPath {
    depth: usize,
    segments: [Option<KaniFsLabel>; KANI_FS_MAX_DEPTH],
}

impl KaniFsPath {
    /// Return the model root.
    #[must_use]
    pub fn root() -> Self {
        Self {
            depth: 0,
            segments: [None; KANI_FS_MAX_DEPTH],
        }
    }

    /// Return a child path under this path.
    #[must_use]
    pub fn join(&self, segment: KaniFsLabel) -> Self {
        assert!(
            self.depth < KANI_FS_MAX_DEPTH,
            "KaniFsPath exceeded the modeled maximum depth"
        );

        let mut segments = self.segments;
        segments[self.depth] = Some(segment);
        Self {
            depth: self.depth + 1,
            segments,
        }
    }

    /// Return the final path segment, if any.
    #[must_use]
    pub fn file_name(&self) -> Option<KaniFsLabel> {
        if self.depth == 0 {
            None
        } else {
            self.segments[self.depth - 1]
        }
    }

    fn parent(&self) -> Option<Self> {
        if self.depth == 0 {
            None
        } else {
            let mut segments = self.segments;
            segments[self.depth - 1] = None;
            Some(Self {
                depth: self.depth - 1,
                segments,
            })
        }
    }

    fn prefixes_without_root(&self) -> [Option<Self>; KANI_FS_MAX_DEPTH] {
        let mut prefixes = [None; KANI_FS_MAX_DEPTH];
        let mut index = 0;

        while index < self.depth {
            let mut segments = [None; KANI_FS_MAX_DEPTH];
            let mut segment_index = 0;
            while segment_index <= index {
                segments[segment_index] = self.segments[segment_index];
                segment_index += 1;
            }
            prefixes[index] = Some(Self {
                depth: index + 1,
                segments,
            });
            index += 1;
        }

        prefixes
    }
}

/// Minimal node kind distinction needed by the current filesystem proofs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KaniFsNodeKind {
    /// Directory-like node.
    Directory,
    /// File-like node.
    File,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct KaniFsNode {
    path: KaniFsPath,
    kind: KaniFsNodeKind,
}

impl KaniFsNode {
    fn directory(path: KaniFsPath) -> Self {
        Self {
            path,
            kind: KaniFsNodeKind::Directory,
        }
    }

    fn file(path: KaniFsPath) -> Self {
        Self {
            path,
            kind: KaniFsNodeKind::File,
        }
    }
}

/// Modeled directory entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KaniFsDirEntry {
    path: KaniFsPath,
}

impl KaniFsDirEntry {
    fn new(path: KaniFsPath) -> Self {
        Self { path }
    }

    /// Return the modeled full path.
    #[must_use]
    pub fn path(&self) -> KaniFsPath {
        self.path
    }

    /// Return the modeled entry name.
    #[must_use]
    pub fn file_name(&self) -> Option<KaniFsLabel> {
        self.path.file_name()
    }
}

/// Observable result of a recursive directory-creation law.
///
/// The assumption this observation stands in for -- that recursive
/// directory creation (`create_dir_all`/`DirBuilder::recursive(true)`)
/// reaches exactly the ancestors implied by joining path segments in
/// order, and nothing else about the real OS-backed directory tree -- is
/// named explicitly as a `Standard` rather than left as prose: the direct
/// `std::fs` path crosses OS-backed state Kani cannot symbolically execute
/// well (see this module's own doc comment), so this bounded observation is
/// what the `DirBuilder` proof actually rests on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Standard)]
#[standard(
    basis = "Self",
    basis_ctor = "Self::new(KaniFsPath::root(), KaniFsLabel::new('a'), KaniFsLabel::new('b'), KaniFsLabel::new('c'))"
)]
pub struct KaniRecursiveDirObservation {
    first_ancestor: KaniFsPath,
    second_ancestor: KaniFsPath,
    leaf: KaniFsPath,
}

impl Provenance for KaniRecursiveDirObservation {
    type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;

    fn metadata(&self) -> Self::MetadataIter {
        Box::new({
            vec![
            MetadataEntry::new(
                "assumed",
                "recursive directory creation reaches exactly the ancestors implied by repeated path-segment joins, standing in for the real OS-backed directory tree",
            ),
            MetadataEntry::new(
                "rationale",
                "the direct std::fs path crosses OS-backed state Kani cannot symbolically execute well today",
            ),
            MetadataEntry::new("first_ancestor", format!("{:?}", self.first_ancestor)),
            MetadataEntry::new("second_ancestor", format!("{:?}", self.second_ancestor)),
            MetadataEntry::new("leaf", format!("{:?}", self.leaf)),
        ]
        .into_iter()
        })
    }
}

impl KaniRecursiveDirObservation {
    /// Model a three-segment recursive directory creation.
    #[must_use]
    pub fn new(
        base: KaniFsPath,
        first: KaniFsLabel,
        second: KaniFsLabel,
        leaf: KaniFsLabel,
    ) -> Self {
        let first_ancestor = base.join(first);
        let second_ancestor = first_ancestor.join(second);
        let leaf = second_ancestor.join(leaf);

        Self {
            first_ancestor,
            second_ancestor,
            leaf,
        }
    }

    /// Return the first ancestor directory guaranteed by recursive creation.
    #[must_use]
    pub fn first_ancestor(&self) -> KaniFsPath {
        self.first_ancestor
    }

    /// Return the second ancestor directory guaranteed by recursive creation.
    #[must_use]
    pub fn second_ancestor(&self) -> KaniFsPath {
        self.second_ancestor
    }

    /// Return the leaf directory guaranteed by recursive creation.
    #[must_use]
    pub fn leaf(&self) -> KaniFsPath {
        self.leaf
    }
}

/// Observable result of reading one created entry from a directory.
///
/// The assumption this observation stands in for -- that a directory entry
/// yielded for a created file reports that file's own name and full path
/// exactly, and nothing else about the real OS-backed directory listing --
/// is named explicitly as a `Standard` rather than left as prose: the
/// direct `std::fs` path crosses OS-backed state Kani cannot symbolically
/// execute well (see this module's own doc comment), so this bounded
/// observation is what the `DirEntry` proof actually rests on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Standard)]
#[standard(
    basis = "Self",
    basis_ctor = "Self::new(KaniFsPath::root(), KaniFsLabel::new('f'))"
)]
pub struct KaniDirEntryObservation {
    entry: KaniFsDirEntry,
}

impl Provenance for KaniDirEntryObservation {
    type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;

    fn metadata(&self) -> Self::MetadataIter {
        Box::new({
            vec![
            MetadataEntry::new(
                "assumed",
                "a directory entry yielded for a created file reports that file's own name and full path exactly, standing in for the real OS-backed directory listing",
            ),
            MetadataEntry::new(
                "rationale",
                "the direct std::fs path crosses OS-backed state Kani cannot symbolically execute well today",
            ),
            MetadataEntry::new("entry_path", format!("{:?}", self.entry.path())),
        ]
        .into_iter()
        })
    }
}

impl KaniDirEntryObservation {
    /// Model a directory containing a single created file entry.
    #[must_use]
    pub fn new(dir: KaniFsPath, file_name: KaniFsLabel) -> Self {
        Self {
            entry: KaniFsDirEntry::new(dir.join(file_name)),
        }
    }

    /// Return the modeled directory entry.
    #[must_use]
    pub fn entry(&self) -> KaniFsDirEntry {
        self.entry
    }
}

/// Minimal owned filesystem state for Kani-facing proofs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KaniFileSystem {
    nodes: Vec<KaniFsNode>,
}

impl Default for KaniFileSystem {
    fn default() -> Self {
        Self {
            nodes: vec![KaniFsNode::directory(KaniFsPath::root())],
        }
    }
}

impl KaniFileSystem {
    /// Create an empty modeled filesystem with just the root directory.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Recursively create every missing ancestor directory and the leaf.
    pub fn create_dir_all(&mut self, path: &KaniFsPath) {
        for prefix in path.prefixes_without_root().into_iter().flatten() {
            if self.node_index(&prefix).is_none() {
                self.nodes.push(KaniFsNode::directory(prefix));
            }
        }
    }

    /// Create a file entry at the given path.
    pub fn create_file(&mut self, path: &KaniFsPath) {
        if let Some(parent) = path.parent() {
            assert!(
                self.is_dir(&parent),
                "create_file requires an existing parent directory in the model"
            );
        }

        match self.node_index(path) {
            Some(index) => self.nodes[index] = KaniFsNode::file(*path),
            None => self.nodes.push(KaniFsNode::file(*path)),
        }
    }

    /// Create a fresh file only when no node already exists at the path.
    ///
    /// # Errors
    ///
    /// Returns `Err(KaniAlreadyExists)` when the modeled path already names an
    /// existing node, mirroring `OpenOptions::create_new(true)`.
    pub fn create_new_file(&mut self, path: &KaniFsPath) -> Result<(), KaniAlreadyExists> {
        if self.node_index(path).is_some() {
            Err(KaniAlreadyExists)
        } else {
            self.create_file(path);
            Ok(())
        }
    }

    /// Return whether the path currently names a directory.
    #[must_use]
    pub fn is_dir(&self, path: &KaniFsPath) -> bool {
        self.node(path)
            .is_some_and(|node| node.kind == KaniFsNodeKind::Directory)
    }

    /// Return whether the path currently names a file.
    #[must_use]
    pub fn is_file(&self, path: &KaniFsPath) -> bool {
        self.node(path)
            .is_some_and(|node| node.kind == KaniFsNodeKind::File)
    }

    /// Return the immediate entries of a modeled directory.
    #[must_use]
    pub fn entries(&self, dir: &KaniFsPath) -> Vec<KaniFsDirEntry> {
        self.nodes
            .iter()
            .filter_map(|node| {
                if node.path == *dir {
                    None
                } else if node.path.parent() == Some(*dir) {
                    Some(KaniFsDirEntry::new(node.path))
                } else {
                    None
                }
            })
            .collect()
    }

    fn node(&self, path: &KaniFsPath) -> Option<&KaniFsNode> {
        self.node_index(path).map(|index| &self.nodes[index])
    }

    fn node_index(&self, path: &KaniFsPath) -> Option<usize> {
        self.nodes.iter().position(|node| node.path == *path)
    }
}

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
    #[must_use]
    pub fn file_is_file(&self) -> bool {
        self.file == KaniFsNodeKind::File
    }

    /// Report whether the modeled file node reports as a directory.
    #[must_use]
    pub fn file_is_dir(&self) -> bool {
        self.file == KaniFsNodeKind::Directory
    }

    /// Report whether the modeled directory node reports as a directory.
    #[must_use]
    pub fn directory_is_dir(&self) -> bool {
        self.directory == KaniFsNodeKind::Directory
    }

    /// Report whether the modeled directory node reports as a file.
    #[must_use]
    pub fn directory_is_file(&self) -> bool {
        self.directory == KaniFsNodeKind::File
    }
}

impl Default for KaniFileTypeObservation {
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
    #[must_use]
    pub fn len(&self) -> u64 {
        u64::from(self.len)
    }

    /// Report whether the modeled file is empty.
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
    #[must_use]
    pub fn modified(&self) -> u64 {
        self.modified_unix_seconds
    }
}

/// Observable result of `OpenOptions::create_new` against a path that may
/// already name a modeled filesystem node.
///
/// The assumption this observation stands in for -- that `create_new`'s
/// outcome is fully determined by a ternary prior existence state (missing,
/// an existing file, or an existing directory), and nothing else about the
/// real OS-backed path -- is named explicitly as a `Standard` rather than
/// left as prose: the general filesystem state machine times out under
/// Kani even for `create_new` alone (see
/// `gallery::filesystem_observation_granularity`), so this narrower
/// observation is what production proofs actually rest on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Standard)]
#[standard(basis = "Self")]
pub struct KaniCreateNewObservation {
    kind: Option<KaniFsNodeKind>,
}

impl Provenance for KaniCreateNewObservation {
    type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;

    fn metadata(&self) -> Self::MetadataIter {
        Box::new({
            let prior_kind = match self.kind {
                None => "missing",
                Some(KaniFsNodeKind::File) => "existing_file",
                Some(KaniFsNodeKind::Directory) => "existing_directory",
            };

            vec![
            MetadataEntry::new(
                "assumed",
                "create_new's outcome is determined entirely by a ternary prior existence state, standing in for the real OS-backed existence check",
            ),
            MetadataEntry::new(
                "rationale",
                "the general filesystem state machine times out under Kani even for create_new alone -- see gallery::filesystem_observation_granularity",
            ),
            MetadataEntry::new("prior_kind", prior_kind),
        ]
        .into_iter()
        })
    }
}

/// Modeled error for `create_new` against a path that already has a file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KaniAlreadyExists;

impl KaniCreateNewObservation {
    /// Model a genuinely fresh path.
    #[must_use]
    pub fn missing() -> Self {
        Self { kind: None }
    }

    /// Model a path that already names a file.
    #[must_use]
    pub fn existing_file() -> Self {
        Self {
            kind: Some(KaniFsNodeKind::File),
        }
    }

    /// Model a path that already names a directory.
    #[must_use]
    pub fn existing_directory() -> Self {
        Self {
            kind: Some(KaniFsNodeKind::Directory),
        }
    }

    /// Model attempting `create_new` against the modeled path.
    ///
    /// # Errors
    ///
    /// Returns `Err(KaniAlreadyExists)` when the modeled path already names an
    /// existing node, mirroring `ErrorKind::AlreadyExists`.
    pub fn create_new(&mut self) -> Result<(), KaniAlreadyExists> {
        if self.kind.is_some() {
            Err(KaniAlreadyExists)
        } else {
            self.kind = Some(KaniFsNodeKind::File);
            Ok(())
        }
    }

    /// Report whether the modeled path now names a file.
    #[must_use]
    pub fn is_file(&self) -> bool {
        self.kind == Some(KaniFsNodeKind::File)
    }
}

/// Observable result of flipping a file's readonly permission bit and
/// reading it back.
///
/// The assumption this observation stands in for -- that flipping
/// `.set_readonly()` and applying it via `fs::set_permissions` is
/// reflected the next time the file's permissions are read, and nothing
/// else about the real OS-backed permission bit -- is named explicitly as
/// a `Standard` rather than left as prose: the direct `std::fs` path
/// crosses OS-backed state Kani cannot symbolically execute well (see this
/// module's own doc comment), so this bounded observation is what the
/// `Permissions` proof actually rests on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Standard)]
#[standard(basis = "Self")]
pub struct KaniPermissionsObservation {
    readonly: bool,
}

impl Provenance for KaniPermissionsObservation {
    type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;

    fn metadata(&self) -> Self::MetadataIter {
        Box::new({
            vec![
            MetadataEntry::new(
                "assumed",
                "flipping .set_readonly() and applying it via fs::set_permissions is reflected the next time the file's permissions are read, standing in for the real OS-backed permission bit",
            ),
            MetadataEntry::new(
                "rationale",
                "the direct std::fs path crosses OS-backed state Kani cannot symbolically execute well today",
            ),
            MetadataEntry::new("readonly", self.readonly.to_string()),
        ]
        .into_iter()
        })
    }
}

impl KaniPermissionsObservation {
    /// Model a freshly created file's permissions (never readonly).
    #[must_use]
    pub fn new() -> Self {
        Self { readonly: false }
    }

    /// Report the modeled readonly bit.
    #[must_use]
    pub fn readonly(&self) -> bool {
        self.readonly
    }

    /// Model `.set_readonly()` followed by applying the change.
    pub fn set_readonly(&mut self, readonly: bool) {
        self.readonly = readonly;
    }
}

impl Default for KaniPermissionsObservation {
    fn default() -> Self {
        Self::new()
    }
}

/// Observable result of reading every entry a directory with two created
/// files contains.
///
/// The assumption this observation stands in for -- that `.read_dir()`
/// yields exactly the files that were created in that directory, no more
/// and no fewer, and nothing else about the real OS-backed directory
/// listing -- is named explicitly as a `Standard` rather than left as
/// prose: the direct `std::fs` path crosses OS-backed state Kani cannot
/// symbolically execute well (see this module's own doc comment), so this
/// bounded observation is what the `ReadDir` proof actually rests on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Standard)]
#[standard(
    basis = "Self",
    basis_ctor = "Self::new(KaniFsPath::root(), KaniFsLabel::new('1'), KaniFsLabel::new('2'))"
)]
pub struct KaniReadDirObservation {
    first: KaniFsDirEntry,
    second: KaniFsDirEntry,
}

impl Provenance for KaniReadDirObservation {
    type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;

    fn metadata(&self) -> Self::MetadataIter {
        Box::new({
            vec![
            MetadataEntry::new(
                "assumed",
                ".read_dir() yields exactly the files that were created in that directory, no more and no fewer, standing in for the real OS-backed directory listing",
            ),
            MetadataEntry::new(
                "rationale",
                "the direct std::fs path crosses OS-backed state Kani cannot symbolically execute well today",
            ),
            MetadataEntry::new("first", format!("{:?}", self.first.path())),
            MetadataEntry::new("second", format!("{:?}", self.second.path())),
        ]
        .into_iter()
        })
    }
}

impl KaniReadDirObservation {
    /// Model a directory containing exactly two created file entries.
    #[must_use]
    pub fn new(dir: KaniFsPath, first_name: KaniFsLabel, second_name: KaniFsLabel) -> Self {
        Self {
            first: KaniFsDirEntry::new(dir.join(first_name)),
            second: KaniFsDirEntry::new(dir.join(second_name)),
        }
    }

    /// Return the modeled entries in the order they were created.
    #[must_use]
    pub fn entries(&self) -> [KaniFsDirEntry; 2] {
        [self.first, self.second]
    }
}

/// Observable result of a second handle's `.try_lock()` while a first
/// handle still holds the modeled file lock.
///
/// The assumption this observation stands in for -- that a second handle's
/// `.try_lock()` fails while a first handle still holds the file lock, and
/// nothing else about the real OS-backed advisory lock -- is named
/// explicitly as a `Standard` rather than left as prose: the direct
/// `std::fs` path crosses OS-backed state Kani cannot symbolically execute
/// well (see this module's own doc comment), so this bounded observation is
/// what the `TryLockError` proof actually rests on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Standard)]
#[standard(basis = "Self")]
pub struct KaniLockObservation {
    locked: bool,
}

impl Provenance for KaniLockObservation {
    type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;

    fn metadata(&self) -> Self::MetadataIter {
        Box::new({
            vec![
            MetadataEntry::new(
                "assumed",
                "a second handle's .try_lock() fails while a first handle still holds the file lock, standing in for the real OS-backed advisory lock",
            ),
            MetadataEntry::new(
                "rationale",
                "the direct std::fs path crosses OS-backed state Kani cannot symbolically execute well today",
            ),
            MetadataEntry::new("locked", self.locked.to_string()),
        ]
        .into_iter()
        })
    }
}

/// Modeled error for a second `try_lock` while the modeled lock is held.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KaniAlreadyLocked;

impl KaniLockObservation {
    /// Model a fresh, unlocked file.
    #[must_use]
    pub fn new() -> Self {
        Self { locked: false }
    }

    /// Model a handle attempting to acquire the modeled lock.
    ///
    /// # Errors
    ///
    /// Returns `Err(KaniAlreadyLocked)` when the modeled file is already
    /// locked, mirroring `TryLockError::WouldBlock`.
    pub fn try_lock(&mut self) -> Result<(), KaniAlreadyLocked> {
        if self.locked {
            Err(KaniAlreadyLocked)
        } else {
            self.locked = true;
            Ok(())
        }
    }
}

impl Default for KaniLockObservation {
    fn default() -> Self {
        Self::new()
    }
}
