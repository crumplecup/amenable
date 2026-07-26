//! Kani-only accommodation model for focused `BTreeMap` / `BTreeSet`
//! ordering semantics.
//!
//! This module is where Amenable stops asking Kani to execute std B-tree
//! internals directly and instead proves against a small package of explicit
//! ordering and post-observation retention laws that the real implementation is
//! expected to refine.
//!
//! The direct symbolic std `BTreeMap` / `BTreeSet` iteration paths remain
//! preserved in the proof gallery as timeout false trails. Production proofs
//! that use this model are therefore conditional:
//!
//! - if the real ordered collections conform to these laws,
//! - then the modeled Kani proof carries the intended Rust-facing claim.

/// Modeled two-entry `BTreeMap` semantics for the focused proof queue.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KaniBTreeMap<K, V> {
    low_key: K,
    low_value: Option<V>,
    high_key: K,
    high_value: Option<V>,
}

/// Modeled two-entry `BTreeSet` semantics for the focused proof queue.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KaniBTreeSet<T> {
    low_value: Option<T>,
    high_value: Option<T>,
}

impl<K, V> KaniBTreeMap<K, V>
where
    K: Ord,
{
    /// Construct a two-entry ordered map model from two distinct logical
    /// entries supplied in any insertion order.
    pub fn new(first_key: K, first_value: V, second_key: K, second_value: V) -> Self {
        if first_key <= second_key {
            Self {
                low_key: first_key,
                low_value: Some(first_value),
                high_key: second_key,
                high_value: Some(second_value),
            }
        } else {
            Self {
                low_key: second_key,
                low_value: Some(second_value),
                high_key: first_key,
                high_value: Some(first_value),
            }
        }
    }

    /// Observe the first iterator entry in ascending key order.
    pub fn first_entry(&self) -> Option<(&K, &V)> {
        self.low_value.as_ref().map(|value| (&self.low_key, value))
    }

    /// Observe the second iterator entry in ascending key order.
    pub fn second_entry(&self) -> Option<(&K, &V)> {
        self.high_value
            .as_ref()
            .map(|value| (&self.high_key, value))
    }

    /// Remove a key and recover its value without disturbing the other entry.
    pub fn remove(&mut self, key: &K) -> Option<V> {
        if key == &self.low_key {
            self.low_value.take()
        } else if key == &self.high_key {
            self.high_value.take()
        } else {
            None
        }
    }

    /// Report whether every modeled entry has been removed.
    pub fn is_empty(&self) -> bool {
        self.low_value.is_none() && self.high_value.is_none()
    }
}

impl<T> KaniBTreeSet<T>
where
    T: Ord,
{
    /// Construct a two-entry ordered set model from two distinct logical
    /// members supplied in any insertion order.
    pub fn new(first_value: T, second_value: T) -> Self {
        if first_value <= second_value {
            Self {
                low_value: Some(first_value),
                high_value: Some(second_value),
            }
        } else {
            Self {
                low_value: Some(second_value),
                high_value: Some(first_value),
            }
        }
    }

    /// Observe the first iterator item in ascending order.
    pub fn first_item(&self) -> Option<&T> {
        self.low_value.as_ref()
    }

    /// Observe the second iterator item in ascending order.
    pub fn second_item(&self) -> Option<&T> {
        self.high_value.as_ref()
    }

    /// Remove an item while leaving the other member intact.
    pub fn remove(&mut self, value: &T) -> bool {
        if self.low_value.as_ref() == Some(value) {
            let _ = self.low_value.take();
            true
        } else if self.high_value.as_ref() == Some(value) {
            let _ = self.high_value.take();
            true
        } else {
            false
        }
    }

    /// Report whether every modeled member has been removed.
    pub fn is_empty(&self) -> bool {
        self.low_value.is_none() && self.high_value.is_none()
    }
}
