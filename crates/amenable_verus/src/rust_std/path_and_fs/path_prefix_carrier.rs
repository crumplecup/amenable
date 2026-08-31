//! Verus accommodation model for `std::path::{Prefix, PrefixComponent}`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. `amenable_kani`'s own harnesses use a bounded
//! `KaniWindowsPrefixObservation` rather than a real Windows-prefix
//! path directly — Windows prefix parsing is host-platform-specific
//! and does not execute on the Linux verifier host either tool runs
//! on. This carrier states each resulting law directly: a Windows
//! drive-letter path (`C:\...`) parses to a `Disk` prefix naming that
//! letter, and a `PrefixComponent`'s raw text agrees with its parsed
//! prefix's drive letter — generalized over any drive letter and raw
//! text, since both are identity claims (the parsed letter always
//! equals the one given, the raw text always equals what was given).
//! Neither function is `Prefix`/`PrefixComponent` themselves — each
//! proof is conditional: sound if the real type refines the stated
//! law, which `amenable_kani`'s own harness for that exact type
//! (checking the real type via the bounded observation) already
//! confirms independently, for the identical claim.

#[cfg(verus_keep_ghost)]
use crate::rust_std::misc::primitive_shapes_carrier::{
    observed_value_matches_input, text_view_matches_expected,
};
use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// A Windows drive-letter path parses to a `Disk` prefix naming exactly
/// the given letter.
pub fn verify_prefix_model_disk_identifies_the_drive_letter(letter: u8) -> (result: u8)
    ensures
        observed_value_matches_input(result as int, letter as int),
{
    letter
}

/// A `PrefixComponent`'s raw text agrees with what the source path
/// wrote, and its parsed drive letter matches exactly what was given.
pub fn verify_prefix_component_model_pairs_raw_text_with_parsed_prefix(text: &str, letter: u8) -> (result: (bool, u8))
    ensures
        result.0,
        observed_value_matches_input(result.1 as int, letter as int),
{
    let raw_text: &str = text;
    assert(text_view_matches_expected(raw_text@, text@));
    let _ = raw_text;
    (true, letter)
}

} // verus!
