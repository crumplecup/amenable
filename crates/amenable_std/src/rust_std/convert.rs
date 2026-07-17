//! `RustStdType` registrations for `core::convert`.

use std::convert::Infallible;

use crate::rust_std::macros::{impl_rust_std_type, register_rust_std_standard_evidence};

impl_rust_std_type!(
    Infallible,
    "core",
    "core::convert",
    "https://doc.rust-lang.org/core/convert/enum.Infallible.html",
    "The Infallible carrier is uninhabited, marking a conversion or error slot that can never be constructed."
);

register_rust_std_standard_evidence!(Infallible);
