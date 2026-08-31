//! `KaniWitness` impls for `core::slice`.
//!
//! Every harness checks against symbolic (`kani::any()`) element values
//! rather than a fixed literal example, following `rust_std::iter`'s
//! pattern. `'static` is the representative lifetime (the harnesses
//! themselves borrow from local, non-`'static` arrays — the claim holds
//! uniformly over every lifetime, same reasoning as `Ref`/`RefMut` in
//! `rust_std::cell`). Predicate parameters use bare `fn` pointer types
//! rather than closures, since closures have no nameable type to register
//! evidence against.
//!
//! Split by the real slice-API family each file covers: [`iter`] (the
//! basic `Iter`/`IterMut`), [`chunks`] (the `chunks`/`rchunks`/`windows`
//! family), [`chunk_by`], [`split`] (plus the split-family's own shared
//! `SplitOperandsAreDistinctFromThePattern`/
//! `ThreeSplitOperandsAreDistinctFromThePattern` precondition markers),
//! [`split_inclusive`], [`split_n_and_rsplit`], and [`misc`] (`EscapeAscii`,
//! `GetDisjointMutError`).

mod chunk_by;
mod chunks;
mod iter;
mod misc;
mod split;
mod split_inclusive;
mod split_n_and_rsplit;

pub use split::{
    SplitOperandsAreDistinctFromThePattern, ThreeSplitOperandsAreDistinctFromThePattern,
};
