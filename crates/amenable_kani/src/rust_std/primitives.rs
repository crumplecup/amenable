//! `KaniWitness` impls for Rust's scalar primitives and `String`.
//!
//! `array`/`fn`/`pointer`/`reference`/`tuple` (the compound primitives, one
//! representative concrete instantiation each) get real checked proofs
//! below rather than the trusted disposition: each has a genuine, non-ZST
//! semantic property worth checking (indexing, field access, dereferencing,
//! calling). `pointer`'s proofs deliberately never dereference the raw
//! pointer -- only checked properties safe code can establish (address
//! reproducibility from a cast), never `unsafe`. `unit` (`()`) is the one
//! exception, trusted alongside the scalars: it has exactly one possible
//! value, nothing to check.

#[cfg(kani)]
use amenable_core::Ensures;
#[cfg(kani)]
use amenable_core::Requires;
use amenable_core::{Establish, Evidence, ProofToken};
#[cfg(kani)]
use amenable_std::ValidUnicodeScalar;
use amenable_std::{AsciiByte, RustStdStandard};

use super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::{
    bridge_kani_witness, impl_kani_witness_trusted, kani_ensures, kani_requires,
};
use crate::{KaniUtf8Buffer, KaniVerifier};

impl_kani_witness_trusted!(
    bool,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    f32,
    f64,
    ()
);

// `checked_add` not overflowing is a real precondition independently
// restated at 9 real sites across `rust_std::slice`'s `chunks_mut`/
// `chunks_exact_mut`/`rchunks_mut`/`rchunks_exact_mut` families (`i32`,
// a fixed literal addend) and `rust_std::time::verify_duration_new_normalizes_nanos_and_carries_into_secs`
// (`u64`, a symbolic addend). `checked_add` is inherent per fixed-width
// integer type, not a shared trait method available without pulling in
// an external crate (`num_traits`), so this is registered per concrete
// width rather than as one generic contract type.
kani_requires!(
    RustStdStandard<i32>,
    "amenable_std::rust_std::RustStdStandard<i32>",
    (i32, i32),
    |(a, b)| a.checked_add(b).is_some()
);

kani_requires!(
    RustStdStandard<u64>,
    "amenable_std::rust_std::RustStdStandard<u64>",
    (u64, u64),
    |(a, b)| a.checked_add(b).is_some()
);

kani_requires!(
    RustStdStandard<i64>,
    "amenable_std::rust_std::RustStdStandard<i64>",
    (i64, i64),
    |(a, b)| a.checked_add(b).is_some()
);

// `i32` equality is independently restated at several real sites
// spanning `rust_std::os_unix`'s raw-fd round-trips and
// `rust_std::num`'s `Wrapping`/`Saturating` inner-operation matches.
// Deliberately registered here rather than in `os_unix.rs` (its first
// real use): that file is `#![cfg(unix)]`-gated, and `rust_std::num`'s
// own sites (not platform-gated) would otherwise depend on a
// registration that silently disappears on a non-unix build.
kani_ensures!(
    RustStdStandard<i32>,
    "amenable_std::rust_std::RustStdStandard<i32>",
    (i32, i32),
    |(actual, expected)| actual == expected
);

// A read/write count matching the buffer's own length is independently
// restated at 5 real sites across `rust_std::io`'s `Repeat`/`Sink`/
// `Chain`/`Cursor` harnesses (`assert_eq!(count, buffer.len(), ...)`).
// `RustStdStandard<usize>`'s `Ensures<KaniVerifier>` slot was free.
kani_ensures!(
    RustStdStandard<usize>,
    "amenable_std::rust_std::RustStdStandard<usize>",
    (usize, usize),
    |(actual, expected)| actual == expected
);

// A round-tripped port number matching the port a socket address was
// constructed with is independently restated at 4 real sites across
// `rust_std::net`'s `SocketAddrV4`/`SocketAddrV6`/`SocketAddr`
// harnesses (`assert_eq!(addr.port(), port, ...)`).
// `RustStdStandard<u16>`'s `Ensures<KaniVerifier>` slot was free.
kani_ensures!(
    RustStdStandard<u16>,
    "amenable_std::rust_std::RustStdStandard<u16>",
    (u16, u16),
    |(actual, expected)| actual == expected
);

// Two hashers over identical input producing matching digests is
// independently restated at 3 real sites (`assert_eq!(h1.finish(),
// h2.finish(), ...)`) across `rust_std::hash`'s `BuildHasherDefault`/
// `SipHasher` harnesses and `rust_std::std_hash`'s `DefaultHasher`
// harness. `RustStdStandard<u64>`'s `Ensures<KaniVerifier>` slot was
// free (only `Requires` was previously registered there, for
// `checked_add`).
kani_ensures!(
    RustStdStandard<u64>,
    "amenable_std::rust_std::RustStdStandard<u64>",
    (u64, u64),
    |(actual, expected)| actual == expected
);

