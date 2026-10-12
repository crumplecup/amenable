//! `KaniWitness` for `amenable_ext::ExtStandard<chrono_tz::TzOffset>`.
//!
//! Trusted, not proven -- and genuinely has no wider claim available to
//! make, not just skipped for convenience. `TzOffset` has no public
//! constructor and no `Arbitrary`: the only way to obtain one is through
//! a real zone-rule lookup (`Tz::offset_from_local_datetime`/
//! `offset_from_utc_datetime`). Its three accessors (`fix()`,
//! `base_utc_offset()`, `dst_offset()`) are all defined directly in
//! terms of the same two private fields (confirmed by reading chrono-
//! tz's source: `fix()` computes `utc_offset + dst_offset`;
//! `base_utc_offset()`/`dst_offset()` return those same two fields
//! separately), so "fix() == base_utc_offset() + dst_offset()" is a
//! restatement of the type's own field layout, true by construction,
//! with no adversarial construction path to vary it over.
//!
//! The grittier question -- does the offset TzOffset carries actually
//! match reality -- has no claim in our scope either way: matching
//! historical/real-world fact is the IANA tzdb's job (external to
//! chrono-tz and to us, the same way we trust any other external data
//! source); matching what chrono-tz's own zone-rule engine computed is
//! tautological once the value came from that same engine. The one
//! place this question has real teeth -- whether a *descriptor-supplied*
//! offset actually matches one of the real offsets chrono-tz's rules
//! produce for a given local time -- is already checked for real by
//! `amenable_ext::chrono::zone`'s own `offset_is_consistent_with_named_
//! zone`, confirmed against a real DST transition.
//!
//! ## Black-box consistency investigation (findings, not left as probes)
//!
//! Reading chrono-tz's source suggested fix()/base_utc_offset()/
//! dst_offset() are tautologically related (same two private fields,
//! three ways). Treating chrono as a black box instead, the real test is
//! whether the PUBLIC API actually confirms that over a symbolic date --
//! isolated via five harnesses (since deleted; the findings below are
//! what they established, not left as live scaffolding):
//!
//! - `Etc::UTC` (one fixed timespan entry): passes at the standard 3m
//!   bound (98s).
//! - `America::New_York`, `Europe::London` (real multi-entry DST
//!   tables): both time out at the standard 3m bound, with identical
//!   unwind-iteration counts (349-350) -- suspicious, since table sizes
//!   genuinely differ.
//! - `America::Phoenix` (DST observed historically, none since 1967):
//!   also times out, at a different count (418-419) -- ruling out "one
//!   shared CBMC bound regardless of table" as the explanation.
//! - Pinning the query year to one concrete value (`America::New_York`,
//!   year=2024, month/day/hour/min/sec still symbolic): no effect --
//!   `offset_from_utc_datetime`'s cost comes from `binary_search`ing the
//!   ZONE's own timespan table, a fixed property of the zone, not from
//!   which date is being searched for.
//!
//! Conclusion: `binary_search` over a real per-zone timespan table is
//! CBMC-hostile for every non-degenerate real zone tested, a genuinely
//! different cost profile from `Tz::from_str`'s hashing wall but equally
//! intractable at full symbolic generality. The resolution path (per
//! [[feedback_proof_wall_is_hitl_moment]]) is sentinel discovery: find
//! the exact, finite set of real `(base_utc_offset, dst_offset)` pairs
//! any real zone can produce (via a black-box, non-Kani scan of the
//! public API, exhaustively bracketed at hourly granularity -- confirmed
//! tractable, 597 zones x the full `NaiveDate` range in under a minute),
//! then drive literal, concrete-date Kani harnesses from that discovered
//! table -- mirroring `Tz`'s own bin-of-64 resolution. The grid-walk
//! bound this scan's own completeness rests on is `amenable_ext::
//! BoundedStepWalk`, proven on all three backends. Still in progress:
//! the discovered table has not yet been turned into the final literal
//! harnesses, so this type remains `trusted` until it is.

use crate::ext_macros::impl_kani_witness_trusted_ext;

impl_kani_witness_trusted_ext!(chrono_tz::TzOffset);
