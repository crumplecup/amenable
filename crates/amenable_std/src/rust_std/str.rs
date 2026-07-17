//! `RustStdType` registrations for `core::str`.
//!
//! `core::str::pattern::*` is deliberately
//! not covered here — the whole `Pattern`/`Searcher` API sits behind the
//! unstable `pattern` feature.
//!
//! Every iterator type generic over a `Pattern` (`MatchIndices`, `Matches`,
//! `RMatchIndices`, `RMatches`, `RSplit`, `RSplitN`, `RSplitTerminator`,
//! `Split`, `SplitInclusive`, `SplitN`, `SplitTerminator`) is *also* not
//! covered, for the same underlying reason even though the struct itself
//! isn't marked `#[unstable]`: each one's definition bounds its pattern
//! parameter by `P: Pattern`, and `Pattern` can't be named or bounded by
//! from a stable toolchain — so no unconstrained impl over them type-checks
//! on stable regardless of the carrier struct's own stability. This is a
//! real gap in item-level stability screening: it only catches a type
//! that's itself `#[unstable]`, not one whose *definition* transitively
//! requires an unstable trait bound.

use std::str::{
    Bytes, CharIndices, Chars, EncodeUtf16, EscapeDebug, EscapeDefault, EscapeUnicode, Lines,
    ParseBoolError, SplitAsciiWhitespace, SplitWhitespace, Utf8Chunk, Utf8Chunks, Utf8Error,
};

use crate::rust_std::macros::{
    impl_rust_std_type, impl_rust_std_type_lifetime0, register_rust_std_standard_evidence,
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
    fn rust_language_provenance() -> crate::RustLanguageProvenance {
        crate::RustLanguageProvenance::for_source("core", "core::str")
    }

    fn rust_doc_url() -> &'static str {
        "https://doc.rust-lang.org/core/str/struct.LinesAny.html"
    }

    fn rust_semantics_summary() -> &'static str {
        "The LinesAny carrier lazily yields the lines of a str split on any of \\n, \\r\\n; deprecated in favor of Lines, but still a real, stable carrier."
    }
}

// `'static` is the representative lifetime this module's proof batch
// covers.
register_rust_std_standard_evidence!(
    Bytes<'static>,
    CharIndices<'static>,
    Chars<'static>,
    EncodeUtf16<'static>,
    EscapeDebug<'static>,
    EscapeDefault<'static>,
    EscapeUnicode<'static>,
    Lines<'static>,
    SplitAsciiWhitespace<'static>,
    SplitWhitespace<'static>,
    Utf8Chunk<'static>,
    Utf8Chunks<'static>,
    ParseBoolError,
    Utf8Error,
    LinesAny<'static>,
);