impl KaniWitness for RustStdStandard<char> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_char_unicode_scalar".to_owned(),
            VERIFY_CHAR_UNICODE_SCALAR_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<char>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<char>",
        "kani",
        || <RustStdStandard<char> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<char>,
    "amenable_std::rust_std::RustStdStandard<char>",
    (char, char),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_CHAR_UNICODE_SCALAR_SRC, {
        /// `char` is constrained to Unicode scalar values (excludes the
        /// surrogate range `0xD800..=0xDFFF`) and round-trips through `u32`.
        ///
        /// The first assertion is the canonical home
        /// `amenable_std::ValidUnicodeScalar` names — see that type for the
        /// same bound stated once (currently sourced from
        /// `rust_std::char`'s `verify_char_try_from_fails_exactly_for_surrogates_and_out_of_range`,
        /// a different but equivalent restatement of this exact claim).
        #[kani::proof]
        fn verify_char_unicode_scalar() {
            let c: char = kani::any();
            let u = c as u32;

            assert!(
                <ValidUnicodeScalar as Ensures<crate::KaniVerifier>>::ensures(u),
                "char is a valid Unicode scalar value"
            );

            let c2 = char::from_u32(u).expect("valid unicode scalar round-trips");
            assert!(
                <RustStdStandard<char> as Ensures<crate::KaniVerifier>>::ensures((c, c2)),
                "char round-trips through u32"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<String> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_string_utf8_valid".to_owned(),
            VERIFY_STRING_UTF8_VALID_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<String>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<String>",
        "kani",
        || <RustStdStandard<String> as KaniWitness>::proof().to_string(),
    )
}

/// Lawful token minted once `RustStdStandard<String>`'s UTF-8 bookkeeping
/// claim has been established from an already-proven `KaniUtf8Buffer<2>` --
/// the buffer's own bookkeeping is proven once, generically, by
/// `utf8_model::verify_kani_utf8_buffer_bookkeeping_is_consistent`; this
/// impl is what lets `String`'s proof rest on that instead of re-deriving
/// the same length/emptiness/byte-recovery facts independently.
pub struct RustStdStringUtf8Token(());

impl ProofToken for RustStdStringUtf8Token {
    type Proposition = RustStdStandard<String>;
}

impl Establish<KaniUtf8Buffer<2>, KaniVerifier> for RustStdStandard<String> {
    type Token = RustStdStringUtf8Token;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(_credential)))]
    fn establish(_credential: KaniUtf8Buffer<2>) -> Self::Token {
        RustStdStringUtf8Token(())
    }
}

/// An `(is_empty, length)` pair known to agree: a buffer's own
/// emptiness check reports `true` exactly when its tracked length is
/// zero.
///
/// Independently hand-written as `assert_eq!(buffer.is_empty(), len ==
/// 0, ...)` at 2 real sites (`rust_std::primitives`'s own `String`
/// buffer bookkeeping, `utf8_model`'s `KaniUtf8Buffer` bookkeeping) --
/// the identical claim regardless of which owned-buffer type is being
/// checked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, amenable_derive::Standard)]
#[standard(
    basis = "RustStdStandard<i32>",
    basis_ctor = "RustStdStandard::<i32>::new()",
    provenance = "<i32 as amenable_std::RustStdType>::provenance()",
    provenance_type = "amenable_std::RustStdProvenance"
)]
pub struct EmptinessTracksZeroLength;

impl KaniWitness for EmptinessTracksZeroLength {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_string_utf8_valid".to_owned(),
            VERIFY_STRING_UTF8_VALID_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(EmptinessTracksZeroLength);

kani_ensures!(
    EmptinessTracksZeroLength,
    "amenable_kani::EmptinessTracksZeroLength",
    (bool, usize),
    |(is_empty, length)| is_empty == (length == 0)
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::EmptinessTracksZeroLength",
        "kani",
        || <EmptinessTracksZeroLength as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_STRING_UTF8_VALID_SRC, {
        /// `String`'s length and emptiness are consistent with its byte
        /// content.
        /// This proof uses `KaniUtf8Buffer` (`utf8_model.rs`), following the
        /// pattern documented in `elicitation`'s
        /// `verification::types::Utf8Bytes<MAX_LEN>`: validity is assumed
        /// symbolically under Kani rather than computed, since both the
        /// real `std::str::from_utf8` path and `utf8_model`'s own full
        /// validation state machine were confirmed to time out even for
        /// two fixed bytes when every byte is valid (see
        /// `gallery::utf8_validation_algorithm_cost`).
        /// `String`'s own type invariant already guarantees its content is
        /// valid UTF-8 by construction (nothing unsafe can produce an
        /// invalid one); what this proof establishes is that the
        /// bookkeeping `String` shares with any owned buffer -- length
        /// tracks the stored bytes, and emptiness tracks a zero length --
        /// holds conditional on that invariant. The claim is established
        /// through `Establish<KaniUtf8Buffer<2>, KaniVerifier> for
        /// RustStdStandard<String>` rather than asserted independently, so
        /// it rests on the buffer's own proven bookkeeping instead of
        /// re-deriving it inline.
        #[kani::proof]
        fn verify_string_utf8_valid() {
            use crate::{KaniUtf8Buffer, KaniUtf8BufferError};

            let bytes: [u8; 2] = kani::any();
            let len: usize = kani::any();
            kani::assume(KaniUtf8Buffer::<2>::requires(len));

            match KaniUtf8Buffer::<2>::new(bytes, len) {
                Ok(buffer) => {
                    let _token = RustStdStandard::<String>::establish(buffer);

                    assert!(
                        KaniUtf8Buffer::<2>::ensures((buffer.len(), len)),
                        "length tracks the stored bytes"
                    );
                    assert!(
                        EmptinessTracksZeroLength::ensures((buffer.is_empty(), len)),
                        "emptiness tracks a zero length"
                    );
                    assert!(KaniUtf8Buffer::<2>::ensures((buffer.as_bytes().len(), len)));
                }
                Err(KaniUtf8BufferError::InvalidUtf8) => {
                    // Bytes can be assumed invalid under Kani's
                    // symbolic-validity model; the bookkeeping claim above
                    // only applies to the accepted construction path.
                }
                Err(KaniUtf8BufferError::TooLong) => {
                    unreachable!("len is assumed <= the buffer's own capacity")
                }
            }
        }
    }
}

impl KaniWitness for RustStdStandard<[i32; 3]> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_array_indexing_and_length".to_owned(),
            VERIFY_ARRAY_INDEXING_AND_LENGTH_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<[i32; 3]>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<[i32; 3]>",
        "kani",
        || <RustStdStandard<[i32; 3]> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<[i32; 3]>,
    "amenable_std::rust_std::RustStdStandard<[i32; 3]>",
    (usize, usize),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_ARRAY_INDEXING_AND_LENGTH_SRC, {
        /// A fixed-size array's `.len()` is the compile-time-known
        /// element count, and each index recovers the element it was
        /// constructed with.
        #[kani::proof]
        fn verify_array_indexing_and_length() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            let c: i32 = kani::any();
            let arr = [a, b, c];
            assert!(
                RustStdStandard::<[i32; 3]>::ensures((arr.len(), 3)),
                "the array's length is its fixed compile-time size"
            );
            assert!(IndexRecoversTheStoredElement::ensures((arr[0], a)));
            assert!(IndexRecoversTheStoredElement::ensures((arr[1], b)));
            assert!(IndexRecoversTheStoredElement::ensures((arr[2], c)));
        }
    }
}

