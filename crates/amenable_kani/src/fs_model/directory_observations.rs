use amenable_core::{MetadataEntry, Provenance};
use amenable_derive::Standard;

use super::create_and_permissions_observations::KaniAlreadyExists;
use super::tree_primitives::{KaniFsDirEntry, KaniFsLabel, KaniFsNode, KaniFsNodeKind, KaniFsPath};

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Standard, derive_getters::Getters)]
#[standard(
    basis = "Self",
    basis_ctor = "Self::new(KaniFsPath::root(), KaniFsLabel::new('a'), KaniFsLabel::new('b'), KaniFsLabel::new('c'))"
)]
pub struct KaniRecursiveDirObservation {
    /// The first ancestor directory guaranteed by recursive creation.
    #[getter(copy)]
    first_ancestor: KaniFsPath,
    /// The second ancestor directory guaranteed by recursive creation.
    #[getter(copy)]
    second_ancestor: KaniFsPath,
    /// The leaf directory guaranteed by recursive creation.
    #[getter(copy)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Standard, derive_getters::Getters)]
#[standard(
    basis = "Self",
    basis_ctor = "Self::new(KaniFsPath::root(), KaniFsLabel::new('f'))"
)]
pub struct KaniDirEntryObservation {
    /// The modeled directory entry.
    #[getter(copy)]
    entry: KaniFsDirEntry,
}

impl Provenance for KaniDirEntryObservation {
    type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
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
}

/// Minimal owned filesystem state for Kani-facing proofs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KaniFileSystem {
    nodes: Vec<KaniFsNode>,
}

impl Default for KaniFileSystem {
    #[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
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
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self, path)))]
    pub fn create_dir_all(&mut self, path: &KaniFsPath) {
        for prefix in path.prefixes_without_root().into_iter().flatten() {
            if self.node_index(&prefix).is_none() {
                self.nodes.push(KaniFsNode::directory(prefix));
            }
        }
    }

    /// Create a file entry at the given path.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self, path)))]
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
    #[cfg_attr(
        not(kani),
        tracing::instrument(level = "debug", skip(self, path), err(level = "warn"))
    )]
    pub fn create_new_file(&mut self, path: &KaniFsPath) -> Result<(), KaniAlreadyExists> {
        if self.node_index(path).is_some() {
            Err(KaniAlreadyExists::new())
        } else {
            self.create_file(path);
            Ok(())
        }
    }

    /// Return whether the path currently names a directory.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, path)))]
    #[must_use]
    pub fn is_dir(&self, path: &KaniFsPath) -> bool {
        self.node(path)
            .is_some_and(|node| node.kind() == KaniFsNodeKind::Directory)
    }

    /// Return whether the path currently names a file.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, path)))]
    #[must_use]
    pub fn is_file(&self, path: &KaniFsPath) -> bool {
        self.node(path)
            .is_some_and(|node| node.kind() == KaniFsNodeKind::File)
    }

    /// Return the immediate entries of a modeled directory.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self, dir)))]
    #[must_use]
    pub fn entries(&self, dir: &KaniFsPath) -> Vec<KaniFsDirEntry> {
        self.nodes
            .iter()
            .filter_map(|node| {
                if node.path() == *dir {
                    None
                } else if node.path().parent() == Some(*dir) {
                    Some(KaniFsDirEntry::new(node.path()))
                } else {
                    None
                }
            })
            .collect()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, path)))]
    fn node(&self, path: &KaniFsPath) -> Option<&KaniFsNode> {
        self.node_index(path).map(|index| &self.nodes[index])
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, path)))]
    fn node_index(&self, path: &KaniFsPath) -> Option<usize> {
        self.nodes.iter().position(|node| node.path() == *path)
    }
}
