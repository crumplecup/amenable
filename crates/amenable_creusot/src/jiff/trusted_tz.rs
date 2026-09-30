//! Trusted `jiff::tz::*` types -- zone/offset builder and iterator
//! types with no reachable constructor or no non-tautological
//! property to state.

//! `jiff::tz::AmbiguousOffset` stays trusted, the same "zero public
//! methods beyond derives" shape as `RoundMode`: checked directly
//! against jiff's real source, it has *no* public methods at all —
//! only a `pub(crate) from_jcore` conversion. Nothing
//! non-tautological to state about it on any backend.
//!
//! `jiff::tz::AmbiguousTimestamp` gets a real checked property too
//! (see `tz_ambiguous_timestamp.rs`), checked here despite being
//! trusted for Kani specifically — see `amenable_kani::ext::jiff`'s
//! own doc comment for the real, confirmed `TimeZone` `Repr`-dispatch
//! CBMC wall. Deliberately NARROWER than a full claim would be:
//! `civil::DateTime` stays opaque (trusted, no decomposition
//! accessors) by deliberate Phase 1 design, so `dt` passes through
//! this proof entirely unexamined; only `.offset()`/`.is_ambiguous()`
//! are checked, decomposed via fresh opaque accessors tied to a
//! `TimeZone::fixed`-specific one (`tz_fixed_seconds_value`, hoisted
//! `pub(crate)` for `TimeZone`'s own future checklist entry to reuse).
//!
//! `jiff::tz::AmbiguousZoned` gets a real checked property too (see
//! `tz_ambiguous_zoned.rs`), the same shape and scope as
//! `AmbiguousTimestamp`: reuses that file's own `tz_fixed_seconds_
//! value` accessor. Trusted for Kani too, without a redundant fresh
//! confirmation run — real jiff source confirms `to_ambiguous_zoned`
//! calls `to_ambiguous_timestamp` directly, the SAME real function
//! already empirically confirmed to time out under CBMC.
//!
//! `jiff::tz::Disambiguation` stays trusted, the same "zero public
//! methods beyond derives" shape as `RoundMode`/`AmbiguousOffset`:
//! there is no `impl Disambiguation` block at all — a real,
//! `#[non_exhaustive]` four-variant configuration marker enum
//! consumed only by OTHER types' methods, never producing or
//! inspecting anything of its own beyond the standard derives.
//!
//! `jiff::tz::Dst` gets a real checked property too (see
//! `tz_dst.rs`): a real, plain (NOT `#[non_exhaustive]`) two-variant
//! enum — matches exhaustively on `self`/`result` directly, needing
//! no opaque accessor and no wildcard arm.
//!
//! `jiff::tz::OffsetArithmetic` also stays trusted, the identical
//! shape to `TimestampArithmetic`/`ZonedArithmetic`: it has no public
//! methods of its own at all, and its one field is private with no
//! getter.
//!
//! `jiff::tz::OffsetConflict` gets a real checked property too (see
//! `tz_offset_conflict.rs`), checked despite being trusted for Kani
//! specifically (real jiff source shows `resolve_with`'s
//! `AlwaysTimeZone` branch calls `TimeZone::into_ambiguous_zoned`
//! directly, the same already-confirmed-timing-out function).
//! Scoped to `AlwaysOffset`/`AlwaysTimeZone` only — `PreferOffset`/
//! `Reject` delegate to private helpers with their own separately
//! nontrivial logic, disproportionate to add here.
//!
//! `jiff::tz::OffsetRound` also stays trusted, the identical
//! builder-only shape to `TimestampRound`/`ZonedRound`: its four
//! public methods are all plain setters, and all three fields are
//! private with no getters.
//!
//! `jiff::tz::TimeZone` gets a real checked property too (see
//! `tz_time_zone.rs`), checked despite being trusted for Kani
//! specifically (the type underlying every `Repr`-dispatch CBMC wall
//! already confirmed this session): `unknown().is_unknown()` is
//! always `true`, `fixed(offset)` is never unknown, and its own
//! `to_fixed_offset()` always round-trips `offset`'s own seconds.
//! `fixed`/`to_ambiguous_timestamp`/`to_ambiguous_zoned`/
//! `into_ambiguous_zoned` are already extern-spec'd in `tz_ambiguous_
//! timestamp.rs`/`tz_ambiguous_zoned.rs`; this file adds `unknown`/
//! `is_unknown`/`to_fixed_offset`, extending `fixed`'s own ensures
//! clause with an extra conjunct there rather than redeclaring it.
//!
//! `jiff::tz::TimeZoneDatabase` gets a real checked property too
//! (see `tz_time_zone_database.rs`): `none()`/`is_definitively_
//! empty()` are the checked subset — see `amenable_kani::ext::jiff::
//! tz_time_zone_database`'s own doc comment for the real reason
//! `get()`/`bundled()`/etc. are out of scope.
//!
//! `jiff::tz::TimeZoneFollowingTransitions<'static>` stays trusted
//! here too, for a genuinely different, confirmed reason: a first
//! attempt to `extern_spec!` its `Iterator::next` directly hit TWO
//! real toolchain walls in succession — (1) a real E0716 "temporary
//! value dropped while borrowed" for `TimeZone::UTC.following(..)`
//! (fixed via `Box::leak` for a genuine `&'static TimeZone`, itself
//! blocked by a second, real "unsupported constant value" error when
//! tried via a local `static` instead), then (2) a real "the trait
//! bound `TimeZoneFollowingTransitions<'_>: creusot_std::prelude::
//! IteratorSpec` is not satisfied" error — `extern_spec!` doesn't
//! support third-party `Iterator` trait impls at all. The fallback
//! accommodation-model pattern `timestamp_series.rs` established for
//! iterator types (compute the expected value directly, never
//! calling the real `next()`) would be genuinely content-free here
//! (`next()` is unconditionally `None`, no formula to check
//! consistency of) — the case this codebase's own tautological-model
//! policy says to accept trusted for rather than build a thin model
//! for its own sake.
//!
//! `jiff::tz::TimeZoneName<'static>` also stays trusted, checked
//! directly against jiff's real source: its own constructor (`fn
//! new`) is completely private — no way to build one from outside
//! jiff at all, only reachable via real IANA tzdb data through
//! `TimeZoneNameIter`.
//!
//! `jiff::tz::TimeZoneNameIter<'static>` also stays trusted here, for
//! a DIFFERENT real reason from `TimeZoneName`: checked on Kani
//! instead (`amenable_kani::ext::jiff::tz_time_zone_name_iter`, real
//! and passing — `TimeZoneDatabase::none().available().next() ==
//! None` is genuinely cheap/dispatch-free, no `TimeZone::Repr`
//! involved) — trusted here only because `extern_spec!` doesn't
//! support third-party `Iterator` trait impls at all (the same real
//! "`IteratorSpec` is not satisfied" error already confirmed for
//! `TimeZoneFollowingTransitions`), and the fallback accommodation-
//! model pattern would be genuinely content-free for this
//! unconditional-constant claim.
//!
//! `jiff::tz::TimeZoneOffsetInfo<'static>` gets a real checked
//! property too (see `tz_time_zone_offset_info.rs`): unlike
//! `TimeZoneFollowingTransitions`, this is an ordinary STRUCT method
//! (`TimeZone::to_offset_info`), not a third-party `Iterator` impl, so
//! the `IteratorSpec` wall doesn't apply — `extern_spec!` works
//! directly. Scoped to the `TimeZone::fixed(offset)` case, reusing
//! `tz_time_zone.rs`'s own `tz_fixed_seconds_value`: `.offset()`
//! always reports `offset`'s own seconds and `.dst()` is always
//! `Dst::No`. Trusted for Kani specifically — real jiff source
//! confirms `to_offset_info` dispatches through the SAME `repr::each!`
//! macro already confirmed to time out under CBMC.
//!
//! `jiff::tz::TimeZonePrecedingTransitions<'static>` stays trusted
//! here too, the exact structural mirror of
//! `TimeZoneFollowingTransitions` confirmed by reading jiff's real
//! source directly (not assumed from the name): `preceding` is an
//! identical trivial constructor, and `next()` calls `TimeZone::
//! previous_transition`, which has the identical `UTC => None` match
//! arm `next_transition` has — the same `IteratorSpec`-not-satisfied
//! wall and content-free fallback carve-out apply unchanged (a
//! general `extern_spec!` limitation, not type-specific).
//!
//! `jiff::tz::TimeZoneTransition<'static>` stays trusted too, the
//! LAST type in this checklist: confirmed directly against jiff's
//! real source, its only real constructor (`from_jcore`) is
//! `pub(crate)` — not reachable from outside jiff at all, and neither
//! transition iterator (both trusted here, scoped to `TimeZone::UTC`)
//! ever actually yields one. The same "no reachable constructor"
//! category `TimeZoneName<'static>` already established.

use crate::CreusotWitness;
use amenable_core::{Evidence, Metadata};
use amenable_ext::{ExtProvenance, ExtStandard};

use super::bridge::{bridge_creusot_witness, impl_creusot_witness_trusted_ext};

impl_creusot_witness_trusted_ext!(
    jiff::tz::AmbiguousOffset,
    jiff::tz::Disambiguation,
    jiff::tz::OffsetArithmetic,
    jiff::tz::OffsetRound,
    jiff::tz::TimeZoneFollowingTransitions<'static>,
    jiff::tz::TimeZoneName<'static>,
    jiff::tz::TimeZoneNameIter<'static>,
    jiff::tz::TimeZonePrecedingTransitions<'static>,
    jiff::tz::TimeZoneTransition<'static>,
);
