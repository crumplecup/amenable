//! Verus accommodation model for `std::path::Ancestors`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document — `vstd` has no spec support for `Path`/`PathBuf`/`OsStr`
//! at all. `amenable_kani`'s own harness checks the real `Ancestors`
//! directly over the fixed literal path `"/a/b/c"` (no timeout
//! concerns). This carrier states the same fixed example directly,
//! asserted via view equality (`@`) since `str`'s own `PartialEq::eq`
//! has no `vstd` spec support. Not `Ancestors` itself — the proof is
//! conditional: sound if the real type refines this example, which
//! `amenable_kani`'s own
//! `verify_ancestors_yields_self_then_each_parent_up_to_root` harness
//! (checking the real type directly) already confirms independently,
//! for the identical example.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

#[cfg(verus_keep_ghost)]
use crate::rust_std::misc::primitive_shapes_carrier::text_view_matches_expected;

verus! {

/// `Path::new("/a/b/c").ancestors()` yields the path itself, then each
/// parent in turn, stopping at the root: `"/a/b/c"`, `"/a/b"`, `"/a"`,
/// `"/"`.
pub fn verify_ancestors_model_yields_self_then_each_parent_up_to_root() -> (result: (&'static str, &'static str, &'static str, &'static str))
    ensures
        text_view_matches_expected(result.0@, "/a/b/c"@),
        text_view_matches_expected(result.1@, "/a/b"@),
        text_view_matches_expected(result.2@, "/a"@),
        text_view_matches_expected(result.3@, "/"@),
{
    ("/a/b/c", "/a/b", "/a", "/")
}

} // verus!
