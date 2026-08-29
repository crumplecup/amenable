//! `KaniWitness` impls for `core::str`.
//!
//! `Bytes`/`CharIndices`/`Chars`/`EncodeUtf16` check a symbolic ASCII byte
//! (`byte as char` is a valid, safe conversion for `byte < 128`, giving a
//! genuinely symbolic single-character `str` without needing to construct
//! arbitrary `String` content). The escaping/splitting/UTF-8-validation
//! adapters instead check fixed representative examples: their behavior
//! turns on specific character classes (whitespace, control characters,
//! invalid byte sequences) that don't reduce to a single symbolic byte the
//! way plain iteration does.
//!
//! The `Pattern`-generic split/match family is monomorphized on `char`,
//! mirroring `amenable_std::rust_std::str`'s registration choice. Of the
//! 11 such types, `Split`/`SplitN`/`SplitInclusive` verify directly
//! against fixed representative strings via `.collect()`. The other 8
//! (`RSplit`, `RSplitN`, `SplitTerminator`, `RSplitTerminator`, `Matches`,
//! `RMatches`, `MatchIndices`, `RMatchIndices`) don't have a passing
//! direct proof for two distinct reasons, both still preserved as gallery
//! false trails:
//!
//! - **Reverse traversal** (`RSplit`/`RSplitN`/`RSplitTerminator`/
//!   `RMatches`/`RMatchIndices`): root-caused via an isolated probe --
//!   `CharSearcher::next_match_back` bottoms out in `memchr::memrchr`,
//!   whose internal scan loop CBMC can't bound even for a single `.next()`
//!   call on a five-byte fixed str (confirmed unwinding past 580
//!   iterations). Independent of proof style. See
//!   `gallery::replace_recommendations::str_rsplit_reverse_pattern_search_times_out_even_for_a_single_next_call`.
//! - **Forward `SplitTerminator`/`Matches`/`MatchIndices`**: root cause
//!   not fully isolated. Switching from `.collect()` to explicit
//!   sequential `.next()` calls passed instantly in a minimal standalone
//!   probe crate, but the identical harness still timed out when actually
//!   run inside this crate — an isolated probe crate's timing does not
//!   reliably predict real-crate behavior for Kani/CBMC (whole-crate
//!   reachability/compilation scale appears to matter on its own). See
//!   `gallery::replace_recommendations::str_split_terminator_matches_forward_pattern_iteration_times_out_in_the_real_crate`.
//!
//! All 8 instead go through `crate::str_pattern_model`'s bounded,
//! symbolic-`char` accommodation model (`KaniStrRSplitObservation`,
//! `KaniStrRSplitNObservation`, `KaniStrSplitTerminatorObservation`,
//! `KaniStrMatchObservation`): each proof is conditional on the real
//! `core::str` path refining the model's fixed window shape, exactly as
//! `slice_split_model`'s proofs already are for `std::slice::Split` and
//! friends (see `amenable_kani::rust_std::slice`).

use std::str::{
    CharIndices, Chars, EncodeUtf16, MatchIndices, Matches, ParseBoolError, RMatchIndices,
    RMatches, RSplit, RSplitN, RSplitTerminator, Split, SplitAsciiWhitespace, SplitInclusive,
    SplitN, SplitTerminator, SplitWhitespace, Utf8Chunk, Utf8Chunks, Utf8Error,
};

use amenable_core::Evidence;
#[cfg(kani)]
use amenable_core::{Ensures, Requires};
#[cfg(kani)]
use amenable_std::AsciiByte;
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::{bridge_kani_witness, impl_kani_witness_trusted, kani_ensures};
#[cfg(kani)]
use crate::{
    FourBytesAreEachAscii, IteratorYieldsNoneWhenExhausted, SplitOperandsAreDistinctFromThePattern,
    ThreeBytesAreEachAscii, ThreeSplitOperandsAreDistinctFromThePattern,
};

