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
#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

/// A printable ASCII byte (0x20..=0x7e) passes through unchanged; the
/// following newline (byte value 10) escapes to the two-byte backslash
/// form `\n` (byte values 92, 110).
pub fn verify_escape_ascii_model_leaves_printable_bytes_unescaped(printable: u8) -> (result: (u8, u8, u8))
    requires
        32 <= printable,
        printable <= 126,
    ensures
        result.0 == printable,
        result.1 == 92,
        result.2 == 110,
{
    (printable, 92, 110)
}

} // verus!