impl KaniWitness for RustStdStandard<[i32]> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_slice_indexing_and_length".to_owned(),
            VERIFY_SLICE_INDEXING_AND_LENGTH_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<[i32]>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<[i32]>",
        "kani",
        || <RustStdStandard<[i32]> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<[i32]>,
    "amenable_std::rust_std::RustStdStandard<[i32]>",
    (usize, usize),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_SLICE_INDEXING_AND_LENGTH_SRC, {
        /// A slice's `.len()` reports the number of elements it views,
        /// and each index recovers the underlying element. Checked via a
        /// safe unsizing coercion from a concrete array (`&arr` coerces
        /// `&[i32; 3]` to `&[i32]`): a bare `[i32]` value can never exist
        /// as a local, only a `&[i32]` reference can, so this is the
        /// only way any code -- proof or otherwise -- interacts with a
        /// slice value at all.
        #[kani::proof]
        fn verify_slice_indexing_and_length() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            let c: i32 = kani::any();
            let arr = [a, b, c];
            let s: &[i32] = &arr;
            assert!(
                RustStdStandard::<[i32]>::ensures((s.len(), 3)),
                "the slice's length is the number of elements it views"
            );
            assert!(IndexRecoversTheStoredElement::ensures((s[0], a)));
            assert!(IndexRecoversTheStoredElement::ensures((s[1], b)));
            assert!(IndexRecoversTheStoredElement::ensures((s[2], c)));
        }
    }
}

impl KaniWitness for RustStdStandard<str> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_str_byte_length_and_content".to_owned(),
            VERIFY_STR_BYTE_LENGTH_AND_CONTENT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<str>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<str>",
        "kani",
        || <RustStdStandard<str> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<str>,
    "amenable_std::rust_std::RustStdStandard<str>",
    (usize, usize),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_STR_BYTE_LENGTH_AND_CONTENT_SRC, {
        /// A `str`'s `.len()` reports its UTF-8 byte length, and its
        /// bytes are exactly its content's UTF-8 encoding -- checked for
        /// any single-byte (ASCII) character, mirroring
        /// `rust_std::str`'s own symbolic-byte convention. A bare `str`
        /// value can never exist as a local, only a `&str` reference
        /// can (here, borrowed from an owned `String`), so this is the
        /// only way any code interacts with a `str` value at all.
        ///
        /// The `kani::assume` call below calls
        /// `AsciiByte::requires` directly rather than restating its
        /// expression — the same precondition every symbolic
        /// single-byte-character proof in `rust_std::str` assumes.
        #[kani::proof]
        fn verify_str_byte_length_and_content() {
            let byte: u8 = kani::any();
            kani::assume(<AsciiByte as Requires<crate::KaniVerifier>>::requires(byte));
            let owned = (byte as char).to_string();
            let s: &str = &owned;
            assert!(
                <RustStdStandard<str> as Ensures<crate::KaniVerifier>>::ensures((s.len(), 1)),
                "a single ASCII char is exactly one UTF-8 byte"
            );
            assert!(IndexRecoversTheStoredElement::ensures((s.as_bytes()[0], byte)));
        }
    }
}

/// [`AsciiByte`] reuses `verify_str_byte_length_and_content` rather than
/// adding a new Kani harness — it names the precondition the harness
/// already assumes, it doesn't prove anything new.
impl KaniWitness for AsciiByte {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_str_byte_length_and_content".to_owned(),
            VERIFY_STR_BYTE_LENGTH_AND_CONTENT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(AsciiByte);

kani_requires!(AsciiByte, "amenable_std::AsciiByte", u8, |byte| byte < 128);

/// Four bytes each known to satisfy [`AsciiByte`]'s own precondition
/// (`< 128`), combined into a single callable predicate.
///
/// Independently hand-written as `kani::assume(a < 128 && pattern < 128
/// && b < 128 && c < 128)` at 5 real sites in `rust_std::str`'s
/// `*n`/`matches`/`match_indices` family -- the same four-way ASCII
/// bound `AsciiByte` already names for a single byte, just applied to
/// all four symbolic bytes a real site needs at once. A separate type
/// rather than four individual `AsciiByte::requires(...)` calls joined
/// by `&&` at the call site: the call-shape scanner only recognizes a
/// `kani::assume(EXPR)` clause as compliant when `EXPR` itself is a
/// single call, not a `&&`-combined expression of several real calls
/// (confirmed the hard way -- see `ThreeSplitOperandsAreDistinctFromThePattern`
/// for the same lesson applied to a `!=` combination).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, amenable_derive::Standard)]
#[standard(
    basis = "RustStdStandard<u8>",
    basis_ctor = "RustStdStandard::<u8>::new()",
    provenance = "<u8 as amenable_std::RustStdType>::provenance()",
    provenance_type = "amenable_std::RustStdProvenance"
)]
pub struct FourBytesAreEachAscii;