impl KaniWitness for RustStdStandard<std::str::Bytes<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_bytes_yields_the_utf8_encoding".to_owned(),
            VERIFY_BYTES_YIELDS_THE_UTF8_ENCODING_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::str::Bytes<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::str::Bytes<'static>>",
        "kani",
        || <RustStdStandard<std::str::Bytes<'static>> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<std::str::Bytes<'static>>,
    "amenable_std::rust_std::RustStdStandard<std::str::Bytes<'static>>",
    (Option<u8>, Option<u8>),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_BYTES_YIELDS_THE_UTF8_ENCODING_SRC, {
        /// `.bytes()` yields the UTF-8 encoding of the str, checked for
        /// any single-byte (ASCII) character.
        #[kani::proof]
        fn verify_bytes_yields_the_utf8_encoding() {
            let byte: u8 = kani::any();
            // Canonical home: amenable_std::AsciiByte's Requires<KaniVerifier> impl
            // (rust_std::primitives) names this exact fragment.
            kani::assume(<AsciiByte as Requires<crate::KaniVerifier>>::requires(byte));
            let s = (byte as char).to_string();
            let mut it = s.bytes();
            assert!(
                RustStdStandard::<std::str::Bytes<'static>>::ensures((it.next(), Some(byte))),
                "bytes yields the str's UTF-8 encoding"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<CharIndices<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_char_indices_pairs_each_char_with_its_byte_offset".to_owned(),
            VERIFY_CHAR_INDICES_PAIRS_EACH_CHAR_WITH_ITS_BYTE_OFFSET_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<CharIndices<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<CharIndices<'static>>",
        "kani",
        || <RustStdStandard<CharIndices<'static>> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<CharIndices<'static>>,
    "amenable_std::rust_std::RustStdStandard<CharIndices<'static>>",
    (Option<(usize, char)>, Option<(usize, char)>),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_CHAR_INDICES_PAIRS_EACH_CHAR_WITH_ITS_BYTE_OFFSET_SRC, {
        /// `.char_indices()` pairs the first char with byte offset 0.
        #[kani::proof]
        fn verify_char_indices_pairs_each_char_with_its_byte_offset() {
            let byte: u8 = kani::any();
            // Canonical home: amenable_std::AsciiByte's Requires<KaniVerifier> impl
            // (rust_std::primitives) names this exact fragment.
            kani::assume(<AsciiByte as Requires<crate::KaniVerifier>>::requires(byte));
            let c = byte as char;
            let s = c.to_string();
            let mut it = s.char_indices();
            assert!(
                RustStdStandard::<CharIndices<'static>>::ensures((it.next(), Some((0, c)))),
                "the first char is paired with byte offset 0"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<Chars<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_chars_yields_the_str_characters".to_owned(),
            VERIFY_CHARS_YIELDS_THE_STR_CHARACTERS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Chars<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Chars<'static>>",
        "kani",
        || <RustStdStandard<Chars<'static>> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<Chars<'static>>,
    "amenable_std::rust_std::RustStdStandard<Chars<'static>>",
    (Option<char>, Option<char>),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_CHARS_YIELDS_THE_STR_CHARACTERS_SRC, {
        /// `.chars()` yields the str's characters, for any (symbolic)
        /// single-character str.
        #[kani::proof]
        fn verify_chars_yields_the_str_characters() {
            let byte: u8 = kani::any();
            // Canonical home: amenable_std::AsciiByte's Requires<KaniVerifier> impl
            // (rust_std::primitives) names this exact fragment.
            kani::assume(<AsciiByte as Requires<crate::KaniVerifier>>::requires(byte));
            let c = byte as char;
            let s = c.to_string();
            let mut it = s.chars();
            assert!(
                <RustStdStandard<Chars<'static>> as Ensures<crate::KaniVerifier>>::ensures((
                    it.next(),
                    Some(c)
                )),
                "chars yields the str's characters"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<EncodeUtf16<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_encode_utf16_yields_utf16_code_units".to_owned(),
            VERIFY_ENCODE_UTF16_YIELDS_UTF16_CODE_UNITS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<EncodeUtf16<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<EncodeUtf16<'static>>",
        "kani",
        || <RustStdStandard<EncodeUtf16<'static>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_ENCODE_UTF16_YIELDS_UTF16_CODE_UNITS_SRC, {
        /// `.encode_utf16()` yields UTF-16 code units; for an ASCII
        /// character, the code unit numerically equals the byte.
        #[kani::proof]
        fn verify_encode_utf16_yields_utf16_code_units() {
            let byte: u8 = kani::any();
            // Canonical home: amenable_std::AsciiByte's Requires<KaniVerifier> impl
            // (rust_std::primitives) names this exact fragment.
            kani::assume(<AsciiByte as Requires<crate::KaniVerifier>>::requires(byte));
            let s = (byte as char).to_string();
            let mut it = s.encode_utf16();
            assert_eq!(
                it.next(),
                Some(byte as u16),
                "an ASCII character's UTF-16 code unit equals its byte value"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<std::str::EscapeDebug<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_escape_debug_escapes_control_characters".to_owned(),
            VERIFY_ESCAPE_DEBUG_ESCAPES_CONTROL_CHARACTERS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::str::EscapeDebug<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::str::EscapeDebug<'static>>",
        "kani",
        || <RustStdStandard<std::str::EscapeDebug<'static>> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<std::str::EscapeDebug<'static>>,
    "amenable_std::rust_std::RustStdStandard<std::str::EscapeDebug<'static>>",
    (String, &'static str),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_ESCAPE_DEBUG_ESCAPES_CONTROL_CHARACTERS_SRC, {
        /// `.escape_debug()` renders a newline as the two-character
        /// escape sequence `\n`, matching `Debug`'s formatting.
        #[kani::proof]
        fn verify_escape_debug_escapes_control_characters() {
            let s = "\n";
            let out: String = s.escape_debug().collect();
            assert!(
                RustStdStandard::<std::str::EscapeDebug<'static>>::ensures((out, "\\n")),
                "escape_debug renders \\n as a two-character escape"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<std::str::EscapeDefault<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_escape_default_escapes_control_characters".to_owned(),
            VERIFY_ESCAPE_DEFAULT_ESCAPES_CONTROL_CHARACTERS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::str::EscapeDefault<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::str::EscapeDefault<'static>>",
        "kani",
        || <RustStdStandard<std::str::EscapeDefault<'static>> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<std::str::EscapeDefault<'static>>,
    "amenable_std::rust_std::RustStdStandard<std::str::EscapeDefault<'static>>",
    (String, &'static str),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_ESCAPE_DEFAULT_ESCAPES_CONTROL_CHARACTERS_SRC, {
        /// `.escape_default()` renders a newline the same way a Rust
        /// string literal would: the two-character escape `\n`.
        #[kani::proof]
        fn verify_escape_default_escapes_control_characters() {
            let s = "\n";
            let out: String = s.escape_default().collect();
            assert!(
                RustStdStandard::<std::str::EscapeDefault<'static>>::ensures((out, "\\n")),
                "escape_default renders \\n as a two-character escape"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<std::str::EscapeUnicode<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_escape_unicode_renders_the_codepoint_escape".to_owned(),
            VERIFY_ESCAPE_UNICODE_RENDERS_THE_CODEPOINT_ESCAPE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::str::EscapeUnicode<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::str::EscapeUnicode<'static>>",
        "kani",
        || <RustStdStandard<std::str::EscapeUnicode<'static>> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<std::str::EscapeUnicode<'static>>,
    "amenable_std::rust_std::RustStdStandard<std::str::EscapeUnicode<'static>>",
    (String, &'static str),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_ESCAPE_UNICODE_RENDERS_THE_CODEPOINT_ESCAPE_SRC, {
        /// `.escape_unicode()` renders every character as a
        /// `\u{...}` codepoint escape, even a plain ASCII letter.
        #[kani::proof]
        fn verify_escape_unicode_renders_the_codepoint_escape() {
            let s = "a";
            let out: String = s.escape_unicode().collect();
            assert!(
                RustStdStandard::<std::str::EscapeUnicode<'static>>::ensures((out, "\\u{61}")),
                "escape_unicode renders every char as \\u{...}"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<std::str::Lines<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_lines_splits_on_line_endings".to_owned(),
            VERIFY_LINES_SPLITS_ON_LINE_ENDINGS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::str::Lines<'static>>);

kani_ensures!(
    RustStdStandard<std::str::Lines<'static>>,
    "amenable_std::rust_std::RustStdStandard<std::str::Lines<'static>>",
    (Option<&'static str>, Option<&'static str>),
    |(actual, expected)| actual == expected
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::str::Lines<'static>>",
        "kani",
        || <RustStdStandard<std::str::Lines<'static>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_LINES_SPLITS_ON_LINE_ENDINGS_SRC, {
        /// `.lines()` splits on `\n`, without yielding a trailing empty
        /// line.
        #[kani::proof]
        fn verify_lines_splits_on_line_endings() {
            let s = "a\nb";
            let mut it = s.lines();
            assert!(RustStdStandard::<std::str::Lines<'static>>::ensures((it.next(), Some("a"))));
            assert!(RustStdStandard::<std::str::Lines<'static>>::ensures((it.next(), Some("b"))));
            assert!(IteratorYieldsNoneWhenExhausted::ensures(it.next()));
        }
    }
}

/// A collected sequence known to match exactly the expected sequence.
///
/// Independently hand-written as `assert_eq!(collected, vec![...],
/// ...)` at 8 real sites spanning `Vec<i32>` (a drained `VecDeque`, a
/// rejected `try_reserve`'s untouched content, `Vec::extract_if`'s two
/// halves), `Vec<u16>` (`encode_wide`'s UTF-16 units), and `Vec<&str>`
/// (`split_ascii_whitespace`/`split_whitespace`/`splitn`'s parts), plus
/// 3 more sites in `rust_std::alloc_string` over `Vec<u8>`
/// (`FromUtf8Error::into_bytes` recovering the original owned bytes) --
/// the identical claim regardless of collected element type. Generic over
/// the collected sequence type rather than one registration per
/// producer, the same reasoning (and the same reason it needs a
/// hand-written `Witness`/`Ensures` impl instead of the
/// `bridge_kani_witness!`/`kani_ensures!` macros) as
/// `DerefReflectsTheStoredValue` (`rust_std::primitives`) -- not a
/// reuse of that type directly, since its name states a deref claim
/// this isn't.
pub struct CollectedSequenceMatchesExpected<T>(std::marker::PhantomData<T>);

impl<T> amenable_core::Standard for CollectedSequenceMatchesExpected<T> {
    type Provenance = amenable_std::RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn provenance(&self) -> Self::Provenance {
        <i32 as amenable_std::RustStdType>::provenance()
    }
}

impl<T> Evidence for CollectedSequenceMatchesExpected<T> {
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

impl<T> KaniWitness for CollectedSequenceMatchesExpected<T> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_split_ascii_whitespace_collapses_runs_of_whitespace".to_owned(),
            VERIFY_SPLIT_ASCII_WHITESPACE_COLLAPSES_RUNS_OF_WHITESPACE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

impl<T> amenable_core::Witness<crate::KaniVerifier> for CollectedSequenceMatchesExpected<T> {
    type SupportingEvidence = <Self as KaniWitness>::SupportingEvidence;
    type ProofArtifact = <Self as KaniWitness>::ProofArtifact;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        <Self as KaniWitness>::proof()
    }
}

impl<T: PartialEq> amenable_core::Ensures<crate::KaniVerifier>
    for CollectedSequenceMatchesExpected<T>
{
    type Input = (T, T);
    type Bound = bool;

    fn ensures((actual, expected): (T, T)) -> bool {
        actual == expected
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_kani::CollectedSequenceMatchesExpected",
        "kani",
        "ensures",
        || stringify!(actual == expected),
    )
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::CollectedSequenceMatchesExpected",
        "kani",
        || <CollectedSequenceMatchesExpected<i32> as KaniWitness>::proof().to_string(),
    )
}

impl KaniWitness for RustStdStandard<SplitAsciiWhitespace<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_split_ascii_whitespace_collapses_runs_of_whitespace".to_owned(),
            VERIFY_SPLIT_ASCII_WHITESPACE_COLLAPSES_RUNS_OF_WHITESPACE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<SplitAsciiWhitespace<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<SplitAsciiWhitespace<'static>>",
        "kani",
        || <RustStdStandard<SplitAsciiWhitespace<'static>> as KaniWitness>::proof()
            .to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_SPLIT_ASCII_WHITESPACE_COLLAPSES_RUNS_OF_WHITESPACE_SRC, {
        /// `.split_ascii_whitespace()` collapses runs of whitespace and
        /// drops leading/trailing whitespace entirely.
        #[kani::proof]
        fn verify_split_ascii_whitespace_collapses_runs_of_whitespace() {
            let s = " a  b ";
            let parts: Vec<&str> = s.split_ascii_whitespace().collect();
            assert!(
                CollectedSequenceMatchesExpected::ensures((parts, vec!["a", "b"])),
                "runs of whitespace collapse to a single split point"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<SplitWhitespace<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_split_whitespace_collapses_runs_of_whitespace".to_owned(),
            VERIFY_SPLIT_WHITESPACE_COLLAPSES_RUNS_OF_WHITESPACE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<SplitWhitespace<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<SplitWhitespace<'static>>",
        "kani",
        || <RustStdStandard<SplitWhitespace<'static>> as KaniWitness>::proof()
            .to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_SPLIT_WHITESPACE_COLLAPSES_RUNS_OF_WHITESPACE_SRC, {
        /// Same collapsing behavior as `SplitAsciiWhitespace`, over
        /// Unicode whitespace.
        #[kani::proof]
        fn verify_split_whitespace_collapses_runs_of_whitespace() {
            let s = " a  b ";
            let parts: Vec<&str> = s.split_whitespace().collect();
            assert!(
                CollectedSequenceMatchesExpected::ensures((parts, vec!["a", "b"])),
                "runs of whitespace collapse to a single split point"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<Utf8Chunks<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_utf8_chunks_yields_one_chunk_for_wholly_valid_input".to_owned(),
            VERIFY_UTF8_CHUNKS_YIELDS_ONE_CHUNK_FOR_WHOLLY_VALID_INPUT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Utf8Chunks<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Utf8Chunks<'static>>",
        "kani",
        || <RustStdStandard<Utf8Chunks<'static>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_UTF8_CHUNKS_YIELDS_ONE_CHUNK_FOR_WHOLLY_VALID_INPUT_SRC, {
        /// Re-validating wholly valid UTF-8 bytes yields exactly one
        /// chunk, with no trailing invalid bytes and nothing left over.
        #[kani::proof]
        fn verify_utf8_chunks_yields_one_chunk_for_wholly_valid_input() {
            let bytes = b"ab";
            let mut chunks = bytes.utf8_chunks();
            let first = chunks.next().unwrap();
            assert!(AccessorRecoversTheExpectedValue::ensures((first.valid(), "ab")));
            assert!(first.invalid().is_empty(), "wholly valid input has no invalid bytes");
            assert!(
                IteratorYieldsNoneWhenExhausted::ensures(chunks.next()),
                "wholly valid input is exactly one chunk"
            );
        }
    }
}

/// An `(actual, expected)` pair known to agree: a plain accessor
/// method call recovers exactly the expected value -- distinct from a
/// field access (`FieldAccessRecoversTheStoredValue`), an index
/// (`IndexRecoversTheStoredElement`), or an iterator's `.next()`
/// (`IteratorYieldsAReferenceToTheStoredValue`) even though the
/// `Ensures` impl body is identical trivial equality either way, same
/// reasoning as keeping those types separate from each other.
///
/// Independently hand-written as `assert_eq!(chunk.valid(), "ab", ...)`
/// / `assert_eq!(chunk.invalid(), &[0xFFu8][..], ...)` at 2 real sites
/// in `Utf8Chunk`'s own `.valid()`/`.invalid()` accessors.
pub struct AccessorRecoversTheExpectedValue<T>(std::marker::PhantomData<T>);

impl<T> amenable_core::Standard for AccessorRecoversTheExpectedValue<T> {
    type Provenance = amenable_std::RustStdProvenance;

    fn provenance(&self) -> Self::Provenance {
        <i32 as amenable_std::RustStdType>::provenance()
    }
}

impl<T> Evidence for AccessorRecoversTheExpectedValue<T> {
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

impl<T> KaniWitness for AccessorRecoversTheExpectedValue<T> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_utf8_chunk_separates_the_valid_prefix_from_invalid_bytes".to_owned(),
            VERIFY_UTF8_CHUNK_SEPARATES_THE_VALID_PREFIX_FROM_INVALID_BYTES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

impl<T> amenable_core::Witness<crate::KaniVerifier> for AccessorRecoversTheExpectedValue<T> {
    type SupportingEvidence = <Self as KaniWitness>::SupportingEvidence;
    type ProofArtifact = <Self as KaniWitness>::ProofArtifact;

    fn proof() -> Self::ProofArtifact {
        <Self as KaniWitness>::proof()
    }
}

impl<T: PartialEq> amenable_core::Ensures<crate::KaniVerifier>
    for AccessorRecoversTheExpectedValue<T>
{
    type Input = (T, T);
    type Bound = bool;

    fn ensures((actual, expected): (T, T)) -> bool {
        actual == expected
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_kani::AccessorRecoversTheExpectedValue",
        "kani",
        "ensures",
        || stringify!(actual == expected),
    )
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::AccessorRecoversTheExpectedValue",
        "kani",
        || <AccessorRecoversTheExpectedValue<i32> as KaniWitness>::proof().to_string(),
    )
}

impl KaniWitness for RustStdStandard<Utf8Chunk<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_utf8_chunk_separates_the_valid_prefix_from_invalid_bytes".to_owned(),
            VERIFY_UTF8_CHUNK_SEPARATES_THE_VALID_PREFIX_FROM_INVALID_BYTES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Utf8Chunk<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Utf8Chunk<'static>>",
        "kani",
        || <RustStdStandard<Utf8Chunk<'static>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_UTF8_CHUNK_SEPARATES_THE_VALID_PREFIX_FROM_INVALID_BYTES_SRC, {
        /// A `Utf8Chunk`'s `.valid()`/`.invalid()` split a byte
        /// sequence containing one bad byte into its valid-UTF-8 prefix
        /// and the trailing invalid byte.
        #[kani::proof]
        fn verify_utf8_chunk_separates_the_valid_prefix_from_invalid_bytes() {
            let bytes = b"ab\xFFcd";
            let mut chunks = bytes.utf8_chunks();
            let first = chunks.next().unwrap();
            assert!(
                AccessorRecoversTheExpectedValue::ensures((first.valid(), "ab")),
                "the chunk's valid() is the UTF-8 prefix before the bad byte"
            );
            assert!(
                AccessorRecoversTheExpectedValue::ensures((first.invalid(), &[0xFFu8][..])),
                "the chunk's invalid() is the bad byte itself"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<Utf8Error> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_utf8_error_reports_the_valid_prefix_length_and_error_span".to_owned(),
            VERIFY_UTF8_ERROR_REPORTS_THE_VALID_PREFIX_LENGTH_AND_ERROR_SPAN_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Utf8Error>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Utf8Error>",
        "kani",
        || <RustStdStandard<Utf8Error> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_UTF8_ERROR_REPORTS_THE_VALID_PREFIX_LENGTH_AND_ERROR_SPAN_SRC, {
        /// `str::from_utf8`'s error reports exactly how much of the
        /// input was valid (`valid_up_to`) and the width of the single
        /// bad byte (`error_len`).
        /// This proof uses the Amenable-owned UTF-8 accommodation model
        /// (`utf8_model.rs`'s `KaniUtf8::error_position`, which exposes the
        /// same `valid_up_to`/`error_len` position information as
        /// `std::str::Utf8Error`): the direct `std::str::from_utf8` path
        /// times out even for fixed concrete literals (see
        /// `gallery::replace_recommendations::from_utf8_error_times_out_even_for_a_fixed_two_byte_invalid_vector`).
        /// `0xF5..=0xFF` are never valid UTF-8 lead bytes anywhere, so any
        /// byte in that range is a lone one-byte error regardless of its
        /// neighbors.
        #[kani::proof]
        fn verify_utf8_error_reports_the_valid_prefix_length_and_error_span() {
            let invalid: u8 = kani::any();
            kani::assume(invalid >= 0xF5);
            let bytes = [b'a', b'b', invalid, b'c'];
            let err = crate::KaniUtf8::error_position(&bytes).unwrap_err();
            assert_eq!(err.valid_up_to(), 2, "two leading bytes were valid UTF-8");
            assert_eq!(err.error_len(), Some(1), "the single bad byte has error_len 1");
        }
    }
}

impl_kani_witness_trusted!(ParseBoolError);

// `LinesAny` is deprecated (in favor of `Lines`) but still stable and real;
// covering it is a coverage-completeness question, not a call to use it.
// `#[expect(deprecated)]` on a macro invocation is silently ignored by
// rustc (confirmed empirically) rather than suppressing warnings from
// inside its expansion, so `bridge_kani_witness!`/`inventory::submit!`
// below would otherwise warn on every mention of the type name. Routing
// through this locally `#[expect(deprecated)]`-attributed alias hides the
// deprecation at every downstream use site instead, confirmed the same way.
#[expect(
    deprecated,
    reason = "LinesAny is stable, only deprecated in favor of Lines; covering it is a coverage-completeness question, not a call to use it"
)]
type LinesAnyStatic = std::str::LinesAny<'static>;

impl KaniWitness for RustStdStandard<LinesAnyStatic> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_lines_any_splits_on_any_line_ending".to_owned(),
            VERIFY_LINES_ANY_SPLITS_ON_ANY_LINE_ENDING_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<LinesAnyStatic>);

kani_ensures!(
    RustStdStandard<LinesAnyStatic>,
    "amenable_std::rust_std::RustStdStandard<LinesAnyStatic>",
    (Option<&'static str>, Option<&'static str>),
    |(actual, expected)| actual == expected
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<LinesAny<'static>>",
        "kani",
        || <RustStdStandard<LinesAnyStatic> as KaniWitness>::proof()
            .to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_LINES_ANY_SPLITS_ON_ANY_LINE_ENDING_SRC, {
        /// Unlike `Lines` (which splits on `\n` alone, treating a
        /// preceding `\r` as part of the line's own trimming),
        /// `LinesAny` is documented to split on either `\n` or `\r\n` —
        /// deprecated in favor of `Lines`, but a real, distinct, still-
        /// stable carrier worth covering. Deprecated: only the call
        /// site needs `#[expect(deprecated)]`, not the whole module.
        #[expect(
            deprecated,
            reason = "LinesAny is stable, only deprecated in favor of Lines; covering it is a coverage-completeness question, not a call to use it"
        )]
        #[kani::proof]
        fn verify_lines_any_splits_on_any_line_ending() {
            let s = "a\r\nb";
            let mut it = s.lines_any();
            assert!(RustStdStandard::<LinesAnyStatic>::ensures((it.next(), Some("a"))));
            assert!(RustStdStandard::<LinesAnyStatic>::ensures((it.next(), Some("b"))));
            assert!(IteratorYieldsNoneWhenExhausted::ensures(it.next()));
        }
    }
}

impl KaniWitness for RustStdStandard<Split<'static, char>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_split_yields_substrings_between_pattern_matches".to_owned(),
            VERIFY_SPLIT_YIELDS_SUBSTRINGS_BETWEEN_PATTERN_MATCHES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Split<'static, char>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::str::Split<'static, char>>",
        "kani",
        || <RustStdStandard<Split<'static, char>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_SPLIT_YIELDS_SUBSTRINGS_BETWEEN_PATTERN_MATCHES_SRC, {
        /// `.split(pat)` yields the substrings between matches, in
        /// forward order.
        #[kani::proof]
        fn verify_split_yields_substrings_between_pattern_matches() {
            let parts: Vec<&str> = "a,b,c".split(',').collect();
            assert!(
                CollectedSequenceMatchesExpected::ensures((parts, vec!["a", "b", "c"])),
                "split yields substrings between matches, forward"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<SplitN<'static, char>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_splitn_limits_to_n_substrings".to_owned(),
            VERIFY_SPLITN_LIMITS_TO_N_SUBSTRINGS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<SplitN<'static, char>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::str::SplitN<'static, char>>",
        "kani",
        || <RustStdStandard<SplitN<'static, char>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_SPLITN_LIMITS_TO_N_SUBSTRINGS_SRC, {
        /// `.splitn(n, pat)` stops after `n` substrings, leaving the
        /// remainder of the str unsplit in the final item.
        #[kani::proof]
        fn verify_splitn_limits_to_n_substrings() {
            let parts: Vec<&str> = "a,b,c".splitn(2, ',').collect();
            assert!(
                CollectedSequenceMatchesExpected::ensures((parts, vec!["a", "b,c"])),
                "splitn stops after n items, leaving the remainder unsplit"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<SplitInclusive<'static, char>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_split_inclusive_keeps_the_delimiter_attached".to_owned(),
            VERIFY_SPLIT_INCLUSIVE_KEEPS_THE_DELIMITER_ATTACHED_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<SplitInclusive<'static, char>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::str::SplitInclusive<'static, char>>",
        "kani",
        || <RustStdStandard<SplitInclusive<'static, char>> as KaniWitness>::proof()
            .to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_SPLIT_INCLUSIVE_KEEPS_THE_DELIMITER_ATTACHED_SRC, {
        /// `.split_inclusive(pat)` keeps each matched delimiter attached
        /// to the end of the substring that precedes it.
        #[kani::proof]
        fn verify_split_inclusive_keeps_the_delimiter_attached() {
            let parts: Vec<&str> = "a,b,c".split_inclusive(',').collect();
            assert!(
                CollectedSequenceMatchesExpected::ensures((parts, vec!["a,", "b,", "c"])),
                "split_inclusive keeps the delimiter attached to each substring"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<RSplit<'static, char>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_rsplit_yields_substrings_from_the_back".to_owned(),
            VERIFY_RSPLIT_YIELDS_SUBSTRINGS_FROM_THE_BACK_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<RSplit<'static, char>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::str::RSplit<'static, char>>",
        "kani",
        || <RustStdStandard<RSplit<'static, char>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_RSPLIT_YIELDS_SUBSTRINGS_FROM_THE_BACK_SRC, {
        /// `.rsplit(pat)` yields the piece after the match, then the
        /// piece before it. This proof uses the Amenable-owned bounded
        /// str-pattern accommodation model (`KaniStrRSplitObservation`,
        /// symbolic over all three ASCII positions): if the real
        /// `rsplit` path refines this one-occurrence
        /// `[before, pattern, after]` window, the Rust-facing claim
        /// follows. The direct path times out under Kani even for a
        /// single `.next()` call on a fixed five-byte str -- see
        /// `gallery::replace_recommendations::str_rsplit_reverse_pattern_search_times_out_even_for_a_single_next_call`.
        #[kani::proof]
        fn verify_rsplit_yields_substrings_from_the_back() {
            let before: u8 = kani::any();
            let pattern: u8 = kani::any();
            let after: u8 = kani::any();
            // Canonical home: amenable_std::AsciiByte's Requires<KaniVerifier>
            // impl (rust_std::primitives, supplementary fragment) names this exact fragment.
            kani::assume(ThreeBytesAreEachAscii::requires((before, pattern, after)));
            kani::assume(SplitOperandsAreDistinctFromThePattern::requires((
                before, pattern, after,
            )));
            let observation =
                crate::KaniStrRSplitObservation::new(before as char, pattern as char, after as char);
            assert_eq!(
                observation.rsplit(),
                [after as char, before as char],
                "rsplit yields the piece after the match, then the piece before it"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<RSplitN<'static, char>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_rsplitn_limits_to_n_substrings_from_the_back".to_owned(),
            VERIFY_RSPLITN_LIMITS_TO_N_SUBSTRINGS_FROM_THE_BACK_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<RSplitN<'static, char>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::str::RSplitN<'static, char>>",
        "kani",
        || <RustStdStandard<RSplitN<'static, char>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_RSPLITN_LIMITS_TO_N_SUBSTRINGS_FROM_THE_BACK_SRC, {
        /// `.rsplitn(2, pat)` yields the piece after the last match,
        /// then everything before it uncut. This proof uses the
        /// Amenable-owned bounded str-pattern accommodation model
        /// (`KaniStrRSplitNObservation`, symbolic over all four ASCII
        /// positions): if the real `rsplitn` path refines this
        /// two-occurrence `[a, pattern, b, pattern, c]` window, the
        /// Rust-facing claim follows. The direct path hits the same
        /// reverse-search timeout as `RSplit` -- see
        /// `gallery::replace_recommendations::str_rsplit_reverse_pattern_search_times_out_even_for_a_single_next_call`.
        #[kani::proof]
        fn verify_rsplitn_limits_to_n_substrings_from_the_back() {
            let a: u8 = kani::any();
            let pattern: u8 = kani::any();
            let b: u8 = kani::any();
            let c: u8 = kani::any();
            // Canonical home: amenable_std::AsciiByte's Requires<KaniVerifier>
            // impl (rust_std::primitives, supplementary fragment) names this exact fragment.
            kani::assume(FourBytesAreEachAscii::requires((a, pattern, b, c)));
            kani::assume(ThreeSplitOperandsAreDistinctFromThePattern::requires((
                a, b, c, pattern,
            )));
            let observation =
                crate::KaniStrRSplitNObservationBuilder::default().a(a as char).pattern(pattern as char).b(b as char).c(c as char).build().expect("all fields set");
            let (first, rest) = observation.rsplitn_two();
            assert_eq!(first, c as char, "rsplitn's first piece is everything after the last match");
            assert_eq!(
                rest,
                [a as char, pattern as char, b as char],
                "rsplitn's second piece is everything before the last match, uncut"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<SplitTerminator<'static, char>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_split_terminator_suppresses_a_trailing_empty_substring".to_owned(),
            VERIFY_SPLIT_TERMINATOR_SUPPRESSES_A_TRAILING_EMPTY_SUBSTRING_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<SplitTerminator<'static, char>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<SplitTerminator<'static, char>>",
        "kani",
        || <RustStdStandard<SplitTerminator<'static, char>> as KaniWitness>::proof()
            .to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_SPLIT_TERMINATOR_SUPPRESSES_A_TRAILING_EMPTY_SUBSTRING_SRC, {
        /// `.split_terminator(pat)` treats the pattern as a terminator
        /// rather than a separator: a match at the very end of the str
        /// does not produce a trailing empty substring. This proof uses
        /// the Amenable-owned bounded str-pattern accommodation model
        /// (`KaniStrSplitTerminatorObservation`, symbolic over all three
        /// ASCII positions): if the real `split_terminator` path refines
        /// this two-occurrence, nothing-after-the-last-match
        /// `[a, pattern, b, pattern]` window, the Rust-facing claim
        /// follows. The direct path times out for real inside this
        /// crate despite passing in an isolated probe crate -- see
        /// `gallery::replace_recommendations::str_split_terminator_matches_forward_pattern_iteration_times_out_in_the_real_crate`.
        #[kani::proof]
        fn verify_split_terminator_suppresses_a_trailing_empty_substring() {
            let a: u8 = kani::any();
            let pattern: u8 = kani::any();
            let b: u8 = kani::any();
            // Canonical home: amenable_std::AsciiByte's Requires<KaniVerifier>
            // impl (rust_std::primitives, supplementary fragment) names this exact fragment.
            kani::assume(ThreeBytesAreEachAscii::requires((a, pattern, b)));
            kani::assume(SplitOperandsAreDistinctFromThePattern::requires((
                a, pattern, b,
            )));
            let observation =
                crate::KaniStrSplitTerminatorObservation::new(a as char, pattern as char, b as char);
            assert_eq!(
                observation.split_terminator(),
                [a as char, b as char],
                "no trailing empty substring after the terminal match"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<RSplitTerminator<'static, char>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_rsplit_terminator_suppresses_a_trailing_empty_substring_from_the_back"
                .to_owned(),
            VERIFY_RSPLIT_TERMINATOR_SUPPRESSES_A_TRAILING_EMPTY_SUBSTRING_FROM_THE_BACK_SRC
                .to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<RSplitTerminator<'static, char>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<RSplitTerminator<'static, char>>",
        "kani",
        || <RustStdStandard<RSplitTerminator<'static, char>> as KaniWitness>::proof()
            .to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_RSPLIT_TERMINATOR_SUPPRESSES_A_TRAILING_EMPTY_SUBSTRING_FROM_THE_BACK_SRC, {
        /// Same terminator-suppression behavior as `SplitTerminator`,
        /// traversed from the back. This proof uses the same
        /// `KaniStrSplitTerminatorObservation` model as `SplitTerminator`
        /// -- see that proof's doc comment.
        #[kani::proof]
        fn verify_rsplit_terminator_suppresses_a_trailing_empty_substring_from_the_back() {
            let a: u8 = kani::any();
            let pattern: u8 = kani::any();
            let b: u8 = kani::any();
            // Canonical home: amenable_std::AsciiByte's Requires<KaniVerifier>
            // impl (rust_std::primitives, supplementary fragment) names this exact fragment.
            kani::assume(ThreeBytesAreEachAscii::requires((a, pattern, b)));
            kani::assume(SplitOperandsAreDistinctFromThePattern::requires((
                a, pattern, b,
            )));
            let observation =
                crate::KaniStrSplitTerminatorObservation::new(a as char, pattern as char, b as char);
            assert_eq!(
                observation.rsplit_terminator(),
                [b as char, a as char],
                "no trailing empty substring after the terminal match, traversed from the back"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<Matches<'static, char>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_matches_yields_every_non_overlapping_occurrence".to_owned(),
            VERIFY_MATCHES_YIELDS_EVERY_NON_OVERLAPPING_OCCURRENCE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Matches<'static, char>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Matches<'static, char>>",
        "kani",
        || <RustStdStandard<Matches<'static, char>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_MATCHES_YIELDS_EVERY_NON_OVERLAPPING_OCCURRENCE_SRC, {
        /// `.matches(pat)` yields every non-overlapping occurrence of
        /// the pattern. This proof uses the Amenable-owned bounded
        /// str-pattern accommodation model (`KaniStrMatchObservation`,
        /// symbolic over all three filler ASCII positions and the
        /// pattern itself): if the real `matches` path refines this
        /// two-occurrence `[f0, pattern, f1, pattern, f2]` window, the
        /// Rust-facing claim follows. The direct path times out for
        /// real inside this crate despite passing in an isolated probe
        /// crate -- see
        /// `gallery::replace_recommendations::str_split_terminator_matches_forward_pattern_iteration_times_out_in_the_real_crate`.
        #[kani::proof]
        fn verify_matches_yields_every_non_overlapping_occurrence() {
            let f0: u8 = kani::any();
            let pattern: u8 = kani::any();
            let f1: u8 = kani::any();
            let f2: u8 = kani::any();
            // Canonical home: amenable_std::AsciiByte's Requires<KaniVerifier>
            // impl (rust_std::primitives, supplementary fragment) names this exact fragment.
            kani::assume(FourBytesAreEachAscii::requires((f0, pattern, f1, f2)));
            kani::assume(ThreeSplitOperandsAreDistinctFromThePattern::requires((
                f0, f1, f2, pattern,
            )));
            let observation =
                crate::KaniStrMatchObservationBuilder::default().f0(f0 as char).pattern(pattern as char).f1(f1 as char).f2(f2 as char).build().expect("all fields set");
            assert_eq!(
                observation.matches(),
                [pattern as char, pattern as char],
                "matches finds every non-overlapping occurrence"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<RMatches<'static, char>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_rmatches_yields_every_non_overlapping_occurrence".to_owned(),
            VERIFY_RMATCHES_YIELDS_EVERY_NON_OVERLAPPING_OCCURRENCE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<RMatches<'static, char>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<RMatches<'static, char>>",
        "kani",
        || <RustStdStandard<RMatches<'static, char>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_RMATCHES_YIELDS_EVERY_NON_OVERLAPPING_OCCURRENCE_SRC, {
        /// Same non-overlapping-occurrence behavior as `Matches`,
        /// traversed from the back; content alone can't distinguish
        /// traversal order for a `char` pattern (see this module's own
        /// doc). This proof uses the same `KaniStrMatchObservation`
        /// model as `Matches` -- see that proof's doc comment.
        #[kani::proof]
        fn verify_rmatches_yields_every_non_overlapping_occurrence() {
            let f0: u8 = kani::any();
            let pattern: u8 = kani::any();
            let f1: u8 = kani::any();
            let f2: u8 = kani::any();
            // Canonical home: amenable_std::AsciiByte's Requires<KaniVerifier>
            // impl (rust_std::primitives, supplementary fragment) names this exact fragment.
            kani::assume(FourBytesAreEachAscii::requires((f0, pattern, f1, f2)));
            kani::assume(ThreeSplitOperandsAreDistinctFromThePattern::requires((
                f0, f1, f2, pattern,
            )));
            let observation =
                crate::KaniStrMatchObservationBuilder::default().f0(f0 as char).pattern(pattern as char).f1(f1 as char).f2(f2 as char).build().expect("all fields set");
            assert_eq!(
                observation.rmatches(),
                [pattern as char, pattern as char],
                "rmatches finds every non-overlapping occurrence"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<MatchIndices<'static, char>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_match_indices_pairs_each_match_with_its_byte_offset".to_owned(),
            VERIFY_MATCH_INDICES_PAIRS_EACH_MATCH_WITH_ITS_BYTE_OFFSET_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<MatchIndices<'static, char>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<MatchIndices<'static, char>>",
        "kani",
        || <RustStdStandard<MatchIndices<'static, char>> as KaniWitness>::proof()
            .to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_MATCH_INDICES_PAIRS_EACH_MATCH_WITH_ITS_BYTE_OFFSET_SRC, {
        /// `.match_indices(pat)` pairs each match with its byte offset,
        /// in forward (left-to-right) order. This proof uses the same
        /// `KaniStrMatchObservation` model as `Matches` -- see that
        /// proof's doc comment. The window's fixed byte offsets (1 and
        /// 3) hold because every field is exactly one ASCII byte.
        #[kani::proof]
        fn verify_match_indices_pairs_each_match_with_its_byte_offset() {
            let f0: u8 = kani::any();
            let pattern: u8 = kani::any();
            let f1: u8 = kani::any();
            let f2: u8 = kani::any();
            // Canonical home: amenable_std::AsciiByte's Requires<KaniVerifier>
            // impl (rust_std::primitives, supplementary fragment) names this exact fragment.
            kani::assume(FourBytesAreEachAscii::requires((f0, pattern, f1, f2)));
            kani::assume(ThreeSplitOperandsAreDistinctFromThePattern::requires((
                f0, f1, f2, pattern,
            )));
            let observation =
                crate::KaniStrMatchObservationBuilder::default().f0(f0 as char).pattern(pattern as char).f1(f1 as char).f2(f2 as char).build().expect("all fields set");
            assert_eq!(
                observation.match_indices(),
                [(1, pattern as char), (3, pattern as char)],
                "match_indices pairs each match with its byte offset, forward"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<RMatchIndices<'static, char>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_rmatch_indices_pairs_each_match_with_its_byte_offset_from_the_back".to_owned(),
            VERIFY_RMATCH_INDICES_PAIRS_EACH_MATCH_WITH_ITS_BYTE_OFFSET_FROM_THE_BACK_SRC
                .to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<RMatchIndices<'static, char>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<RMatchIndices<'static, char>>",
        "kani",
        || <RustStdStandard<RMatchIndices<'static, char>> as KaniWitness>::proof()
            .to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_RMATCH_INDICES_PAIRS_EACH_MATCH_WITH_ITS_BYTE_OFFSET_FROM_THE_BACK_SRC, {
        /// `.rmatch_indices(pat)` pairs each match with its byte offset,
        /// same set as `match_indices` but in reverse (right-to-left)
        /// order -- the one place in this cluster where forward/reverse
        /// traversal is directly assertable by value, since the byte
        /// offset (unlike the matched substring itself) differs per
        /// occurrence. This proof uses the same `KaniStrMatchObservation`
        /// model as `Matches` -- see that proof's doc comment.
        #[kani::proof]
        fn verify_rmatch_indices_pairs_each_match_with_its_byte_offset_from_the_back() {
            let f0: u8 = kani::any();
            let pattern: u8 = kani::any();
            let f1: u8 = kani::any();
            let f2: u8 = kani::any();
            // Canonical home: amenable_std::AsciiByte's Requires<KaniVerifier>
            // impl (rust_std::primitives, supplementary fragment) names this exact fragment.
            kani::assume(FourBytesAreEachAscii::requires((f0, pattern, f1, f2)));
            kani::assume(ThreeSplitOperandsAreDistinctFromThePattern::requires((
                f0, f1, f2, pattern,
            )));
            let observation =
                crate::KaniStrMatchObservationBuilder::default().f0(f0 as char).pattern(pattern as char).f1(f1 as char).f2(f2 as char).build().expect("all fields set");
            assert_eq!(
                observation.rmatch_indices(),
                [(3, pattern as char), (1, pattern as char)],
                "rmatch_indices pairs each match with its byte offset, in reverse"
            );
        }
    }
}
