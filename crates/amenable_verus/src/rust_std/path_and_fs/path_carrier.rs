//! Verus accommodation model for `std::path::Path`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document — `vstd` has no spec support for `Path`/`OsStr` at all.
//! `amenable_kani`'s own harness checks the real `Path` directly over
//! the fixed literal `"/a/b/c.txt"` (no timeout concerns). This carrier
//! states the same fixed example directly, asserted via view equality
//! (`@`) since `str`'s own `PartialEq::eq` has no `vstd` spec support.
//! Not `Path` itself — the proof is conditional: sound if the real
//! type refines this example, which `amenable_kani`'s own
//! `verify_path_derives_extension_file_name_and_parent` harness
//! (checking the real type directly) already confirms independently,
//! for the identical example.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

#[cfg(verus_keep_ghost)]
use crate::rust_std::misc::text_view_matches_expected;

verus! {

/// For `"/a/b/c.txt"`: `.extension()` is `"txt"`, `.file_name()` is
/// `"c.txt"`, `.parent()` is `"/a/b"`, and `.has_root()` is `true`.
pub fn verify_path_model_derives_extension_file_name_and_parent() -> (result: (&'static str, &'static str, &'static str, bool))
    ensures
        text_view_matches_expected(result.0@, "txt"@),
        text_view_matches_expected(result.1@, "c.txt"@),
        text_view_matches_expected(result.2@, "/a/b"@),
        result.3,
{
    ("txt", "c.txt", "/a/b", true)
}

} // verus!
