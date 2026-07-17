//! Creusot verifier backend for the `amenable` constitutional trait family.
//!
//! `CreusotVerifier` is defined *here*, not in `amenable_core` — there is
//! only one verifier Creusot works with, Creusot, so the marker belongs
//! with the crate that means it. That locality is what makes
//! `impl amenable_core::Witness<CreusotVerifier> for amenable_std::RustStdStandard<T>`
//! legal under Rust's orphan rule, one concrete type at a time: the rule
//! requires *some* type in `Witness<CreusotVerifier>`'s type list to be
//! local, and `CreusotVerifier` now is. A blanket impl over a bare type
//! parameter still isn't legal (the parameter itself is never "covered"),
//! which is why each type gets its own [`CreusotWitness`] impl plus a
//! one-line mechanical bridge, rather than one generic impl for all of
//! them — see `rust_std.rs`.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod rust_std;
mod witness;

pub use rust_std::CheckedProof;
pub use witness::{CreusotVerifier, CreusotVerifierMetadata, CreusotWitness};
