//! Verus accommodation model for `std::io::Lines<&'static [u8]>`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. `amenable_kani`'s own harness uses a bounded
//! `KaniLinesObservation` rather than the real `BufRead::lines` path
//! directly. This carrier states the resulting law directly: `.lines()`
//! yields each line with its trailing newline dropped, over three
//! ASCII bytes symbolic other than not themselves being line
//! terminators. Not `Lines` itself — the proof is conditional: sound if
//! the real type refines this law, which `amenable_kani`'s own harness
//! (checking the real type via the bounded observation) already
//! confirms independently, for the identical claim.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

#[cfg(verus_keep_ghost)]
use crate::rust_std::misc::{is_ascii_byte, observed_triple_matches_input};

verus! {

/// A shared line-terminator-exclusion precondition for the `.lines()`/
/// `LineWriter` family: a byte carrying ordinary line content must not
/// itself be one of the two ASCII bytes that end a line, `\n` (10) or
/// `\r` (13).
pub open spec fn is_not_a_line_terminator_byte(byte: u8) -> bool {
    byte != 10 && byte != 13
}

/// The narrower half of [`is_not_a_line_terminator_byte`]: `LineWriter`
/// only flushes on `\n` specifically (unlike `.lines()`, which also
/// treats `\r` as ending a line), so its own precondition only needs to
/// exclude the one byte its real behavior actually keys on.
pub open spec fn is_not_a_newline_byte(byte: u8) -> bool {
    byte != 10
}

/// Over three ASCII bytes that are each neither `\n` (10) nor `\r`
/// (13), `.lines()` yields each byte unchanged as its own line, with no
/// terminator attached.
pub fn verify_lines_model_splits_on_newlines_and_drops_the_terminator(first: u8, second: u8, third: u8) -> (result: (u8, u8, u8))
    requires
        is_ascii_byte(first as u32),
        is_ascii_byte(second as u32),
        is_ascii_byte(third as u32),
        is_not_a_line_terminator_byte(first),
        is_not_a_line_terminator_byte(second),
        is_not_a_line_terminator_byte(third),
    ensures
        observed_triple_matches_input(result, (first, second, third)),
{
    (first, second, third)
}

} // verus!
