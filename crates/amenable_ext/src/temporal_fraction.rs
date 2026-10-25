//! Fractional-second digit conversions shared by every temporal backend.
//!
//! Both the jiff and chrono backends turn an ISO 8601 decimal-fraction suffix
//! into whole nanoseconds, and back. The conversion is the same for every
//! backend, so it lives here once rather than in each backend.

use amenable_time::{NonDigitFractionByteSource, TemporalError, TemporalErrorKind};

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
pub fn fractional_digit_bytes_to_nanos(digits: &[u8; 9]) -> i64 {
    let mut nanos: i64 = 0;
    for &byte in digits {
        // Widen to `i64` before subtracting, not `byte - b'0'`: a `u8`
        // subtraction carries a defensive underflow check that, to be
        // discharged, needs Kani to relate every byte's value back
        // through this function's caller (`nanos_to_fractional_digit_
        // bytes`'s own 9-iteration loop) -- confirmed to blow up the
        // SAT encoding, the same cross-function relational cost this
        // whole module's history keeps running into. Subtracting in
        // `i64` instead needs no such relation: the bound holds for
        // every possible `u8`, not just digit bytes, so Kani discharges
        // it on this function's own signature alone.
        nanos = nanos * 10 + (i64::from(byte) - i64::from(b'0'));
    }
    nanos
}

/// The inverse core: whole nanoseconds (`0..=999_999_999`) to a full,
/// zero-padded 9-digit buffer, with no trimming -- trimming is a
/// `String`-wrapping-layer concern (see `nanos_to_fractional_seconds_
/// digits`), not part of this type's own arithmetic.
#[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
pub fn nanos_to_fractional_digit_bytes(nanos: i64) -> [u8; 9] {
    let mut digits = [b'0'; 9];
    let mut remaining = nanos;
    for digit in digits.iter_mut().rev() {
        *digit = b'0' + u8::try_from(remaining % 10).unwrap_or(0);
        remaining /= 10;
    }
    digits
}

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
/// This wrapper's own text-encoding step is not what Kani proves the
/// round trip against; only the core above is.
#[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
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
    // confirmed Kani-expensive over a (symbolic-content) byte array,
    // worse than the `format!` cost it replaced.
    digits[..end].iter().map(|&byte| byte as char).collect()
}
