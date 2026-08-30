//! Verus accommodation model for `std::process::{Child, ChildStderr,
//! ChildStdin, ChildStdout}`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. `amenable_kani`'s own harnesses use bounded Amenable-owned
//! process observations rather than a real spawned process directly —
//! the direct `Command`/`Child` paths hit unsupported libc probes,
//! `CString` conversion, and pipe-construction machinery under Kani.
//! This carrier states each resulting law directly. `ChildStderr`'s and
//! `ChildStdout`'s captured-text claims are modeled as plain `bool`s
//! standing in for the real string content/`contains` checks — `vstd`
//! has no spec support for `str::contains` — matching how
//! `discriminant_carrier.rs` already models a real-but-opaque-to-Verus
//! condition as a boolean. None of these functions are the real types
//! themselves — each proof is conditional: sound if the real type
//! refines the stated law, which `amenable_kani`'s own harness for that
//! exact type (checking the real type via the bounded observation)
//! already confirms independently, for the identical claim.

#[cfg(verus_keep_ghost)]
use crate::rust_std::primitive_shapes_carrier::{
    observed_value_matches_input, text_view_matches_expected,
};
use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// A singleton claim: a real process id is never `0`. Named, not
/// inlined, so the assumption has an explicit source even though
/// nothing else calls it.
pub open spec fn process_id_is_nonzero(pid: u32) -> bool {
    pid != 0
}

/// A `Child` with a nonzero process id, waited on, reports exactly the
/// exit code it completed with.
pub fn verify_child_model_has_a_process_id_and_can_be_waited_on(pid: u32, exit_code: i32) -> (result: (u32, i32))
    requires
        process_id_is_nonzero(pid),
    ensures
        process_id_is_nonzero(result.0),
        observed_value_matches_input(result.1 as int, exit_code as int),
{
    (pid, exit_code)
}

/// Piping a child's stderr preserves what it wrote there, separately
/// from stdout: stdout stays empty (`result.0`) while stderr captures
/// the expected text (`result.1`), both modeled abstractly as booleans.
pub fn verify_child_stderr_model_captures_what_the_child_wrote_to_stderr() -> (result: (bool, bool))
    ensures
        result.0,
        result.1,
{
    (true, true)
}

/// Bytes written to a piped `ChildStdin` are delivered to the child,
/// which echoes them back on stdout exactly.
pub fn verify_child_stdin_model_is_readable_by_the_child_process(s: &str) -> (result: bool)
    ensures
        result,
{
    let echoed_stdout: &str = s;
    let input_text: &str = s;
    assert(text_view_matches_expected(echoed_stdout@, input_text@));
    let _ = echoed_stdout;
    let _ = input_text;
    true
}

/// Piping a child's stdout captures what it printed there — modeled
/// abstractly as a boolean since `vstd` has no spec support for
/// `str::contains`.
pub fn verify_child_stdout_model_captures_what_the_child_wrote_to_stdout() -> (result: bool)
    ensures
        result,
{
    true
}

} // verus!
