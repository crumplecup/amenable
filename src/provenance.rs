//! Provenance metadata for constitutional proof claims.

/// One provenance metadata fact expressed as a key-value pair.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MetadataEntry {
    /// Stable metadata key.
    key: String,
    /// Stable metadata value.
    value: String,
}

impl MetadataEntry {
    /// Create a new provenance metadata entry.
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

/// Structured provenance describing how a claim is sourced and audited.
///
/// This trait describes a capability, not a storage structure. Implementors
/// own their data however they like — a `Vec`, a `const` slice walked
/// lazily, a `HashMap` — and expose it only through [`Provenance::iter`].
/// Every other method is a default derived from it, so implementors never
/// need to commit to a collection type the trait itself does not require.
pub trait Provenance {
    /// Iterate over the provenance facts describing this claim's source of
    /// trust, generated on demand rather than pre-built into a stored
    /// collection.
    fn iter(&self) -> impl Iterator<Item = MetadataEntry>;

    /// Look up the fact for a given key, if present.
    fn get(&self, key: &str) -> Option<MetadataEntry> {
        self.iter().find(|entry| entry.key() == key)
    }

    /// Return whether a fact with the given key is present.
    fn contains_key(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    /// Number of provenance facts.
    fn len(&self) -> usize {
        self.iter().count()
    }

    /// Whether there are no provenance facts.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
