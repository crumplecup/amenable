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

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::{Establish, Evidence, MetadataEntry, ProofToken, Provenance, Witness};
use amenable_derive::Standard;

use crate::KaniCompose;
use crate::compose::{kani_assume, symbolic_any};
use crate::rust_std::macros::kani_ensures;
use crate::{CalculationProof, KaniVerifier};

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
    /// before this assumption had a name.
    #[must_use]
    pub fn decide(_bytes: &[u8]) -> Self {
        #[cfg(kani)]
        let valid: bool = kani::any();
        #[cfg(not(kani))]
        let valid = is_valid_utf8(_bytes);

        Self { valid }
    }

    /// A canonical "assumed valid" instance, used where the type -- not a
    /// specific decided value -- is what matters: `Evidence::basis`'s
    /// static claim, or a downstream `Establish` call whose credential is
    /// a compile-time formality (see `Establish`'s own doc: establishing a
    /// token never invokes a verifier) rather than runtime-inspected data.
    #[must_use]
    pub fn asserted_valid() -> Self {
        Self { valid: true }
    }

    /// Report the assumed/computed validity.
    #[must_use]
    pub fn holds(&self) -> bool {
        self.valid
    }
}

impl Provenance for KaniAssumedUtf8Validity {
    type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;

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
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Consume the modeled valid UTF-8 bytes.
    pub fn into_bytes(self) -> Vec<u8> {
        self.0
    }

    /// Report the byte length.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Report whether the modeled string is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Borrow the modeled valid UTF-8 content as `&str`.
    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.0).expect("KaniUtf8String stores validated UTF-8")
    }
}

impl KaniFromUtf8Error {
    /// Borrow the original invalid owned bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Consume the error and recover the original owned bytes.
    pub fn into_bytes(self) -> Vec<u8> {
        self.0
    }
}

impl KaniCompose for KaniUtf8String {
    fn kani_depth0() -> Self {
        Self(Vec::new())
    }

    fn kani_depth1() -> Self {
        Self(vec![symbolic_ascii_byte()])
    }

    fn kani_depth2() -> Self {
        Self(vec![symbolic_ascii_byte(), symbolic_ascii_byte()])
    }

    fn kani_any() -> Self {
        let len: usize = symbolic_any();
        kani_assume(len <= MAX_KANI_UTF8_BYTES);
        let mut bytes = Vec::new();
        for _ in 0..len {
            bytes.push(symbolic_ascii_byte());
        }
        Self(bytes)
    }
}

impl KaniCompose for KaniFromUtf8Error {
    fn kani_depth0() -> Self {
        Self(vec![0xFFu8])
    }

    fn kani_depth1() -> Self {
        Self(vec![b'x', 0xFFu8])
    }

    fn kani_depth2() -> Self {
        Self(vec![b'x', b'y', 0xFFu8])
    }

    fn kani_any() -> Self {
        let prefix_len: usize = symbolic_any();
        kani_assume(prefix_len < MAX_KANI_UTF8_BYTES);
        let mut bytes = Vec::new();
        for _ in 0..prefix_len {
            bytes.push(symbolic_ascii_byte());
        }
        bytes.push(0xFFu8);
        Self(bytes)
    }
}

/// Modeled UTF-8 validation error exposing the same position information as
/// `std::str::Utf8Error`: how many leading bytes were valid, and the byte
/// width of the invalid sequence at that position (when determinable
/// without more input).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KaniUtf8PositionError {
    valid_up_to: usize,
    error_len: Option<usize>,
}

impl KaniUtf8PositionError {
    /// The number of leading bytes that were valid UTF-8.
    pub fn valid_up_to(&self) -> usize {
        self.valid_up_to
    }

    /// The byte width of the invalid sequence at `valid_up_to`, or `None`
    /// if more input would be needed to determine it (an incomplete
    /// trailing multi-byte sequence).
    pub fn error_len(&self) -> Option<usize> {
        self.error_len
    }
}

