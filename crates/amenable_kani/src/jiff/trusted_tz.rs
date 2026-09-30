//! Trusted `jiff::tz::*` types for Kani -- zone/offset builder and
//! iterator types with no reachable constructor, or a real Kani-
//! specific wall (see each paragraph below).

//! `jiff::tz::AmbiguousOffset` stays trusted, the same "zero public
//! methods beyond derives" shape as `RoundMode`: checked directly
//! against jiff's real source, it has *no* public methods at all —
//! only a `pub(crate) from_jcore` conversion. Its three variants
//! (`Unambiguous`/`Gap`/`Fold`) do have public struct-style fields,
//! but with nothing beyond the standard derives
//! (`Clone`/`Copy`/`Debug`/`Eq`/`PartialEq`) producing or consuming
//! this type from outside jiff, there is nothing non-tautological to
//! state about it on any backend.
//!
//! `jiff::tz::AmbiguousTimestamp` stays trusted for Kani
//! specifically, for a real reason confirmed empirically (not the
//! default): the only way to build one from outside jiff is via
//! `TimeZone::to_ambiguous_timestamp(dt)`, and even a fully
//! CONCRETE `TimeZone::UTC.to_offset(..)` call already times out
//! per `gallery::jiff_error_drop_cost`'s own doc comment — the wall
//! is in `TimeZone`'s own hand-rolled pointer-tagged `Repr`-dispatch
//! machinery (`repr::each!`), not in symbolic-input complexity or
//! Drop glue, so no bounds-check-before-construct trick routes
//! around it. Checked on Creusot and Verus instead (`ext_jiff::
//! tz_ambiguous_timestamp`/`ext::jiff::tz_ambiguous_timestamp`),
//! neither of which shares this Rust-CBMC-specific mechanism.
//!
//! `jiff::tz::AmbiguousZoned` also stays trusted for Kani, without a
//! redundant fresh confirmation run this time: real jiff source
//! confirms `TimeZone::to_ambiguous_zoned` calls `self.clone().
//! into_ambiguous_zoned(dt)`, which calls `self.to_ambiguous_
//! timestamp(dt)` directly — the SAME real function just confirmed
//! above to time out under CBMC even for a fully concrete call.
//! Checked on Creusot and Verus instead.
//!
//! `jiff::tz::Disambiguation` stays trusted, the same "zero public
//! methods beyond derives" shape as `RoundMode`/`AmbiguousOffset`:
//! checked directly against jiff's real source, there is no `impl
//! Disambiguation` block at all — a real, `#[non_exhaustive]`
//! four-variant configuration marker enum (`Compatible`/`Earlier`/
//! `Later`/`Reject`) consumed only by OTHER types' methods
//! (`AmbiguousTimestamp::compatible`/`DateTimeParser::
//! disambiguation`/etc.), never producing or inspecting anything of
//! its own beyond the standard derives (`Clone`/`Copy`/`Debug`/
//! `Default`).
//!
//! `jiff::tz::Dst` gets a real checked property too (see
//! `tz_dst.rs`): a real, plain two-variant enum (`No`/`Yes`) —
//! `From<bool>` always builds `Yes` for `true`/`No` for `false`, and
//! `is_dst()`/`is_std()` are each other's exact complement.
//!
//! `jiff::tz::OffsetArithmetic` stays trusted, the identical shape to
//! `TimestampArithmetic`/`ZonedArithmetic`: checked directly against
//! jiff's real source, it has *no* public methods of its own at all
//! — `checked_add`/`checked_neg`/`is_negative` are all private; only
//! the `From` impls (construction, not inspection) are public. Its
//! one field (`duration`) is private with no getter.
//!
//! `jiff::tz::OffsetConflict` also stays trusted for Kani, without a
//! redundant fresh confirmation run: real jiff source shows
//! `resolve_with`'s `AlwaysTimeZone` branch calls `TimeZone::
//! into_ambiguous_zoned` directly, the SAME real function already
//! confirmed to time out under CBMC. Checked on Creusot and Verus
//! instead (scoped to `AlwaysOffset`/`AlwaysTimeZone` — see
//! `ext_jiff::tz_offset_conflict`'s own doc comment for why
//! `PreferOffset`/`Reject` are out of scope).
//!
//! `jiff::tz::OffsetRound` stays trusted for the same builder-only
//! reason as `TimestampRound`/`ZonedRound`: checked directly against
//! jiff's real source, its four public methods (`new`/`smallest`/
//! `mode`/`increment`) are all plain setters, and all three fields
//! (`smallest`/`mode`/`increment`) are private with no getters —
//! `round` (the real rounding logic) is private too.
//!
//! `jiff::tz::TimeZone` also stays trusted for Kani, the type
//! underlying every `Repr`-dispatch CBMC wall already confirmed this
//! session for `AmbiguousTimestamp`/`AmbiguousZoned`/
//! `OffsetConflict` (all three call into it directly), plus the
//! original `gallery::jiff_error_drop_cost` confirmation of a fully
//! concrete `TimeZone::UTC.to_offset(..)` call timing out —
//! `to_fixed_offset`'s own real body ALSO dispatches through the same
//! `repr::each!` macro (confirmed by reading jiff's real source
//! directly). Checked on Creusot and Verus instead (`ext_jiff::
//! tz_time_zone`/`ext::jiff::tz_time_zone`), neither of which
//! executes the real body at all.
//!
//! `jiff::tz::TimeZoneDatabase` gets a real checked property too
//! (see `tz_time_zone_database.rs`): `none()`/`is_definitively_
//! empty()` are the checked subset — real jiff source confirms both
//! are genuinely cheap and dispatch-free for the `Repr::Empty` case,
//! a DIFFERENT (simpler) internal repr from `TimeZone`'s own
//! pointer-tagged union, checked directly rather than assumed unsafe
//! by resemblance.
//!
//! `jiff::tz::TimeZoneFollowingTransitions<'static>` stays trusted on
//! ALL THREE backends, for genuinely different, confirmed reasons
//! each: Kani — real jiff source confirms `TimeZone::following` is a
//! trivial, dispatch-free constructor, but the iterator's own
//! `next()` calls `TimeZone::next_transition`, which dispatches
//! through the SAME `repr::each!` macro already confirmed to time
//! out under CBMC. Creusot — a first attempt to `extern_spec!` its
//! `Iterator::next` hit a real "`IteratorSpec` is not satisfied"
//! error (`extern_spec!` doesn't support third-party `Iterator`
//! impls at all), and the fallback accommodation-model pattern
//! (`timestamp_series.rs`'s own precedent) would be genuinely
//! content-free here (`next()` is unconditionally `None`, no formula
//! to check consistency of) — see `amenable_creusot::ext::jiff`'s own
//! doc comment for the full toolchain findings. Verus — for
//! consistency with the same content-free reasoning (no real
//! internal branching to model beyond a constant).
//!
//! `jiff::tz::TimeZoneName<'static>` stays trusted too, checked
//! directly against jiff's real source: its own constructor (`fn
//! new`) is completely PRIVATE (not even `pub(crate)`) — there is no
//! way to build one from outside jiff at all. The only way to obtain
//! one is via `TimeZoneNameIter` (itself only populated by real IANA
//! tzdb data through `TimeZoneDatabase::available()`, deliberately
//! out of scope for this checklist). Its one real public method
//! (`as_str`) can never be exercised without a real instance.
//!
//! `jiff::tz::TimeZoneNameIter<'static>` gets a real checked property
//! too (see `tz_time_zone_name_iter.rs`): `TimeZoneDatabase::
//! none().available().next() == None` — real jiff source confirms
//! this is genuinely cheap and dispatch-free (a real, empty `Vec::
//! new().into_iter()`), unlike `TimeZoneFollowingTransitions`'s own
//! `TimeZone::Repr`-dispatching `next()`.
//!
//! `jiff::tz::TimeZoneOffsetInfo<'static>` also stays trusted for
//! Kani specifically, for the same `TimeZone::Repr`-dispatch reason
//! as `TimeZone`/`AmbiguousTimestamp`/`AmbiguousZoned`/
//! `OffsetConflict`: its only real production path,
//! `TimeZone::to_offset_info(timestamp)`, dispatches through the SAME
//! `repr::each!` macro already confirmed to time out under CBMC
//! (confirmed by reading jiff's real source directly — every match
//! arm, including the `FIXED` case, routes through that macro).
//! Checked on Creusot and Verus instead (`ext_jiff::
//! tz_time_zone_offset_info`/`ext::jiff::tz_time_zone_offset_info`),
//! neither of which executes the real body at all.
//!
//! `jiff::tz::TimeZonePrecedingTransitions<'static>` stays trusted on
//! ALL THREE backends too, the exact structural mirror of
//! `TimeZoneFollowingTransitions` confirmed by reading jiff's real
//! source directly (not assumed from the name): `preceding` is an
//! identical trivial, dispatch-free constructor
//! (`TimeZonePrecedingTransitions { tz: self, cur: timestamp }`), and
//! its iterator's own `next()` calls `TimeZone::previous_transition`,
//! which dispatches through the SAME `repr::each!` macro, with the
//! identical `UTC => None` match arm — so the same content-free
//! carve-out applies on Creusot/Verus (see `amenable_creusot::
//! ext::jiff`'s own doc comment for `TimeZoneFollowingTransitions`'s
//! full toolchain findings, which apply here unchanged: `extern_spec!`
//! fundamentally cannot support ANY third-party `Iterator` impl,
//! a general toolchain limitation, not one specific to that type).
//!
//! `jiff::tz::TimeZoneTransition<'static>` stays trusted too, the
//! LAST type in this checklist: confirmed by reading jiff's real
//! source directly, its only real constructor (`from_jcore`) is
//! `pub(crate)` — not reachable from outside jiff at all. The only
//! public production paths are `TimeZoneFollowingTransitions::next()`/
//! `TimeZonePrecedingTransitions::next()`, both already trusted this
//! session and scoped honestly to `TimeZone::UTC`, where neither ever
//! actually yields an item (`UTC => None` in both `next_transition`
//! and `previous_transition`) — so there is no way to construct or
//! inspect a real instance within this checklist's own real-tzdb-
//! avoidance scope. The same "no reachable constructor" category
//! `TimeZoneName<'static>` already established, confirmed
//! independently rather than assumed from that resemblance.

use crate::jiff::macros::impl_kani_witness_trusted_ext;

impl_kani_witness_trusted_ext!(
    jiff::tz::AmbiguousOffset,
    jiff::tz::AmbiguousTimestamp,
    jiff::tz::AmbiguousZoned,
    jiff::tz::Disambiguation,
    jiff::tz::OffsetArithmetic,
    jiff::tz::OffsetConflict,
    jiff::tz::OffsetRound,
    jiff::tz::TimeZone,
    jiff::tz::TimeZoneFollowingTransitions<'static>,
    jiff::tz::TimeZoneName<'static>,
    jiff::tz::TimeZoneOffsetInfo<'static>,
    jiff::tz::TimeZonePrecedingTransitions<'static>,
    jiff::tz::TimeZoneTransition<'static>,
);
