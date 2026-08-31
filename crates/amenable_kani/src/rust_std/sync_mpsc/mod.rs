//! `KaniWitness` impls for `std::sync::mpsc`.
//!
//! Every harness runs single-threaded: sending, then immediately
//! receiving (or checking a disconnect/timeout/full condition) without
//! ever blocking on another thread. This is enough to check each carrier's
//! own contract — the channel's transport behavior, not cross-thread
//! scheduling.
//!
//! Split by the real API family each file covers: [`sender`] (`Sender`,
//! `SyncSender`), [`receiver_and_into_iter`] (`Receiver`, `IntoIter`),
//! [`iter_and_try_iter`] (`Iter`, `TryIter`), [`recv_errors`] (`RecvError`,
//! `RecvTimeoutError`), and [`send_errors`] (`SendError`, `TrySendError`,
//! `TryRecvError`).

mod iter_and_try_iter;
mod receiver_and_into_iter;
mod recv_errors;
mod send_errors;
mod sender;