impl KaniUtf8 {
    /// Validate a bounded byte slice, reporting the same `valid_up_to` /
    /// `error_len` position information as `std::str::from_utf8`'s error on
    /// the first invalid byte found.
    ///
    /// # Errors
    ///
    /// Returns `Err(KaniUtf8PositionError)` when the bytes are not valid
    /// UTF-8.
    pub fn error_position(bytes: &[u8]) -> Result<(), KaniUtf8PositionError> {
        assert!(
            bytes.len() <= MAX_KANI_UTF8_BYTES,
            "KaniUtf8 models at most {MAX_KANI_UTF8_BYTES} bytes"
        );

        match first_invalid_position(bytes) {
            None => Ok(()),
            Some((valid_up_to, error_len)) => Err(KaniUtf8PositionError {
                valid_up_to,
                error_len,
            }),
        }
    }
}

/// Same byte-classification shape as `is_valid_utf8`, but reports where
/// validation first fails instead of a bare bool.
fn first_invalid_position(bytes: &[u8]) -> Option<(usize, Option<usize>)> {
    let len = bytes.len();
    let mut i = 0;

    while i < len {
        let byte = bytes[i];

        if byte & 0b1000_0000 == 0 {
            i += 1;
            continue;
        }

        if byte & 0b1110_0000 == 0b1100_0000 {
            if byte & 0b0001_1110 == 0 {
                return Some((i, Some(1)));
            }
            if i + 1 >= len {
                return Some((i, None));
            }
            if bytes[i + 1] & 0b1100_0000 != 0b1000_0000 {
                return Some((i, Some(1)));
            }
            i += 2;
            continue;
        }

        if byte & 0b1111_0000 == 0b1110_0000 {
            if i + 1 >= len {
                return Some((i, None));
            }
            if bytes[i + 1] & 0b1100_0000 != 0b1000_0000
                || (byte == 0b1110_0000 && bytes[i + 1] & 0b0010_0000 == 0)
            {
                return Some((i, Some(1)));
            }
            if i + 2 >= len {
                return Some((i, None));
            }
            if bytes[i + 2] & 0b1100_0000 != 0b1000_0000 {
                return Some((i, Some(2)));
            }

            let code_point = ((byte & 0x0F) as u32) << 12
                | ((bytes[i + 1] & 0x3F) as u32) << 6
                | (bytes[i + 2] & 0x3F) as u32;
            if (0xD800..=0xDFFF).contains(&code_point) {
                return Some((i, Some(1)));
            }

            i += 3;
            continue;
        }

        if byte & 0b1111_1000 == 0b1111_0000 {
            if i + 1 >= len {
                return Some((i, None));
            }
            if bytes[i + 1] & 0b1100_0000 != 0b1000_0000
                || (byte == 0b1111_0000 && bytes[i + 1] & 0b0011_0000 == 0)
                || byte > 0b1111_0100
            {
                return Some((i, Some(1)));
            }
            if i + 2 >= len {
                return Some((i, None));
            }
            if bytes[i + 2] & 0b1100_0000 != 0b1000_0000 {
                return Some((i, Some(2)));
            }
            if i + 3 >= len {
                return Some((i, None));
            }
            if bytes[i + 3] & 0b1100_0000 != 0b1000_0000 {
                return Some((i, Some(3)));
            }

            let code_point = ((byte & 0x07) as u32) << 18
                | ((bytes[i + 1] & 0x3F) as u32) << 12
                | ((bytes[i + 2] & 0x3F) as u32) << 6
                | (bytes[i + 3] & 0x3F) as u32;
            if code_point > 0x10FFFF {
                return Some((i, Some(1)));
            }

            i += 4;
            continue;
        }

        return Some((i, Some(1)));
    }

    None
}

/// Modeled UTF-8 validation error for a fixed-capacity buffer: `len`
/// exceeded the buffer's capacity, or the content was not valid UTF-8.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KaniUtf8BufferError {
    /// The requested length exceeded the buffer's `MAX_LEN` capacity.
    TooLong,
    /// The content was not valid UTF-8.
    InvalidUtf8,
}

/// Fixed-capacity, const-generic UTF-8 buffer, following the pattern
/// documented in `elicitation`'s `verification::types::Utf8Bytes<MAX_LEN>`:
/// a `[u8; MAX_LEN]` array (not a `Vec<u8>`) so Kani's unwinder sees a
/// compile-time loop bound, and validity is an *assumed symbolic fact*
/// under Kani rather than a re-run of the validation algorithm or a real
/// `std::str::from_utf8` call -- both were confirmed to time out even for
/// two fixed bytes when every byte is valid (see
/// `gallery::full_traversal_validation`). This proves the wrapper's own
/// invariants (length bookkeeping, byte recovery) conditional on validity,
/// not the validation algorithm itself; `is_valid_utf8` above is exercised
/// directly (outside Kani, and inside Kani only on its early-return/invalid
/// path) for that separate concern.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KaniUtf8Buffer<const MAX_LEN: usize> {
    bytes: [u8; MAX_LEN],
    len: usize,
}

