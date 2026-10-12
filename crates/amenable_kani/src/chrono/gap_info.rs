//! `KaniWitness` for `amenable_ext::ExtStandard<chrono_tz::GapInfo>`.
//!
//! The claim: for every discovered gap-transition witness row in
//! `amenable_ext::DISCOVERED_GAP_TABLE`, the real local date-time really
//! falls in a gap (`TimeZone::from_local_datetime` returns
//! `LocalResult::None`), `GapInfo::new` agrees (returns `Some`), and its
//! `begin`/`end` fields genuinely bracket the witness: `begin`'s own
//! local time (when present) is strictly before it, `end`'s own local
//! time (when present) is strictly after it.
//!
//! Domain discovery mirrors `TzOffset`'s own resolution: a real,
//! black-box, non-Kani scan (hourly grid, refined to one-second
//! precision via binary search once a transition hour is detected)
//! found that chrono-tz's real gap events -- 27,693 of them, 1970-2100
//! -- collapse to exactly 94 distinct `(prev_base, prev_dst) ->
//! (next_base, next_dst)` transitions, each confirmed directly against
//! the real public API before being committed (not assumed). The
//! completeness bound is the same `BoundedStepWalk` already proven: a
//! grid walk spaced `delta` apart cannot miss a real interval of length
//! `>= delta`, and detecting a transition only needs comparing the
//! offset just before versus just after a grid step -- true regardless
//! of how narrow the gap itself is, since no sample needs to land
//! *inside* the gap to detect that a change occurred.
//!
//! **Decomposition before partitioning** (isolating the expensive
//! operation first, not just varying bin size around the whole claim):
//! construction alone (`NaiveDate`/`NaiveTime`/`NaiveDateTime`, no zone
//! lookup) is trivial (0.12s); `from_local_datetime` alone costs 92.2s;
//! `GapInfo::new` alone costs 134.9s -- and both together cost only
//! 137.2s, barely more than `GapInfo::new` alone. The two real lookups
//! are not additive: once CBMC has explored one zone's timespan table
//! for one binary search, a second search over the same table is
//! nearly free, almost certainly shared symbolic/bit-vector state. The
//! real cost is `GapInfo::new`'s own internal logic (two binary
//! searches plus `begin`/`end`'s own extra date conversions), not the
//! combination -- there is no cheaper reformulation available; the
//! claim genuinely needs `GapInfo::new`'s full cost, and dropping
//! `from_local_datetime` would only save ~2s while weakening the
//! actual agreement claim being tested.
//!
//! With the cost isolated, not just the whole claim's timing varied:
//! bins of 2 are confirmed safe with consistent ~28-33s margin at
//! three different table positions (items 0-2: 152.2s, 46-48: 147.6s,
//! 92-94: 153.8s), while a single call already costs 137.2s and
//! combining four calls (169.2s) leaves only 11s of margin -- `ceil(94
//! / 2) = 47` harnesses, smaller bins than `TzOffset`'s 16 because
//! `GapInfo::new`'s own per-item cost is genuinely higher, confirmed
//! directly rather than assumed to transfer.

use amenable_core::Evidence;
use amenable_ext::ExtStandard;
use chrono_tz::GapInfo;

use crate::ext_macros::ExtCheckedProof;
use crate::rust_std::bridge_kani_witness;

impl crate::KaniWitness for ExtStandard<GapInfo> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "gap_info_consistency_holds".to_owned(),
            GAP_INFO_CONSISTENCY_HOLDS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(ExtStandard<GapInfo>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<chrono_tz::GapInfo>",
        "kani",
        || <ExtStandard<GapInfo> as crate::KaniWitness>::proof().to_string(),
    )
}

amenable_derive::gallery_harness! {
    kani, GAP_INFO_CONSISTENCY_HOLDS_SRC, {
        /// The claim, for one discovered table row: the real local
        /// witness genuinely falls in a gap, `GapInfo::new` agrees, and
        /// its fields genuinely bracket the witness.
        fn gap_info_consistency_holds(
            tz_index: usize,
            year: i32,
            month: u32,
            day: u32,
            hour: u32,
            minute: u32,
            second: u32,
        ) -> bool {
            use chrono::{NaiveDate, NaiveTime, TimeZone};

            let Some(date) = NaiveDate::from_ymd_opt(year, month, day) else {
                return false;
            };
            let Some(time) = NaiveTime::from_hms_opt(hour, minute, second) else {
                return false;
            };
            let local = chrono::NaiveDateTime::new(date, time);
            let tz = chrono_tz::TZ_VARIANTS[tz_index];

            if !matches!(tz.from_local_datetime(&local), chrono::LocalResult::None) {
                return false;
            }

            let Some(info) = chrono_tz::GapInfo::new(&local, &tz) else {
                return false;
            };

            if let Some((begin_local, _begin_offset)) = info.begin
                && begin_local >= local
            {
                return false;
            }
            if let Some(end_zoned) = info.end
                && end_zoned.naive_local() <= local
            {
                return false;
            }
            true
        }
    }
}

seq_macro::seq!(N in 0..47 {
    amenable_derive::harness! {
        kani, BIN_SRC_~N, {
            /// Bin N of 47: every discovered table row whose own index
            /// falls in this bin's sub-range (concrete at each unrolled
            /// step, never symbolic) satisfies the claim.
            #[kani::proof]
            fn gap_info_bin_~N() {
                let start = N * 2;
                let end = core::cmp::min(start + 2, amenable_ext::DISCOVERED_GAP_TABLE.len());
                for i in start..end {
                    let (tz_index, year, month, day, hour, minute, second, ..) =
                        amenable_ext::DISCOVERED_GAP_TABLE[i];
                    assert!(gap_info_consistency_holds(
                        tz_index, year, month, day, hour, minute, second
                    ));
                }
            }
        }
    }
});
