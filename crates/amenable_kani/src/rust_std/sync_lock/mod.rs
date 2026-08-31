//! `KaniWitness` impls for `std::sync`'s locking primitives.
//!
//! Direct guard, `RwLock`, `Once`, `OnceLock`, and `LazyLock` laws still verify
//! against the real standard-library types. The `Mutex` exclusion, `Barrier`,
//! `Condvar`, poisoning, and timeout-result laws instead use Amenable-owned
//! observations: their direct std paths either hit unsupported Kani
//! boundaries (`futex_wait`, `clock_gettime`, `catch_unwind`) or rely on
//! mutual-exclusion behavior that Kani's no-concurrency environment model does
//! not enforce. The false trails remain preserved in the gallery.
//!
//! Split by the real API family each file covers: [`mutex`], [`rwlock`],
//! [`once_and_lazy`] (`Once`, `OnceState`, `OnceLock`, `LazyLock`, plus the
//! `OnceStateIsPoisonedReportsTrue` marker), [`barrier`], [`condvar`], and
//! [`poison_and_timeout_errors`] (`PoisonError`, `TryLockError`,
//! `WaitTimeoutResult`).

mod barrier;
mod condvar;
mod mutex;
mod once_and_lazy;
mod poison_and_timeout_errors;
mod rwlock;
