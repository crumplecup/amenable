//! Verus accommodation model for `jiff::fmt::StdIoWrite<Vec<u8>>::
//! write_str` (via `jiff::fmt::Write`).
//!
//! `jiff` has zero `vstd` coverage and Verus never resolves
//! `Cargo.toml` at all — unlike Kani/Creusot, there is no mechanism
//! for Verus to reach jiff's actual code, full stop. The established
//! response (see `offset.rs`'s own doc comment for the precedent) is
//! a hand-verified Verus-native model reproducing the type's
//! documented behavior.
//!
//! **Not scoped down the way `amenable_creusot::ext_jiff::
//! fmt_std_io_write`'s own witness is**, the identical relationship
//! `fmt_std_fmt_write.rs`'s own doc comment documents for `StdFmtWrite<
//! String>`: this model never extern-specs jiff's real generic impl at
//! all, only reproduces `Vec<u8>`'s own documented behavior using
//! `vstd`'s already-contracted `Vec::extend_from_slice`. States the
//! FULL round-trip claim, matching `amenable_kani::ext::jiff::
//! fmt_std_io_write`'s own stronger claim rather than
//! `amenable_creusot`'s narrower one: the write always succeeds AND
//! the written bytes are appended exactly.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// The append postcondition `write_str` establishes: the buffer's
/// final content is its prior content with `data` appended, named so
/// the `@`-sequence claim is a citable fact.
pub open spec fn fmt_std_io_write_appends_bytes(
    before: Seq<u8>,
    after: Seq<u8>,
    data: Seq<u8>,
) -> bool {
    after == before + data
}

/// A model of `jiff::fmt::StdIoWrite<Vec<u8>>::write_str`: standing in
/// for `self.0.write_all(string.as_bytes()).map_err(..)` with
/// `self.0: Vec<u8>` modeled directly as the `&mut Vec<u8>` receiver,
/// since jiff's real wrapper delegates to exactly this call and jiff
/// itself is unreachable to Verus. Always succeeds and appends `data`
/// exactly — the same claim `amenable_kani::ext::jiff::
/// fmt_std_io_write` checks by symbolic execution against jiff's real
/// API.
pub fn verify_fmt_std_io_write_write_str_model(buf: &mut Vec<u8>, data: &[u8]) -> (result: bool)
    ensures
        result,
        fmt_std_io_write_appends_bytes(old(buf)@, final(buf)@, data@),
{
    buf.extend_from_slice(data);
    assert(buf@ =~= old(buf)@ + data@);
    true
}

} // verus!
