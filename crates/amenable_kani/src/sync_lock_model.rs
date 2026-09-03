//! Kani-only accommodation model for focused `std::sync` lock laws.
//!
//! The direct `Mutex`/`Barrier`/`Condvar` poisoning and timed-wait paths
//! either reach unsupported foreign boundaries (`futex_wait`, `clock_gettime`,
//! `catch_unwind`) or run under Kani's documented no-concurrency environment
//! model, which does not enforce real mutual exclusion for `try_lock`.
//! This module captures the smaller observable laws the production proofs
//! actually claim.

use amenable_core::{MetadataEntry, Provenance};
use amenable_derive::Standard;

/// Observable result of locking once, rejecting a second lock while held, then
/// allowing a new lock after release.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Standard, derive_getters::Getters, derive_new::new)]
#[standard(basis = "Self", basis_ctor = "Self::new(0)")]
pub struct KaniMutexExclusionObservation {
    /// The value observed through the first guard.
    #[getter(copy)]
    held_value: i32,
}

impl KaniMutexExclusionObservation {
    /// Report whether a second lock attempt is rejected while the first guard
    /// is still live.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn try_lock_while_held_is_err(&self) -> bool {
        true
    }

    /// Report whether locking succeeds again once the first guard is dropped.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn try_lock_after_release_is_ok(&self) -> bool {
        true
    }
}

impl Provenance for KaniMutexExclusionObservation {
    type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn metadata(&self) -> Self::MetadataIter {
        Box::new({
            vec![
            MetadataEntry::new(
                "assumed",
                "locking once exposes the wrapped value, rejects a second lock while the first guard is live, and allows a fresh lock once that guard is released",
            ),
            MetadataEntry::new(
                "rationale",
                "Kani's no-concurrency environment model does not enforce the real Mutex try_lock exclusion guarantee",
            ),
            MetadataEntry::new("held_value", self.held_value.to_string()),
        ]
        .into_iter()
        })
    }
}

/// Observable result of a one-party barrier wait.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Standard)]
#[standard(basis = "Self")]
pub struct KaniBarrierLeaderObservation;

impl KaniBarrierLeaderObservation {
    /// Model the one-party barrier case.
    #[must_use]
    pub fn sole_participant() -> Self {
        Self
    }

    /// Report whether the lone participant is the barrier leader.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn is_leader(&self) -> bool {
        true
    }
}

impl Provenance for KaniBarrierLeaderObservation {
    type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn metadata(&self) -> Self::MetadataIter {
        Box::new({
            vec![
            MetadataEntry::new(
                "assumed",
                "a barrier configured for exactly one participant returns immediately and reports that participant as the leader",
            ),
            MetadataEntry::new(
                "rationale",
                "the direct Barrier wait path reaches an unsupported futex syscall boundary under Kani",
            ),
            MetadataEntry::new("leader", "true"),
        ]
        .into_iter()
        })
    }
}

/// Observable result of a never-notified timeout wait.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Standard)]
#[standard(basis = "Self")]
pub struct KaniWaitTimeoutObservation;

impl KaniWaitTimeoutObservation {
    /// Model the timed-out case.
    #[must_use]
    pub fn timed_out() -> Self {
        Self
    }

    /// Report whether the modeled wait timed out.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn did_time_out(&self) -> bool {
        true
    }
}

impl Provenance for KaniWaitTimeoutObservation {
    type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn metadata(&self) -> Self::MetadataIter {
        Box::new({
            vec![
            MetadataEntry::new(
                "assumed",
                "a never-notified conditional-variable wait with a timeout reports that it timed out",
            ),
            MetadataEntry::new(
                "rationale",
                "the direct Condvar wait_timeout path reaches an unsupported clock_gettime boundary under Kani",
            ),
            MetadataEntry::new("timed_out", "true"),
        ]
        .into_iter()
        })
    }
}

/// Observable result of lock poisoning and non-blocking lock failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Standard, derive_getters::Getters, derive_new::new)]
#[standard(basis = "Self", basis_ctor = "Self::new(0, 1)")]
pub struct KaniMutexFailureObservation {
    /// The value from the poisoned case -- only exposed under a different
    /// name, via [`KaniMutexFailureObservation::poisoned_recovered_value`].
    #[getter(skip)]
    poisoned_value: i32,
    /// The value held in the would-block case.
    #[getter(copy)]
    held_value: i32,
}

impl KaniMutexFailureObservation {
    /// Report the recovered value from the poisoned case.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn poisoned_recovered_value(&self) -> i32 {
        self.poisoned_value
    }

    /// Report whether the poisoned case is classified as poisoned.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn poisoned_case_reports_poisoned(&self) -> bool {
        true
    }

    /// Report whether the held case is classified as would-block.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn held_case_reports_would_block(&self) -> bool {
        true
    }
}

impl Provenance for KaniMutexFailureObservation {
    type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn metadata(&self) -> Self::MetadataIter {
        Box::new({
            vec![
            MetadataEntry::new(
                "assumed",
                "a panic while a guard is live poisons the lock without discarding the guarded value, and a separate already-held try_lock case reports WouldBlock",
            ),
            MetadataEntry::new(
                "rationale",
                "the direct poisoning path reaches unsupported catch_unwind under Kani, while the direct would-block path is distorted by Kani's no-concurrency environment model",
            ),
            MetadataEntry::new("poisoned_value", self.poisoned_value.to_string()),
            MetadataEntry::new("held_value", self.held_value.to_string()),
        ]
        .into_iter()
        })
    }
}
