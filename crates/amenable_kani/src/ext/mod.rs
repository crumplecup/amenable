//! `Witness<KaniVerifier>` registrations for `amenable_ext`'s third-party
//! carriers, one module per target crate.
//!
//! A new sibling to `rust_std`, not nested under it: `rust_std` names
//! the Rust standard library specifically, and jiff/chrono/etc. are not
//! that (Open decision 2 in `docs/AMENABLE_EXT_PLAN.md`).

// Shared by every target module's registrations; gated the same way
// `amenable_ext`'s own shared macros are -- with no target feature
// active there is nothing left to consume it. Extend to `any(feature =
// "jiff", feature = "chrono", ...)` once a second target lands.
#[cfg(feature = "jiff")]
mod macros;

#[cfg(feature = "jiff")]
mod jiff;
