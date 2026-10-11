//! `KaniWitness` for `amenable_ext::ExtStandard<chrono_tz::Tz>`.
//!
//! The claim: every one of chrono-tz's 597 compiled-in zones round-trips
//! through its own `name()`/`FromStr` pair.
//!
//! **Investigation, before settling on this shape** (full decomposition
//! recorded for the record, not just the conclusion):
//!
//! - The naive approach -- a single harness, symbolic `usize` index into
//!   the real `TZ_VARIANTS` array, `TZ_VARIANTS[index].name().parse() ==
//!   Ok(...)` -- times out. Isolating why, piece by piece: `.name()`
//!   alone over a full symbolic index is cheap (~1.6s); `.parse()` alone
//!   on one hand-typed literal is cheap (~3.2s); even reducing the
//!   symbolic index's reachable range to exactly one value (`assume(index
//!   < 1)`) still times out for the combined pipeline. The cost is real
//!   and lives specifically in `.parse()`'s hash (`chrono_tz`'s `FromStr`
//!   resolves through a `siphasher`-based static table): confirmed
//!   directly by feeding `.parse()` a single genuinely-symbolic ASCII
//!   character (built via `KaniCompose`, which never calls the real UTF-8
//!   validator -- an earlier, cruder symbolic-byte-array attempt
//!   conflated that unrelated wall in first). Even one symbolic byte of
//!   *content* reaching the hash is enough; a fully concrete `Tz` (a
//!   literal variant, or `TZ_VARIANTS[3]` with `3` a syntactic literal)
//!   round-trips the whole pipeline in ~3.7s regardless of which concrete
//!   value.
//! - That split -- cheap when every value is concrete, expensive the
//!   moment any value is symbolic, independent of range size -- means a
//!   single symbolic-index harness can never work here, but many
//!   concrete-index harnesses can. A single `for i in 0..597` loop inside
//!   one proof (no symbolic index, `i` concrete at each unrolled step)
//!   avoids the hashing wall, but hits a different one: measured growth
//!   is clearly super-linear (4: 6.8s, 8: 12.4s, 16: 25.4s, 32: 54.7s, 64:
//!   131.7s, 128: 456.4s -- the doubling ratio climbs from 1.84x to
//!   3.47x), so one loop covering all 597 is impractical.
//! - The chosen compromise: bins of 64 (comfortably measured tractable,
//!   ~132s), giving `ceil(597 / 64) = 10` harnesses instead of either one
//!   (intractable) or 597 (excessive). Each bin is a plain `for` loop
//!   over its own concrete sub-range, generated via `seq_macro` rather
//!   than hand-written 10 times.

use amenable_core::Evidence;
use amenable_ext::ExtStandard;
use chrono_tz::Tz;

use crate::ext_macros::ExtCheckedProof;
use crate::rust_std::bridge_kani_witness;

impl crate::KaniWitness for ExtStandard<Tz> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "tz_name_and_from_str_round_trip_holds".to_owned(),
            TZ_NAME_AND_FROM_STR_ROUND_TRIP_HOLDS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(ExtStandard<Tz>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<chrono_tz::Tz>",
        "kani",
        || <ExtStandard<Tz> as crate::KaniWitness>::proof().to_string(),
    )
}

amenable_derive::gallery_harness! {
    kani, TZ_NAME_AND_FROM_STR_ROUND_TRIP_HOLDS_SRC, {
        /// The claim: `name()` and `FromStr` are inverses, for every real
        /// `Tz` variant. A plain function, not a Kani contract -- asserted
        /// by the 10 binned harnesses below, never proven directly as one
        /// full-domain harness (see the module doc comment for why).
        fn tz_name_and_from_str_round_trip_holds(tz: Tz) -> bool {
            use std::str::FromStr;
            Tz::from_str(tz.name()) == Ok(tz)
        }
    }
}

seq_macro::seq!(N in 0..10 {
    amenable_derive::harness! {
        kani, BIN_SRC_~N, {
            /// Bin N of 10: every real `Tz` variant whose `TZ_VARIANTS`
            /// index falls in this bin's sub-range (concrete at each
            /// unrolled step, never symbolic) satisfies the claim.
            #[kani::proof]
            fn tz_bin_~N() {
                let start = N * 64;
                let end = core::cmp::min(start + 64, chrono_tz::TZ_VARIANTS.len());
                for i in start..end {
                    assert!(tz_name_and_from_str_round_trip_holds(chrono_tz::TZ_VARIANTS[i]));
                }
            }
        }
    }
});
