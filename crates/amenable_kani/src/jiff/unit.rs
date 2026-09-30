//! `KaniWitness` for `amenable_ext::ExtStandard<jiff::Unit>` — a real,
//! checked ordering property over jiff's actual public API
//! (`Ord`/`PartialOrd` for `Unit`), not a trusted stub.
//!
//! `jiff::Unit` is a fieldless enum with explicit discriminants
//! (`Year = 9` down to `Nanosecond = 0`, confirmed by reading jiff's
//! real source, `src/span.rs`) and derives `PartialOrd`/`Ord` — its
//! own doc comment states the real law directly: "bigger units
//! compare greater than smaller units" (`Unit::Year >
//! Unit::Nanosecond`, etc.). Ten variants, a finite domain — checked
//! by exhaustive enumeration of all 100 ordered pairs, not symbolic
//! execution (there is no `kani::Arbitrary` for a foreign enum with no
//! derive for it, and none is needed for a domain this small).
//!
//! Trusted on Creusot specifically, for a real, confirmed reason: a
//! first attempt at an `extern_spec!` for `Ord::cmp` on `jiff::Unit`
//! compiled fine as a bare declaration, but calling it for real inside
//! a harness failed with "the trait bound `jiff::Unit:
//! creusot_std::model::DeepModel` is not satisfied" — the same class
//! of constraint this crate's own `rust_std::cmp_carriers` already
//! documents for `Reverse<T>: OrdLogic` (a foreign type's comparison
//! machinery needs a model Creusot can't derive for third-party
//! types), just reached directly on a concrete enum this time rather
//! than through a generic wrapper's blanket impl.

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

use super::macros::{ExtCheckedProof, kani_ensures_ext};
use crate::rust_std::bridge_kani_witness;

impl crate::KaniWitness for ExtStandard<jiff::Unit> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_unit_ordering_matches_discriminant_order".to_owned(),
            VERIFY_UNIT_ORDERING_MATCHES_DISCRIMINANT_ORDER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(ExtStandard<jiff::Unit>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<jiff::Unit>",
        "kani",
        || <ExtStandard<jiff::Unit> as crate::KaniWitness>::proof().to_string(),
    )
}

/// All ten `Unit` variants, in the same declaration/discriminant order
/// jiff's own source uses (`Year` highest down to `Nanosecond` lowest).
const ALL_UNITS: [jiff::Unit; 10] = [
    jiff::Unit::Year,
    jiff::Unit::Month,
    jiff::Unit::Week,
    jiff::Unit::Day,
    jiff::Unit::Hour,
    jiff::Unit::Minute,
    jiff::Unit::Second,
    jiff::Unit::Millisecond,
    jiff::Unit::Microsecond,
    jiff::Unit::Nanosecond,
];

kani_ensures_ext!(
    ExtStandard<jiff::Unit>,
    "amenable_ext::ExtStandard<jiff::Unit>",
    (),
    |()| {
        let mut all_ok = true;
        let mut i = 0;
        while i < ALL_UNITS.len() {
            let mut j = 0;
            while j < ALL_UNITS.len() {
                let real = ALL_UNITS[i].cmp(&ALL_UNITS[j]);
                // `ALL_UNITS` is declared highest-rank-first (`Year` at
                // index 0 down to `Nanosecond` at index 9), so a
                // *lower* index means a *higher*-ranked unit -- the
                // expected comparison is the reverse of the plain
                // index comparison.
                let expected = i.cmp(&j).reverse();
                all_ok = all_ok && real == expected;
                j += 1;
            }
            i += 1;
        }
        all_ok
    }
);

amenable_derive::harness! {
    kani, VERIFY_UNIT_ORDERING_MATCHES_DISCRIMINANT_ORDER_SRC, {
        /// `Unit`'s derived `Ord` compares by discriminant value —
        /// "bigger units compare greater," exactly as jiff's own doc
        /// comment states. Checked by exhaustive enumeration of all
        /// 100 ordered pairs among the ten real variants, not an
        /// assumed slice.
        #[kani::proof]
        fn verify_unit_ordering_matches_discriminant_order() {
            assert!(
                ExtStandard::<jiff::Unit>::ensures(()),
                "Unit's derived Ord must compare every pair of variants in exactly the \
                 documented declaration order, largest (Year) to smallest (Nanosecond)"
            );
        }
    }
}