impl KaniWitness for FourBytesAreEachAscii {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_str_byte_length_and_content".to_owned(),
            VERIFY_STR_BYTE_LENGTH_AND_CONTENT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(FourBytesAreEachAscii);

kani_requires!(
    FourBytesAreEachAscii,
    "amenable_kani::FourBytesAreEachAscii",
    (u8, u8, u8, u8),
    |(a, pattern, b, c)| a < 128 && pattern < 128 && b < 128 && c < 128
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::FourBytesAreEachAscii",
        "kani",
        || <FourBytesAreEachAscii as KaniWitness>::proof().to_string(),
    )
}

/// Three bytes each known to satisfy [`AsciiByte`]'s own precondition
/// (`< 128`), combined into a single callable predicate — the
/// three-operand sibling of [`FourBytesAreEachAscii`], same reasoning.
///
/// Independently hand-written as `kani::assume(before < 128 && pattern
/// < 128 && after < 128)` at 3 real sites in `rust_std::str`'s
/// `rsplit`/`split_terminator`/`rsplit_terminator` family -- the same
/// three-way ASCII bound applied to all three symbolic bytes a real
/// site needs at once, for the same call-shape-scanner reason
/// `FourBytesAreEachAscii` documents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, amenable_derive::Standard)]
#[standard(
    basis = "RustStdStandard<u8>",
    basis_ctor = "RustStdStandard::<u8>::new()",
    provenance = "<u8 as amenable_std::RustStdType>::provenance()",
    provenance_type = "amenable_std::RustStdProvenance"
)]
pub struct ThreeBytesAreEachAscii;

impl KaniWitness for ThreeBytesAreEachAscii {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_str_byte_length_and_content".to_owned(),
            VERIFY_STR_BYTE_LENGTH_AND_CONTENT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(ThreeBytesAreEachAscii);

kani_requires!(
    ThreeBytesAreEachAscii,
    "amenable_kani::ThreeBytesAreEachAscii",
    (u8, u8, u8),
    |(before, pattern, after)| before < 128 && pattern < 128 && after < 128
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::ThreeBytesAreEachAscii",
        "kani",
        || <ThreeBytesAreEachAscii as KaniWitness>::proof().to_string(),
    )
}

/// A `(value, low, high)` triple known to satisfy the precondition
/// every proof over a small, symbolic-but-bounded value assumes: the
/// value falls within the inclusive range `low..=high`.
///
/// Independently hand-written as `kani::assume((low..=high).contains(&value))`
/// at 2 real sites (`rust_std::iter`'s `FlatMap` over `0..=4`,
/// `rust_std::slice`'s printable-ASCII bound over `0x20..=0x7e`) --
/// the identical range-membership precondition regardless of the
/// concrete bounds or element type. Generic over the element type
/// rather than one registration per bound, the same reasoning (and the
/// same reason it needs a hand-written `Witness`/`Requires` impl
/// instead of the `bridge_kani_witness!`/`kani_requires!` macros) as
/// `SplitOperandsAreDistinctFromThePattern` (`rust_std::slice`).
pub struct ValueIsWithinInclusiveRange<T>(std::marker::PhantomData<T>);

impl<T> amenable_core::Standard for ValueIsWithinInclusiveRange<T> {
    type Provenance = amenable_std::RustStdProvenance;

    fn provenance(&self) -> Self::Provenance {
        <i32 as amenable_std::RustStdType>::provenance()
    }
}

impl<T> Evidence for ValueIsWithinInclusiveRange<T> {
    type Basis = RustStdStandard<i32>;
    type Audit = amenable_std::RustStdProvenance;

    fn basis() -> Self::Basis {
        RustStdStandard::<i32>::new()
    }

    fn audit(&self) -> Self::Audit {
        <i32 as amenable_std::RustStdType>::provenance()
    }

    fn is_root() -> bool {
        false
    }
}

impl<T> KaniWitness for ValueIsWithinInclusiveRange<T> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_flat_map_flattens_each_generated_iterator".to_owned(),
            crate::rust_std::iter::VERIFY_FLAT_MAP_FLATTENS_EACH_GENERATED_ITERATOR_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

impl<T> amenable_core::Witness<crate::KaniVerifier> for ValueIsWithinInclusiveRange<T> {
    type SupportingEvidence = <Self as KaniWitness>::SupportingEvidence;
    type ProofArtifact = <Self as KaniWitness>::ProofArtifact;

    fn proof() -> Self::ProofArtifact {
        <Self as KaniWitness>::proof()
    }
}

impl<T: PartialOrd> amenable_core::Requires<crate::KaniVerifier>
    for ValueIsWithinInclusiveRange<T>
{
    type Input = (T, T, T);
    type Bound = bool;

    fn requires((value, low, high): (T, T, T)) -> bool {
        low <= value && value <= high
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_kani::ValueIsWithinInclusiveRange",
        "kani",
        "requires",
        || stringify!(low <= value && value <= high),
    )
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::ValueIsWithinInclusiveRange",
        "kani",
        || <ValueIsWithinInclusiveRange<i32> as KaniWitness>::proof().to_string(),
    )
}

/// The negation of [`ValueIsWithinInclusiveRange`]: a `(value, low,
/// high)` triple known to satisfy the precondition a proof over a
/// symbolic value assumes when it must fall *outside* an excluded
/// inclusive range -- e.g. a UTF-16 code unit that isn't a surrogate.
/// Generic and hand-written for the same reason.
///
/// Independently hand-written as
/// `kani::assume(!(0xD800..=0xDFFF).contains(&unit))` at 1 real site
/// (`rust_std::char`'s non-surrogate UTF-16 code unit bound).
pub struct ValueIsOutsideInclusiveRange<T>(std::marker::PhantomData<T>);

