//! Provenance metadata for constitutional proof claims.

/// One provenance metadata fact expressed as a key-value pair.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MetadataEntry {
    /// Stable metadata key.
    key: &'static str,
    /// Stable metadata value.
    value: &'static str,
}

impl MetadataEntry {
    /// Create a new provenance metadata entry.
    pub const fn new(key: &'static str, value: &'static str) -> Self {
        Self { key, value }
    }

    /// Return the metadata key.
    pub const fn key(&self) -> &'static str {
        self.key
    }

    /// Return the metadata value.
    pub const fn value(&self) -> &'static str {
        self.value
    }
}

/// Structured provenance describing how a claim is sourced and audited.
pub trait Provenance {
    /// Return the provenance metadata describing this claim's source of trust.
    fn metadata() -> Vec<MetadataEntry>;
}
