//! Fractional-second digit conversions shared by every temporal backend.
//!
//! Both the jiff and chrono backends turn an ISO 8601 decimal-fraction suffix
//! into whole nanoseconds, and back. The conversion is the same for every
//! backend, so it lives here once rather than in each backend.

use amenable_time::{NonDigitFractionByteSource, TemporalError, TemporalErrorKind};
#[cfg(kani)]
use crate::fractional_digit_chain::{
    combine_digit_2, combine_digit_3, combine_digit_4, combine_digit_5, combine_digit_6,
    combine_digit_7, combine_digit_8, combine_digit_9,
};

/// The pure arithmetic core of the digits<->nanos conversion: a full,
/// zero-padded 9-digit buffer in, the whole number it encodes out.
/// Every byte is assumed to already be an ASCII digit (the two public
/// wrappers below are what enforce that against real text) -- `Copy`,
/// fixed-size, no heap allocation anywhere. `String`s are not
/// generally tractable for Kani to reason about (confirmed repeatedly
/// in this module's own history: `.parse()`, `format!`, `String::
/// from_utf8`, `trim_end_matches` were each independently confirmed
/// expensive over symbolic content), so the round trip this type
/// exists for is proven against *this* core, never against the
/// `String`-wrapping public functions directly.
#[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
#[cfg_attr(kani, kani::requires(digits.iter().all(|&b| b.is_ascii_digit())))]
#[cfg_attr(kani, kani::ensures(|result: &i64| *result >= 0 && *result <= 999_999_999))]
pub fn fractional_digit_bytes_to_nanos(digits: &[u8; 9]) -> i64 {
    let mut nanos: i64 = 0;
    for &byte in digits {
        // Widen to `i64` before subtracting, not `byte - b'0'`: a `u8`
        // subtraction carries a defensive underflow check that, to be
        // discharged, needs Kani to relate every byte's value back
        // through this function's caller's own loop -- the bound
        // holds for every possible `u8`, not just digit bytes, so
        // Kani discharges it on this function's own signature alone
        // when the arithmetic is widened first.
        nanos = nanos * 10 + (i64::from(byte) - i64::from(b'0'));
    }
    nanos
}

/// The inverse core: whole nanoseconds (`0..=999_999_999`) to a full,
/// zero-padded 9-digit buffer, with no trimming -- trimming is a
/// `String`-wrapping-layer concern (see `nanos_to_fractional_seconds_
/// digits`), not part of this type's own arithmetic.
#[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
#[cfg_attr(kani, kani::requires(nanos >= 0 && nanos <= 999_999_999))]
#[cfg_attr(
    kani,
    kani::ensures(|result: &[u8; 9]| {
        // Digit-validity checked first, short-circuiting `&&`: Kani
        // evaluates this predicate against a raw, unfiltered stub
        // candidate before anything has excluded non-digit bytes, so
        // calling the digit-validity-requiring `fractional_digit_
        // bytes_to_nanos` on an un-checked candidate first is a real
        // precondition violation, not a hypothetical one -- confirmed
        // directly (a fast, 24.6s counterexample) once this contract
        // was stubbed rather than only ever proven against its own
        // real (always-valid) body. Same ordering lesson as `gallery::
        // string_depth_induction`'s own `no_embedded_nul` bug.
        result.iter().all(|&b| b.is_ascii_digit()) && fractional_digit_bytes_to_nanos(result) == nanos
    })
)]
pub fn nanos_to_fractional_digit_bytes(nanos: i64) -> [u8; 9] {
    #[cfg(kani)]
    {
        let d1 = [b'0' + u8::try_from((nanos / 100_000_000) % 10).unwrap_or(0)];
        let d2 = combine_digit_2(d1, b'0' + u8::try_from((nanos / 10_000_000) % 10).unwrap_or(0));
        let d3 = combine_digit_3(d2, b'0' + u8::try_from((nanos / 1_000_000) % 10).unwrap_or(0));
        let d4 = combine_digit_4(d3, b'0' + u8::try_from((nanos / 100_000) % 10).unwrap_or(0));
        let d5 = combine_digit_5(d4, b'0' + u8::try_from((nanos / 10_000) % 10).unwrap_or(0));
        let d6 = combine_digit_6(d5, b'0' + u8::try_from((nanos / 1_000) % 10).unwrap_or(0));
        let d7 = combine_digit_7(d6, b'0' + u8::try_from((nanos / 100) % 10).unwrap_or(0));
        let d8 = combine_digit_8(d7, b'0' + u8::try_from((nanos / 10) % 10).unwrap_or(0));
        return combine_digit_9(d8, b'0' + u8::try_from(nanos % 10).unwrap_or(0));
    }

    #[cfg(not(kani))]
    {
        let mut digits = [b'0'; 9];
        let mut remaining = nanos;
        for digit in digits.iter_mut().rev() {
            *digit = b'0' + u8::try_from(remaining % 10).unwrap_or(0);
            remaining /= 10;
        }
        digits
    }
}

