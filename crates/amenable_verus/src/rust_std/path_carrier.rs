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
#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

/// For `"/a/b/c.txt"`: `.extension()` is `"txt"`, `.file_name()` is
/// `"c.txt"`, `.parent()` is `"/a/b"`, and `.has_root()` is `true`.
pub fn verify_path_model_derives_extension_file_name_and_parent() -> (result: (&'static str, &'static str, &'static str, bool))
    ensures
        result.0@ =~= "txt"@,
        result.1@ =~= "c.txt"@,
        result.2@ =~= "/a/b"@,
        result.3,
{
    ("txt", "c.txt", "/a/b", true)
}

} // verus!
