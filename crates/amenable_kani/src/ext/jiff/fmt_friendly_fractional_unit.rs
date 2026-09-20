//! `KaniWitness` for `amenable_ext::ExtStandard<jiff::fmt::friendly::
//! FractionalUnit>` — a real, checked property over jiff's actual
//! public API (`From<FractionalUnit> for Unit`), not a trusted stub.
//!
//! Unlike `Designator`/`Direction`, `FractionalUnit` has one real
//! public conversion (checked directly against jiff's real source,
//! `src/fmt/friendly/printer.rs`): `impl From<FractionalUnit> for
//! Unit`, a documented per-variant mapping (`Hour` -> `Unit::Hour`,
//! `Minute` -> `Unit::Minute`, `Second` -> `Unit::Second`,
//! `Millisecond` -> `Unit::Millisecond`, `Microsecond` ->
//! `Unit::Microsecond`). Checked by exhaustive enumeration of all 5
//! variants, not symbolic execution (there is no `kani::Arbitrary` for
//! a foreign, `#[non_exhaustive]` enum, and none is needed for a
//! domain this small).

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

use super::super::macros::{ExtCheckedProof, kani_ensures_ext};
use crate::rust_std::bridge_kani_witness;

impl crate::KaniWitness for ExtStandard<jiff::fmt::friendly::FractionalUnit> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_fmt_friendly_fractional_unit_from_matches_documented_mapping".to_owned(),
            VERIFY_FMT_FRIENDLY_FRACTIONAL_UNIT_FROM_MATCHES_DOCUMENTED_MAPPING_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(ExtStandard<jiff::fmt::friendly::FractionalUnit>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<jiff::fmt::friendly::FractionalUnit>",
        "kani",
        || <ExtStandard<jiff::fmt::friendly::FractionalUnit> as crate::KaniWitness>::proof()
            .to_string(),
    )
}

/// All 5 documented `FractionalUnit` variants paired with the `Unit`
/// each must convert to, in jiff's own declaration order.
const ALL_FRACTIONAL_UNIT_PAIRS: [(jiff::fmt::friendly::FractionalUnit, jiff::Unit); 5] = [
    (jiff::fmt::friendly::FractionalUnit::Hour, jiff::Unit::Hour),
    (
        jiff::fmt::friendly::FractionalUnit::Minute,
        jiff::Unit::Minute,
    ),
    (
        jiff::fmt::friendly::FractionalUnit::Second,
        jiff::Unit::Second,
    ),
    (
        jiff::fmt::friendly::FractionalUnit::Millisecond,
        jiff::Unit::Millisecond,
    ),
    (
        jiff::fmt::friendly::FractionalUnit::Microsecond,
        jiff::Unit::Microsecond,
    ),
];

kani_ensures_ext!(
    ExtStandard<jiff::fmt::friendly::FractionalUnit>,
    "amenable_ext::ExtStandard<jiff::fmt::friendly::FractionalUnit>",
    (),
    |()| {
        let mut all_ok = true;
        let mut i = 0;
        while i < ALL_FRACTIONAL_UNIT_PAIRS.len() {
            let (fractional, expected) = ALL_FRACTIONAL_UNIT_PAIRS[i];
            let actual: jiff::Unit = fractional.into();
            all_ok = all_ok && actual == expected;
            i += 1;
        }
        all_ok
    }
);

amenable_derive::harness! {
    kani, VERIFY_FMT_FRIENDLY_FRACTIONAL_UNIT_FROM_MATCHES_DOCUMENTED_MAPPING_SRC, {
        /// `FractionalUnit`'s `From`-conversion into `Unit` matches
        /// jiff's own documented per-variant mapping exactly, checked
        /// over all 5 documented variants — not an assumed slice.
        #[kani::proof]
        fn verify_fmt_friendly_fractional_unit_from_matches_documented_mapping() {
            assert!(
                ExtStandard::<jiff::fmt::friendly::FractionalUnit>::ensures(()),
                "FractionalUnit's From<Unit> conversion must match jiff's documented mapping"
            );
        }
    }
}
