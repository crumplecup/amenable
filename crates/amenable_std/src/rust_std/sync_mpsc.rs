//! `RustStdType` registrations for `std::sync::mpsc`.

use std::sync::mpsc::{
    IntoIter, Iter, Receiver, RecvError, RecvTimeoutError, SendError, Sender, SyncSender, TryIter,
    TryRecvError, TrySendError,
};

use crate::rust_std::macros::{
    impl_rust_std_type, impl_rust_std_type_generic1, impl_rust_std_type_lifetime1,
    register_rust_std_standard_evidence,
};

impl_rust_std_type_generic1!(
    Sender,
    "std",
    "std::sync::mpsc",
    "https://doc.rust-lang.org/std/sync/mpsc/struct.Sender.html",
    "The Sender carrier is the sending half of an asynchronous, unbounded mpsc channel."
);

impl_rust_std_type_generic1!(
    SyncSender,
    "std",
    "std::sync::mpsc",
    "https://doc.rust-lang.org/std/sync/mpsc/struct.SyncSender.html",
    "The SyncSender carrier is the sending half of a synchronous, bounded mpsc channel."
);

impl_rust_std_type_generic1!(
    Receiver,
    "std",
    "std::sync::mpsc",
    "https://doc.rust-lang.org/std/sync/mpsc/struct.Receiver.html",
    "The Receiver carrier is the receiving half of an mpsc channel."
);

impl_rust_std_type_generic1!(
    IntoIter,
    "std",
    "std::sync::mpsc",
    "https://doc.rust-lang.org/std/sync/mpsc/struct.IntoIter.html",
    "The IntoIter carrier consumes a Receiver, blocking on each call until a value arrives or the channel closes."
);

impl_rust_std_type_lifetime1!(
    Iter,
    "std",
    "std::sync::mpsc",
    "https://doc.rust-lang.org/std/sync/mpsc/struct.Iter.html",
    "The Iter carrier borrows a Receiver, blocking on each call until a value arrives or the channel closes."
);

impl_rust_std_type_lifetime1!(
    TryIter,
    "std",
    "std::sync::mpsc",
    "https://doc.rust-lang.org/std/sync/mpsc/struct.TryIter.html",
    "The TryIter carrier borrows a Receiver, yielding only values already available without blocking."
);

impl_rust_std_type!(
    RecvError,
    "std",
    "std::sync::mpsc",
    "https://doc.rust-lang.org/std/sync/mpsc/struct.RecvError.html",
    "The RecvError carrier reports that a blocking receive failed because the channel is closed and empty."
);

impl_rust_std_type!(
    RecvTimeoutError,
    "std",
    "std::sync::mpsc",
    "https://doc.rust-lang.org/std/sync/mpsc/enum.RecvTimeoutError.html",
    "The RecvTimeoutError carrier reports why a time-bounded receive failed: Timeout or Disconnected."
);

impl_rust_std_type_generic1!(
    SendError,
    "std",
    "std::sync::mpsc",
    "https://doc.rust-lang.org/std/sync/mpsc/struct.SendError.html",
    "The SendError carrier reports that a send failed because the channel's receiving half was dropped, and returns the unsent value."
);

impl_rust_std_type_generic1!(
    TrySendError,
    "std",
    "std::sync::mpsc",
    "https://doc.rust-lang.org/std/sync/mpsc/enum.TrySendError.html",
    "The TrySendError carrier reports why a non-blocking send failed (Full or Disconnected), and returns the unsent value."
);

impl_rust_std_type!(
    TryRecvError,
    "std",
    "std::sync::mpsc",
    "https://doc.rust-lang.org/std/sync/mpsc/enum.TryRecvError.html",
    "The TryRecvError carrier reports why a non-blocking receive failed: Empty or Disconnected."
);

// `i32` (and `'static` for the borrowing iterators) is the representative
// element type/lifetime this module's proof batch covers.
//
// Sender/Receiver/IntoIter/Iter/TryIter/RecvTimeoutError/TryRecvError are
// written fully-qualified: `std::sync::mpmc` and `std::sync::oneshot`
// (both unstable, but already present in rustdoc's own inventory) define
// their own types with the exact same bare names, so only the qualified
// path disambiguates which channel family a given registration means for
// tooling reading the registry (e.g. `elicit_doc`'s coverage report).
register_rust_std_standard_evidence!(
    std::sync::mpsc::Sender<i32>,
    SyncSender<i32>,
    std::sync::mpsc::Receiver<i32>,
    std::sync::mpsc::IntoIter<i32>,
    std::sync::mpsc::Iter<'static, i32>,
    std::sync::mpsc::TryIter<'static, i32>,
    RecvError,
    std::sync::mpsc::RecvTimeoutError,
    SendError<i32>,
    TrySendError<i32>,
    std::sync::mpsc::TryRecvError,
);
