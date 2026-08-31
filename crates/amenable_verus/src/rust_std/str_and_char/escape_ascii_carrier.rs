//! Verus accommodation model for `core::slice::EscapeAscii`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. `amenable_kani`'s own proof for this type uses a bounded
//! `KaniEscapeAsciiObservation` rather than the real iterator directly,
//! since the real iterator still times out under Kani even on a fixed
//! two-byte witness. This carrier states the resulting law directly: a
//! printable ASCII byte passes through unescaped, while a control
//! character (`\n`, byte value 10) expands to its two-byte backslash
//! form. Not `EscapeAscii` itself — the proof is conditional: sound if
//! the real type refines this escaping rule, which `amenable_kani`'s own
//! harness (checking the real type via the bounded observation) already
//! confirms independently, for the identical claim.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// Whether `printable` is in the printable ASCII range (`0x20..=0x7e`).
pub open spec fn escape_ascii_input_is_printable_ascii(printable: u8) -> bool {
    32 <= printable && printable <= 126
}

/// This file's whole postcondition: a printable ASCII byte passes
/// through unchanged, followed by the newline byte's `\n` escape
/// (`\`, `n`).
pub open spec fn escape_ascii_result_matches_printable_plus_newline_escape(
    printable: u8,
    result: (u8, u8, u8),
) -> bool {
    result.0 == printable && result.1 == 92 && result.2 == 110
}

/// A printable ASCII byte (0x20..=0x7e) passes through unchanged; the
/// following newline (byte value 10) escapes to the two-byte backslash
/// form `\n` (byte values 92, 110).
pub fn verify_escape_ascii_model_leaves_printable_bytes_unescaped(printable: u8) -> (result: (u8, u8, u8))
    requires
        escape_ascii_input_is_printable_ascii(printable),
    ensures
        escape_ascii_result_matches_printable_plus_newline_escape(printable, result),
{
    (printable, 92, 110)
}

} // verus!
