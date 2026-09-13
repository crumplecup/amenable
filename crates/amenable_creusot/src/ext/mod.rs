//! `Witness<CreusotVerifier>` registrations for `amenable_ext`'s
//! third-party carriers, one module per target crate.
//!
//! A new sibling to `rust_std_witness`, not nested under it — same
//! reasoning as `amenable_kani::ext`'s own doc comment (Open decision 2
//! in `docs/AMENABLE_EXT_PLAN.md`): `rust_std`/`rust_std_witness` name
//! the Rust standard library specifically, and jiff/chrono/etc. are not
//! that.

#[cfg(feature = "jiff")]
mod jiff;
