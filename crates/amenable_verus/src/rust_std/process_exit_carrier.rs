//! Verus accommodation model for `std::process::{ExitStatus, Output}`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. `amenable_kani`'s own harnesses use bounded Amenable-owned
//! process observations rather than a real spawned process's exit
//! status directly — the direct `Command::status`/`.output()` paths
//! reach Kani's unsupported spawn/`Stdio`-conversion boundaries. This
//! carrier states each resulting law directly. `Output`'s stdout-text
//! claim is modeled abstractly as a `bool`, matching
//! `process_child_carrier.rs`'s own reasoning (`vstd` has no spec
//! support for `str::contains`) — its exit code stays the literal `0`
//! rather than a symbolic parameter, since a zero code is specifically
//! what makes `success()` true (unlike `ExitStatus`'s law here, which
//! holds for any nonzero code). Neither function is `ExitStatus`/
//! `Output` themselves — each proof is conditional: sound if the real
//! type refines the stated law, which `amenable_kani`'s own harness for
//! that exact type (checking the real type via the bounded observation)
//! already confirms independently, for the identical claim.

use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

/// A process that exits with any nonzero code reports `!success()` and
/// exactly that code.
pub fn verify_exit_status_model_reports_a_nonzero_exit_code(exit_code: i32) -> (result: (bool, i32))
    requires
        exit_code != 0,
    ensures
        !result.0,
        result.1 == exit_code,
{
    (false, exit_code)
}

/// `.output()` bundles a command's exit status (code 0, success) with
/// the stdout it produced — the text claim modeled abstractly as a
/// boolean.
pub fn verify_output_model_captures_stdout_and_the_exit_status() -> (result: (bool, i32, bool))
    ensures
        result.0,
        result.1 == 0,
        result.2,
{
    (true, 0, true)
}

} // verus!
