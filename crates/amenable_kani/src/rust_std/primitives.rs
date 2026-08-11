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

impl KaniWitness for RustStdStandard<char> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_char_unicode_scalar".to_owned(),
            claim: VERIFY_CHAR_UNICODE_SCALAR_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<char>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<char>",
        verifier: "kani",
        describe: || <RustStdStandard<char> as KaniWitness>::proof().to_string(),
    }
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
                u <= 0xD7FF || (0xE000..=0x10FFFF).contains(&u),
                "char is a valid Unicode scalar value"
            );

            let c2 = char::from_u32(u).expect("valid unicode scalar round-trips");
            assert!(
                RustStdStandard::<char>::ensures((c, c2)),
                "char round-trips through u32"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<String> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_string_utf8_valid".to_owned(),
            claim: VERIFY_STRING_UTF8_VALID_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<String>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<String>",
        verifier: "kani",
        describe: || <RustStdStandard<String> as KaniWitness>::proof().to_string(),
    }
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

    fn establish(_credential: KaniUtf8Buffer<2>) -> Self::Token {
        RustStdStringUtf8Token(())
    }
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
            kani::assume(len <= 2);

            match KaniUtf8Buffer::<2>::new(bytes, len) {
                Ok(buffer) => {
                    let _token = RustStdStandard::<String>::establish(buffer);

                    assert!(
                        KaniUtf8Buffer::<2>::ensures((buffer.len(), len)),
                        "length tracks the stored bytes"
                    );
                    assert_eq!(buffer.is_empty(), len == 0, "emptiness tracks a zero length");
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
        CheckedProof {
            harness: "verify_array_indexing_and_length".to_owned(),
            claim: VERIFY_ARRAY_INDEXING_AND_LENGTH_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<[i32; 3]>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<[i32; 3]>",
        verifier: "kani",
        describe: || <RustStdStandard<[i32; 3]> as KaniWitness>::proof().to_string(),
    }
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
            assert_eq!(arr[0], a);
            assert_eq!(arr[1], b);
            assert_eq!(arr[2], c);
        }
    }
}

impl KaniWitness for RustStdStandard<[i32]> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_slice_indexing_and_length".to_owned(),
            claim: VERIFY_SLICE_INDEXING_AND_LENGTH_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<[i32]>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<[i32]>",
        verifier: "kani",
        describe: || <RustStdStandard<[i32]> as KaniWitness>::proof().to_string(),
    }
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
            assert_eq!(s[0], a);
            assert_eq!(s[1], b);
            assert_eq!(s[2], c);
        }
    }
}

impl KaniWitness for RustStdStandard<str> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_str_byte_length_and_content".to_owned(),
            claim: VERIFY_STR_BYTE_LENGTH_AND_CONTENT_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<str>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<str>",
        verifier: "kani",
        describe: || <RustStdStandard<str> as KaniWitness>::proof().to_string(),
    }
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
            kani::assume(AsciiByte::requires(byte));
            let owned = (byte as char).to_string();
            let s: &str = &owned;
            assert!(
                RustStdStandard::<str>::ensures((s.len(), 1)),
                "a single ASCII char is exactly one UTF-8 byte"
            );
            assert_eq!(s.as_bytes()[0], byte);
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
        CheckedProof {
            harness: "verify_str_byte_length_and_content".to_owned(),
            claim: VERIFY_STR_BYTE_LENGTH_AND_CONTENT_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(AsciiByte);

kani_requires!(AsciiByte, "amenable_std::AsciiByte", u8, |byte| byte < 128);

impl KaniWitness for RustStdStandard<(i32, i32)> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_tuple_field_access".to_owned(),
            claim: VERIFY_TUPLE_FIELD_ACCESS_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<(i32, i32)>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<(i32, i32)>",
        verifier: "kani",
        describe: || <RustStdStandard<(i32, i32)> as KaniWitness>::proof().to_string(),
    }
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
            assert_eq!(t.0, a);
            assert_eq!(t.1, b);
        }
    }
}

impl KaniWitness for RustStdStandard<fn(i32) -> i32> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_fn_pointer_calls_the_underlying_function".to_owned(),
            claim: VERIFY_FN_POINTER_CALLS_THE_UNDERLYING_FUNCTION_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<fn(i32) -> i32>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<fn(i32) -> i32>",
        verifier: "kani",
        describe: || <RustStdStandard<fn(i32) -> i32> as KaniWitness>::proof().to_string(),
    }
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
            assert_eq!(f(value), increment(value), "the fn pointer calls the function it was assigned from");
        }
    }
}

impl KaniWitness for RustStdStandard<*const i32> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_const_pointer_cast_is_reproducible".to_owned(),
            claim: VERIFY_CONST_POINTER_CAST_IS_REPRODUCIBLE_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<*const i32>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<*const i32>",
        verifier: "kani",
        describe: || <RustStdStandard<*const i32> as KaniWitness>::proof().to_string(),
    }
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
        CheckedProof {
            harness: "verify_mut_pointer_cast_is_reproducible".to_owned(),
            claim: VERIFY_MUT_POINTER_CAST_IS_REPRODUCIBLE_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<*mut i32>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<*mut i32>",
        verifier: "kani",
        describe: || <RustStdStandard<*mut i32> as KaniWitness>::proof().to_string(),
    }
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
        CheckedProof {
            harness: "verify_shared_reference_dereferences_to_the_referent".to_owned(),
            claim: VERIFY_SHARED_REFERENCE_DEREFERENCES_TO_THE_REFERENT_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<&'static i32>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<&'static i32>",
        verifier: "kani",
        describe: || <RustStdStandard<&'static i32> as KaniWitness>::proof().to_string(),
    }
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
        CheckedProof {
            harness: "verify_shared_reference_dereferences_to_the_referent".to_owned(),
            claim: VERIFY_SHARED_REFERENCE_DEREFERENCES_TO_THE_REFERENT_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
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
    ::amenable_core::ContractRecord {
        evidence: "amenable_kani::DerefReflectsTheStoredValue",
        verifier: "kani",
        kind: "ensures",
        fragment: || stringify!(dereferenced == expected),
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_kani::DerefReflectsTheStoredValue",
        verifier: "kani",
        describe: || <DerefReflectsTheStoredValue<i32> as KaniWitness>::proof().to_string(),
    }
}

impl KaniWitness for RustStdStandard<&'static mut i32> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_mutable_reference_dereferences_to_and_updates_the_referent".to_owned(),
            claim: VERIFY_MUTABLE_REFERENCE_DEREFERENCES_TO_AND_UPDATES_THE_REFERENT_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<&'static mut i32>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<&'static mut i32>",
        verifier: "kani",
        describe: || <RustStdStandard<&'static mut i32> as KaniWitness>::proof().to_string(),
    }
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
