//! The Unicode-scalar-value validity bound, named once instead of restated
//! per verifier.
//!
//! "This `u32` is at most `U+10FFFF` and outside the surrogate range
//! `0xD800..=0xDFFF`" is independently hand-written in Kani
//! (`amenable_kani::rust_std::char`), Creusot
//! (`amenable_creusot::rust_std::VERIFY_CHAR_ROUNDTRIP_SRC`), and Verus
//! (`amenable_verus::rust_std::char_carrier`/`char_try_from_carrier`), using
//! three different but equivalent phrasings, with nothing tying them
//! together. `ValidUnicodeScalar` is the first contract type in the
//! `amenable_core::Ensures`/`Requires` worklist: it doesn't add a new proof,
//! it names an existing one, once, so a future correction to this bound has
//! a single home to start from instead of at least five hand-restated
//! sites.

use amenable_derive::Standard;

use crate::{RustStdProvenance, RustStdStandard, RustStdType};

/// A `u32` known to be a valid Unicode scalar value: at most `U+10FFFF`,
/// excluding the surrogate range `0xD800..=0xDFFF`.
///
/// A derived claim about `char`, not a fresh root authority — its evidence
/// chain rests on `char`'s own already-registered standard-library
/// provenance ([`RustStdStandard<char>`]), the same basis
/// `char::try_from(u32)`'s real proofs already stand on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Standard)]
#[standard(
    basis = "RustStdStandard<char>",
    basis_ctor = "RustStdStandard::<char>::new()",
    provenance = "<char as RustStdType>::provenance()",
    provenance_type = "RustStdProvenance"
)]
pub struct ValidUnicodeScalar {
    value: u32,
}

impl ValidUnicodeScalar {
    /// Wrap a `u32` already known to be a valid Unicode scalar value.
    pub const fn new(value: u32) -> Self {
        Self { value }
    }

    /// The wrapped code point.
    pub const fn value(&self) -> u32 {
        self.value
    }
}
