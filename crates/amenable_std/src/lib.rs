//! `RustStdType`: interface and concrete registrations for Rust
//! standard-library types.
//!
//! Traits meant to be implemented directly on foreign standard-library types
//! must live in the crate that defines them — Rust's orphan rules leave no
//! other option, since neither the trait nor `bool`/`i32`/`String`/etc. is
//! local anywhere else. So rather than an interface crate plus a downstream
//! consumer, this crate defines `RustStdType` and its full std-lib coverage
//! together, alongside the default concrete certificate and registry types,
//! serving as the canonical gold-standard registrations other crates can
//! depend on instead of re-registering the same std types themselves.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod cert;
#[cfg(feature = "creusot")]
mod creusot_gallery;
#[cfg(feature = "creusot")]
mod creusot_witness;
mod rust_std;
mod valid_unicode_scalar;
#[cfg(feature = "verus")]
mod verus_gallery;
#[cfg(feature = "verus")]
mod verus_witness;

pub use cert::{CertId, CertRegistry, ProvenanceCertificate};
#[cfg(feature = "creusot")]
pub use creusot_gallery::{
    CreusotGalleryCase, CreusotGalleryDisposition, CreusotGalleryExpectation,
    CreusotGalleryRegistration,
};
#[cfg(feature = "creusot")]
pub use creusot_witness::CheckedProof;
pub use rust_std::{
    RustLanguageProvenance, RustStdProvenance, RustStdStandard, RustStdType,
    write_rust_std_certificate_artifacts,
};
pub use valid_unicode_scalar::ValidUnicodeScalar;
#[cfg(feature = "verus")]
pub use verus_gallery::{
    VerusGalleryCase, VerusGalleryDisposition, VerusGalleryExpectation, VerusGalleryRegistration,
};
#[cfg(feature = "verus")]
pub use verus_witness::{VerusCheckedProof, VerusVerifier, VerusVerifierMetadata, VerusWitness};
