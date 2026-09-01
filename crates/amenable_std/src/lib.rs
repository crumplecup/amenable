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

mod argv_includes_program_path;
mod array_into_iter_advance_matches_position;
mod array_into_iter_starts_at_first_position;
mod ascii_byte;
mod cert;
mod compose_any_length_is_bounded;
mod compose_array_length_is_fixed;
mod compose_depth_zero_is_empty;
mod compose_field_presence_tracks_depth;
mod creusot_gallery;
mod drains_two_values_in_order_and_empties;
mod error;
mod increment_headroom;
mod indexing_and_length;
mod iter_yields_value_once_then_ends;
mod non_nul_byte;
mod nul_only_at_the_end_validates;
mod observed_option_matches_input;
mod observed_pair_matches_input;
mod observed_value_matches_input;
mod rust_std;
mod valid_unicode_scalar;
mod value_unchanged;
#[cfg(feature = "verus")]
mod verus_call_shape_derive;
#[cfg(feature = "verus")]
mod verus_derive_canary;
#[cfg(feature = "verus")]
mod verus_gallery;
#[cfg(feature = "verus")]
mod verus_witness;
mod windows_handle_or_invalid_rejects_only_the_sentinel;
mod write_stores_new_value;
mod yields_three_values_in_order_then_ends;
mod yields_two_values_in_order_then_ends;

pub use argv_includes_program_path::ArgvIncludesProgramPath;
pub use array_into_iter_advance_matches_position::ArrayIntoIterAdvanceMatchesPosition;
pub use array_into_iter_starts_at_first_position::ArrayIntoIterStartsAtFirstPosition;
pub use ascii_byte::AsciiByte;
pub use cert::{CertId, CertRegistry, ProvenanceCertificate};
pub use compose_any_length_is_bounded::ComposeAnyLengthIsBounded;
pub use compose_array_length_is_fixed::ComposeArrayLengthIsFixed;
pub use compose_depth_zero_is_empty::ComposeDepthZeroIsEmpty;
pub use compose_field_presence_tracks_depth::ComposeFieldPresenceTracksDepth;
pub use creusot_gallery::{
    CreusotGalleryCase, CreusotGalleryDisposition, CreusotGalleryExpectation,
    CreusotGalleryRegistration,
};
pub use drains_two_values_in_order_and_empties::DrainsTwoValuesInOrderAndEmpties;
pub use error::{AmenableStdError, AmenableStdErrorKind, AmenableStdResult};
pub use increment_headroom::IncrementHeadroom;
pub use indexing_and_length::IndexingAndLength;
pub use iter_yields_value_once_then_ends::IterYieldsValueOnceThenEnds;
pub use non_nul_byte::NonNulByte;
pub use nul_only_at_the_end_validates::NulOnlyAtTheEndValidates;
pub use observed_option_matches_input::ObservedOptionMatchesInput;
pub use observed_pair_matches_input::ObservedPairMatchesInput;
pub use observed_value_matches_input::ObservedValueMatchesInput;
pub use rust_std::{
    RustLanguageProvenance, RustStdProvenance, RustStdStandard, RustStdType,
    write_rust_std_certificate_artifacts,
};
pub use valid_unicode_scalar::ValidUnicodeScalar;
pub use value_unchanged::ValueUnchanged;
#[cfg(feature = "verus")]
pub use verus_derive_canary::{
    CheckedVerusExportLeaf, RawTemplateVerusExportLeaf, RequiresVerusExportLeaf,
    TrustedVerusExportLeaf, VerusExportCanaryEnum, VerusExportMultiCheckedEnum,
};
#[cfg(feature = "verus")]
pub use verus_gallery::{
    VerusGalleryCase, VerusGalleryDisposition, VerusGalleryExpectation, VerusGalleryRegistration,
};
#[cfg(feature = "verus")]
pub use verus_witness::{
    VerusCallKind, VerusCallShape, VerusCallShapeRecord, VerusCheckedProof, VerusImport,
    VerusParam, VerusVerifier, VerusVerifierMetadata, VerusWitness, verus_call_shape,
};
pub use windows_handle_or_invalid_rejects_only_the_sentinel::WindowsHandleOrInvalidRejectsOnlyTheSentinel;
pub use write_stores_new_value::WriteStoresNewValue;
pub use yields_three_values_in_order_then_ends::YieldsThreeValuesInOrderThenEnds;
pub use yields_two_values_in_order_then_ends::YieldsTwoValuesInOrderThenEnds;
