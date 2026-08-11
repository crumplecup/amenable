//! `KaniWitness` impls for Rust standard-library carriers, split into one
//! module per source-library area — mirrors `amenable_std::rust_std`'s
//! module boundaries, so the Kani proof for e.g. `core::num::NonZero<T>`
//! lives in `num.rs` here just as its `RustStdType` impl lives in
//! `amenable_std::rust_std::num`.
//!
//! Each type gets exactly the harness that's actually true of it — see
//! `macros.rs` for the two dispositions every type falls into (`trusted`:
//! nothing beyond what the type system already guarantees, so `proof()` is
//! just the chain-derived provenance; `checked`: a real `#[kani::proof]`
//! harness, captured verbatim via `amenable_derive::harness!` so it can
//! never drift from what `cargo kani` would actually run).

mod alloc_borrow;
mod alloc_boxed;
mod alloc_collections;
mod alloc_ffi;
mod alloc_rc;
mod alloc_string;
mod alloc_sync;
mod alloc_vec;
mod any;
mod array;
mod ascii;
mod backtrace;
mod cell;
mod char;
mod cmp;
mod convert;
mod core_alloc;
mod env;
mod ffi;
mod fmt;
mod fs;
mod future;
mod hash;
mod io;
mod iter;
pub(crate) mod macros;
mod marker;
mod mem;
mod net;
mod num;
mod ops;
mod option_result;
mod os_unix;
mod os_windows;
mod panic;
mod path;
mod pin;
mod primitives;
mod process;
mod ptr;
mod slice;
mod std_alloc;
mod std_collections;
mod std_ffi;
mod std_hash;
mod std_net;
mod std_panic;
mod std_time;
mod str;
mod sync_atomic;
mod sync_lock;
mod sync_mpsc;
mod task;
mod thread;
mod time;

pub use alloc_collections::EmptiedContainerReportsEmpty;
pub use alloc_vec::VecLengthTracksPushesAndPops;
pub use iter::{IteratorMatchesReferenceStepByStep, IteratorYieldsNoneWhenExhausted};
pub use macros::CheckedProof;
pub use primitives::DerefReflectsTheStoredValue;
pub use sync_atomic::AtomicLoadReflectsTheLastWrite;
