//! `RustStdType` registrations for `std::sync`'s locking primitives.
//!
//! Deliberately not covered here, all unstable: `MappedMutexGuard`/
//! `MappedRwLockReadGuard`/`MappedRwLockWriteGuard` (`mapped_lock_guards`),
//! `ReentrantLock`/`ReentrantLockGuard` (`reentrant_lock`). `std::sync::
//! mpmc`/`nonpoison`/`oneshot` are also entirely unstable (`mpmc_channel`,
//! `nonpoison_mutex`/`nonpoison_rwlock`/`sync_nonpoison`, presumably an
//! analogous gate for `oneshot`) — not covered at all.

use std::sync::{
    Barrier, BarrierWaitResult, Condvar, LazyLock, Mutex, MutexGuard, Once, OnceLock, OnceState,
    PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard, TryLockError, WaitTimeoutResult,
};

use crate::rust_std::macros::{
    impl_rust_std_type, impl_rust_std_type_generic1, impl_rust_std_type_generic2,
    impl_rust_std_type_lifetime1,
};

impl_rust_std_type_generic1!(
    Mutex,
    "std",
    "std::sync",
    "https://doc.rust-lang.org/std/sync/struct.Mutex.html",
    "The Mutex carrier provides mutual-exclusion access to its contents, blocking other threads while held."
);

impl_rust_std_type_lifetime1!(
    MutexGuard,
    "std",
    "std::sync",
    "https://doc.rust-lang.org/std/sync/struct.MutexGuard.html",
    "The MutexGuard carrier is an RAII guard granting exclusive access to a Mutex's contents while it is held."
);

impl_rust_std_type_generic1!(
    RwLock,
    "std",
    "std::sync",
    "https://doc.rust-lang.org/std/sync/struct.RwLock.html",
    "The RwLock carrier allows any number of concurrent readers or one writer to access its contents."
);

impl_rust_std_type_lifetime1!(
    RwLockReadGuard,
    "std",
    "std::sync",
    "https://doc.rust-lang.org/std/sync/struct.RwLockReadGuard.html",
    "The RwLockReadGuard carrier is an RAII guard granting shared read access to an RwLock's contents while it is held."
);

impl_rust_std_type_lifetime1!(
    RwLockWriteGuard,
    "std",
    "std::sync",
    "https://doc.rust-lang.org/std/sync/struct.RwLockWriteGuard.html",
    "The RwLockWriteGuard carrier is an RAII guard granting exclusive write access to an RwLock's contents while it is held."
);

impl_rust_std_type!(
    Once,
    "std",
    "std::sync",
    "https://doc.rust-lang.org/std/sync/struct.Once.html",
    "The Once carrier ensures a given closure runs exactly once, no matter how many threads call it concurrently."
);

impl_rust_std_type!(
    OnceState,
    "std",
    "std::sync",
    "https://doc.rust-lang.org/std/sync/struct.OnceState.html",
    "The OnceState carrier reports whether a Once's closure poisoned it while running."
);

impl_rust_std_type_generic1!(
    OnceLock,
    "std",
    "std::sync",
    "https://doc.rust-lang.org/std/sync/struct.OnceLock.html",
    "The OnceLock carrier holds a value that can be initialized exactly once, safely across threads."
);

impl_rust_std_type_generic2!(
    LazyLock,
    "std",
    "std::sync",
    "https://doc.rust-lang.org/std/sync/struct.LazyLock.html",
    "The LazyLock carrier holds a value computed by an initializer closure on first access, safely across threads."
);

impl_rust_std_type!(
    Barrier,
    "std",
    "std::sync",
    "https://doc.rust-lang.org/std/sync/struct.Barrier.html",
    "The Barrier carrier blocks a fixed number of threads until all of them have reached it."
);

impl_rust_std_type!(
    BarrierWaitResult,
    "std",
    "std::sync",
    "https://doc.rust-lang.org/std/sync/struct.BarrierWaitResult.html",
    "The BarrierWaitResult carrier reports whether a thread was the last to arrive at a Barrier."
);

impl_rust_std_type!(
    Condvar,
    "std",
    "std::sync",
    "https://doc.rust-lang.org/std/sync/struct.Condvar.html",
    "The Condvar carrier lets threads block until notified, used together with a Mutex."
);

impl_rust_std_type_generic1!(
    PoisonError,
    "std",
    "std::sync",
    "https://doc.rust-lang.org/std/sync/struct.PoisonError.html",
    "The PoisonError carrier wraps a lock guard, reporting that the lock was poisoned by a panic while held."
);

impl_rust_std_type_generic1!(
    TryLockError,
    "std",
    "std::sync",
    "https://doc.rust-lang.org/std/sync/enum.TryLockError.html",
    "The TryLockError carrier reports why a non-blocking lock attempt failed: Poisoned or WouldBlock."
);

impl_rust_std_type!(
    WaitTimeoutResult,
    "std",
    "std::sync",
    "https://doc.rust-lang.org/std/sync/struct.WaitTimeoutResult.html",
    "The WaitTimeoutResult carrier reports whether a Condvar wait timed out before being notified."
);
