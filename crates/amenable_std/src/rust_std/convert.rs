//! `RustStdType` registrations for `core::convert`.

use crate::rust_std::macros::impl_rust_std_type;

impl_rust_std_type!(
    core::convert::Infallible,
    "core",
    "core::convert",
    "https://doc.rust-lang.org/core/convert/enum.Infallible.html",
    "The Infallible carrier is uninhabited, marking a conversion or error slot that can never be constructed."
);