impl<T> amenable_core::Standard for ValueIsOutsideInclusiveRange<T> {
    type Provenance = amenable_std::RustStdProvenance;

    fn provenance(&self) -> Self::Provenance {
        <i32 as amenable_std::RustStdType>::provenance()
    }
}

impl<T> Evidence for ValueIsOutsideInclusiveRange<T> {
    type Basis = RustStdStandard<i32>;
    type Audit = amenable_std::RustStdProvenance;

    fn basis() -> Self::Basis {
        RustStdStandard::<i32>::new()
    }

    fn audit(&self) -> Self::Audit {
        <i32 as amenable_std::RustStdType>::provenance()
    }

    fn is_root() -> bool {
        false
    }
}

impl<T> KaniWitness for ValueIsOutsideInclusiveRange<T> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_decode_utf16_round_trips_a_bmp_code_unit".to_owned(),
            crate::rust_std::char::VERIFY_DECODE_UTF16_ROUND_TRIPS_A_BMP_CODE_UNIT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

impl<T> amenable_core::Witness<crate::KaniVerifier> for ValueIsOutsideInclusiveRange<T> {
    type SupportingEvidence = <Self as KaniWitness>::SupportingEvidence;
    type ProofArtifact = <Self as KaniWitness>::ProofArtifact;

    fn proof() -> Self::ProofArtifact {
        <Self as KaniWitness>::proof()
    }
}

impl<T: PartialOrd> amenable_core::Requires<crate::KaniVerifier>
    for ValueIsOutsideInclusiveRange<T>
{
    type Input = (T, T, T);
    type Bound = bool;

    fn requires((value, low, high): (T, T, T)) -> bool {
        value < low || value > high
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_kani::ValueIsOutsideInclusiveRange",
        "kani",
        "requires",
        || stringify!(value < low || value > high),
    )
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::ValueIsOutsideInclusiveRange",
        "kani",
        || <ValueIsOutsideInclusiveRange<i32> as KaniWitness>::proof().to_string(),
    )
}

/// A `(value, minimum)` pair known to satisfy the precondition every
/// proof over a symbolic value with a one-sided lower bound assumes:
/// the value is at least the given minimum.
///
/// Independently hand-written as `kani::assume(value >= minimum)` at 1
/// real site (`rust_std::str`'s UTF-8 lead-byte lower bound, `0xF5`) --
/// a singleton today, named for the same reason every other bound in
/// this worklist is: it makes the assumption explicit and auditable,
/// not because it's shared across multiple sites. Generic over the
/// element type and hand-written for the same reason
/// `ValueIsWithinInclusiveRange` is.
pub struct ValueIsAtLeast<T>(std::marker::PhantomData<T>);

impl<T> amenable_core::Standard for ValueIsAtLeast<T> {
    type Provenance = amenable_std::RustStdProvenance;

    fn provenance(&self) -> Self::Provenance {
        <i32 as amenable_std::RustStdType>::provenance()
    }
}

impl<T> Evidence for ValueIsAtLeast<T> {
    type Basis = RustStdStandard<i32>;
    type Audit = amenable_std::RustStdProvenance;

    fn basis() -> Self::Basis {
        RustStdStandard::<i32>::new()
    }

    fn audit(&self) -> Self::Audit {
        <i32 as amenable_std::RustStdType>::provenance()
    }

    fn is_root() -> bool {
        false
    }
}

impl<T> KaniWitness for ValueIsAtLeast<T> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_utf8_error_reports_the_valid_prefix_length_and_error_span".to_owned(),
            crate::rust_std::str::VERIFY_UTF8_ERROR_REPORTS_THE_VALID_PREFIX_LENGTH_AND_ERROR_SPAN_SRC
                .to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

impl<T> amenable_core::Witness<crate::KaniVerifier> for ValueIsAtLeast<T> {
    type SupportingEvidence = <Self as KaniWitness>::SupportingEvidence;
    type ProofArtifact = <Self as KaniWitness>::ProofArtifact;

    fn proof() -> Self::ProofArtifact {
        <Self as KaniWitness>::proof()
    }
}

impl<T: PartialOrd> amenable_core::Requires<crate::KaniVerifier> for ValueIsAtLeast<T> {
    type Input = (T, T);
    type Bound = bool;

    fn requires((value, minimum): (T, T)) -> bool {
        value >= minimum
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_kani::ValueIsAtLeast",
        "kani",
        "requires",
        || stringify!(value >= minimum),
    )
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::ValueIsAtLeast",
        "kani",
        || <ValueIsAtLeast<i32> as KaniWitness>::proof().to_string(),
    )
}

/// The postcondition counterpart to `ValueIsAtLeast`'s own
/// `Requires` impl, same body, for real sites that assert this shape
/// as an `Ensures` claim rather than assume it as a `Requires`
/// precondition -- both directions register their own `ContractRecord`
/// (Kani's `(verifier, kind)` lookup is keyed separately for
/// `"requires"` vs `"ensures"` clauses), so one type can carry both.
impl<T: PartialOrd> amenable_core::Ensures<crate::KaniVerifier> for ValueIsAtLeast<T> {
    type Input = (T, T);
    type Bound = bool;

    fn ensures((value, minimum): (T, T)) -> bool {
        value >= minimum
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_kani::ValueIsAtLeast",
        "kani",
        "ensures",
        || stringify!(value >= minimum),
    )
}

/// A `(value, bound)` pair known to satisfy the precondition every
/// proof over a symbolic value with a one-sided upper bound assumes:
/// the value is strictly below the given bound. The mirror image of
/// `ValueIsAtLeast`, generic and hand-written for the same reason.
///
/// Independently hand-written as `kani::assume((c as u32) < 0x10000)`
/// at 1 real site (`os_windows_model`'s BMP-character bound for
/// `EncodeWide`).
pub struct ValueIsBelow<T>(std::marker::PhantomData<T>);

