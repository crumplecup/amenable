//! Kani-only accommodation model for `std::fs`.
//!
//! The real `std::fs` path crosses OS-backed filesystem state that Kani cannot
//! symbolically execute well today. This model captures the bounded observable
//! laws the production proofs actually claim, starting with recursive
//! directory creation and directory entry reporting.

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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KaniRecursiveDirObservation {
    first_ancestor: KaniFsPath,
    second_ancestor: KaniFsPath,
    leaf: KaniFsPath,
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KaniDirEntryObservation {
    entry: KaniFsDirEntry,
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

    /// Return whether the path currently names a directory.
    #[must_use]
    pub fn is_dir(&self, path: &KaniFsPath) -> bool {
        self.node(path)
            .is_some_and(|node| node.kind == KaniFsNodeKind::Directory)
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
