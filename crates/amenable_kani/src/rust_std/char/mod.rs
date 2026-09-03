//! `KaniWitness` impls for `core::char`.
//!
//! `CharTryFromError`/`TryFromCharError` check a fully symbolic `u32`/`char`:
//! both reduce to a pure numeric range comparison, no string formatting
//! involved. `DecodeUtf16`/`DecodeUtf16Error` check a symbolic `u16` code
//! unit through a single `.next()` call on a one-element array iterator —
//! bounded, no `.collect()`. The escaping/case-mapping adapters
//! (`EscapeDebug`/`EscapeDefault`/`EscapeUnicode`/`ToLowercase`/
//! `ToUppercase`) instead check fixed representative examples, matching
//! `core::str`'s already-verified Escape* proofs: `.collect()`ing even a
//! two-byte symbolic `EscapeAscii` iterator times out under Kani (see
//! `gallery::slice_escape_ascii`), so these check one concrete `char` at a
//! time rather than a symbolic one. `ParseCharError` checks fixed
//! representative strings, since it has no accessor to check beyond
//! success/failure.

mod adapters;
mod scalar;
mod utf16;

pub(crate) use utf16::VERIFY_DECODE_UTF16_ROUND_TRIPS_A_BMP_CODE_UNIT_SRC;
