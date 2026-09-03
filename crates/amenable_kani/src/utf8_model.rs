//! Kani-only accommodation model for focused owned UTF-8 conversion laws.
//!
//! This module is where Amenable stops asking Kani to execute the direct owned
//! `String::from_utf8` / `FromUtf8Error` std path and instead proves against a
//! small package of explicit bounded UTF-8 and byte-recovery laws that the
//! real implementation is expected to refine.
//!
//! The direct std timeout path remains preserved in the proof gallery as a
//! false trail. Production proofs that use this model are therefore
//! conditional:
//!
//! - if the real owned UTF-8 conversion path conforms to these laws,
//! - then the modeled Kani proof carries the intended Rust-facing claim.

use amenable_core::{MetadataEntry, Provenance};
use amenable_derive::Standard;

mod buffer;
#[cfg(kani)]
mod mirror;
mod validation;

pub use buffer::{KaniUtf8Buffer, KaniUtf8BufferError, KaniUtf8BufferToken};
pub use validation::KaniUtf8PositionError;

use validation::is_valid_utf8;

const MAX_KANI_UTF8_BYTES: usize = 4;

/// The root assumption `KaniUtf8Buffer` rests on: under Kani, a byte
/// sequence's UTF-8 validity is asserted symbolically rather than computed
/// by running the real validation algorithm. Naming this as an explicit
/// `Standard` turns the "if the real path conforms" sentence in this
/// module's own doc comment into an auditable `Provenance` record instead
/// of prose -- `KaniUtf8Buffer`, and everything built on it, rests on this
/// assumption, not on a machine-checked fact about the real algorithm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Standard)]
#[standard(basis = "Self")]
pub struct KaniAssumedUtf8Validity {
    valid: bool,
}

impl KaniAssumedUtf8Validity {
    /// Decide validity for the given bytes: assumed symbolically
    /// (`kani::any()`) under Kani, computed for real (`is_valid_utf8`)
    /// otherwise -- the same split `KaniUtf8Buffer::new` used inline
    /// before this assumption had a name. Two `#[cfg]`-gated definitions,
    /// not one with a single shared parameter name, since `bytes` is
    /// genuinely read under `not(kani)` but genuinely unused under
    /// `kani` -- no single name is honest for both.
    #[cfg(not(kani))]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(bytes)))]
    #[must_use]
    pub fn decide(bytes: &[u8]) -> Self {
        let valid = is_valid_utf8(bytes);
        Self { valid }
    }

    /// A canonical "assumed valid" instance, used where the type -- not a
    /// specific decided value -- is what matters: `Evidence::basis`'s
    /// static claim, or a downstream `Establish` call whose credential is
    /// a compile-time formality (see `Establish`'s own doc: establishing a
    /// token never invokes a verifier) rather than runtime-inspected data.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
    #[must_use]
    pub fn asserted_valid() -> Self {
        Self { valid: true }
    }

    /// Report the assumed/computed validity.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn holds(&self) -> bool {
        self.valid
    }
}

impl Provenance for KaniAssumedUtf8Validity {
    type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn metadata(&self) -> Self::MetadataIter {
        Box::new({
            vec![
                MetadataEntry::new(
                    "assumed",
                    "UTF-8 validity, standing in for the real std::str::from_utf8 algorithm",
                ),
                MetadataEntry::new(
                    "rationale",
                    "the real validation algorithm times out under Kani even for two fully-valid bytes -- see gallery::utf8_validation_algorithm_cost",
                ),
                MetadataEntry::new("valid", self.valid.to_string()),
            ]
            .into_iter()
        })
    }
}

/// Modeled owned valid UTF-8 bytes.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KaniUtf8String(Vec<u8>);

/// Modeled owned UTF-8 conversion error that preserves the original bytes.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KaniFromUtf8Error(Vec<u8>);

/// Namespace for focused owned UTF-8 conversion laws.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KaniUtf8;

impl KaniUtf8 {
    /// Classify one owned byte vector as valid UTF-8 or an owned recovery error.
    pub fn classify_owned(bytes: Vec<u8>) -> Result<KaniUtf8String, KaniFromUtf8Error> {
        assert!(
            bytes.len() <= MAX_KANI_UTF8_BYTES,
            "KaniUtf8 models at most {MAX_KANI_UTF8_BYTES} bytes"
        );

        if is_valid_utf8(&bytes) {
            Ok(KaniUtf8String(bytes))
        } else {
            Err(KaniFromUtf8Error(bytes))
        }
    }

    /// Report whether the bounded byte slice is valid UTF-8.
    pub fn is_valid(bytes: &[u8]) -> bool {
        assert!(
            bytes.len() <= MAX_KANI_UTF8_BYTES,
            "KaniUtf8 models at most {MAX_KANI_UTF8_BYTES} bytes"
        );
        is_valid_utf8(bytes)
    }
}

impl KaniUtf8String {
    /// Borrow the modeled valid UTF-8 bytes.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Consume the modeled valid UTF-8 bytes.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    pub fn into_bytes(self) -> Vec<u8> {
        self.0
    }

    /// Report the byte length.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Report whether the modeled string is empty.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Borrow the modeled valid UTF-8 content as `&str`.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.0).expect("KaniUtf8String stores validated UTF-8")
    }
}

impl KaniFromUtf8Error {
    /// Borrow the original invalid owned bytes.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Consume the error and recover the original owned bytes.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    pub fn into_bytes(self) -> Vec<u8> {
        self.0
    }
}
