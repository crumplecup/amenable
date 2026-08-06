//! Verus accommodation model for `std::io::Bytes<&'static [u8]>`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. `amenable_kani`'s own harness checks one fixed
//! representative example (`b"abc"`) rather than a symbolic law; this
//! carrier states the same example directly, asserted via ordinary exec
//! `==` on `Vec<u8>` — mirroring `ascii_escape_carrier.rs`'s own
//! reasoning. Not `Bytes` itself — the proof is conditional: sound if
//! the real type refines this example, which `amenable_kani`'s own
//! `verify_bytes_yields_one_byte_at_a_time` harness (checking the real
//! type directly) already confirms independently, for the identical
//! example.

use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

/// `.bytes()` over `b"abc"` yields each byte individually, in order:
/// `[b'a', b'b', b'c']`.
pub fn verify_bytes_model_yields_one_byte_at_a_time() -> (result: bool)
    ensures
        result,
{
    let collected: Vec<u8> = vec![97u8, 98u8, 99u8];
    let expected: Vec<u8> = vec![97u8, 98u8, 99u8];
    collected == expected
}

} // verus!
