//! Typed, queryable metadata: a flat key namespace over structured values.
//!
//! This module is the mechanism layer beneath [`Provenance`](crate::Provenance).
//! [`Metadata`] is a bag of typed facts making no claim about meaning;
//! [`ErasedEntry`] is the object-safe view of one fact, the element type every
//! `Metadata` record yields; [`OwnedEntry`] is the universal carrier, holding
//! its value behind an [`Arc`] so an entry is `Clone` while the concrete value
//! type stays reachable for downcasting. [`Entry`] is the typed authoring
//! contract for a single canonically-keyed fact — its implementors live in
//! `provenance_vocab.rs`.
//!
//! Deliberately *not* `#[path]`-included into `amenable_verus`: it uses
//! `Arc<dyn ...>` and [`Any`], which the raw `verus` compiler has no need to
//! see. `MetadataEntry` (the frozen snapshot) lives in `metadata_entry.rs`,
//! which *is* included, for that reason.

use std::{
    any::Any,
    fmt::{self, Debug, Display, Formatter},
    sync::Arc,
};

use crate::MetadataEntry;

/// A value carried by a metadata entry.
///
/// Blanket-implemented for every owned `Display + Debug + Send + Sync +
/// 'static` type — never implemented by hand. The two accessors exist instead
/// of `Display` / `Any` supertraits so that `&dyn MetadataValue` projects to
/// `&dyn Display` / `&dyn Any` without trait upcasting, which is not stable on
/// every verifier backend's pinned toolchain.
pub trait MetadataValue: Debug + Send + Sync + 'static {
    /// Borrow this value as a [`Display`] trait object.
    fn as_display(&self) -> &dyn Display;

    /// Borrow this value as an [`Any`] trait object, for downcasting.
    fn as_any(&self) -> &dyn Any;
}

/// A single, canonically-keyed metadata fact — the unit a provenance
/// vocabulary is built from.
///
/// `KEY` is fixed per type: every [`Authority`](crate::Authority) keys
/// `"authority"`, every [`SourceUrl`](crate::SourceUrl) keys `"source_url"`,
/// so a vocabulary shared across provenance records (and across the three
/// verifier backends) cannot drift. Vocabulary types are also [`Metadata`]
/// (a one-entry record holding the value by clone) and [`MetadataValue`], so
/// an `#[entry(flatten)]` field of a vocabulary type projects to its one
/// canonically-keyed entry and `record.get_as::<Authority>(Authority::KEY)`
/// recovers the typed value.
pub trait Entry {
    /// The canonical key shared by every value of this type.
    const KEY: &'static str;

    /// The value this entry carries.
    type Value: ?Sized;

    /// Borrow the value.
    fn value(&self) -> &Self::Value;

    /// The canonical key ([`KEY`](Entry::KEY)).
    fn key(&self) -> &'static str {
        Self::KEY
    }

    /// Freeze this vocabulary value into its one [`OwnedEntry`].
    fn into_entry(self) -> OwnedEntry
    where
        Self: Sized + MetadataValue,
    {
        OwnedEntry::new(Self::KEY, self)
    }
}

/// Object-safe view of one metadata fact: the element type every [`Metadata`]
/// record yields.
///
/// [`OwnedEntry`] implements it directly.
pub trait ErasedEntry: Debug {
    /// Fully-qualified key, e.g. `"motor.max_rpm"` after nesting.
    fn key(&self) -> &str;

    /// Renderable view of the value.
    fn value(&self) -> &dyn Display;

    /// Structured view of the value: `entry.value_any().downcast_ref::<Rpm>()`
    /// returns the real value, not a stringification. Survives nesting.
    fn value_any(&self) -> &dyn Any;
}

