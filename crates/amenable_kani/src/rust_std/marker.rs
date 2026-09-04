//! `KaniWitness` impls for `core::marker`.
//!
//! Both carriers here are zero-sized markers with no behavior to check at
//! runtime: `PhantomData<T>` has no methods at all, and `PhantomPinned`'s
//! only effect (making its containing type `!Unpin`) is a compile-time
//! trait fact, not something a symbolic-execution harness observes. Both
//! stay at the trusted disposition.

use std::marker::{PhantomData, PhantomPinned};

use crate::rust_std::impl_kani_witness_trusted;

impl_kani_witness_trusted!(PhantomData<i32>, PhantomPinned);
