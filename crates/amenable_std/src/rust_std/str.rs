//! `RustStdType` registrations for `core::str`.
//!
//! `core::str::pattern::*` is deliberately not covered here — the whole
//! `Pattern`/`Searcher` API sits behind the unstable `pattern` feature, and
//! `Pattern` can't be named in a trait bound from a stable toolchain
//! (confirmed empirically). But that unnameability only blocks an
//! *unconstrained* impl generic over `P`; it doesn't block a concrete
//! instantiation. Every iterator type generic over a `Pattern`
//! (`MatchIndices`, `Matches`, `RMatchIndices`, `RMatches`, `RSplit`,
//! `RSplitN`, `RSplitTerminator`, `Split`, `SplitInclusive`, `SplitN`,
//! `SplitTerminator`) *is* covered below, monomorphized on `char` — the
//! same representative concrete pattern this module already uses
//! elsewhere — via [`impl_rust_std_type_lifetime1_concrete`], which fixes
//! the inner type parameter instead of leaving it unconstrained.

use std::str::{
    Bytes, CharIndices, Chars, EncodeUtf16, EscapeDebug, EscapeDefault, EscapeUnicode, Lines,
    MatchIndices, Matches, ParseBoolError, RMatchIndices, RMatches, RSplit, RSplitN,
    RSplitTerminator, Split, SplitAsciiWhitespace, SplitInclusive, SplitN, SplitTerminator,
    SplitWhitespace, Utf8Chunk, Utf8Chunks, Utf8Error,
};

use crate::rust_std::macros::{
    impl_rust_std_type, impl_rust_std_type_lifetime0, impl_rust_std_type_lifetime1_concrete,
    register_rust_std_standard_evidence,
};

macro_rules! str_iter0 {
    ($ty:ident, $summary:expr) => {
        impl_rust_std_type_lifetime0!(
            $ty,
            "core",
            "core::str",
            concat!(
                "https://doc.rust-lang.org/core/str/struct.",
                stringify!($ty),
                ".html"
            ),
            $summary
        );
    };
}

str_iter0!(
    Bytes,
    "The Bytes carrier lazily yields the bytes of a str's UTF-8 encoding."
);
str_iter0!(
    CharIndices,
    "The CharIndices carrier lazily yields each char of a str paired with its byte offset."
);
str_iter0!(Chars, "The Chars carrier lazily yields the chars of a str.");
str_iter0!(
    EncodeUtf16,
    "The EncodeUtf16 carrier lazily yields the UTF-16 code units of a str."
);
str_iter0!(
    EscapeDebug,
    "The EscapeDebug carrier lazily yields a str's characters, Debug-escaped."
);
str_iter0!(
    EscapeDefault,
    "The EscapeDefault carrier lazily yields a str's characters, escaped as in a Rust string literal."
);
str_iter0!(
    EscapeUnicode,
    "The EscapeUnicode carrier lazily yields a str's characters as \\u{{...}} escape sequences."
);
str_iter0!(
    Lines,
    "The Lines carrier lazily yields the lines of a str, split on line endings."
);
str_iter0!(
    SplitAsciiWhitespace,
    "The SplitAsciiWhitespace carrier lazily splits a str on runs of ASCII whitespace."
);
str_iter0!(
    SplitWhitespace,
    "The SplitWhitespace carrier lazily splits a str on runs of Unicode whitespace."
);
str_iter0!(
    Utf8Chunk,
    "The Utf8Chunk carrier holds one valid-UTF-8 run plus any trailing invalid bytes found while re-validating a byte slice."
);
str_iter0!(
    Utf8Chunks,
    "The Utf8Chunks carrier lazily re-validates a byte slice as UTF-8, yielding valid runs interspersed with error markers."
);

impl_rust_std_type!(
    ParseBoolError,
    "core",
    "core::str",
    "https://doc.rust-lang.org/core/str/struct.ParseBoolError.html",
    "The ParseBoolError carrier reports that a string could not be parsed as a bool."
);

impl_rust_std_type!(
    Utf8Error,
    "core",
    "core::str",
    "https://doc.rust-lang.org/core/str/struct.Utf8Error.html",
    "The Utf8Error carrier reports that a byte slice was not valid UTF-8, along with how much of it was."
);

