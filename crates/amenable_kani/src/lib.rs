//! Kani verifier backend for the `amenable` constitutional trait family.
//!
//! `KaniVerifier` is defined *here*, not in `amenable_core` — there is only
//! one verifier Kani works with, Kani, so the marker belongs with the crate
//! that means it. That locality is what makes
//! `impl amenable_core::Witness<KaniVerifier> for amenable_std::RustStdStandard<T>`
//! legal under Rust's orphan rule, one concrete type at a time: the rule
//! requires *some* type in `Witness<KaniVerifier>`'s type list to be local,
//! and `KaniVerifier` now is. A blanket impl over a bare type parameter
//! still isn't legal (the parameter itself is never "covered"), which is
//! why each type gets its own [`KaniWitness`] impl plus a one-line
//! mechanical bridge, rather than one generic impl for all of them — see
//! `rust_std.rs`.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

extern crate self as amenable_kani;

mod calculator;
mod compose;
mod fd_model;
mod gallery;
mod pipe_model;
mod registry;
mod rust_std;
mod stoplight;
mod witness;

pub use calculator::{AddEvidence, AddToken, CalculationProof, Credit, Debit, Sum, add};
pub use compose::KaniCompose;
pub use fd_model::{KaniBorrowedFd, KaniFd, KaniFile};
pub use pipe_model::{KaniPipe, KaniPipeReader, KaniPipeWriter};
pub use registry::{
    KaniGalleryCase, KaniGalleryDisposition, KaniGalleryExpectation, KaniGalleryRegistration,
    KaniProof, KaniProofRegistration,
};
pub use rust_std::CheckedProof;
pub use stoplight::{
    Color, Green, GreenToken, Red, RedToken, SequentialCycle, Stoplight, Yellow, YellowToken,
};
pub use witness::{KaniVerifier, KaniVerifierMetadata, KaniWitness};
