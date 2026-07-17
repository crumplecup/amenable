//! `RustStdType` registrations for `core::future`.

use std::future::{Pending, PollFn, Ready};

use crate::rust_std::macros::impl_rust_std_type_generic1;

impl_rust_std_type_generic1!(
    Pending,
    "core",
    "core::future",
    "https://doc.rust-lang.org/core/future/struct.Pending.html",
    "The Pending carrier is a Future that never resolves, always yielding Poll::Pending."
);

impl_rust_std_type_generic1!(
    PollFn,
    "core",
    "core::future",
    "https://doc.rust-lang.org/core/future/struct.PollFn.html",
    "The PollFn carrier turns a closure with Future::poll's signature into a Future."
);

impl_rust_std_type_generic1!(
    Ready,
    "core",
    "core::future",
    "https://doc.rust-lang.org/core/future/struct.Ready.html",
    "The Ready carrier is a Future that resolves immediately with an already-computed value."
);
