//! Kani-only accommodation model for `std::sync::mpsc` channel semantics.
//!
//! This module is where Amenable stops asking Kani to execute the real
//! `std::sync::mpsc` channel (a flavor-switching, atomics-backed queue) directly
//! and instead proves against a small package of explicit delivery, ordering,
//! and disconnect laws that the real implementation is expected to refine.
//!
//! Confirmed empirically: even the simplest possible use (`send` immediately
//! followed by `recv` on a freshly created unbounded channel, with no actual
//! blocking) times out under Kani's native harness timeout -- this is a pure
//! in-memory implementation cost, not a foreign-boundary or unwinding-bound
//! issue like several other proof families in this crate. `recv_timeout` (and
//! anything else that computes a deadline) additionally reaches the same
//! `clock_gettime` foreign boundary already documented for `Condvar`, and
//! remains on the direct path rather than being modeled here.
//!
//! The direct `std::sync::mpsc` path remains preserved in the proof gallery as
//! a timeout false trail. Production proofs that use this model are therefore
//! conditional:
//!
//! - if the real channel conforms to these laws,
//! - then the modeled Kani proof carries the intended Rust-facing claim.

const KANI_MPSC_MAX_QUEUE: usize = 2;

/// Modeled error recovering an unsent value: `send`/`try_send` failing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KaniSendError<T> {
    /// The paired receiver has been dropped; the channel is disconnected.
    Disconnected(T),
    /// The channel is at capacity (bounded/rendezvous channels only).
    Full(T),
}

/// Modeled error for a non-blocking receive against an empty channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KaniRecvError {
    /// The channel is open but has no buffered value yet.
    Empty,
    /// The channel is empty and the paired sender has been dropped.
    Disconnected,
}

/// A small, bounded FIFO channel model shared by an `mpsc::Sender`-shaped
/// producer and an `mpsc::Receiver`-shaped consumer.
///
/// `capacity`: `None` models an unbounded `channel()`; `Some(n)` models a
/// `sync_channel(n)` (`Some(0)` is a zero-capacity rendezvous channel, whose
/// `try_send` always reports `Full` since nothing here models a receiver
/// actively rendezvousing).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KaniChannel<T> {
    queue: Vec<T>,
    sender_open: bool,
    receiver_open: bool,
    capacity: Option<usize>,
}

impl<T> KaniChannel<T> {
    /// Construct a fresh, open, empty unbounded channel.
    #[must_use]
    pub fn unbounded() -> Self {
        Self {
            queue: Vec::new(),
            sender_open: true,
            receiver_open: true,
            capacity: None,
        }
    }

    /// Construct a fresh, open, empty bounded (`sync_channel`) channel.
    #[must_use]
    pub fn bounded(capacity: usize) -> Self {
        Self {
            queue: Vec::new(),
            sender_open: true,
            receiver_open: true,
            capacity: Some(capacity),
        }
    }

    /// Drop the modeled sender endpoint.
    pub fn drop_sender(&mut self) {
        self.sender_open = false;
    }

    /// Drop the modeled receiver endpoint.
    pub fn drop_receiver(&mut self) {
        self.receiver_open = false;
    }

    /// Model a blocking `send`: succeeds while the receiver is open and the
    /// channel isn't at capacity, otherwise recovers the value in the error.
    ///
    /// # Errors
    ///
    /// Returns `KaniSendError::Disconnected` if the receiver has been
    /// dropped, or `KaniSendError::Full` if a bounded channel is at capacity.
    pub fn send(&mut self, value: T) -> Result<(), KaniSendError<T>> {
        if !self.receiver_open {
            return Err(KaniSendError::Disconnected(value));
        }
        if let Some(capacity) = self.capacity
            && self.queue.len() >= capacity
        {
            return Err(KaniSendError::Full(value));
        }
        assert!(
            self.queue.len() < KANI_MPSC_MAX_QUEUE,
            "KaniChannel models at most {KANI_MPSC_MAX_QUEUE} queued values"
        );
        self.queue.push(value);
        Ok(())
    }

    /// Model a non-blocking `try_send`, identical to `send` for this model
    /// (which never actually blocks).
    ///
    /// # Errors
    ///
    /// Same as [`Self::send`].
    pub fn try_send(&mut self, value: T) -> Result<(), KaniSendError<T>> {
        self.send(value)
    }

    /// Model a blocking `recv`: yields the oldest queued value in FIFO
    /// order, or fails once the queue is empty and the sender has been
    /// dropped.
    ///
    /// # Errors
    ///
    /// Returns `KaniRecvError::Disconnected` once the queue is drained and
    /// the sender is closed. Never returns `KaniRecvError::Empty` -- a real
    /// blocking `recv` on an open, empty channel doesn't return at all
    /// until a value arrives or the sender disconnects, so this model
    /// requires the sender closed once the queue is empty.
    pub fn recv(&mut self) -> Result<T, KaniRecvError> {
        if let Some(value) = self.pop_front() {
            return Ok(value);
        }
        assert!(
            !self.sender_open,
            "modeled recv on an empty, still-open channel would block forever"
        );
        Err(KaniRecvError::Disconnected)
    }

    /// Model a non-blocking `try_recv`: distinguishes an open-but-empty
    /// channel from one that's empty and disconnected.
    ///
    /// # Errors
    ///
    /// Returns `KaniRecvError::Empty` if the queue is empty and the sender
    /// is still open, or `KaniRecvError::Disconnected` if the queue is
    /// empty and the sender has been dropped.
    pub fn try_recv(&mut self) -> Result<T, KaniRecvError> {
        if let Some(value) = self.pop_front() {
            return Ok(value);
        }
        if self.sender_open {
            Err(KaniRecvError::Empty)
        } else {
            Err(KaniRecvError::Disconnected)
        }
    }

    /// Report the number of currently queued values.
    #[must_use]
    pub fn queue_len(&self) -> usize {
        self.queue.len()
    }

    fn pop_front(&mut self) -> Option<T> {
        if self.queue.is_empty() {
            None
        } else {
            Some(self.queue.remove(0))
        }
    }
}