/// A queryable metadata record: a flat namespace of fully-qualified keys over
/// structured, owned values.
///
/// The one required method is [`snapshot`](Metadata::snapshot); every other
/// method is built from it. All returns are owned — this is what lets a
/// zero-sized type implement `Metadata` (it builds a record on call rather than
/// storing one) and what lets [`Provenance`](crate::Provenance) be a supertrait
/// rather than an associated type.
///
/// Not a lazy stream: `snapshot` allocates a fresh `Vec` per call. That is
/// deliberate — a borrowing iterator would force a GAT or a return-position
/// `impl Trait` at impl sites, and at least one verifier toolchain panics on
/// the latter (see `provenance.rs`). Metadata is an audit / pre-flight surface,
/// not a hot path.
pub trait Metadata {
    /// Every entry, fully-qualified keys, in deterministic source order.
    fn snapshot(&self) -> Vec<OwnedEntry>;

    /// Look up one entry by fully-qualified key.
    ///
    /// The default scans a fresh [`snapshot`](Metadata::snapshot); keys are
    /// already fully qualified, so this is correct at any nesting depth. A
    /// map-backed implementation can override it.
    fn get(&self, key: &str) -> Option<OwnedEntry> {
        self.snapshot().into_iter().find(|entry| entry.key() == key)
    }

    /// Look up one value by key and downcast it to `T`, cloning it out.
    fn get_as<T: Any + Clone>(&self, key: &str) -> Option<T> {
        self.get(key)
            .and_then(|entry| entry.value_any().downcast_ref::<T>().cloned())
    }

    /// Whether an entry with the given key is present.
    fn contains_key(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    /// Every fully-qualified key, in snapshot order.
    fn keys(&self) -> Vec<String> {
        self.snapshot()
            .iter()
            .map(|entry| entry.key().to_owned())
            .collect()
    }

    /// Every value, rendered to a string, in snapshot order.
    fn values(&self) -> Vec<String> {
        self.snapshot()
            .iter()
            .map(|entry| entry.value().to_string())
            .collect()
    }

    /// Number of entries.
    fn len(&self) -> usize {
        self.snapshot().len()
    }

    /// Whether the record has no entries.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Borrow a human-readable rendering of the entries.
    fn report(&self) -> MetadataReport<'_, Self>
    where
        Self: Sized,
    {
        MetadataReport::new(self)
    }
}

impl<T: Display + Debug + Send + Sync + 'static> MetadataValue for T {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn as_display(&self) -> &dyn Display {
        self
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// The universal metadata carrier: the one entry type every [`Metadata`] record
/// yields.
///
/// Owns its key. Holds its value behind [`Arc`] so the entry is `Clone` (needed
/// by [`Metadata::snapshot`] and by hand-built records) while the concrete
/// value type stays reachable through [`value_any`](ErasedEntry::value_any).
#[derive(Debug, Clone)]
pub struct OwnedEntry {
    key: String,
    value: Arc<dyn MetadataValue>,
}

impl OwnedEntry {
    /// Build an entry from a key and any owned [`MetadataValue`].
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(key, value)))]
    pub fn new(key: impl Into<String>, value: impl MetadataValue) -> Self {
        Self {
            key: key.into(),
            value: Arc::new(value),
        }
    }

    /// Replace this entry's key, keeping the value [`Arc`].
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, key)))]
    pub fn with_key(self, key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: self.value,
        }
    }

    /// Namespace this entry under a parent field: `"max_rpm"` becomes
    /// `"motor.max_rpm"`. Applied to every entry of an `#[entry(nested)]` field
    /// during assembly; the value [`Arc`] is moved through untouched.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    pub fn prefixed(self, parent: &str) -> Self {
        let key = format!("{parent}.{}", self.key);
        self.with_key(key)
    }
}

impl ErasedEntry for OwnedEntry {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn key(&self) -> &str {
        &self.key
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn value(&self) -> &dyn Display {
        self.value.as_display()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn value_any(&self) -> &dyn Any {
        self.value.as_any()
    }
}

/// Freeze any live entry into a [`MetadataEntry`] snapshot: the rendered,
/// `Clone + Ord + Hash` form a `Certificate` stores. The structured value is
/// not preserved — that is the point of a snapshot.
impl<E: ErasedEntry + ?Sized> From<&E> for MetadataEntry {
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(entry)))]
    fn from(entry: &E) -> Self {
        MetadataEntry::new(entry.key(), entry.value().to_string())
    }
}