// Hand-written rather than via impl_rust_std_type_lifetime0!: #[expect(deprecated)]
// needs to attach to the actual impl item, not a macro invocation.
#[expect(
    deprecated,
    reason = "LinesAny is stable, only deprecated in favor of Lines; covering it is a coverage-completeness question, not a call to use it"
)]
impl<'a> crate::RustStdType for std::str::LinesAny<'a> {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn rust_language_provenance() -> crate::RustLanguageProvenance {
        crate::RustLanguageProvenance::for_source("core", "core::str")
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn rust_doc_url() -> &'static str {
        "https://doc.rust-lang.org/core/str/struct.LinesAny.html"
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
    fn rust_semantics_summary() -> &'static str {
        "The LinesAny carrier lazily yields the lines of a str split on any of \\n, \\r\\n; deprecated in favor of Lines, but still a real, stable carrier."
    }
}

macro_rules! str_pattern_iter_char {
    ($ty:ident, $summary:expr) => {
        impl_rust_std_type_lifetime1_concrete!(
            $ty,
            char,
            "core",
            "core::str",
            concat!(
                "https://doc.rust-lang.org/core/str/struct.",
                stringify!($ty),
                ".html"
            ),
            $summary
        );
    };
}

str_pattern_iter_char!(
    Split,
    "The Split carrier lazily splits a str on a pattern, yielding substrings."
);
str_pattern_iter_char!(
    RSplit,
    "The RSplit carrier lazily splits a str on a pattern, yielding substrings from the back."
);
str_pattern_iter_char!(
    SplitN,
    "The SplitN carrier lazily splits a str on a pattern into at most n substrings."
);
str_pattern_iter_char!(
    RSplitN,
    "The RSplitN carrier lazily splits a str on a pattern into at most n substrings, from the back."
);
str_pattern_iter_char!(
    SplitInclusive,
    "The SplitInclusive carrier lazily splits a str on a pattern, keeping the matched delimiter at the end of each substring."
);
str_pattern_iter_char!(
    SplitTerminator,
    "The SplitTerminator carrier lazily splits a str on a pattern, not yielding a trailing empty substring after a terminal match."
);
str_pattern_iter_char!(
    RSplitTerminator,
    "The RSplitTerminator carrier lazily splits a str on a pattern from the back, not yielding a trailing empty substring after a terminal match."
);
str_pattern_iter_char!(
    Matches,
    "The Matches carrier lazily yields the non-overlapping substrings of a str that match a pattern."
);
str_pattern_iter_char!(
    RMatches,
    "The RMatches carrier lazily yields the non-overlapping substrings of a str that match a pattern, from the back."
);
str_pattern_iter_char!(
    MatchIndices,
    "The MatchIndices carrier lazily yields the non-overlapping substrings of a str that match a pattern, paired with their byte offset."
);
str_pattern_iter_char!(
    RMatchIndices,
    "The RMatchIndices carrier lazily yields the non-overlapping substrings of a str that match a pattern, paired with their byte offset, from the back."
);

// `'static` is the representative lifetime this module's proof batch
// covers.
//
// Bytes/Lines/EscapeDebug/EscapeDefault/EscapeUnicode/Split/RSplit/SplitN/
// RSplitN/SplitInclusive are written fully-qualified: each bare name is
// shared by another module (e.g. `std::io::Bytes`/`std::io::Lines`,
// `char`'s escape iterators, `core::slice`'s differently-shaped
// `Split`/`RSplit`/`SplitN`/`RSplitN`/`SplitInclusive` family) — only the
// qualified path disambiguates which one a given registration means for
// tooling reading the registry (e.g. `elicit_doc`'s coverage report).
register_rust_std_standard_evidence!(
    std::str::Bytes<'static>,
    CharIndices<'static>,
    Chars<'static>,
    EncodeUtf16<'static>,
    std::str::EscapeDebug<'static>,
    std::str::EscapeDefault<'static>,
    std::str::EscapeUnicode<'static>,
    std::str::Lines<'static>,
    SplitAsciiWhitespace<'static>,
    SplitWhitespace<'static>,
    Utf8Chunk<'static>,
    Utf8Chunks<'static>,
    ParseBoolError,
    Utf8Error,
    LinesAny<'static>,
    std::str::Split<'static, char>,
    std::str::RSplit<'static, char>,
    std::str::SplitN<'static, char>,
    std::str::RSplitN<'static, char>,
    std::str::SplitInclusive<'static, char>,
    SplitTerminator<'static, char>,
    RSplitTerminator<'static, char>,
    Matches<'static, char>,
    RMatches<'static, char>,
    MatchIndices<'static, char>,
    RMatchIndices<'static, char>,
);
