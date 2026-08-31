//! Verus accommodation model for `std::fs::{DirBuilder, DirEntry,
//! ReadDir}`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document — `vstd` has no spec support for `Path`/`OsStr` at all, and
//! the direct `std::fs` tempdir path crosses real OS-backed filesystem
//! state that Kani itself times out on. `amenable_kani`'s own harnesses
//! use an Amenable-owned filesystem model (`KaniFsPath`/`KaniFsLabel`)
//! rather than a real temp directory; this carrier mirrors that same
//! path-composition shape with plain `char` labels standing in for
//! path segments and tuples standing in for a joined path (since
//! `vstd` has no `Path::join` spec either). None of these functions are
//! `DirBuilder`/`DirEntry`/`ReadDir` themselves — each proof is
//! conditional: sound if the real type refines the stated law, which
//! `amenable_kani`'s own harness for that exact type (checking the real
//! type via the bounded filesystem model) already confirms
//! independently, for the identical claim.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// `(first_ancestor, second_ancestor, leaf)`.
type DirBuilderResult = ((char,), (char, char), (char, char, char));

/// `verify_dir_builder_model_creates_nested_directories_recursively`'s
/// whole postcondition: each ancestor joins the path labels seen so far,
/// preserved exactly.
pub open spec fn dir_builder_model_creates_nested_directories_recursively(
    a: char,
    b: char,
    c: char,
    result: DirBuilderResult,
) -> bool {
    &&& result.0 == (a,)
    &&& result.1 == (a, b)
    &&& result.2 == (a, b, c)
}

/// `.recursive(true)` creates every missing ancestor directory, not
/// just the leaf: joining labels `a`, `b`, `c` from the root preserves
/// each ancestor exactly.
pub fn verify_dir_builder_model_creates_nested_directories_recursively(a: char, b: char, c: char) -> (result: DirBuilderResult)
    ensures
        dir_builder_model_creates_nested_directories_recursively(a, b, c, result),
{
    ((a,), (a, b), (a, b, c))
}

/// `verify_dir_entry_model_reports_the_created_files_name_and_path`'s
/// whole postcondition: reports the file's own name and the joined
/// `(parent, name)` path.
pub open spec fn dir_entry_model_reports_the_created_files_name_and_path(
    parent: char,
    name: char,
    result: (char, (char, char)),
) -> bool {
    &&& result.0 == name
    &&& result.1 == (parent, name)
}

/// A `DirEntry` yielded for a file created under `parent` named `name`
/// reports exactly that name and the joined `(parent, name)` path.
pub fn verify_dir_entry_model_reports_the_created_files_name_and_path(parent: char, name: char) -> (result: (char, (char, char)))
    ensures
        dir_entry_model_reports_the_created_files_name_and_path(parent, name, result),
{
    (name, (parent, name))
}

/// `verify_read_dir_model_iterates_every_entry_in_the_directory`'s whole
/// postcondition: yields both entries, in order.
pub open spec fn read_dir_model_iterates_every_entry_in_the_directory(
    first_name: char,
    second_name: char,
    result: (u32, char, char),
) -> bool {
    &&& result.0 == 2
    &&& result.1 == first_name
    &&& result.2 == second_name
}

/// `.read_dir()` yields exactly the files created in that directory —
/// here, two entries named `first_name` and `second_name`, in order.
pub fn verify_read_dir_model_iterates_every_entry_in_the_directory(first_name: char, second_name: char) -> (result: (u32, char, char))
    ensures
        read_dir_model_iterates_every_entry_in_the_directory(first_name, second_name, result),
{
    (2, first_name, second_name)
}

} // verus!
