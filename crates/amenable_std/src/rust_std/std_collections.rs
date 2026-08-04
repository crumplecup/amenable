//! `RustStdType` registrations for `std::collections` (`HashMap`/`HashSet`).
//!
//! Both cover only the default-hasher case (`HashMap<K, V, RandomState>`),
//! same tradeoff as `Box`'s default-allocator-only coverage (see
//! `rust_std::alloc_boxed`'s doc comment): the generic macro omits the
//! trailing `S` parameter, which resolves to its default (`RandomState`,
//! itself covered in `rust_std::std_hash`) rather than being generic over
//! it.

use std::collections::{HashMap, HashSet};

use crate::rust_std::macros::{
    impl_rust_std_type_generic1, impl_rust_std_type_generic2, register_rust_std_standard_evidence,
};

impl_rust_std_type_generic2!(
    HashMap,
    "std",
    "std::collections",
    "https://doc.rust-lang.org/std/collections/struct.HashMap.html",
    "The HashMap carrier is an unordered key-value map backed by a hash table."
);

impl_rust_std_type_generic1!(
    HashSet,
    "std",
    "std::collections",
    "https://doc.rust-lang.org/std/collections/struct.HashSet.html",
    "The HashSet carrier is an unordered set backed by a hash table."
);

register_rust_std_standard_evidence!(HashMap<i32, i32>, HashSet<i32>);