impl<T> amenable_core::Standard for ValueIsBelow<T> {
    type Provenance = amenable_std::RustStdProvenance;

    fn provenance(&self) -> Self::Provenance {
        <i32 as amenable_std::RustStdType>::provenance()
    }
}

impl<T> Evidence for ValueIsBelow<T> {
    type Basis = RustStdStandard<i32>;
    type Audit = amenable_std::RustStdProvenance;

    fn basis() -> Self::Basis {
        RustStdStandard::<i32>::new()
    }

    fn audit(&self) -> Self::Audit {
        <i32 as amenable_std::RustStdType>::provenance()
    }

    fn is_root() -> bool {
        false
    }
}

impl<T> KaniWitness for ValueIsBelow<T> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_encode_wide_encodes_a_bmp_char_as_one_code_unit".to_owned(),
            crate::os_windows_model::VERIFY_ENCODE_WIDE_ENCODES_A_BMP_CHAR_AS_ONE_CODE_UNIT_SRC
                .to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

impl<T> amenable_core::Witness<crate::KaniVerifier> for ValueIsBelow<T> {
    type SupportingEvidence = <Self as KaniWitness>::SupportingEvidence;
    type ProofArtifact = <Self as KaniWitness>::ProofArtifact;

    fn proof() -> Self::ProofArtifact {
        <Self as KaniWitness>::proof()
    }
}

impl<T: PartialOrd> amenable_core::Requires<crate::KaniVerifier> for ValueIsBelow<T> {
    type Input = (T, T);
    type Bound = bool;

    fn requires((value, bound): (T, T)) -> bool {
        value < bound
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_kani::ValueIsBelow",
        "kani",
        "requires",
        || stringify!(value < bound),
    )
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::ValueIsBelow",
        "kani",
        || <ValueIsBelow<i32> as KaniWitness>::proof().to_string(),
    )
}

impl KaniWitness for RustStdStandard<(i32, i32)> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_tuple_field_access".to_owned(),
            VERIFY_TUPLE_FIELD_ACCESS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<(i32, i32)>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<(i32, i32)>",
        "kani",
        || <RustStdStandard<(i32, i32)> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_TUPLE_FIELD_ACCESS_SRC, {
        /// A tuple's `.0`/`.1` recover exactly the values it was
        /// constructed with, in position order.
        #[kani::proof]
        fn verify_tuple_field_access() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            let t = (a, b);
            assert!(FieldAccessRecoversTheStoredValue::ensures((t.0, a)));
            assert!(FieldAccessRecoversTheStoredValue::ensures((t.1, b)));
        }
    }
}

