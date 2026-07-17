//! `RustStdType` registrations for `core::marker`.
//!
//! The variance-marker family (`PhantomContravariant`, `PhantomCovariant`,
//! `PhantomInvariant`, and their `*Lifetime` counterparts) is not covered
//! here — it sits behind an unstable feature gate, not nameable from a
//! stable toolchain.

use std::marker::{PhantomData, PhantomPinned};

use crate::rust_std::macros::{
    impl_rust_std_type, impl_rust_std_type_generic1, register_rust_std_standard_evidence,
};

impl_rust_std_type_generic1!(
    PhantomData,
    "core",
    "core::marker",
    "https://doc.rust-lang.org/core/marker/struct.PhantomData.html",
    "The PhantomData carrier is a zero-sized marker telling the type system a generic parameter is used without storing a value of it."
);

impl_rust_std_type!(
    PhantomPinned,
    "core",
    "core::marker",
    "https://doc.rust-lang.org/core/marker/struct.PhantomPinned.html",
    "The PhantomPinned carrier is a zero-sized marker that makes its containing type !Unpin."
);

register_rust_std_standard_evidence!(PhantomData<i32>, PhantomPinned);
