//! `RustStdType` registrations for Rust's scalar primitives and `String`.

use crate::rust_std::macros::register_rust_std_standard_evidence;
use crate::{RustLanguageProvenance, RustStdType};

macro_rules! impl_rust_std_primitive {
    ($ty:ty, $url:expr, $summary:expr) => {
        impl RustStdType for $ty {
            fn rust_doc_url() -> &'static str {
                $url
            }

            fn rust_semantics_summary() -> &'static str {
                $summary
            }
        }
    };
}

macro_rules! impl_rust_std_fixed_width_integer {
    ($(($ty:ty, $signedness:literal, $bits:literal)),* $(,)?) => {
        $(
            impl_rust_std_primitive!(
                $ty,
                concat!("https://doc.rust-lang.org/std/primitive.", stringify!($ty), ".html"),
                concat!(
                    "The ",
                    $signedness,
                    " ",
                    stringify!($bits),
                    "-bit integer carrier stores values in the ",
                    stringify!($ty),
                    " range defined by Rust."
                )
            );
        )*
    };
}

impl_rust_std_primitive!(
    bool,
    "https://doc.rust-lang.org/std/primitive.bool.html",
    "The boolean carrier admits exactly the truth values false and true."
);
impl_rust_std_primitive!(
    char,
    "https://doc.rust-lang.org/std/primitive.char.html",
    "The character carrier stores a Unicode scalar value."
);
impl_rust_std_fixed_width_integer!(
    (i8, "signed", 8),
    (i16, "signed", 16),
    (i32, "signed", 32),
    (i64, "signed", 64),
    (i128, "signed", 128),
    (u8, "unsigned", 8),
    (u16, "unsigned", 16),
    (u32, "unsigned", 32),
    (u64, "unsigned", 64),
    (u128, "unsigned", 128),
);
impl_rust_std_primitive!(
    isize,
    "https://doc.rust-lang.org/std/primitive.isize.html",
    "The pointer-sized signed integer carrier stores values in the isize range defined by Rust."
);
impl_rust_std_primitive!(
    usize,
    "https://doc.rust-lang.org/std/primitive.usize.html",
    "The pointer-sized unsigned integer carrier stores values in the usize range defined by Rust."
);
impl_rust_std_primitive!(
    f32,
    "https://doc.rust-lang.org/std/primitive.f32.html",
    "The 32-bit floating-point carrier follows Rust's f32 semantics."
);
impl_rust_std_primitive!(
    f64,
    "https://doc.rust-lang.org/std/primitive.f64.html",
    "The 64-bit floating-point carrier follows Rust's f64 semantics."
);

impl RustStdType for String {
    fn rust_language_provenance() -> RustLanguageProvenance {
        RustLanguageProvenance::alloc_string()
    }

    fn rust_doc_url() -> &'static str {
        "https://doc.rust-lang.org/std/string/struct.String.html"
    }

    fn rust_semantics_summary() -> &'static str {
        "The String carrier stores owned UTF-8 text as defined by Rust's standard library."
    }
}

// `array`/`fn`/`pointer`/`reference`/`tuple`/`unit` are compound primitives
// (rustdoc documents them the same way as `bool`/`char`/the integers: one
// `ItemKind::Primitive` doc page each, under both `core` and `std`). `str`
// and `slice` share this same primitive-page shape but are unsized (`str`,
// `[T]`) — deliberately not covered here pending a decision on what a
// Kani proof for an unsized type even means, since a value can only exist
// behind a reference (`&str`/`&[T]`, both already covered as their own
// distinct carriers elsewhere).
impl_rust_std_primitive!(
    [i32; 3],
    "https://doc.rust-lang.org/std/primitive.array.html",
    "The array carrier stores a fixed number of elements of one type, with the length fixed at compile time."
);
impl_rust_std_primitive!(
    fn(i32) -> i32,
    "https://doc.rust-lang.org/std/primitive.fn.html",
    "The fn pointer carrier holds the address of a plain (non-capturing) function with a fixed signature."
);
impl_rust_std_primitive!(
    *const i32,
    "https://doc.rust-lang.org/std/primitive.pointer.html",
    "The raw pointer carrier holds an address with no ownership, borrowing, or validity guarantees enforced by the compiler."
);
impl_rust_std_primitive!(
    *mut i32,
    "https://doc.rust-lang.org/std/primitive.pointer.html",
    "The raw pointer carrier holds an address with no ownership, borrowing, or validity guarantees enforced by the compiler."
);
impl_rust_std_primitive!(
    &'static i32,
    "https://doc.rust-lang.org/std/primitive.reference.html",
    "The reference carrier borrows a value, guaranteeing it stays valid and (for a shared reference) unchanged for the reference's lifetime."
);
impl_rust_std_primitive!(
    &'static mut i32,
    "https://doc.rust-lang.org/std/primitive.reference.html",
    "The reference carrier borrows a value, guaranteeing it stays valid and (for a mutable reference) exclusively accessible for the reference's lifetime."
);
impl_rust_std_primitive!(
    (i32, i32),
    "https://doc.rust-lang.org/std/primitive.tuple.html",
    "The tuple carrier groups a fixed number of possibly-different-typed values into one compound value."
);
impl_rust_std_primitive!(
    (),
    "https://doc.rust-lang.org/std/primitive.unit.html",
    "The unit carrier is the zero-element tuple, holding no data -- Rust's answer to \"no meaningful value\"."
);

register_rust_std_standard_evidence!(
    bool,
    char,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    f32,
    f64,
    String,
    [i32; 3],
    fn(i32) -> i32,
    *const i32,
    *mut i32,
    &'static i32,
    &'static mut i32,
    (i32, i32),
    (),
);
