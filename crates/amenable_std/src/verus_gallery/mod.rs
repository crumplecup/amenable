//! Verus proof gallery: documented findings about the real `verus`
//! toolchain's behavior, discovered while building `amenable_verus`'s real
//! proof pipeline. Mirrors `amenable_std::creusot_gallery`: each `claim`
//! is a plain string constant holding a reduced repro, hand-verified once
//! against `just verify-verus` and recorded as a fact.
//!
//! Split into `infra` (the `VerusGalleryDisposition`/`Expectation`/`Case`/
//! `Registration` types) and per-theme case files: `numeric_cases`,
//! `spec_cases`, `binder_cases`.

mod binder_cases;
mod infra;
mod numeric_cases;
mod spec_cases;

pub use infra::{
    VerusGalleryCase, VerusGalleryDisposition, VerusGalleryExpectation, VerusGalleryRegistration,
};
