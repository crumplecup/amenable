//! `RustStdType` registrations for `core::char`.

use core::char::{
    CharTryFromError, DecodeUtf16, DecodeUtf16Error, EscapeDebug, EscapeDefault, EscapeUnicode,
    ParseCharError, ToLowercase, ToUppercase, TryFromCharError,
};

use crate::rust_std::macros::{impl_rust_std_type, register_rust_std_standard_evidence};

impl_rust_std_type!(
    CharTryFromError,
    "core",
    "core::char",
    "https://doc.rust-lang.org/core/char/struct.CharTryFromError.html",
    "The CharTryFromError carrier reports that a u32 value was not a valid Unicode scalar value when converting it into a char."
);

impl<I: Iterator<Item = u16>> crate::RustStdType for DecodeUtf16<I> {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn rust_language_provenance() -> crate::RustLanguageProvenance {
        crate::RustLanguageProvenance::for_source("core", "core::char")
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn rust_doc_url() -> &'static str {
        "https://doc.rust-lang.org/core/char/struct.DecodeUtf16.html"
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
    fn rust_semantics_summary() -> &'static str {
        "The DecodeUtf16 carrier lazily decodes an iterator of UTF-16 code units into chars, yielding an error for unpaired surrogates."
    }
}

impl_rust_std_type!(
    DecodeUtf16Error,
    "core",
    "core::char",
    "https://doc.rust-lang.org/core/char/struct.DecodeUtf16Error.html",
    "The DecodeUtf16Error carrier reports that an unpaired surrogate was found while decoding UTF-16."
);

impl_rust_std_type!(
    EscapeDebug,
    "core",
    "core::char",
    "https://doc.rust-lang.org/core/char/struct.EscapeDebug.html",
    "The EscapeDebug carrier lazily yields a char's Debug-escaped representation, one char at a time."
);

impl_rust_std_type!(
    EscapeDefault,
    "core",
    "core::char",
    "https://doc.rust-lang.org/core/char/struct.EscapeDefault.html",
    "The EscapeDefault carrier lazily yields a char's representation as in a Rust string literal, one char at a time."
);

impl_rust_std_type!(
    EscapeUnicode,
    "core",
    "core::char",
    "https://doc.rust-lang.org/core/char/struct.EscapeUnicode.html",
    "The EscapeUnicode carrier lazily yields a char's \\u{{...}} escape sequence, one char at a time."
);

impl_rust_std_type!(
    ParseCharError,
    "core",
    "core::char",
    "https://doc.rust-lang.org/core/char/struct.ParseCharError.html",
    "The ParseCharError carrier reports that a string could not be parsed as a single char."
);

impl_rust_std_type!(
    ToLowercase,
    "core",
    "core::char",
    "https://doc.rust-lang.org/core/char/struct.ToLowercase.html",
    "The ToLowercase carrier lazily yields the lowercase-mapped chars of a single char, which may expand to more than one."
);

impl_rust_std_type!(
    ToUppercase,
    "core",
    "core::char",
    "https://doc.rust-lang.org/core/char/struct.ToUppercase.html",
    "The ToUppercase carrier lazily yields the uppercase-mapped chars of a single char, which may expand to more than one."
);

impl_rust_std_type!(
    TryFromCharError,
    "core",
    "core::char",
    "https://doc.rust-lang.org/core/char/struct.TryFromCharError.html",
    "The TryFromCharError carrier reports that a char could not be converted into a narrower integer type."
);

// EscapeDebug/EscapeDefault/EscapeUnicode are written fully-qualified:
// `core::str` and `core::ascii` each define their own carrier by the same
// bare name — only the qualified path disambiguates which one a given
// registration means for tooling reading the registry (e.g. `elicit_doc`'s
// coverage report).
register_rust_std_standard_evidence!(
    CharTryFromError,
    DecodeUtf16<std::array::IntoIter<u16, 1>>,
    DecodeUtf16Error,
    core::char::EscapeDebug,
    core::char::EscapeDefault,
    core::char::EscapeUnicode,
    ParseCharError,
    ToLowercase,
    ToUppercase,
    TryFromCharError,
);
