//! The frozen key/value metadata snapshot a [`Certificate`](crate::Certificate)
//! stores.
//!
//! Split out of `provenance.rs` so it can be `#[path]`-included into
//! `amenable_verus` on its own: the raw `verus` compiler needs `MetadataEntry`
//! (for `cert.rs`'s certificate surface) but must not see the live [`Metadata`]
//! (crate::Metadata) trait layer, which now lives in `metadata.rs` and uses
//! `Arc<dyn ...>` / `Any`.

use std::fmt::{self, Display, Formatter};

/// One metadata fact frozen as a rendered key/value pair.
///
/// This is the audit snapshot, not the live interface: a `Certificate` stores
/// `Vec<MetadataEntry>` because it wants cheap comparison and hashing and never
/// needs the structured value back. Live, typed metadata is `Metadata` /
/// `OwnedEntry` (in `metadata.rs`, not visible from this `#[path]`-shared
/// file); freeze one into a `MetadataEntry` with the `From<&E>` bridge.
///
/// No intra-doc links in this file: it is `#[path]`-included into
/// `amenable_verus` too, and a link that resolves in `amenable_core` need not
/// resolve there.
///
/// Hand-written `new`/`key`/`value` instead of `derive_new`/`derive_getters`,
/// despite otherwise matching that exact shape: `amenable_verus`
/// `#[path]`-includes this file directly into its own crate, and the real
/// `verus` binary is invoked as a bare compiler over a single file tree
/// (`verus --crate-type=lib crates/amenable_verus/src/lib.rs`) that never reads
/// `Cargo.toml` at all -- so a dependency declared there (even a real one, as
/// confirmed by `cargo check --all-features` passing) is still invisible to the
/// real verifier. Confirmed the hard way, twice: a version of this file gained
/// these derives, `cargo check --all-features` passed clean (only
/// `amenable_verus`'s own `Cargo.toml` dependency, checked via ordinary
/// `cargo`, which does read `Cargo.toml`), and the real `verus` binary still
/// failed with "cannot find crate `derive_getters`" -- the two toolchains
/// disagree about what's resolvable here, and only the real `verus` invocation
/// is authoritative for this file.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MetadataEntry {
    /// Stable metadata key.
    key: String,
    /// Stable metadata value.
    value: String,
}

impl MetadataEntry {
    /// Create a new metadata entry.
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }

    /// Return the metadata key.
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Return the metadata value.
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl Display for MetadataEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.key, self.value)
    }
}
