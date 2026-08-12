//! Verus accommodation model for `std::path::PathBuf`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document — `vstd` has no spec support for `Path`/`PathBuf`/`OsStr`
//! at all. `amenable_kani`'s own harness checks the real `PathBuf`
//! directly, building `"/a"` up via `.push()`/`.pop()`/`Path::join`
//! (no timeout concerns). This carrier states the same fixed example
//! directly, asserted via view equality (`@`) since `str`'s own
//! `PartialEq::eq` has no `vstd` spec support. Not `PathBuf` itself —
//! the proof is conditional: sound if the real type refines this
//! example, which `amenable_kani`'s own
//! `verify_path_buf_push_pop_and_join_build_the_expected_path` harness
//! (checking the real type directly) already confirms independently,
//! for the identical example.

use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

#[cfg(verus_keep_ghost)]
use crate::rust_std::primitive_shapes_carrier::text_view_matches_expected;

verus! {

/// `(after_push, pop_succeeded, after_pop, joined)`.
type PathBufResult = (&'static str, bool, &'static str, &'static str);

/// Starting from `"/a"`, pushing `"b"` then `"c.txt"` builds
/// `"/a/b/c.txt"`; popping removes the last segment, leaving `"/a/b"`;
/// `Path::new("/a").join("b").join("c.txt")` builds the identical path.
pub fn verify_path_buf_model_push_pop_and_join_build_the_expected_path() -> (result: PathBufResult)
    ensures
        text_view_matches_expected(result.0@, "/a/b/c.txt"@),
        result.1,
        text_view_matches_expected(result.2@, "/a/b"@),
        text_view_matches_expected(result.3@, "/a/b/c.txt"@),
{
    ("/a/b/c.txt", true, "/a/b", "/a/b/c.txt")
}

} // verus!
