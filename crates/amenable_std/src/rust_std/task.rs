//! `RustStdType` registrations for `core::task`.

use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

use crate::rust_std::macros::{
    impl_rust_std_type, impl_rust_std_type_generic1, impl_rust_std_type_lifetime0,
    register_rust_std_standard_evidence,
};

impl_rust_std_type_lifetime0!(
    Context,
    "core",
    "core::task",
    "https://doc.rust-lang.org/core/task/struct.Context.html",
    "The Context carrier bundles the state an async task's Future::poll needs, chiefly a Waker."
);

impl_rust_std_type_generic1!(
    Poll,
    "core",
    "core::task",
    "https://doc.rust-lang.org/core/task/enum.Poll.html",
    "The Poll carrier represents an async computation as either Ready with a value or still Pending."
);

impl_rust_std_type!(
    RawWaker,
    "core",
    "core::task",
    "https://doc.rust-lang.org/core/task/struct.RawWaker.html",
    "The RawWaker carrier pairs a data pointer with a RawWakerVTable, the low-level building block a Waker is constructed from."
);

impl_rust_std_type!(
    RawWakerVTable,
    "core",
    "core::task",
    "https://doc.rust-lang.org/core/task/struct.RawWakerVTable.html",
    "The RawWakerVTable carrier holds the clone/wake/wake_by_ref/drop function pointers backing a RawWaker."
);

impl_rust_std_type!(
    Waker,
    "core",
    "core::task",
    "https://doc.rust-lang.org/core/task/struct.Waker.html",
    "The Waker carrier lets an async task's executor be notified when the task should be polled again."
);

// `'static` is the representative lifetime for `Context` (see
// `register_rust_std_standard_evidence!`'s own doc comment).
register_rust_std_standard_evidence!(Context<'static>, Poll<i32>, RawWaker, RawWakerVTable, Waker,);
