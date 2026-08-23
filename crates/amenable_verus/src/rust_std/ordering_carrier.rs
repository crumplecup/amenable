//! Verus spec for `std::cmp::Ordering`.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

// `Ordering::reverse`'s real body isn't visible to Verus (a plain `match`
// over three variants, same as the Kani/Creusot harnesses check, but
// Verus reports "not supported" for it directly — same class of "trust
// the real method via an external contract" gap Creusot's own
// `Ordering::reverse` needed `extern_spec!` for). `assume_specification`
// is Verus's equivalent: a trusted axiom on the real, unmodified
// `std::cmp::Ordering::reverse`, not a shadow/local reimplementation.
pub assume_specification [core::cmp::Ordering::reverse] (o: core::cmp::Ordering) -> (result: core::cmp::Ordering)
    ensures
        o == core::cmp::Ordering::Less ==> result == core::cmp::Ordering::Greater,
        o == core::cmp::Ordering::Equal ==> result == core::cmp::Ordering::Equal,
        o == core::cmp::Ordering::Greater ==> result == core::cmp::Ordering::Less,
;

/// `Ordering` has exactly three inhabitants, and `.reverse()` swaps
/// `Less`/`Greater` while fixing `Equal` — the same claim the Kani/Creusot
/// harnesses check, restated here as a real, `verus`-checked
/// postcondition. Rests on the `assume_specification` above, the same
/// trusted-axiom relationship every non-primitive Verus proof in this
/// crate has to the real method it exercises.
pub fn verify_ordering_reverse_swaps_less_and_greater(o: core::cmp::Ordering) -> (result: core::cmp::Ordering)
    ensures
        o == core::cmp::Ordering::Less ==> result == core::cmp::Ordering::Greater,
        o == core::cmp::Ordering::Equal ==> result == core::cmp::Ordering::Equal,
        o == core::cmp::Ordering::Greater ==> result == core::cmp::Ordering::Less,
{
    o.reverse()
}

} // verus!
