//! Kani-only accommodation model for PATH-style helper semantics.
//!
//! This module is where Amenable stops asking Kani to reason directly about
//! `std::env::join_paths()` / `split_paths()` and instead proves against a
//! small package of explicit separator and error laws over a bounded subset of
//! path-like strings.
//!
//! The direct std helper path remains preserved in the proof gallery as a
//! false trail. Production proofs that use this model are therefore
//! conditional:
//!
//! - if the real PATH-style helpers refine these modeled laws on the bounded
//!   subset represented here,
//! - then the modeled Kani proof carries the intended Rust-facing claim.

/// One modeled PATH-style segment inside the bounded amenable subset.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KaniEnvPath(String);

/// One modeled PATH-style path list.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KaniEnvPathList(Vec<KaniEnvPath>);

/// Modeled error for a path that cannot be joined in the bounded subset.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, derive_getters::Getters)]
pub struct KaniJoinPathsError {
    /// The path that violated the bounded join law.
    offending_path: String,
}

/// Semantic joined PATH-style payload for Kani-facing round-trip proofs.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KaniJoinedEnvPaths(KaniEnvPathList);

/// Namespace for modeled PATH-style helper laws.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KaniEnvPaths;

impl KaniEnvPath {
    /// Classify one raw path string as joinable or not in the bounded model.
    pub fn new(path: String) -> Result<Self, KaniJoinPathsError> {
        if KaniEnvPaths::is_joinable(&path) {
            Ok(Self(path))
        } else {
            Err(KaniJoinPathsError {
                offending_path: path,
            })
        }
    }

    /// Borrow the modeled path string.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume the modeled path string.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl KaniEnvPathList {
    /// Construct one modeled path list from already-classified path entries.
    pub fn new(paths: Vec<KaniEnvPath>) -> Self {
        Self(paths)
    }

    /// Construct one modeled path list from raw path strings.
    pub fn from_strings(paths: Vec<String>) -> Result<Self, KaniJoinPathsError> {
        let mut modeled = Vec::with_capacity(paths.len());
        for path in paths {
            modeled.push(KaniEnvPath::new(path)?);
        }
        Ok(Self::new(modeled))
    }

    /// Report how many modeled paths are present.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Report whether the modeled path list is empty.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Borrow the modeled path entries.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    pub fn paths(&self) -> &[KaniEnvPath] {
        &self.0
    }

    /// Materialize the modeled path list back into owned strings.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    pub fn as_strings(&self) -> Vec<String> {
        self.0.iter().map(|path| path.as_str().to_owned()).collect()
    }
}

impl KaniJoinPathsError {
    /// Consume the error and recover the offending path.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    pub fn into_offending_path(self) -> String {
        self.offending_path
    }
}

impl KaniJoinedEnvPaths {
    /// Recover the modeled path list carried by the semantic joined payload.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    pub fn into_paths(self) -> KaniEnvPathList {
        self.0
    }
}

impl KaniEnvPaths {
    /// Return the modeled platform separator.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
    pub fn separator() -> char {
        if cfg!(windows) { ';' } else { ':' }
    }

    /// Report whether the raw path stays inside the bounded modeled subset.
    pub fn is_joinable(path: &str) -> bool {
        !path.contains(Self::separator()) && !(cfg!(windows) && path.contains('"'))
    }

    /// Join one modeled path list with the modeled platform separator.
    pub fn join(paths: &KaniEnvPathList) -> String {
        let mut joined = String::new();

        for (index, path) in paths.paths().iter().enumerate() {
            if index > 0 {
                joined.push(Self::separator());
            }
            joined.push_str(path.as_str());
        }

        joined
    }

    /// Split one modeled joined string back into a modeled path list.
    pub fn split(joined: &str) -> KaniEnvPathList {
        if joined.is_empty() {
            return KaniEnvPathList::new(vec![KaniEnvPath(String::new())]);
        }

        let mut paths = Vec::new();
        let mut current = String::new();

        for ch in joined.chars() {
            if ch == Self::separator() {
                paths.push(KaniEnvPath(current));
                current = String::new();
            } else {
                current.push(ch);
            }
        }

        paths.push(KaniEnvPath(current));
        KaniEnvPathList::new(paths)
    }

    /// Join one modeled path list at the semantic boundary used by Kani proofs.
    pub fn join_semantic(paths: KaniEnvPathList) -> KaniJoinedEnvPaths {
        KaniJoinedEnvPaths(paths)
    }

    /// Split one semantic joined payload back into its path list.
    pub fn split_semantic(joined: KaniJoinedEnvPaths) -> KaniEnvPathList {
        joined.into_paths()
    }
}