impl<const MAX_LEN: usize> KaniUtf8Buffer<MAX_LEN> {
    /// Construct from a fixed-capacity byte array, validating UTF-8
    /// encoding.
    ///
    /// Under Kani, validity is assumed symbolically (`kani::any()`) rather
    /// than computed, so this models both the valid and invalid
    /// construction paths without the unwinding cost of running a real
    /// validation algorithm to completion. Outside Kani, real validation
    /// applies.
    ///
    /// # Errors
    ///
    /// Returns `KaniUtf8BufferError::TooLong` if `len > MAX_LEN`, or
    /// `KaniUtf8BufferError::InvalidUtf8` if the modeled content is not
    /// valid UTF-8.
    pub fn new(bytes: [u8; MAX_LEN], len: usize) -> Result<Self, KaniUtf8BufferError> {
        if len > MAX_LEN {
            return Err(KaniUtf8BufferError::TooLong);
        }

        if KaniAssumedUtf8Validity::decide(&bytes[..len]).holds() {
            Ok(Self { bytes, len })
        } else {
            Err(KaniUtf8BufferError::InvalidUtf8)
        }
    }

    /// Report the byte length.
    #[must_use]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Report whether the modeled buffer is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Borrow the modeled valid UTF-8 content bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes[..self.len]
    }
}

impl<const MAX_LEN: usize> Evidence for KaniUtf8Buffer<MAX_LEN> {
    type Basis = KaniAssumedUtf8Validity;
    type Audit = [u8; MAX_LEN];

    fn basis() -> Self::Basis {
        KaniAssumedUtf8Validity::asserted_valid()
    }

    fn audit(&self) -> Self::Audit {
        self.bytes
    }
}

/// Backs `Establish<KaniAssumedUtf8Validity, KaniVerifier> for
/// KaniUtf8Buffer<2>`: without this impl, that `Establish` impl does not
/// compile, so no downstream proof can establish a claim from a
/// `KaniUtf8Buffer<2>` until the model's own bookkeeping is proven. `2` is
/// the representative instantiation this crate's std-facing proofs
/// (`String`, `OsStr`) actually use; `Evidence` above is implemented
/// generically over every `MAX_LEN` since it states a structural fact, but
/// Kani harnesses are concrete, so the one dedicated proof below is for
/// this one representative size.
impl Witness<KaniVerifier> for KaniUtf8Buffer<2> {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    fn proof() -> Self::ProofArtifact {
        CalculationProof {
            harness: "verify_kani_utf8_buffer_bookkeeping_is_consistent".to_owned(),
            claim: VERIFY_KANI_UTF8_BUFFER_BOOKKEEPING_IS_CONSISTENT_SRC.to_owned(),
        }
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: concat!(module_path!(), "::", stringify!(KaniUtf8Buffer)),
        verifier: "kani",
        describe: || <KaniUtf8Buffer<2> as Witness<KaniVerifier>>::proof().to_string(),
    }
}

kani_ensures!(
    KaniUtf8Buffer<2>,
    "amenable_kani::utf8_model::KaniUtf8Buffer<2>",
    (usize, usize),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_KANI_UTF8_BUFFER_BOOKKEEPING_IS_CONSISTENT_SRC, {
        /// `KaniUtf8Buffer`'s own invariant, proven once here rather than
        /// re-derived independently by every downstream std-facing proof:
        /// given an accepted construction (the assumed-validity credential
        /// holds), length tracks the stored bytes, emptiness tracks a zero
        /// length, and the recovered bytes match exactly what was passed
        /// in.
        #[kani::proof]
        fn verify_kani_utf8_buffer_bookkeeping_is_consistent() {
            let bytes: [u8; 2] = kani::any();
            let len: usize = kani::any();
            kani::assume(len <= 2);

            if let Ok(buffer) = KaniUtf8Buffer::<2>::new(bytes, len) {
                assert!(
                    KaniUtf8Buffer::<2>::ensures((buffer.len(), len)),
                    "length tracks the stored bytes"
                );
                assert_eq!(buffer.is_empty(), len == 0, "emptiness tracks a zero length");
                let recovered = buffer.as_bytes();
                assert!(KaniUtf8Buffer::<2>::ensures((recovered.len(), len)));
                if len >= 1 {
                    assert_eq!(recovered[0], bytes[0]);
                }
                if len >= 2 {
                    assert_eq!(recovered[1], bytes[1]);
                }
            }
        }
    }
}

