//! Kani-only accommodation models for focused runtime-observation laws.
//!
//! The direct std paths for `std::thread::current()` and `Instant::now()`
//! bottom out in unsupported foreign boundaries under Kani
//! (`pthread_key_create` and `clock_gettime`, respectively). This module keeps
//! the smaller Rust-facing laws the production proofs actually claim.

use amenable_core::{MetadataEntry, Provenance};
use amenable_derive::Standard;

/// Observable result of repeatedly querying the currently running thread.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Standard)]
#[standard(basis = "Self")]
pub struct KaniCurrentThreadObservation;

impl KaniCurrentThreadObservation {
    /// Model the already-running current thread.
    #[must_use]
    pub fn current() -> Self {
        Self
    }

    /// Report whether repeated `thread::current()` calls identify the same
    /// thread handle.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn handle_is_stable(&self) -> bool {
        true
    }

    /// Report whether repeated `thread::current().id()` calls identify the
    /// same thread id.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn id_is_stable(&self) -> bool {
        true
    }
}

impl Provenance for KaniCurrentThreadObservation {
    type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn metadata(&self) -> Self::MetadataIter {
        Box::new({
            vec![
            MetadataEntry::new(
                "assumed",
                "repeated queries for the already-running current thread return the same handle and thread id",
            ),
            MetadataEntry::new(
                "rationale",
                "the direct std::thread::current path reaches an unsupported pthread_key_create boundary under Kani",
            ),
            MetadataEntry::new("handle_stable", "true"),
            MetadataEntry::new("id_stable", "true"),
        ]
        .into_iter()
        })
    }
}

/// Observable result of reading a monotonic clock twice in sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Standard)]
#[standard(basis = "Self")]
pub struct KaniInstantObservation;

impl KaniInstantObservation {
    /// Model a monotonic clock source.
    #[must_use]
    pub fn monotonic() -> Self {
        Self
    }

    /// Report whether the later reading is never earlier than the first one.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn later_is_not_earlier(&self) -> bool {
        true
    }
}

impl Provenance for KaniInstantObservation {
    type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn metadata(&self) -> Self::MetadataIter {
        Box::new({
            vec![
            MetadataEntry::new(
                "assumed",
                "a later monotonic clock reading is never earlier than an earlier one",
            ),
            MetadataEntry::new(
                "rationale",
                "the direct Instant::now path reaches an unsupported clock_gettime boundary under Kani",
            ),
            MetadataEntry::new("monotonic", "true"),
        ]
        .into_iter()
        })
    }
}
