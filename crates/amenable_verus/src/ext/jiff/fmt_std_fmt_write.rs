//! Verus accommodation model for `jiff::fmt::StdFmtWrite<String>::
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
//! fmt_std_fmt_write`'s own witness is.** Creusot hit a real,
//! CONFIRMED compiler wall trying to extern-spec jiff's actual generic
//! `impl<W: core::fmt::Write> Write for StdFmtWrite<W>` at the concrete
//! `W = String` instantiation ("extern spec generics don't match" —
//! Creusot's extern-spec matching requires identical generics to the
//! real impl, so it can only ever state a claim true for *every*
//! `W`, which infallibility isn't). This model has no such
//! constraint: it never extern-specs jiff's real impl at all, only
//! reproduces the documented behavior of the concrete `String` case
//! using `vstd`'s own real, already-contracted `String::append`
//! (`vstd::string::StringExecFns::append`, whose own postcondition —
//! `final(self)@ == old(self)@ + other@` — is genuinely checked, not
//! assumed here). So this model states the FULL round-trip claim,
//! matching `amenable_kani::ext::jiff::fmt_std_fmt_write`'s own
//! stronger claim rather than `amenable_creusot`'s narrower one: the
//! write always succeeds AND the written string is appended exactly.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// A model of `jiff::fmt::StdFmtWrite<String>::write_str`: standing in
/// for `self.0.write_str(string).map_err(..)` with `self.0: String`
/// modeled directly as the `&mut String` receiver, since jiff's real
/// wrapper delegates to exactly this call and jiff itself is
/// unreachable to Verus. Always succeeds and appends `s` exactly —
/// the same claim `amenable_kani::ext::jiff::fmt_std_fmt_write`
/// checks by symbolic execution against jiff's real API.
pub fn verify_fmt_std_fmt_write_write_str_model(string: &mut String, s: &str) -> (result: bool)
    ensures
        result,
        final(string)@ == old(string)@ + s@,
{
    string.append(s);
    true
}

} // verus!
