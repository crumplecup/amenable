//! `RustStdType` registrations for `core::cell`.
//!
//! `SyncUnsafeCell` is deliberately not covered here — unstable
//! (`sync_unsafe_cell`, rust-lang/rust#95439), not nameable from a stable
//! toolchain.

use std::cell::{Cell, LazyCell, OnceCell, Ref, RefCell, RefMut, UnsafeCell};

use crate::rust_std::macros::{
    impl_rust_std_type, impl_rust_std_type_generic1, impl_rust_std_type_generic2,
    impl_rust_std_type_lifetime1,
};

impl_rust_std_type_generic1!(
    Cell,
    "core",
    "core::cell",
    "https://doc.rust-lang.org/core/cell/struct.Cell.html",
    "The Cell carrier provides shared-reference interior mutability by moving values in and out rather than handing out references."
);

impl_rust_std_type_generic1!(
    RefCell,
    "core",
    "core::cell",
    "https://doc.rust-lang.org/core/cell/struct.RefCell.html",
    "The RefCell carrier provides shared-reference interior mutability with dynamically checked borrowing rules."
);

impl_rust_std_type_lifetime1!(
    Ref,
    "core",
    "core::cell",
    "https://doc.rust-lang.org/core/cell/struct.Ref.html",
    "The Ref carrier is a shared, dynamically checked borrow of a RefCell's contents."
);

impl_rust_std_type_lifetime1!(
    RefMut,
    "core",
    "core::cell",
    "https://doc.rust-lang.org/core/cell/struct.RefMut.html",
    "The RefMut carrier is a mutable, dynamically checked borrow of a RefCell's contents."
);

impl_rust_std_type_generic1!(
    OnceCell,
    "core",
    "core::cell",
    "https://doc.rust-lang.org/core/cell/struct.OnceCell.html",
    "The OnceCell carrier holds a value that can be initialized exactly once."
);

impl_rust_std_type_generic1!(
    UnsafeCell,
    "core",
    "core::cell",
    "https://doc.rust-lang.org/core/cell/struct.UnsafeCell.html",
    "The UnsafeCell carrier is the core primitive for interior mutability, giving raw access to the aliasing rules any other cell type is built from."
);

impl_rust_std_type_generic2!(
    LazyCell,
    "core",
    "core::cell",
    "https://doc.rust-lang.org/core/cell/struct.LazyCell.html",
    "The LazyCell carrier holds a value that is computed by an initializer closure on first access."
);

impl_rust_std_type!(
    core::cell::BorrowError,
    "core",
    "core::cell",
    "https://doc.rust-lang.org/core/cell/struct.BorrowError.html",
    "The BorrowError carrier reports that a RefCell was already mutably borrowed when a shared borrow was requested."
);

impl_rust_std_type!(
    core::cell::BorrowMutError,
    "core",
    "core::cell",
    "https://doc.rust-lang.org/core/cell/struct.BorrowMutError.html",
    "The BorrowMutError carrier reports that a RefCell was already borrowed when a mutable borrow was requested."
);
