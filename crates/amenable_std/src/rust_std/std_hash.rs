//! `RustStdType` registrations for `std::hash`.

use std::hash::{DefaultHasher, RandomState};

use crate::rust_std::macros::{impl_rust_std_type, register_rust_std_standard_evidence};

impl_rust_std_type!(
    DefaultHasher,
    "std",
    "std::hash",
    "https://doc.rust-lang.org/std/hash/struct.DefaultHasher.html",
    "The DefaultHasher carrier is std's default, non-cryptographic Hasher implementation."
);

impl_rust_std_type!(
    RandomState,
    "std",
    "std::hash",
    "https://doc.rust-lang.org/std/hash/struct.RandomState.html",
    "The RandomState carrier builds DefaultHasher instances seeded with process-random keys, guarding HashMap against algorithmic-complexity attacks."
);

register_rust_std_standard_evidence!(DefaultHasher, RandomState);
