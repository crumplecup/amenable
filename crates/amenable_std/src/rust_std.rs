//! `RustStdType`: interface and concrete registrations for Rust
//! standard-library carriers, defined together in one crate.
//!
//! `RustStdType` is implemented directly on foreign standard-library types
//! (`bool`, `i32`, `String`, ...), which Rust's orphan rules only permit
//! from the crate that defines the trait. So unlike the core constitutional
//! roles in `amenable`, this trait and its std-lib coverage live together
//! here rather than split across an interface crate and a downstream
//! consumer.

/// Provenance helper for Rust standard-library-backed carriers.
///
/// This trait names the authoritative Rust documentation surface and semantic
/// summary for concrete std or core types used as trusted roots.
pub trait RustStdType: 'static {
    /// The Rust crate that normatively defines the type.
    fn rust_source_crate() -> &'static str;

    /// The Rust module path that normatively defines the type.
    fn rust_source_module() -> &'static str;

    /// The canonical documentation URL for the type.
    fn rust_doc_url() -> &'static str;

    /// Concise summary of the semantic promise made by the standard library.
    fn rust_semantics_summary() -> &'static str;
}

macro_rules! impl_rust_std_primitive {
    ($ty:ty, $module:expr, $url:expr, $summary:expr) => {
        impl RustStdType for $ty {
            fn rust_source_crate() -> &'static str {
                "core"
            }

            fn rust_source_module() -> &'static str {
                $module
            }

            fn rust_doc_url() -> &'static str {
                $url
            }

            fn rust_semantics_summary() -> &'static str {
                $summary
            }
        }
    };
}

impl_rust_std_primitive!(
    bool,
    "core::primitive",
    "https://doc.rust-lang.org/std/primitive.bool.html",
    "The boolean carrier admits exactly the truth values false and true."
);
impl_rust_std_primitive!(
    char,
    "core::primitive",
    "https://doc.rust-lang.org/std/primitive.char.html",
    "The character carrier stores a Unicode scalar value."
);
impl_rust_std_primitive!(
    i8,
    "core::primitive",
    "https://doc.rust-lang.org/std/primitive.i8.html",
    "The signed 8-bit integer carrier stores values in the i8 range defined by Rust."
);
impl_rust_std_primitive!(
    i16,
    "core::primitive",
    "https://doc.rust-lang.org/std/primitive.i16.html",
    "The signed 16-bit integer carrier stores values in the i16 range defined by Rust."
);
impl_rust_std_primitive!(
    i32,
    "core::primitive",
    "https://doc.rust-lang.org/std/primitive.i32.html",
    "The signed 32-bit integer carrier stores values in the i32 range defined by Rust."
);
impl_rust_std_primitive!(
    i64,
    "core::primitive",
    "https://doc.rust-lang.org/std/primitive.i64.html",
    "The signed 64-bit integer carrier stores values in the i64 range defined by Rust."
);
impl_rust_std_primitive!(
    i128,
    "core::primitive",
    "https://doc.rust-lang.org/std/primitive.i128.html",
    "The signed 128-bit integer carrier stores values in the i128 range defined by Rust."
);
impl_rust_std_primitive!(
    isize,
    "core::primitive",
    "https://doc.rust-lang.org/std/primitive.isize.html",
    "The pointer-sized signed integer carrier stores values in the isize range defined by Rust."
);
impl_rust_std_primitive!(
    u8,
    "core::primitive",
    "https://doc.rust-lang.org/std/primitive.u8.html",
    "The unsigned 8-bit integer carrier stores values in the u8 range defined by Rust."
);
impl_rust_std_primitive!(
    u16,
    "core::primitive",
    "https://doc.rust-lang.org/std/primitive.u16.html",
    "The unsigned 16-bit integer carrier stores values in the u16 range defined by Rust."
);
impl_rust_std_primitive!(
    u32,
    "core::primitive",
    "https://doc.rust-lang.org/std/primitive.u32.html",
    "The unsigned 32-bit integer carrier stores values in the u32 range defined by Rust."
);
impl_rust_std_primitive!(
    u64,
    "core::primitive",
    "https://doc.rust-lang.org/std/primitive.u64.html",
    "The unsigned 64-bit integer carrier stores values in the u64 range defined by Rust."
);
impl_rust_std_primitive!(
    u128,
    "core::primitive",
    "https://doc.rust-lang.org/std/primitive.u128.html",
    "The unsigned 128-bit integer carrier stores values in the u128 range defined by Rust."
);
impl_rust_std_primitive!(
    usize,
    "core::primitive",
    "https://doc.rust-lang.org/std/primitive.usize.html",
    "The pointer-sized unsigned integer carrier stores values in the usize range defined by Rust."
);
impl_rust_std_primitive!(
    f32,
    "core::primitive",
    "https://doc.rust-lang.org/std/primitive.f32.html",
    "The 32-bit floating-point carrier follows Rust's f32 semantics."
);
impl_rust_std_primitive!(
    f64,
    "core::primitive",
    "https://doc.rust-lang.org/std/primitive.f64.html",
    "The 64-bit floating-point carrier follows Rust's f64 semantics."
);

impl RustStdType for String {
    fn rust_source_crate() -> &'static str {
        "alloc"
    }

    fn rust_source_module() -> &'static str {
        "alloc::string"
    }

    fn rust_doc_url() -> &'static str {
        "https://doc.rust-lang.org/std/string/struct.String.html"
    }

    fn rust_semantics_summary() -> &'static str {
        "The String carrier stores owned UTF-8 text as defined by Rust's standard library."
    }
}
