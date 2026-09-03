//! `KaniWitness` impls for `core::sync::atomic`, split by value category:
//! `boolean` (also home to the shared
//! [`AtomicLoadReflectsTheLastWrite`] contract type), `signed` and
//! `unsigned` for the atomic integers, `pointer` for `AtomicPtr`, and
//! `ordering` for the one non-`SeqCst` memory-ordering harness.

mod boolean;
mod ordering;
mod pointer;
mod signed;
mod unsigned;

pub use boolean::AtomicLoadReflectsTheLastWrite;
