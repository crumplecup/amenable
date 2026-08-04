//! Kani-only accommodation model for focused `HashMap` / `HashSet`
//! membership semantics.
//!
//! The direct `HashMap::new()`/`HashSet::new()` path times out under Kani
//! even for a single fixed (non-symbolic) key/value pair — confirmed
//! empirically, and consistent with this crate's other "pure in-memory std
//! implementation blow-up" cases (`DefaultHasher`, `BTreeMap`, and
//! friends). This module keeps the narrow contract the production proofs
//! actually claim: insert-then-lookup recovers what was inserted. Unlike
//! `BTreeMap`, `HashMap`/`HashSet` make no ordering guarantee to model, so
//! a single-entry witness suffices.
//!
//! Production proofs that use this model are therefore conditional: if the
//! real `HashMap`/`HashSet` refine this one-entry insert/lookup law, the
//! Rust-facing claim follows.

/// Modeled one-entry `HashMap` semantics for the focused proof queue.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KaniHashMap<K, V> {
    key: K,
    value: Option<V>,
}

impl<K: PartialEq, V> KaniHashMap<K, V> {
    /// Construct a one-entry map model from a freshly inserted key/value pair.
    #[must_use]
    pub fn new(key: K, value: V) -> Self {
        Self {
            key,
            value: Some(value),
        }
    }

    /// Model `get(key)`: `Some` only for the one modeled key, `None`
    /// otherwise (including after removal).
    #[must_use]
    pub fn get(&self, key: &K) -> Option<&V> {
        if &self.key == key {
            self.value.as_ref()
        } else {
            None
        }
    }

    /// Model `remove(key)`, returning the removed value if present.
    pub fn remove(&mut self, key: &K) -> Option<V> {
        if &self.key == key {
            self.value.take()
        } else {
            None
        }
    }
}

/// Modeled one-entry `HashSet` semantics for the focused proof queue.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KaniHashSet<T> {
    value: T,
    present: bool,
}

impl<T: PartialEq> KaniHashSet<T> {
    /// Construct a one-entry set model from a freshly inserted value.
    #[must_use]
    pub fn new(value: T) -> Self {
        Self {
            value,
            present: true,
        }
    }

    /// Model `contains(value)`.
    #[must_use]
    pub fn contains(&self, value: &T) -> bool {
        self.present && &self.value == value
    }

    /// Model `remove(value)`, reporting whether it was present.
    pub fn remove(&mut self, value: &T) -> bool {
        if self.contains(value) {
            self.present = false;
            true
        } else {
            false
        }
    }
}
