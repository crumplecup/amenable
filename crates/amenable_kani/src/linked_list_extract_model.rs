//! Kani-only accommodation model for `LinkedList::extract_if`.
//!
//! This module captures the two behaviors the production proof queue relies on:
//!
//! - visiting the list yields exactly the matching elements in encounter order
//! - dropping the extractor early leaves every unvisited element in place
//!
//! The direct std `LinkedList::extract_if` path remains preserved in the proof
//! gallery as a timeout false trail. Production proofs that use this model are
//! therefore conditional:
//!
//! - if the real `LinkedList::extract_if` path refines these modeled laws,
//! - then the modeled Kani proof carries the intended Rust-facing claim.

/// Minimal owned `extract_if` state machine for Kani-facing proofs.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KaniLinkedListExtractIf<T> {
    items: Vec<Option<T>>,
    cursor: usize,
}

impl<T> KaniLinkedListExtractIf<T> {
    /// Construct one modeled extractor from the original list contents.
    pub fn new(items: Vec<T>) -> Self {
        Self {
            items: items.into_iter().map(Some).collect(),
            cursor: 0,
        }
    }

    /// Visit elements until the next predicate match is found and removed.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self, predicate)))]
    pub fn next(&mut self, predicate: fn(&mut T) -> bool) -> Option<T> {
        while self.cursor < self.items.len() {
            let slot = &mut self.items[self.cursor];
            self.cursor += 1;

            if let Some(item) = slot.as_mut()
                && predicate(item)
            {
                return slot.take();
            }
        }

        None
    }

    /// Recover the list contents that remain after the visited removals.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    pub fn into_remaining(self) -> Vec<T> {
        self.items.into_iter().flatten().collect()
    }
}
