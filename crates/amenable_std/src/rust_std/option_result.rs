//! `RustStdType` registrations for `core::option` and `core::result`.

use std::option::{IntoIter as OptionIntoIter, Iter as OptionIter, IterMut as OptionIterMut};
use std::result::{IntoIter as ResultIntoIter, Iter as ResultIter, IterMut as ResultIterMut};

use crate::rust_std::macros::{
    impl_rust_std_type_generic1, impl_rust_std_type_generic2, impl_rust_std_type_lifetime1,
    register_rust_std_standard_evidence,
};

impl_rust_std_type_generic1!(
    Option,
    "core",
    "core::option",
    "https://doc.rust-lang.org/core/option/enum.Option.html",
    "The Option carrier represents an optional value: Some(value) or None."
);

impl_rust_std_type_generic2!(
    Result,
    "core",
    "core::result",
    "https://doc.rust-lang.org/core/result/enum.Result.html",
    "The Result carrier represents either success (Ok) or failure (Err)."
);

impl_rust_std_type_generic1!(
    OptionIntoIter,
    "core",
    "core::option",
    "https://doc.rust-lang.org/core/option/struct.IntoIter.html",
    "The IntoIter carrier consumes an Option, yielding zero or one owned value."
);

impl_rust_std_type_lifetime1!(
    OptionIter,
    "core",
    "core::option",
    "https://doc.rust-lang.org/core/option/struct.Iter.html",
    "The Iter carrier borrows an Option, yielding zero or one shared reference."
);

impl_rust_std_type_lifetime1!(
    OptionIterMut,
    "core",
    "core::option",
    "https://doc.rust-lang.org/core/option/struct.IterMut.html",
    "The IterMut carrier mutably borrows an Option, yielding zero or one mutable reference."
);

impl_rust_std_type_generic1!(
    ResultIntoIter,
    "core",
    "core::result",
    "https://doc.rust-lang.org/core/result/struct.IntoIter.html",
    "The IntoIter carrier consumes a Result, yielding zero or one owned Ok value."
);

impl_rust_std_type_lifetime1!(
    ResultIter,
    "core",
    "core::result",
    "https://doc.rust-lang.org/core/result/struct.Iter.html",
    "The Iter carrier borrows a Result, yielding zero or one shared reference to its Ok value."
);

impl_rust_std_type_lifetime1!(
    ResultIterMut,
    "core",
    "core::result",
    "https://doc.rust-lang.org/core/result/struct.IterMut.html",
    "The IterMut carrier mutably borrows a Result, yielding zero or one mutable reference to its Ok value."
);

// `i32` (and `'static` for the borrowing iterators) is the representative
// element type/lifetime this module's proof batch covers. The iterator
// carriers are written fully-qualified: their bare names (`IntoIter`,
// `Iter`, `IterMut`) recur across many other types with the exact same
// spelling, so only the qualified path disambiguates which carrier a
// given registration means for tooling reading the registry (e.g.
// `elicit_doc`'s coverage report).
register_rust_std_standard_evidence!(
    Option<i32>,
    Result<i32, i32>,
    core::option::IntoIter<i32>,
    core::option::Iter<'static, i32>,
    core::option::IterMut<'static, i32>,
    core::result::IntoIter<i32>,
    core::result::Iter<'static, i32>,
    core::result::IterMut<'static, i32>,
);