/// A hand-assembled or merged [`Metadata`] record.
///
/// Not what `#[derive(Metadata)]` produces — that implements [`Metadata`]
/// directly on the schema struct. Use this when building metadata by hand or
/// merging several sources.
#[derive(Debug, Clone, Default)]
pub struct MetadataRecord {
    entries: Vec<OwnedEntry>,
}

impl MetadataRecord {
    /// An empty record.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
    pub fn new() -> Self {
        Self::default()
    }

    /// Append one entry.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self, entry)))]
    pub fn push(&mut self, entry: OwnedEntry) {
        self.entries.push(entry);
    }
}

impl FromIterator<OwnedEntry> for MetadataRecord {
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(iter)))]
    fn from_iter<I: IntoIterator<Item = OwnedEntry>>(iter: I) -> Self {
        Self {
            entries: iter.into_iter().collect(),
        }
    }
}

impl Extend<OwnedEntry> for MetadataRecord {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, iter)))]
    fn extend<I: IntoIterator<Item = OwnedEntry>>(&mut self, iter: I) {
        self.entries.extend(iter);
    }
}

impl Metadata for MetadataRecord {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn snapshot(&self) -> Vec<OwnedEntry> {
        self.entries.clone()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn get(&self, key: &str) -> Option<OwnedEntry> {
        self.entries
            .iter()
            .find(|entry| entry.key() == key)
            .cloned()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn len(&self) -> usize {
        self.entries.len()
    }
}

/// Borrowed human-readable rendering of a [`Metadata`] record: one
/// `key: value` line per entry, `(no metadata)` when empty.
#[derive(Debug, Clone, Copy)]
pub struct MetadataReport<'a, M>
where
    M: Metadata + ?Sized,
{
    metadata: &'a M,
}

impl<'a, M> MetadataReport<'a, M>
where
    M: Metadata + ?Sized,
{
    /// Wrap a metadata record for rendering.
    pub const fn new(metadata: &'a M) -> Self {
        Self { metadata }
    }
}

impl<M> Display for MetadataReport<'_, M>
where
    M: Metadata + ?Sized,
{
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, f)))]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut entries = self.metadata.snapshot().into_iter();

        match entries.next() {
            Some(entry) => {
                write!(f, "{}: {}", entry.key(), entry.value())?;

                for entry in entries {
                    write!(f, "\n{}: {}", entry.key(), entry.value())?;
                }

                Ok(())
            }
            None => write!(f, "(no metadata)"),
        }
    }
}

/// Owned human-readable rendering of a [`Metadata`] record.
#[derive(Debug, Clone)]
pub struct OwnedMetadataReport<M>
where
    M: Metadata,
{
    metadata: M,
}

impl<M> OwnedMetadataReport<M>
where
    M: Metadata,
{
    /// Wrap an owned metadata record for rendering.
    pub const fn new(metadata: M) -> Self {
        Self { metadata }
    }
}

impl<M> Display for OwnedMetadataReport<M>
where
    M: Metadata,
{
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, f)))]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&MetadataReport::new(&self.metadata), f)
    }
}

/// Leaf `Metadata` for scalar carriers: one entry keyed `"value"`.
///
/// A field with no `#[entry(..)]` attribute on a `#[derive(Metadata)]` type is
/// projected by walking its `Metadata::snapshot()`; a plain `String` / `u32` /
/// `bool` field needs the field type to be `Metadata` for that. The parent
/// re-keys the `"value"` entry to the field name. `#[entry]` sidesteps this.
macro_rules! impl_scalar_metadata {
    ($($ty:ty),* $(,)?) => {
        $(
            impl Metadata for $ty {
                fn snapshot(&self) -> Vec<OwnedEntry> {
                    ::std::vec![OwnedEntry::new("value", self.clone())]
                }
            }
        )*
    };
}

impl_scalar_metadata!(
    bool, char, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64, String,
);
