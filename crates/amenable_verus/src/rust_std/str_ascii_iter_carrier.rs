//! Verus accommodation model for `core::str::{Bytes, CharIndices,
//! EncodeUtf16}`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. `amenable_kani`'s own module doc explains why these three
//! adapters check a genuinely symbolic single-character `str` (`byte as
//! char` for `byte < 128` is a valid, safe conversion) rather than a
//! fixed literal: plain iteration over one ASCII character doesn't run
//! into the timeouts that force the escaping/splitting adapters onto
//! fixed examples. This carrier takes the ASCII `char` directly as the
//! symbolic parameter (mirroring `char_carrier.rs`'s own `c as u32`
//! numeric-cast pattern) rather than reconstructing it from a `u8`,
//! sidestepping any `u8 -> char` spec-cast question entirely. None of
//! these functions are `Bytes`/`CharIndices`/`EncodeUtf16` themselves —
//! each proof is conditional: sound if the real type refines the stated
//! law, which `amenable_kani`'s own harness for that exact type
//! (checking the real type directly) already confirms independently,
//! for the identical claim.
//!
//! Every `requires` clause below calls `amenable_std::AsciiByte`'s own
//! shared Verus predicate, `is_ascii_byte` (defined once in
//! `primitive_shapes_carrier`).

#[cfg(verus_keep_ghost)]
use crate::rust_std::primitive_shapes_carrier::is_ascii_byte;
use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

/// `.bytes()` yields the UTF-8 encoding of the str, checked for any
/// single-byte (ASCII) character: the one byte numerically equals the
/// character.
pub fn verify_bytes_model_yields_the_utf8_encoding(c: char) -> (result: u8)
    requires
        is_ascii_byte(c as u32),
    ensures
        (result as u32) == (c as u32),
{
    c as u8
}

/// `.char_indices()` pairs the first (and only) char with byte offset 0.
pub fn verify_char_indices_model_pairs_each_char_with_its_byte_offset(c: char) -> (result: (u32, char))
    requires
        is_ascii_byte(c as u32),
    ensures
        result.0 == 0,
        result.1 == c,
{
    (0, c)
}

/// `.encode_utf16()` yields UTF-16 code units; for an ASCII character,
/// the code unit numerically equals the character.
pub fn verify_encode_utf16_model_yields_utf16_code_units(c: char) -> (result: u16)
    requires
        is_ascii_byte(c as u32),
    ensures
        (result as u32) == (c as u32),
{
    c as u16
}

} // verus!