impl KaniWitness for RustStdStandard<fn(i32) -> i32> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_fn_pointer_calls_the_underlying_function".to_owned(),
            VERIFY_FN_POINTER_CALLS_THE_UNDERLYING_FUNCTION_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<fn(i32) -> i32>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<fn(i32) -> i32>",
        "kani",
        || <RustStdStandard<fn(i32) -> i32> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_FN_POINTER_CALLS_THE_UNDERLYING_FUNCTION_SRC, {
        /// Calling through a `fn` pointer invokes exactly the function it
        /// was assigned from.
        #[kani::proof]
        fn verify_fn_pointer_calls_the_underlying_function() {
            fn increment(x: i32) -> i32 {
                x.wrapping_add(1)
            }
            let f: fn(i32) -> i32 = increment;
            let value: i32 = kani::any();
            assert!(
                RustStdStandard::<i32>::ensures((f(value), increment(value))),
                "the fn pointer calls the function it was assigned from"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<*const i32> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_const_pointer_cast_is_reproducible".to_owned(),
            VERIFY_CONST_POINTER_CAST_IS_REPRODUCIBLE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<*const i32>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<*const i32>",
        "kani",
        || <RustStdStandard<*const i32> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<*const i32>,
    "amenable_std::rust_std::RustStdStandard<*const i32>",
    (*const i32, *const i32),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_CONST_POINTER_CAST_IS_REPRODUCIBLE_SRC, {
        /// Casting the same reference to a raw pointer twice gives the
        /// same address, without ever dereferencing the pointer -- a
        /// safe property of the cast itself, deliberately checked without
        /// `unsafe` (this crate forbids it in its own source).
        #[kani::proof]
        fn verify_const_pointer_cast_is_reproducible() {
            let value: i32 = kani::any();
            let first: *const i32 = &value;
            let second: *const i32 = &value;
            assert!(
                RustStdStandard::<*const i32>::ensures((first, second)),
                "casting the same reference twice gives the same address"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<*mut i32> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_mut_pointer_cast_is_reproducible".to_owned(),
            VERIFY_MUT_POINTER_CAST_IS_REPRODUCIBLE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<*mut i32>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<*mut i32>",
        "kani",
        || <RustStdStandard<*mut i32> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<*mut i32>,
    "amenable_std::rust_std::RustStdStandard<*mut i32>",
    (*mut i32, *mut i32),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_MUT_POINTER_CAST_IS_REPRODUCIBLE_SRC, {
        /// Same as the `*const i32` proof, for a mutable raw pointer:
        /// casting the same exclusive reference to a raw pointer twice
        /// gives the same address, without ever dereferencing it.
        #[kani::proof]
        fn verify_mut_pointer_cast_is_reproducible() {
            let mut value: i32 = kani::any();
            let first: *mut i32 = &mut value;
            let second: *mut i32 = &mut value;
            assert!(
                RustStdStandard::<*mut i32>::ensures((first, second)),
                "casting the same reference twice gives the same address"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<&'static i32> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_shared_reference_dereferences_to_the_referent".to_owned(),
            VERIFY_SHARED_REFERENCE_DEREFERENCES_TO_THE_REFERENT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<&'static i32>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<&'static i32>",
        "kani",
        || <RustStdStandard<&'static i32> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_SHARED_REFERENCE_DEREFERENCES_TO_THE_REFERENT_SRC, {
        /// Dereferencing a shared reference recovers exactly the value
        /// it borrows. `Box::leak` gives a genuinely `'static` reference
        /// to symbolic heap data without needing a `const`/`static` item
        /// (which can't hold a `kani::any()` value) -- ordinary safe
        /// Rust, not a workaround for anything unsafe. Calls
        /// `DerefReflectsTheStoredValue::ensures` directly rather than
        /// restating the comparison -- see that type for why this is the
        /// one harness its registration reuses as a witness.
        #[kani::proof]
        fn verify_shared_reference_dereferences_to_the_referent() {
            let value: i32 = kani::any();
            let leaked: &'static i32 = Box::leak(Box::new(value));
            assert!(
                DerefReflectsTheStoredValue::ensures((*leaked, value)),
                "dereferencing recovers the referent"
            );
        }
    }
}

/// A `(dereferenced, expected)` pair known to agree: dereferencing a
/// smart pointer, guard, or reference recovers exactly the value stored
/// in (or borrowed by) it.
///
/// Independently hand-written as `assert_eq!(*wrapper, expected, ...)` at
/// 28 real sites spanning `Cow`, `Box`, `BinaryHeap::PeekMut`, `Rc`,
/// `Arc`, `RefCell`'s `Ref`/`RefMut`, `ManuallyDrop`, `Option`/`Result`'s
/// `IterMut`, `AssertUnwindSafe`, `Pin<Box<_>>`, shared/mutable
/// references, `slice::IterMut`, and `Mutex`/`RwLock`'s guards -- the
/// identical claim regardless of which wrapper type derefs. Generic over
/// the pointee type rather than one registration per wrapper type, the
/// same reasoning (and the same reason it needs a hand-written
/// `Witness`/`Ensures` impl instead of the
/// `bridge_kani_witness!`/`kani_ensures!` macros) as
/// `IteratorYieldsNoneWhenExhausted` in `rust_std::iter` and
/// `AtomicLoadReflectsTheLastWrite` in `rust_std::sync_atomic`.
pub struct DerefReflectsTheStoredValue<T>(std::marker::PhantomData<T>);

impl<T> amenable_core::Standard for DerefReflectsTheStoredValue<T> {
    type Provenance = amenable_std::RustStdProvenance;

    fn provenance(&self) -> Self::Provenance {
        <i32 as amenable_std::RustStdType>::provenance()
    }
}

impl<T> Evidence for DerefReflectsTheStoredValue<T> {
    type Basis = RustStdStandard<i32>;
    type Audit = amenable_std::RustStdProvenance;

    fn basis() -> Self::Basis {
        RustStdStandard::<i32>::new()
    }

    fn audit(&self) -> Self::Audit {
        <i32 as amenable_std::RustStdType>::provenance()
    }

    fn is_root() -> bool {
        false
    }
}

impl<T> KaniWitness for DerefReflectsTheStoredValue<T> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_shared_reference_dereferences_to_the_referent".to_owned(),
            VERIFY_SHARED_REFERENCE_DEREFERENCES_TO_THE_REFERENT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

impl<T> amenable_core::Witness<crate::KaniVerifier> for DerefReflectsTheStoredValue<T> {
    type SupportingEvidence = <Self as KaniWitness>::SupportingEvidence;
    type ProofArtifact = <Self as KaniWitness>::ProofArtifact;

    fn proof() -> Self::ProofArtifact {
        <Self as KaniWitness>::proof()
    }
}

impl<T: PartialEq> amenable_core::Ensures<crate::KaniVerifier> for DerefReflectsTheStoredValue<T> {
    type Input = (T, T);
    type Bound = bool;

    fn ensures((dereferenced, expected): (T, T)) -> bool {
        dereferenced == expected
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_kani::DerefReflectsTheStoredValue",
        "kani",
        "ensures",
        || stringify!(dereferenced == expected),
    )
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::DerefReflectsTheStoredValue",
        "kani",
        || <DerefReflectsTheStoredValue<i32> as KaniWitness>::proof().to_string(),
    )
}

/// An `(actual, expected)` pair known to agree: indexing a fixed-length
/// container at a position recovers exactly the element known to be
/// stored there.
///
/// Independently hand-written as `assert_eq!(container[i], expected,
/// ...)` at 9 real sites spanning `Vec<i32>`, a `[u8; 4]` array indexed
/// through `IoSliceMut`, `[i32; 3]`/`[i32; 1]` arrays, and a `&[i32]`
/// slice -- the identical claim regardless of container kind or element
/// type. This is the Kani-side sibling of
/// `amenable_std::IndexingAndLength`'s Creusot postcondition, not a
/// reuse of that type directly: `IndexingAndLength` is a fixed,
/// non-generic wrapper bundling a length check together with three
/// specific indices in one Pearlite predicate, which cannot vary its
/// `Input` type per real site's element type the way a Kani `Ensures`
/// impl needs to (`i32` here, `u8` for the `IoSliceMut` site). Generic
/// over the element type instead, same reasoning (and the same reason
/// it needs a hand-written `Witness`/`Ensures` impl instead of the
/// `bridge_kani_witness!`/`kani_ensures!` macros) as
/// `DerefReflectsTheStoredValue` just above.
pub struct IndexRecoversTheStoredElement<T>(std::marker::PhantomData<T>);

impl<T> amenable_core::Standard for IndexRecoversTheStoredElement<T> {
    type Provenance = amenable_std::RustStdProvenance;

    fn provenance(&self) -> Self::Provenance {
        <i32 as amenable_std::RustStdType>::provenance()
    }
}

impl<T> Evidence for IndexRecoversTheStoredElement<T> {
    type Basis = RustStdStandard<i32>;
    type Audit = amenable_std::RustStdProvenance;

    fn basis() -> Self::Basis {
        RustStdStandard::<i32>::new()
    }

    fn audit(&self) -> Self::Audit {
        <i32 as amenable_std::RustStdType>::provenance()
    }

    fn is_root() -> bool {
        false
    }
}

impl<T> KaniWitness for IndexRecoversTheStoredElement<T> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_array_indexing_and_length".to_owned(),
            VERIFY_ARRAY_INDEXING_AND_LENGTH_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

impl<T> amenable_core::Witness<crate::KaniVerifier> for IndexRecoversTheStoredElement<T> {
    type SupportingEvidence = <Self as KaniWitness>::SupportingEvidence;
    type ProofArtifact = <Self as KaniWitness>::ProofArtifact;

    fn proof() -> Self::ProofArtifact {
        <Self as KaniWitness>::proof()
    }
}

impl<T: PartialEq> amenable_core::Ensures<crate::KaniVerifier>
    for IndexRecoversTheStoredElement<T>
{
    type Input = (T, T);
    type Bound = bool;

    fn ensures((actual, expected): (T, T)) -> bool {
        actual == expected
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_kani::IndexRecoversTheStoredElement",
        "kani",
        "ensures",
        || stringify!(actual == expected),
    )
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::IndexRecoversTheStoredElement",
        "kani",
        || <IndexRecoversTheStoredElement<i32> as KaniWitness>::proof().to_string(),
    )
}

/// An `(actual, expected)` pair known to agree: a struct or tuple
/// field access recovers exactly the value known to be stored there.
///
/// Independently hand-written as `assert_eq!(value.field, expected,
/// ...)` at 5 real sites: `verify_tuple_field_access` (`(a, b)`'s `.0`/
/// `.1` projections, 2 sites), `calculator::Debit`/`Credit`'s own
/// `.value` field access constructors (2 sites), and
/// `verify_assert_unwind_safe_derefs_transparently`'s `.0` projection
/// after a `DerefMut` write-through (1 site) -- the identical claim
/// regardless of whether the access is a named field or a tuple index.
/// A distinct access pattern from `IndexRecoversTheStoredElement`
/// (`[i]`) and `DerefReflectsTheStoredValue` (`*x`) even though the
/// `Ensures` impl body is identical trivial equality either way --
/// same reasoning as keeping `CollectedSequenceMatchesExpected`
/// separate from `DerefReflectsTheStoredValue` despite type-level
/// overlap.
pub struct FieldAccessRecoversTheStoredValue<T>(std::marker::PhantomData<T>);

impl<T> amenable_core::Standard for FieldAccessRecoversTheStoredValue<T> {
    type Provenance = amenable_std::RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn provenance(&self) -> Self::Provenance {
        <i32 as amenable_std::RustStdType>::provenance()
    }
}

impl<T> Evidence for FieldAccessRecoversTheStoredValue<T> {
    type Basis = RustStdStandard<i32>;
    type Audit = amenable_std::RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn basis() -> Self::Basis {
        RustStdStandard::<i32>::new()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn audit(&self) -> Self::Audit {
        <i32 as amenable_std::RustStdType>::provenance()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", ret))]
    fn is_root() -> bool {
        false
    }
}

impl<T> KaniWitness for FieldAccessRecoversTheStoredValue<T> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_tuple_field_access".to_owned(),
            VERIFY_TUPLE_FIELD_ACCESS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

impl<T> amenable_core::Witness<crate::KaniVerifier> for FieldAccessRecoversTheStoredValue<T> {
    type SupportingEvidence = <Self as KaniWitness>::SupportingEvidence;
    type ProofArtifact = <Self as KaniWitness>::ProofArtifact;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        <Self as KaniWitness>::proof()
    }
}

impl<T: PartialEq> amenable_core::Ensures<crate::KaniVerifier>
    for FieldAccessRecoversTheStoredValue<T>
{
    type Input = (T, T);
    type Bound = bool;

    fn ensures((actual, expected): (T, T)) -> bool {
        actual == expected
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_kani::FieldAccessRecoversTheStoredValue",
        "kani",
        "ensures",
        || stringify!(actual == expected),
    )
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::FieldAccessRecoversTheStoredValue",
        "kani",
        || <FieldAccessRecoversTheStoredValue<i32> as KaniWitness>::proof().to_string(),
    )
}

impl KaniWitness for RustStdStandard<&'static mut i32> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_mutable_reference_dereferences_to_and_updates_the_referent".to_owned(),
            VERIFY_MUTABLE_REFERENCE_DEREFERENCES_TO_AND_UPDATES_THE_REFERENT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<&'static mut i32>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<&'static mut i32>",
        "kani",
        || <RustStdStandard<&'static mut i32> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_MUTABLE_REFERENCE_DEREFERENCES_TO_AND_UPDATES_THE_REFERENT_SRC, {
        /// Dereferencing a mutable reference recovers the value it
        /// borrows, and writing through it updates the referent visibly
        /// through the same reference.
        #[kani::proof]
        fn verify_mutable_reference_dereferences_to_and_updates_the_referent() {
            let initial: i32 = kani::any();
            let next: i32 = kani::any();
            let leaked: &'static mut i32 = Box::leak(Box::new(initial));
            assert!(
                DerefReflectsTheStoredValue::ensures((*leaked, initial)),
                "dereferencing recovers the referent"
            );
            *leaked = next;
            assert!(
                DerefReflectsTheStoredValue::ensures((*leaked, next)),
                "writing through the reference updates the referent"
            );
        }
    }
}
