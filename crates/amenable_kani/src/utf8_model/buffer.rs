use amenable_core::{Establish, Evidence, ProofToken, Witness};

use crate::rust_std::macros::{kani_ensures, kani_requires};
use crate::{CalculationProof, KaniVerifier};

// `Ensures`/`Requires`/`EmptinessTracksZeroLength`/`IndexRecoversTheStoredElement`
// stay behind `#[cfg(kani)]` here because the `harness! { .. }` expansion is
// pasted into this module's top level but only generates a proof body under
// `kani`; outside that build those names are genuinely unused.
#[cfg(kani)]
use crate::{EmptinessTracksZeroLength, IndexRecoversTheStoredElement};
#[cfg(kani)]
use amenable_core::{Ensures, Requires};

use super::KaniAssumedUtf8Validity;

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, derive_getters::Getters)]
pub struct KaniUtf8Buffer<const MAX_LEN: usize> {
    /// The modeled buffer content, valid up to `len`.
    #[getter(skip)]
    bytes: [u8; MAX_LEN],
    /// The byte length.
    #[getter(copy)]
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
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(bytes)))]
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

    /// Report whether the modeled buffer is empty.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Borrow the modeled valid UTF-8 content bytes.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes[..self.len]
    }
}

impl<const MAX_LEN: usize> Evidence for KaniUtf8Buffer<MAX_LEN> {
    type Basis = KaniAssumedUtf8Validity;
    type Audit = [u8; MAX_LEN];

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn basis() -> Self::Basis {
        KaniAssumedUtf8Validity::asserted_valid()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "verify_kani_utf8_buffer_bookkeeping_is_consistent".to_owned(),
            VERIFY_KANI_UTF8_BUFFER_BOOKKEEPING_IS_CONSISTENT_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::utf8_model::KaniUtf8Buffer",
        "kani",
        || <KaniUtf8Buffer<2> as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    KaniUtf8Buffer<2>,
    "amenable_kani::utf8_model::KaniUtf8Buffer<2>",
    (usize, usize),
    |(actual, expected)| actual == expected
);

// A symbolic length bounded by the buffer's own fixed capacity is
// independently restated at 4 real sites (`kani::assume(len <= 2)`)
// across `rust_std::alloc_string`/`rust_std::primitives`/
// `rust_std::std_ffi` and this module's own bookkeeping proof.
kani_requires!(
    KaniUtf8Buffer<2>,
    "amenable_kani::utf8_model::KaniUtf8Buffer<2>",
    usize,
    |len| len <= 2
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
            kani::assume(KaniUtf8Buffer::<2>::requires(len));

            if let Ok(buffer) = KaniUtf8Buffer::<2>::new(bytes, len) {
                assert!(
                    KaniUtf8Buffer::<2>::ensures((buffer.len(), len)),
                    "length tracks the stored bytes"
                );
                assert!(
                    EmptinessTracksZeroLength::ensures((buffer.is_empty(), len)),
                    "emptiness tracks a zero length"
                );
                let recovered = buffer.as_bytes();
                assert!(KaniUtf8Buffer::<2>::ensures((recovered.len(), len)));
                if len >= 1 {
                    assert!(IndexRecoversTheStoredElement::ensures((recovered[0], bytes[0])));
                }
                if len >= 2 {
                    assert!(IndexRecoversTheStoredElement::ensures((recovered[1], bytes[1])));
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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(_credential)))]
    fn establish(_credential: KaniUtf8Buffer<2>) -> Self::Token {
        KaniUtf8BufferToken(())
    }
}
