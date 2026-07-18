//! `RustStdType` registrations for `std::thread`.

use std::thread::{
    AccessError, Builder, JoinHandle, LocalKey, Scope, ScopedJoinHandle, Thread, ThreadId,
};

use crate::rust_std::macros::{
    impl_rust_std_type, impl_rust_std_type_generic1, impl_rust_std_type_lifetime1,
    register_rust_std_standard_evidence,
};

impl_rust_std_type!(
    AccessError,
    "std",
    "std::thread",
    "https://doc.rust-lang.org/std/thread/struct.AccessError.html",
    "The AccessError carrier reports that a thread-local value could not be accessed, typically because it is being destroyed."
);

impl_rust_std_type!(
    Builder,
    "std",
    "std::thread",
    "https://doc.rust-lang.org/std/thread/struct.Builder.html",
    "The Builder carrier configures a new thread's name and stack size before spawning it."
);

impl_rust_std_type_generic1!(
    JoinHandle,
    "std",
    "std::thread",
    "https://doc.rust-lang.org/std/thread/struct.JoinHandle.html",
    "The JoinHandle carrier is an owned handle to a spawned thread, used to wait for it to finish and collect its result."
);

impl_rust_std_type_generic1!(
    LocalKey,
    "std",
    "std::thread",
    "https://doc.rust-lang.org/std/thread/struct.LocalKey.html",
    "The LocalKey carrier is a handle to a thread_local! value, giving each thread its own independent instance."
);

// Scope<'scope, 'env> — two lifetimes, no type parameter, the same shape
// as core::fmt's Debug*-builder family.
impl<'scope, 'env> crate::RustStdType for Scope<'scope, 'env> {
    fn rust_language_provenance() -> crate::RustLanguageProvenance {
        crate::RustLanguageProvenance::for_source("std", "std::thread")
    }

    fn rust_doc_url() -> &'static str {
        "https://doc.rust-lang.org/std/thread/struct.Scope.html"
    }

    fn rust_semantics_summary() -> &'static str {
        "The Scope carrier lets a thread::scope call spawn threads that can safely borrow from the enclosing scope."
    }
}

impl_rust_std_type_lifetime1!(
    ScopedJoinHandle,
    "std",
    "std::thread",
    "https://doc.rust-lang.org/std/thread/struct.ScopedJoinHandle.html",
    "The ScopedJoinHandle carrier is an owned handle to a scoped thread, used to wait for it to finish and collect its result."
);

impl_rust_std_type!(
    Thread,
    "std",
    "std::thread",
    "https://doc.rust-lang.org/std/thread/struct.Thread.html",
    "The Thread carrier is a handle to a thread, used to query or interact with it (e.g. unpark it)."
);

impl_rust_std_type!(
    ThreadId,
    "std",
    "std::thread",
    "https://doc.rust-lang.org/std/thread/struct.ThreadId.html",
    "The ThreadId carrier uniquely identifies a thread for the lifetime of the process."
);

// `i32` is the representative payload type for JoinHandle/LocalKey, and
// `'static` the representative lifetime for Scope/ScopedJoinHandle.
register_rust_std_standard_evidence!(
    AccessError,
    Builder,
    JoinHandle<i32>,
    LocalKey<std::cell::Cell<i32>>,
    Scope<'static, 'static>,
    ScopedJoinHandle<'static, i32>,
    Thread,
    ThreadId,
);
