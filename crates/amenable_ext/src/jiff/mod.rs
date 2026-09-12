//! `ExtType` registrations for [`jiff`], split into one module per real
//! `jiff` area, mirroring `amenable_std::rust_std`'s own per-source-area
//! split.
//!
//! Nothing here is `pub use`d: `impl_ext_type!` implements `ExtType`
//! directly on the foreign `jiff` type, so there is no new local type to
//! export — the same shape `amenable_std::rust_std`'s own per-type
//! registration files follow.

mod timestamp;
