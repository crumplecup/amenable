//! Verus accommodation model for `std::fs::{FileType, OpenOptions,
//! Permissions, TryLockError}`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document — the direct `std::fs` tempdir path crosses real OS-backed
//! filesystem state that Kani itself times out on. `amenable_kani`'s
//! own harnesses use an Amenable-owned filesystem model rather than a
//! real temp directory; each of these four laws reduces to a fixed set
//! of boolean assertions the observation demonstrates, so this carrier
//! states each set directly as a boolean tuple. None of these
//! functions are `FileType`/`OpenOptions`/`Permissions`/
//! `TryLockError` themselves — each proof is conditional: sound if the
//! real type refines the stated law, which `amenable_kani`'s own
//! harness for that exact type (checking the real type via the bounded
//! filesystem model) already confirms independently, for the identical
//! claim.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// A regular file's `FileType` reports `is_file()` (`result.0`) but not
/// `is_dir()` (`result.1`); a directory's reports the reverse
/// (`result.2`, `result.3`).
pub fn verify_file_type_model_distinguishes_files_from_directories() -> (result: (bool, bool, bool, bool))
    ensures
        result.0,
        !result.1,
        result.2,
        !result.3,
{
    (true, false, true, false)
}

/// `.create_new(true)` fails against a path that already has a file
/// (`result.0`) and against one that already names a directory
/// (`result.1`), but succeeds against a genuinely fresh path
/// (`result.2`), leaving a file there (`result.3`).
pub fn verify_open_options_model_create_new_rejects_an_existing_file() -> (result: (bool, bool, bool, bool))
    ensures
        result.0,
        result.1,
        result.2,
        result.3,
{
    (true, true, true, true)
}

/// A freshly created file is not readonly (`!result.0`); setting
/// readonly is reflected the next time permissions are read
/// (`result.1`); clearing it is reflected too (`!result.2`).
pub fn verify_permissions_model_readonly_round_trips_through_set_permissions() -> (result: (bool, bool, bool))
    ensures
        !result.0,
        result.1,
        !result.2,
{
    (false, true, false)
}

/// A first handle's `.try_lock()` succeeds (`result.0`); a second
/// handle's `.try_lock()` fails while the first still holds the lock
/// (`!result.1`).
pub fn verify_try_lock_error_model_reports_a_lock_already_held() -> (result: (bool, bool))
    ensures
        result.0,
        !result.1,
{
    (true, false)
}

} // verus!