mod kani_only {
    #![cfg(kani)]

    /// Renders a full 9-digit buffer to `String` via an *unconditional*
    /// sequence of exactly 9 `.push()` calls -- no branching, no symbolic
    /// trim length at all, confirmed cheap (21.3s) in isolation. Used only
    /// under `#[cfg(kani)]`, as a cheap stand-in for `nanos_to_fractional_
    /// seconds_digits`'s real, symbolic-trim-length body wherever a
    /// larger proof just needs *some* correctly-typed, non-empty `String`
    /// to exist and does not inspect its content -- confirmed directly
    /// that reading back from a push-built `String` after construction
    /// (even one byte) reintroduces the same OOM, so this function's own
    /// result must never be read back inside a Kani proof, only passed
    /// through to a type that needs a `String`.
    pub fn digit_bytes_to_string_unconditional(digits: [u8; 9]) -> String {
        let mut s = String::new();
        s.push(digits[0] as char);
        s.push(digits[1] as char);
        s.push(digits[2] as char);
        s.push(digits[3] as char);
        s.push(digits[4] as char);
        s.push(digits[5] as char);
        s.push(digits[6] as char);
        s.push(digits[7] as char);
        s.push(digits[8] as char);
        s
    }
}

#[cfg(kani)]
pub use kani_only::digit_bytes_to_string_unconditional;


/// Convert a lexical fractional-second digit string (ISO 8601's
/// decimal-fraction suffix, e.g. `"5"` for `.5`) into whole nanoseconds.
///
/// Canonically right-padded to 9 digits (nanosecond precision) and
/// truncated beyond that — real, but sufficient-not-exhaustive:
/// sub-nanosecond digits are documented precision loss, the same
/// "state the sufficient range, not the exact algorithm" convention
/// `amenable_ext`'s own jiff witnesses already use.
///
/// A thin wrapper over [`fractional_digit_bytes_to_nanos`]: validate
/// and pad real text into a `[u8; 9]` buffer, then hand off the actual
/// arithmetic to the core. Manual byte-level validation, not `str::
/// parse` -- confirmed Kani-expensive for a symbolic-content digit
/// string, but this wrapper itself is never what Kani proves the
/// round trip against; only its core is.
#[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
pub fn fractional_seconds_digits_to_nanos(digits: &str) -> Result<i64, TemporalError> {
    let bytes = digits.as_bytes();
    let mut buffer = [b'0'; 9];
    for (index, slot) in buffer.iter_mut().enumerate() {
        match bytes.get(index) {
            Some(byte) if byte.is_ascii_digit() => *slot = *byte,
            Some(byte) => {
                return Err(TemporalError::new(TemporalErrorKind::NonDigitFractionByte(
                    NonDigitFractionByteSource::new(*byte),
                )));
            }
            None => {}
        }
    }
    Ok(fractional_digit_bytes_to_nanos(&buffer))
}

