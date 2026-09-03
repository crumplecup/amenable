use amenable_core::{MetadataEntry, Provenance};
use amenable_derive::Standard;

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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
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
/// Not `PartialEq`/`Eq`/`Hash`/`PartialOrd`/`Ord`: location tracking
/// makes comparison confusing (this workspace's own error-type
/// exception, `CLAUDE.md`), and not `Copy`: owned `file` is a `String`.
#[derive(Debug, Clone, derive_more::Display, derive_more::Error, derive_getters::Getters)]
#[display("the modeled lock is already held")]
pub struct KaniAlreadyLocked {
    /// Source line of the call site that produced this error.
    #[getter(copy)]
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl KaniAlreadyLocked {
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

impl Default for KaniAlreadyLocked {
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
    fn default() -> Self {
        Self::new()
    }
}

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
    #[cfg_attr(
        not(kani),
        tracing::instrument(level = "debug", skip(self), err(level = "warn"))
    )]
    pub fn try_lock(&mut self) -> Result<(), KaniAlreadyLocked> {
        if self.locked {
            Err(KaniAlreadyLocked::new())
        } else {
            self.locked = true;
            Ok(())
        }
    }
}

impl Default for KaniLockObservation {
    #[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
    fn default() -> Self {
        Self::new()
    }
}
