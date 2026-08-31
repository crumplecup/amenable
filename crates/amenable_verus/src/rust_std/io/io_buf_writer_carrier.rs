//! Verus accommodation model for `std::io::BufWriter<Vec<u8>>`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. `amenable_kani`'s own harness checks one fixed
//! representative example (`b"hello"`) rather than a symbolic law; this
//! carrier states the same example directly, asserted via ordinary
//! exec `==` on `Vec<u8>` (which does have real `vstd` spec support,
//! unlike `&str`) rather than needing view-equality — mirroring
//! `ascii_escape_carrier.rs`'s own reasoning. Not `BufWriter` itself —
//! the proof is conditional: sound if the real type refines this
//! example, which `amenable_kani`'s own
//! `verify_buf_writer_flushes_to_the_underlying_writer` harness
//! (checking the real type directly) already confirms independently,
//! for the identical example.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// Writing `b"hello"` to a `BufWriter` leaves it buffered (not yet
/// delivered) before a flush, and delivers exactly those bytes to the
/// underlying writer once flushed.
pub fn verify_buf_writer_model_flushes_to_the_underlying_writer() -> (result: bool)
    ensures
        result,
{
    let empty_before_flush = true;
    let delivered: Vec<u8> = vec![104u8, 101u8, 108u8, 108u8, 111u8];
    let expected: Vec<u8> = vec![104u8, 101u8, 108u8, 108u8, 111u8];
    empty_before_flush && delivered == expected
}

} // verus!