/// Convert whole nanoseconds (`0..=999_999_999`) back into a canonical
/// lexical fractional-second digit string, trimmed of trailing zeros
/// (e.g. `500_000_000` -> `"5"`).
///
/// A thin wrapper over [`nanos_to_fractional_digit_bytes`]: get the
/// core's full 9-digit buffer, trim trailing zeros, render to `String`.
///
/// This wrapper's own contract isolates the trim+render+re-parse claim
/// (`fractional_seconds_digits_to_nanos(result) == Ok(nanos)`) away
/// from any caller's own surrounding state (e.g. a real `chrono`
/// date/time's own symbolic construction) -- a caller that stubs this
/// function instead of inlining it never has to symbolically execute
/// the trailing-zero scan or the `char`-collecting `String` build over
/// a stub-generated, symbolic-length byte range, confirmed to be a
/// major contributor to a real state-space blowup (OOM) when left
/// inlined inside a much larger proof.
#[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
#[cfg_attr(kani, kani::requires(nanos >= 0 && nanos <= 999_999_999))]
#[cfg_attr(
    kani,
    kani::ensures(|result: &String| {
        matches!(fractional_seconds_digits_to_nanos(result), Ok(decoded) if decoded == nanos)
    })
)]
pub fn nanos_to_fractional_seconds_digits(nanos: i64) -> String {
    let digits = nanos_to_fractional_digit_bytes(nanos);
    // Manual trailing-zero scan, not `str::trim_end_matches`: the
    // generic `Pattern`/`CharSearcher` machinery behind `trim_end_
    // matches` was confirmed Kani-expensive even over this fixed
    // 9-byte array, on the same dead-for-the-chrono-instant-bridge
    // branch as the two fixes above (`nanos` is always 0 there).
    let mut end = digits.len();
    while end > 0 && digits[end - 1] == b'0' {
        end -= 1;
    }
    if end == 0 {
        return "0".to_owned();
    }
    // `u8 as char`, not `String::from_utf8`: every byte here is a
    // known ASCII digit, so the conversion needs no UTF-8 validation
    // at all -- `String::from_utf8`'s own real validation loop was
    // confirmed Kani-expensive over a (symbolic-content) byte array.
    #[cfg(kani)]
    {
        // A `match` on `end` (10 fixed arms, one per possible trimmed
        // length), each a straight-line, compile-time-constant
        // sequence of `.push()` calls -- not a loop whose trip count
        // is the symbolic `end` itself. Confirmed directly: a loop
        // bounded by a symbolic trip count is the expensive shape
        // (matching `KaniUtf8Buffer::push_char`'s own doc comment,
        // which contrasts its fixed 4-iteration loop against exactly
        // this), not repeated `.push()` in general -- each arm here
        // still pushes a known, fixed number of times, just a
        // different fixed number depending on which arm Kani takes.
        match end {
            1 => {
                let mut s = String::new();
                s.push(digits[0] as char);
                s
            }
            2 => {
                let mut s = String::new();
                s.push(digits[0] as char);
                s.push(digits[1] as char);
                s
            }
            3 => {
                let mut s = String::new();
                s.push(digits[0] as char);
                s.push(digits[1] as char);
                s.push(digits[2] as char);
                s
            }
            4 => {
                let mut s = String::new();
                s.push(digits[0] as char);
                s.push(digits[1] as char);
                s.push(digits[2] as char);
                s.push(digits[3] as char);
                s
            }
            5 => {
                let mut s = String::new();
                s.push(digits[0] as char);
                s.push(digits[1] as char);
                s.push(digits[2] as char);
                s.push(digits[3] as char);
                s.push(digits[4] as char);
                s
            }
            6 => {
                let mut s = String::new();
                s.push(digits[0] as char);
                s.push(digits[1] as char);
                s.push(digits[2] as char);
                s.push(digits[3] as char);
                s.push(digits[4] as char);
                s.push(digits[5] as char);
                s
            }
            7 => {
                let mut s = String::new();
                s.push(digits[0] as char);
                s.push(digits[1] as char);
                s.push(digits[2] as char);
                s.push(digits[3] as char);
                s.push(digits[4] as char);
                s.push(digits[5] as char);
                s.push(digits[6] as char);
                s
            }
            8 => {
                let mut s = String::new();
                s.push(digits[0] as char);
                s.push(digits[1] as char);
                s.push(digits[2] as char);
                s.push(digits[3] as char);
                s.push(digits[4] as char);
                s.push(digits[5] as char);
                s.push(digits[6] as char);
                s.push(digits[7] as char);
                s
            }
            _ => {
                let mut s = String::new();
                s.push(digits[0] as char);
                s.push(digits[1] as char);
                s.push(digits[2] as char);
                s.push(digits[3] as char);
                s.push(digits[4] as char);
                s.push(digits[5] as char);
                s.push(digits[6] as char);
                s.push(digits[7] as char);
                s.push(digits[8] as char);
                s
            }
        }
    }
    #[cfg(not(kani))]
    {
        let mut result = String::new();
        for (index, &byte) in digits.iter().enumerate() {
            if index < end {
                result.push(byte as char);
            }
        }
        result
    }
}
