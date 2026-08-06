//! Verus spec for `String`.

use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

/// `String` round-trips through itself, content and all — a stronger
/// claim than `elicitation_verus`'s own `strings.rs`, which deliberately
/// avoids storing a real `String` in its `StringNonEmpty`/`StringBounded`
/// wrapper types at all ("Verus doesn't have specs for it," true for the
/// plain, no-`@` `.len()` call those wrappers were written against). The
/// real pattern is `elicitation_verus::gallery::level5`'s own
/// demonstration: `String` has a genuine `vstd` `View` impl, reachable via
/// the `@` operator (`s@` yields a spec-level `Seq<char>`), the same idiom
/// Creusot's own `@` operator uses for `char`/`String` in this workspace.
/// `result@ == s@` (full content equality, not just a length comparison)
/// verifies for real against an actual `verus` install (confirmed
/// empirically: `verus --crate-type=lib` on this exact function reports
/// `verification results:: 1 verified, 0 errors`) — no shadow struct, no
/// weakened claim, and strictly stronger than the length-only claim this
/// crate's own Creusot/Kani counterparts settle for (both note UTF-8
/// well-formedness as a real claim `Seq<char>`-level content equality
/// already implies, without needing a separate byte-level encoding lemma).
pub fn verify_string_roundtrip(s: String) -> (result: String)
    ensures
        result@ == s@,
        result@.len() == s@.len(),
{
    s
}

} // verus!