/// Lawful token minted once `KaniUtf8Buffer<2>`'s bookkeeping has been
/// established.
pub struct KaniUtf8BufferToken(());

impl ProofToken for KaniUtf8BufferToken {
    type Proposition = KaniUtf8Buffer<2>;
}

// `KaniUtf8Buffer<2>` itself, not the bare `KaniAssumedUtf8Validity` axiom,
// is the lawful credential: `KaniUtf8Buffer::new` already runs the
// assumed-validity check internally and only returns `Ok` once it holds
// (its fields are private, so there's no other way to obtain one), while a
// freestanding `KaniAssumedUtf8Validity::asserted_valid()` call is
// disconnected from any particular buffer instance. This is the same
// "evidence by construction" shape as `AddEvidence`/`Sum` in
// `calculator.rs`.
impl ProofToken for KaniUtf8Buffer<2> {
    type Proposition = KaniUtf8Buffer<2>;
}

impl Establish<KaniUtf8Buffer<2>, KaniVerifier> for KaniUtf8Buffer<2> {
    type Token = KaniUtf8BufferToken;

    fn establish(_credential: KaniUtf8Buffer<2>) -> Self::Token {
        KaniUtf8BufferToken(())
    }
}

fn symbolic_ascii_byte() -> u8 {
    let byte: u8 = symbolic_any();
    kani_assume(byte < 0x80);
    byte
}

fn is_valid_utf8(bytes: &[u8]) -> bool {
    let len = bytes.len();
    let mut i = 0;

    while i < len {
        let byte = bytes[i];

        if byte & 0b1000_0000 == 0 {
            i += 1;
            continue;
        }

        if byte & 0b1110_0000 == 0b1100_0000 {
            if i + 1 >= len {
                return false;
            }
            if bytes[i + 1] & 0b1100_0000 != 0b1000_0000 {
                return false;
            }
            if byte & 0b0001_1110 == 0 {
                return false;
            }
            i += 2;
            continue;
        }

        if byte & 0b1111_0000 == 0b1110_0000 {
            if i + 2 >= len {
                return false;
            }
            if bytes[i + 1] & 0b1100_0000 != 0b1000_0000 {
                return false;
            }
            if bytes[i + 2] & 0b1100_0000 != 0b1000_0000 {
                return false;
            }
            if byte == 0b1110_0000 && bytes[i + 1] & 0b0010_0000 == 0 {
                return false;
            }

            let code_point = ((byte & 0x0F) as u32) << 12
                | ((bytes[i + 1] & 0x3F) as u32) << 6
                | (bytes[i + 2] & 0x3F) as u32;
            if (0xD800..=0xDFFF).contains(&code_point) {
                return false;
            }

            i += 3;
            continue;
        }

        if byte & 0b1111_1000 == 0b1111_0000 {
            if i + 3 >= len {
                return false;
            }
            if bytes[i + 1] & 0b1100_0000 != 0b1000_0000 {
                return false;
            }
            if bytes[i + 2] & 0b1100_0000 != 0b1000_0000 {
                return false;
            }
            if bytes[i + 3] & 0b1100_0000 != 0b1000_0000 {
                return false;
            }
            if byte == 0b1111_0000 && bytes[i + 1] & 0b0011_0000 == 0 {
                return false;
            }

            let code_point = ((byte & 0x07) as u32) << 18
                | ((bytes[i + 1] & 0x3F) as u32) << 12
                | ((bytes[i + 2] & 0x3F) as u32) << 6
                | (bytes[i + 3] & 0x3F) as u32;
            if code_point > 0x10FFFF {
                return false;
            }

            i += 4;
            continue;
        }

        return false;
    }

    true
}
