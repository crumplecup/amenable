//! `RustStdType` registrations for `alloc::boxed`.
//!
//! `ThinBox<T>` is deliberately not covered — unstable (`thin_box`).
//! `BoxedArrayIntoIter<T, const N: usize>` is also not covered: unlike a
//! plain const-generic bound (which just needs a hand-written impl, not a
//! macro), this type doesn't resolve via `std::boxed::BoxedArrayIntoIter`
//! at all on stable (`error[E0425]: cannot find type`, verified empirically
//! — not merely deprecated or feature-gated-with-a-warning, genuinely
//! unnameable from outside `alloc` on this toolchain). See the patch list
//! (`amenable.json` in cordial's patches) for the tracked exception.
//!
//! `Box<T>` covers only the default-allocator, `Sized` case
//! (`Box<T, Global>`): the generic1 macro's `impl<T>` implicitly bounds `T:
//! Sized` and omits the allocator parameter, which resolves to its default
//! (`Global`, itself stable) rather than being generic over it. Real
//! `Box<T: ?Sized, A: Allocator = Global>` coverage over unsized `T` or a
//! custom allocator is narrower than this, but this is still real,
//! non-vacuous coverage of the overwhelmingly common case.

use std::boxed::Box;

use crate::rust_std::macros::{impl_rust_std_type_generic1, register_rust_std_standard_evidence};

impl_rust_std_type_generic1!(
    Box,
    "alloc",
    "alloc::boxed",
    "https://doc.rust-lang.org/alloc/boxed/struct.Box.html",
    "The Box carrier is a unique, heap-allocated owner of its contents."
);

register_rust_std_standard_evidence!(Box<i32>);
