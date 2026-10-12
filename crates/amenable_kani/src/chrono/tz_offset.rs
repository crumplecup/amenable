//! `KaniWitness` for `amenable_ext::ExtStandard<chrono_tz::TzOffset>`.
//!
//! The claim: for every discovered `(zone, witness date, base, dst)` row
//! in `amenable_ext::DISCOVERED_OFFSET_TABLE`, calling the real
//! `Tz::offset_from_utc_datetime` at that exact witness produces a
//! `TzOffset` whose `fix()` equals `base_utc_offset() + dst_offset()`,
//! and whose `base_utc_offset()`/`dst_offset()` match the table's own
//! stored values (confirming the committed table hasn't gone stale).
//!
//! **Investigation, before settling on this shape** (full decomposition
//! recorded for the record, not just the conclusion -- see the module's
//! own prior doc comment, preserved here as narrative since the probe
//! code itself has been superseded):
//!
//! - `TzOffset` has no public constructor and no `Arbitrary`: the only
//!   way to obtain one is through a real zone-rule lookup. Reading
//!   chrono-tz's source suggested `fix()`/`base_utc_offset()`/
//!   `dst_offset()` are tautologically related (same two private
//!   fields, three ways) -- but treating chrono as a black box, the
//!   real test is whether the public API confirms that, not our own
//!   reading of source.
//! - A single harness, fully symbolic date, real zone lookup: times out
//!   for every non-degenerate real zone tested (`America::New_York`,
//!   `Europe::London`, `America::Phoenix` all time out at the standard
//!   3-minute bound with different-but-nonzero unwind-iteration counts;
//!   only `Etc::UTC`, a one-entry table, passes). Isolated to
//!   `offset_from_utc_datetime`'s own `binary_search` over each zone's
//!   real timespan table -- a structurally different cost than `Tz::
//!   from_str`'s hashing wall, but similarly CBMC-hostile at full
//!   symbolic generality. Pinning the query year to one concrete value
//!   had no effect: `timespans.len()` is a fixed property of the zone,
//!   not of which date is searched for.
//! - Resolution: sentinel discovery, the same shape that resolved `Tz`
//!   itself. Dates look unbounded, but real transitions are not: a
//!   black-box, non-Kani scan (`amenable_ext::offset_discovery`'s real
//!   `OffsetDiscoveryRequest`/`DiscoveredOffsetTable` `Exchange`) finds
//!   the exact, finite set of real `(base, dst)` pairs any real zone
//!   produces, each with one concrete witness date -- turning an
//!   apparently-unbounded date space into a bounded, complete table the
//!   same way `TZ_VARIANTS` bounded `Tz` itself. The completeness bound
//!   that scan rests on is `amenable_ext::BoundedStepWalk` (proven on
//!   all three backends): a grid walk spaced `delta` apart cannot miss
//!   any real interval of length `>= delta`, so anything the table
//!   doesn't record had a real duration strictly less than `delta` --
//!   confirmed tractable and correct via two independent granularities
//!   (daily and hourly over `1850-01-01..2100-01-01` both find exactly
//!   453 distinct pairs).
//! - Each row is now a fully concrete `(zone, date)` pair, not a
//!   symbolic one -- but unlike `Tz`'s own per-item cost (dominated by
//!   hashing, collapsing cheaply once concrete), `TzOffset`'s per-item
//!   cost (`binary_search` over a real zone's timespan table) does
//!   *not* collapse as fully: a fixed ~100s baseline dominates small
//!   bins, but combining ranges within one proof still compounds
//!   super-linearly (measured directly, not assumed: four independent
//!   16-item windows at different table positions all land at
//!   108-146s, while a single 64-item bin -- and even two *disjoint*
//!   32-item halves combined into one 64-item proof -- times out at
//!   the standard 3-minute bound, even though each 32-item half alone
//!   is comfortably tractable). Bins of 16 (`ceil(453 / 16) = 29`
//!   harnesses) is the size actually confirmed safe with real margin
//!   across the table, not merely assumed from `Tz`'s own different
//!   cost profile -- generated via `seq_macro`.

use amenable_core::Evidence;
use amenable_ext::ExtStandard;
use chrono_tz::TzOffset;

use crate::ext_macros::ExtCheckedProof;
use crate::rust_std::bridge_kani_witness;

impl crate::KaniWitness for ExtStandard<TzOffset> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "tz_offset_consistency_holds".to_owned(),
            TZ_OFFSET_CONSISTENCY_HOLDS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(ExtStandard<TzOffset>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<chrono_tz::TzOffset>",
        "kani",
        || <ExtStandard<TzOffset> as crate::KaniWitness>::proof().to_string(),
    )
}

amenable_derive::gallery_harness! {
    kani, TZ_OFFSET_CONSISTENCY_HOLDS_SRC, {
        /// The claim, for one discovered table row: the real `TzOffset`
        /// at this exact witness satisfies `fix() == base_utc_offset()
        /// + dst_offset()`, and its `base_utc_offset()`/`dst_offset()`
        /// match the table's own stored values. A plain function, not a
        /// Kani contract -- asserted by the 29 binned harnesses below,
        /// never proven directly as one full-domain harness (see the
        /// module doc comment for why).
        fn tz_offset_consistency_holds(
            tz_index: usize,
            year: i32,
            month: u32,
            day: u32,
            hour: u32,
            table_base: i64,
            table_dst: i64,
        ) -> bool {
            use chrono::{NaiveDate, Offset, TimeZone};
            use chrono_tz::OffsetComponents;

            let Some(date) = NaiveDate::from_ymd_opt(year, month, day) else {
                return false;
            };
            let Some(time) = chrono::NaiveTime::from_hms_opt(hour, 0, 0) else {
                return false;
            };
            let dt = chrono::NaiveDateTime::new(date, time);
            let tz = chrono_tz::TZ_VARIANTS[tz_index];
            let offset = tz.offset_from_utc_datetime(&dt);

            let fix_secs = i64::from(offset.fix().local_minus_utc());
            let base_secs = offset.base_utc_offset().num_seconds();
            let dst_secs = offset.dst_offset().num_seconds();

            fix_secs == base_secs + dst_secs && base_secs == table_base && dst_secs == table_dst
        }
    }
}

seq_macro::seq!(N in 0..29 {
    amenable_derive::harness! {
        kani, BIN_SRC_~N, {
            /// Bin N of 29: every discovered table row whose own index
            /// falls in this bin's sub-range (concrete at each unrolled
            /// step, never symbolic) satisfies the claim.
            #[kani::proof]
            fn tz_offset_bin_~N() {
                let start = N * 16;
                let end = core::cmp::min(start + 16, amenable_ext::DISCOVERED_OFFSET_TABLE.len());
                for i in start..end {
                    let (tz_index, year, month, day, hour, base, dst) =
                        amenable_ext::DISCOVERED_OFFSET_TABLE[i];
                    assert!(tz_offset_consistency_holds(
                        tz_index, year, month, day, hour, base, dst
                    ));
                }
            }
        }
    }
});
