//! `RustStdType`: interface and concrete registrations for Rust
//! standard-library carriers, split into one module per source-library
//! area so no single file grows unbounded as coverage widens.
//!
//! `RustStdType` is implemented directly on foreign standard-library types
//! (`bool`, `i32`, `String`, ...), which Rust's orphan rules only permit
//! from the crate that defines the trait. So unlike the core constitutional
//! roles in `amenable`, this trait and its std-lib coverage live together
//! here rather than split across an interface crate and a downstream
//! consumer.

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
mod certificate;
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
mod macros;
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
mod types;

pub use certificate::write_rust_std_certificate_artifacts;
pub use types::{RustLanguageProvenance, RustStdProvenance, RustStdStandard, RustStdType};
