//! Provenance metadata for constitutional proof claims.

use std::fmt::{self, Display, Formatter};

use crate::Registry;

/// One provenance metadata fact expressed as a key-value pair.
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, derive_getters::Getters, derive_new::new,
)]
pub struct MetadataEntry {
    /// Stable metadata key.
    #[new(into)]
    key: String,
    /// Stable metadata value.
    #[new(into)]
    value: String,
}

impl Display for MetadataEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.key, self.value)
    }
}

/// Borrowed human-readable rendering of provenance metadata.
#[derive(Debug, Clone, Copy)]
pub struct ProvenanceReport<'a, P>
where
    P: Provenance + ?Sized,
{
    provenance: &'a P,
}

impl<'a, P> ProvenanceReport<'a, P>
where
    P: Provenance + ?Sized,
{
    /// Create a borrowed report view over a provenance record.
    pub const fn new(provenance: &'a P) -> Self {
        Self { provenance }
    }
}

impl<P> Display for ProvenanceReport<'_, P>
where
    P: Provenance + ?Sized,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut entries = self.provenance.metadata();

        match entries.next() {
            Some(entry) => {
                write!(f, "{entry}")?;

                for entry in entries {
                    write!(f, "\n{entry}")?;
                }

                Ok(())
            }
            None => write!(f, "(no provenance metadata)"),
        }
    }
}

/// Owned human-readable rendering of provenance metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnedProvenanceReport<P>
where
    P: Provenance,
{
    provenance: P,
}

impl<P> OwnedProvenanceReport<P>
where
    P: Provenance,
{
    /// Create an owned report view over a provenance record.
    pub const fn new(provenance: P) -> Self {
        Self { provenance }
    }
}

impl<P> Display for OwnedProvenanceReport<P>
where
    P: Provenance,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&ProvenanceReport::new(&self.provenance), f)
    }
}

/// Structured provenance describing how a root obligation is sourced and
/// audited.
///
/// The concrete schema lives on the implementing Rust type itself, typically
/// a user-defined struct or enum. This trait is the common reporting view
/// over that structured object: it projects the implementor into a
/// deterministic stream of metadata entries without forcing every provenance
/// carrier into one storage representation.
pub trait Provenance {
    /// Concrete iterator type backing [`metadata`](Provenance::metadata).
    ///
    /// An associated type rather than a return-position `impl Trait`: the
    /// latter desugars to a compiler-synthesized opaque type at every impl
    /// site, and at least one verifier toolchain (Creusot's `creusot-rustc`,
    /// as of the `0.11.0` release pinned in `amenable_creusot`) panics
    /// enumerating local def-ids when a translated crate contains one —
    /// confirmed via a real ICE on `CreusotVerifierMetadata`'s own impl, not
    /// a defensive guess. An associated type is an ordinary named item, not
    /// an opaque one, and sidesteps the bug entirely while still letting
    /// each implementor pick its own concrete iterator rather than being
    /// forced into one storage representation (e.g. `Vec<MetadataEntry>`)
    /// for every carrier.
    type MetadataIter: Iterator<Item = MetadataEntry>;

    /// Iterate over the provenance facts describing this claim's source of
    /// trust, generated on demand rather than pre-built into a stored
    /// collection.
    fn metadata(&self) -> Self::MetadataIter;

    /// Backward-compatible alias for the projected metadata stream.
    fn iter(&self) -> impl Iterator<Item = MetadataEntry> {
        self.metadata()
    }

    /// Look up the fact for a given key, if present.
    fn get(&self, key: &str) -> Option<MetadataEntry> {
        self.metadata().find(|entry| entry.key() == key)
    }

    /// Return whether a fact with the given key is present.
    fn contains_key(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    /// Number of provenance facts.
    fn len(&self) -> usize {
        self.metadata().count()
    }

    /// Whether there are no provenance facts.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Iterate over every metadata key.
    fn keys(&self) -> impl Iterator<Item = String> {
        self.metadata().map(|entry| entry.key().to_owned())
    }

    /// Iterate over every metadata value.
    fn values(&self) -> impl Iterator<Item = String> {
        self.metadata().map(|entry| entry.value().to_owned())
    }

    /// Borrow a human-readable rendering of the projected provenance facts.
    fn report(&self) -> ProvenanceReport<'_, Self>
    where
        Self: Sized,
    {
        ProvenanceReport::new(self)
    }

    /// Issue a tracked provenance certificate through the registry.
    fn certification<R>(&self, registry: &mut R, subject: impl Display) -> R::Certificate
    where
        Self: Sized,
        R: Registry,
    {
        registry.issue_provenance_certificate(subject, self)
    }
}

macro_rules! impl_scalar_provenance {
    ($($ty:ty),* $(,)?) => {
        $(
            impl Provenance for $ty {
                type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;

                fn metadata(&self) -> Self::MetadataIter {
                    Box::new(vec![MetadataEntry::new("value", self.to_string())].into_iter())
                }
            }
        )*
    };
}

impl_scalar_provenance!(
    bool, char, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64, String,
);
