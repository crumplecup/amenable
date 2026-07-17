//! `RustStdType` registrations for `core::num`.
//!
//! `NonZero<T>` cannot get a single blanket `impl<T> RustStdType for
//! NonZero<T>`: it's actually `NonZero<T: ZeroablePrimitive>`, and
//! `ZeroablePrimitive` is a sealed trait not nameable outside `core` — so
//! each concrete instantiation is registered individually below. Each one
//! also covers the corresponding alias (`NonZero<i8>` covers `NonZeroI8`,
//! etc.), since an alias resolves to its underlying concrete type at the
//! impl site; there is no separate impl to write for the alias name itself.
//! `Atomic<T>`'s `AtomicBool`/`AtomicI8`/.. aliases work the same way and
//! are covered in the `sync` batch instead, alongside the rest of
//! `core::sync::atomic`.

use std::num::{NonZero, Saturating, Wrapping};

use crate::rust_std::macros::{impl_rust_std_type, impl_rust_std_type_generic1};

macro_rules! impl_rust_std_nonzero {
    ($($ty:ty),* $(,)?) => {
        $(
            impl_rust_std_type!(
                NonZero<$ty>,
                "core",
                "core::num",
                "https://doc.rust-lang.org/core/num/struct.NonZero.html",
                concat!(
                    "The NonZero carrier stores a ",
                    stringify!($ty),
                    " value statically known to never equal zero."
                )
            );
        )*
    };
}

impl_rust_std_nonzero!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);

impl_rust_std_type_generic1!(
    Wrapping,
    "core",
    "core::num",
    "https://doc.rust-lang.org/core/num/struct.Wrapping.html",
    "The Wrapping carrier wraps an integer so its arithmetic operators wrap around on overflow instead of panicking or trapping."
);

impl_rust_std_type_generic1!(
    Saturating,
    "core",
    "core::num",
    "https://doc.rust-lang.org/core/num/struct.Saturating.html",
    "The Saturating carrier wraps an integer so its arithmetic operators saturate at the numeric bounds instead of overflowing."
);

impl_rust_std_type!(
    core::num::FpCategory,
    "core",
    "core::num",
    "https://doc.rust-lang.org/core/num/enum.FpCategory.html",
    "The FpCategory carrier classifies a floating-point value as Nan, Infinite, Zero, Subnormal, or Normal."
);

impl_rust_std_type!(
    core::num::IntErrorKind,
    "core",
    "core::num",
    "https://doc.rust-lang.org/core/num/enum.IntErrorKind.html",
    "The IntErrorKind carrier names the specific reason an integer parse failed."
);

impl_rust_std_type!(
    core::num::ParseIntError,
    "core",
    "core::num",
    "https://doc.rust-lang.org/core/num/struct.ParseIntError.html",
    "The ParseIntError carrier reports that a string could not be parsed as an integer."
);

impl_rust_std_type!(
    core::num::ParseFloatError,
    "core",
    "core::num",
    "https://doc.rust-lang.org/core/num/struct.ParseFloatError.html",
    "The ParseFloatError carrier reports that a string could not be parsed as a floating-point number."
);

impl_rust_std_type!(
    core::num::TryFromIntError,
    "core",
    "core::num",
    "https://doc.rust-lang.org/core/num/struct.TryFromIntError.html",
    "The TryFromIntError carrier reports that a fallible integer conversion did not fit the target type."
);
