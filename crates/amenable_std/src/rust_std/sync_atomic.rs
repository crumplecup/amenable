//! `RustStdType` registrations for `core::sync::atomic`.
//!
//! `Atomic<T>` — the generic consolidation that `AtomicBool`/`AtomicI8`/..
//! would otherwise be type aliases of — is itself unstable (`generic_atomic`,
//! rust-lang/rust#130539). On the current stable toolchain, each of these
//! is still its own separately-defined, non-generic, stable type, so each
//! gets its own concrete registration below rather than one generic impl.

use std::sync::atomic::{
    AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize, AtomicPtr, AtomicU8,
    AtomicU16, AtomicU32, AtomicU64, AtomicUsize, Ordering,
};

use crate::rust_std::macros::{
    impl_rust_std_type, impl_rust_std_type_generic1, register_rust_std_standard_evidence,
};

macro_rules! impl_rust_std_atomic {
    ($($ty:ident => $inner:literal),* $(,)?) => {
        $(
            impl_rust_std_type!(
                $ty,
                "core",
                "core::sync::atomic",
                concat!("https://doc.rust-lang.org/core/sync/atomic/struct.", stringify!($ty), ".html"),
                concat!(
                    "The ",
                    stringify!($ty),
                    " carrier provides lock-free, thread-safe access to a ",
                    $inner,
                    " value."
                )
            );
        )*
    };
}

impl_rust_std_atomic!(
    AtomicBool => "bool",
    AtomicI8 => "i8",
    AtomicI16 => "i16",
    AtomicI32 => "i32",
    AtomicI64 => "i64",
    AtomicIsize => "isize",
    AtomicU8 => "u8",
    AtomicU16 => "u16",
    AtomicU32 => "u32",
    AtomicU64 => "u64",
    AtomicUsize => "usize",
);

register_rust_std_standard_evidence!(
    AtomicBool,
    AtomicI8,
    AtomicI16,
    AtomicI32,
    AtomicI64,
    AtomicIsize,
    AtomicU8,
    AtomicU16,
    AtomicU32,
    AtomicU64,
    AtomicUsize,
);

impl_rust_std_type_generic1!(
    AtomicPtr,
    "core",
    "core::sync::atomic",
    "https://doc.rust-lang.org/core/sync/atomic/struct.AtomicPtr.html",
    "The AtomicPtr carrier provides lock-free, thread-safe access to a raw pointer value."
);

register_rust_std_standard_evidence!(AtomicPtr<i32>);

impl_rust_std_type!(
    Ordering,
    "core",
    "core::sync::atomic",
    "https://doc.rust-lang.org/core/sync/atomic/enum.Ordering.html",
    "The Ordering carrier specifies how memory operations around an atomic access are synchronized across threads."
);

register_rust_std_standard_evidence!(Ordering);
