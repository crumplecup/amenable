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
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self, segment)))]
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
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn file_name(&self) -> Option<KaniFsLabel> {
        if self.depth == 0 {
            None
        } else {
            self.segments[self.depth - 1]
        }
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    pub(super) fn parent(&self) -> Option<Self> {
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

    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    pub(super) fn prefixes_without_root(&self) -> [Option<Self>; KANI_FS_MAX_DEPTH] {
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
pub(super) struct KaniFsNode {
    pub(super) path: KaniFsPath,
    pub(super) kind: KaniFsNodeKind,
}

impl KaniFsNode {
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(path)))]
    pub(super) fn directory(path: KaniFsPath) -> Self {
        Self {
            path,
            kind: KaniFsNodeKind::Directory,
        }
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(path)))]
    pub(super) fn file(path: KaniFsPath) -> Self {
        Self {
            path,
            kind: KaniFsNodeKind::File,
        }
    }
}

/// Modeled directory entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, derive_getters::Getters, derive_new::new)]
#[new(visibility = "pub(super)")]
pub struct KaniFsDirEntry {
    /// The modeled full path.
    #[getter(copy)]
    path: KaniFsPath,
}

impl KaniFsDirEntry {
    /// Return the modeled entry name.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn file_name(&self) -> Option<KaniFsLabel> {
        self.path.file_name()
    }
}
