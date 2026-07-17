//! Verus verifier backend for the `amenable` constitutional trait family.
//!
//! `VerusVerifier` is defined *here*, not in `amenable_core` — there is
//! only one verifier Verus works with, Verus, so the marker belongs with
//! the crate that means it. That locality is what makes
//! `impl amenable_core::Witness<VerusVerifier> for amenable_std::RustStdStandard<T>`
//! legal under Rust's orphan rule, one concrete type at a time: the rule
//! requires *some* type in `Witness<VerusVerifier>`'s type list to be
//! local, and `VerusVerifier` now is. A blanket impl over a bare type
//! parameter still isn't legal (the parameter itself is never "covered"),
//! which is why each type gets its own [`VerusWitness`] impl plus a
//! one-line mechanical bridge, rather than one generic impl for all of
//! them — see `rust_std.rs`.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod rust_std;
mod witness;

pub use rust_std::CheckedProof;
pub use witness::{VerusVerifier, VerusVerifierMetadata, VerusWitness};
