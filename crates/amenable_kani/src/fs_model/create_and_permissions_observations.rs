use amenable_core::{MetadataEntry, Provenance};
use amenable_derive::Standard;

use super::tree_primitives::{KaniFsDirEntry, KaniFsLabel, KaniFsNodeKind, KaniFsPath};

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
/// Not `PartialEq`/`Eq`/`Hash`/`PartialOrd`/`Ord`: location tracking
/// makes comparison confusing (this workspace's own error-type
/// exception, `CLAUDE.md`), and not `Copy`: owned `file` is a `String`.
#[derive(Debug, Clone, derive_more::Display, derive_more::Error, derive_getters::Getters)]
#[display("the modeled path already exists")]
pub struct KaniAlreadyExists {
    /// Source line of the call site that produced this error.
    #[getter(copy)]
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl KaniAlreadyExists {
    /// Construct the error, recording the caller's location.
    ///
    /// `Location::caller()` is itself an unsupported construct under
    /// Kani (confirmed via a real `cargo kani` run: every harness
    /// reaching this constructor failed on it directly), the same wall
    /// `panic_model::KaniCallerLocationObservation`'s own doc comment
    /// already documents. Real location tracking only has genuine value
    /// on the ordinary-`cargo test` path anyway (`fs_model_test.rs`'s
    /// `.into_diagnostic()?`, never Kani-executed) -- a Kani-reachable
    /// panic is its own failure signal regardless of what file/line this
    /// carries, the same reasoning `kani_reach` already applies on the
    /// cordial side to a Kani harness's own panic site.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
    #[must_use]
    pub fn new() -> Self {
        #[cfg(kani)]
        let (line, file) = (0, String::new());
        #[cfg(not(kani))]
        let (line, file) = {
            let loc = std::panic::Location::caller();
            (loc.line(), loc.file().to_string())
        };
        Self { line, file }
    }
}

impl Default for KaniAlreadyExists {
    #[track_caller]
    fn default() -> Self {
        Self::new()
    }
}

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
    #[cfg_attr(
        not(kani),
        tracing::instrument(level = "debug", skip(self), err(level = "warn"))
    )]
    pub fn create_new(&mut self) -> Result<(), KaniAlreadyExists> {
        if self.kind.is_some() {
            Err(KaniAlreadyExists::new())
        } else {
            self.kind = Some(KaniFsNodeKind::File);
            Ok(())
        }
    }

    /// Report whether the modeled path now names a file.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
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
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Standard, derive_getters::Getters, derive_setters::Setters,
)]
#[standard(basis = "Self")]
#[setters(prefix = "with_")]
pub struct KaniPermissionsObservation {
    /// The modeled readonly bit.
    #[getter(copy)]
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
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn entries(&self) -> [KaniFsDirEntry; 2] {
        [self.first, self.second]
    }
}
