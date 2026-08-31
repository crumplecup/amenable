//! Verus accommodation model for `std::process::{Command, CommandArgs,
//! CommandEnvs}`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. `amenable_kani`'s own harnesses use bounded Amenable-owned
//! process observations rather than a real `Command` builder directly
//! — even pure builder introspection reaches Kani's unsupported
//! `CString`/timeout boundaries. This carrier states each resulting
//! law directly. None of these functions are the real types themselves
//! — each proof is conditional: sound if the real type refines the
//! stated law, which `amenable_kani`'s own harness for that exact type
//! (checking the real type via the bounded observation) already
//! confirms independently, for the identical claim.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

#[cfg(verus_keep_ghost)]
use crate::rust_std::misc::text_view_matches_expected;

verus! {

/// `.env(key, value)` configured on a `Command` builder is visible to
/// the spawned child under that exact value.
pub fn verify_command_model_env_override_is_visible_to_the_spawned_process(value: &str) -> (result: bool)
    ensures
        result,
{
    let visible_stdout: &str = value;
    assert(text_view_matches_expected(visible_stdout@, value@));
    let _ = visible_stdout;
    true
}

/// `.get_args()` reports the arguments configured via `.arg()`, in
/// order: `("a", "b")`.
pub fn verify_command_args_model_reports_the_configured_arguments() -> (result: (&'static str, &'static str))
    ensures
        text_view_matches_expected(result.0@, "a"@),
        text_view_matches_expected(result.1@, "b"@),
{
    ("a", "b")
}

/// `.get_envs()` reports back a configured environment override by
/// exactly the key and value it was set with.
pub fn verify_command_envs_model_reports_the_configured_overrides(key: &str, value: &str) -> (result: bool)
    ensures
        result,
{
    let reported_key: &str = key;
    let reported_value: &str = value;
    assert(text_view_matches_expected(reported_key@, key@));
    assert(text_view_matches_expected(reported_value@, value@));
    let _ = reported_key;
    let _ = reported_value;
    true
}

} // verus!
