//! `Witness<KaniVerifier>` registrations for `amenable_ext::ExtStandard<T>`
//! over jiff's registered carriers, split one file per real jiff area
//! that has earned a checked harness, plus the trusted carriers that
//! haven't (below).
//!
//! `Timestamp`/`Zoned`/`civil::DateTime` stay trusted here: they're
//! opaque, high-level composite types (calendar + zone + instant) with
//! no simple black-box property statable from jiff's public API alone —
//! see `docs/AMENABLE_EXT_PLAN.md`'s Phase 1 for why that set was chosen
//! first. Simpler value types get a real per-type assessment as they're
//! added (see `offset.rs` for the first checked example).
//!
//! `jiff::Error` also stays trusted, but for a different, real reason
//! confirmed empirically (not the default): every public accessor
//! (`Display`, `is_range`, `is_invalid_parameter`, `is_crate_feature`)
//! routes through `Error::chain()`'s iterator traversal, which times
//! out under Kani's unwinder even for a single, deterministic,
//! `mem::forget`-ed value — see `gallery::jiff_error_drop_cost`'s own
//! doc comment for the full isolation. There is no accessor-level
//! property left to check once every accessor hits the same wall.
//!
//! `jiff::RoundMode` stays trusted for a third, simpler real reason:
//! checked directly against jiff's real source, it has *no* public
//! methods at all beyond the standard derives (`Clone`/`Copy`/`Debug`/
//! `Eq`/`Hash`/`PartialEq`) — its actual rounding logic
//! (`round_by_duration`) is `pub(crate)`. There is nothing non-
//! tautological to state about a fieldless, behaviorless configuration
//! marker on any backend.
//!
//! `jiff::SignedDurationRound` also stays trusted, for the builder-
//! type shape of that same reason: its only public methods are
//! `new`/`smallest`/`mode`/`increment`, plain field setters returning a
//! new value via `..self` — no getters, and its actual rounding logic
//! (`round`, called from `SignedDuration::round`) is private. There is
//! no publicly-inspectable state to state a property about. The same
//! shape recurs across jiff's other `*Round`/`*Arithmetic`/`*Compare`/
//! `*Difference`/`*Total` builder-option types (confirmed for
//! `SpanRound` too, which has the identical setter-only surface) —
//! expect most of that family to land here for the same reason.
//!
//! `jiff::SpanArithmetic<'static>`/`jiff::SpanCompare<'static>` are the
//! first two of that family actually confirmed and landed: checked
//! directly against jiff's real source, each has exactly one public
//! method (`days_are_24_hours(self) -> Self`), no getters at all
//! (their fields are both private), and their own `checked_add`/
//! `compare`/`relative` are `pub(crate)`. Nothing publicly inspectable
//! to state a property about.
//!
//! `jiff::SpanFieldwise` gets a real checked property (see
//! `span_fieldwise.rs`): its own added value over `Span` is `Neg`/`Eq`/
//! `Hash`, and negating flips `Span`'s one shared `sign` field, which
//! every unit getter multiplies through — so negating a `SpanFieldwise`
//! negates every one of its ten unit getters simultaneously, checked
//! reusing `span.rs`'s own construction discipline.
//!
//! `jiff::SpanRelativeTo<'static>` stays trusted, even more opaque than
//! `SpanArithmetic`/`SpanCompare`: checked directly against jiff's real
//! source, its only public "method" is `days_are_24_hours() -> Self`, a
//! bare `pub const fn` static marker constructor (not even
//! `self`-consuming) — no getters, its one field (`kind`) is private,
//! and its own `to_relative` is private too. Nothing publicly
//! inspectable to state a property about.
//!
//! `jiff::SpanRound<'static>` stays trusted too, now actually
//! confirmed rather than just predicted by the `SignedDurationRound`
//! resemblance noted above: checked directly against jiff's real
//! source, its seven public methods (`new`/`smallest`/`largest`/
//! `mode`/`increment`/`relative`/`days_are_24_hours`) are all plain
//! setters, all five of its fields are private with no getters, and
//! its own `round` (the real rounding logic) is private too.
//!
//! `jiff::SpanTotal<'static>` stays trusted for the same one-method
//! shape as `SpanArithmetic`/`SpanCompare`: checked directly against
//! jiff's real source, its only public method is
//! `days_are_24_hours(self) -> Self`; `new`/`relative`/`total`/
//! `total_invariant` are all private, and both fields (`unit`/
//! `relative`) are private with no getters.
//!
//! `jiff::TimestampArithmetic` stays trusted too, even more opaque
//! than `SpanArithmetic`: checked directly against jiff's real source
//! (`src/timestamp.rs`), it has *no* public methods of its own at
//! all — `checked_add`/`saturating_add`/`checked_neg`/`is_negative`
//! are all private; only the `From` impls (construction, not
//! inspection) are public. Its one field (`duration`) is private with
//! no getter.
//!
//! `jiff::TimestampDifference` stays trusted for the same builder-only
//! reason as `SpanRound`: checked directly against jiff's real source,
//! its five public methods (`new`/`smallest`/`largest`/`mode`/
//! `increment`) are all plain setters; `rounding_may_change_span`/
//! `until_with_largest_unit` are private, and both fields
//! (`timestamp`/`round`) are private with no getters.
//!
//! `jiff::TimestampDisplayWithOffset` stays trusted for a genuinely
//! different reason from the rest of this file: unlike the builder
//! types above, it has no `pub fn`s at all, but it DOES have real
//! public behavior — a `Display` impl formatting an RFC 3339 string
//! with the given (never `Z`/`-00:00`) offset. Checking that output
//! exactly would mean reproducing jiff's entire
//! `temporal::DateTimePrinter` formatting algorithm character-for-
//! character, wildly disproportionate to this thin two-field
//! (`timestamp`/`offset`, both private, no getters) formatting
//! adapter's own complexity.
//!
//! `jiff::TimestampRound` stays trusted for the same builder-only
//! reason as `TimestampDifference`: checked directly against jiff's
//! real source, its four public methods (`new`/`smallest`/`mode`/
//! `increment`) are all plain setters, and all three fields
//! (`smallest`/`mode`/`increment`) are private with no getters.
//!
//! `jiff::TimestampSeries` also stays trusted for Kani specifically,
//! for a real reason confirmed empirically (not the default), the
//! same recursive-Arc `jiff::Error` Drop-glue wall the module doc
//! comment above already names for `jiff::Error` itself, reached
//! through a new, *unavoidable* call site this time: `Timestamp::
//! series(period)`'s own implementation drops a transient `Result<_,
//! jiff::Error>` internally (`SignedDuration::try_from(period).ok()`)
//! before `.series()` even returns, with no way for a caller to gate
//! around it the way `Offset::from_seconds`'s own harness could — see
//! `gallery::jiff_error_drop_cost`'s own doc comment for the full
//! isolation. Checked on Creusot and Verus instead (`ext_jiff::
//! timestamp_series`/`ext::jiff::timestamp_series`), neither of which
//! shares this Rust-Drop-glue mechanism.
//!
//! `jiff::ZonedArithmetic` stays trusted too, the identical shape to
//! `TimestampArithmetic`: checked directly against jiff's real source
//! (`src/zoned.rs`), it has *no* public methods of its own at all —
//! `checked_add`/`checked_neg`/`is_negative` are all private; only the
//! `From` impls (construction, not inspection) are public. Its one
//! field (`duration`) is private with no getter.
//!
//! `jiff::ZonedDifference<'static>` stays trusted for the same
//! builder-only reason as `TimestampDifference`: checked directly
//! against jiff's real source, its five public methods
//! (`new`/`smallest`/`largest`/`mode`/`increment`) are all plain
//! setters; `rounding_may_change_span`/`until_with_largest_unit` are
//! private, and both fields (`zoned`/`round`) are private with no
//! getters.
//!
//! `jiff::ZonedRound` stays trusted for the same builder-only reason
//! as `TimestampRound`: checked directly against jiff's real source,
//! its four public methods (`new`/`smallest`/`mode`/`increment`) are
//! all plain setters; `round_days` is private, and its one field
//! (`round`) is private with no getter.
//!
//! `jiff::ZonedSeries` also stays trusted for Kani specifically, for a
//! real reason confirmed empirically, genuinely different from
//! `TimestampSeries`'s: not `jiff::Error`'s recursive-Arc Drop glue,
//! but `TimeZone`'s own hand-rolled pointer-tagged `Repr` (a `usize`-
//! to-pointer `transmute` and its reverse in `Repr::tag()`), which
//! times out CBMC even for a single, fully concrete `TimeZone::UTC.
//! to_offset(Timestamp::from_second(0))` call — before any series or
//! iteration logic runs at all, confirmed by narrowing a first
//! (wrong) hypothesis blaming `ZonedSeries::next()`'s own
//! `checked_mul`/`checked_add` calls down to this earlier, more
//! fundamental construction-time wall. See `gallery::
//! jiff_error_drop_cost`'s own doc comment for the full isolation.
//! Checked on Creusot and Verus instead (`ext_jiff::zoned_series`/
//! `ext::jiff::zoned_series`), neither of which shares CBMC's
//! pointer/memory model — each scoped honestly to `TimeZone::UTC`,
//! where `ZonedSeries::next()`'s real DST-repeat retry loop is
//! structurally unreachable (see those modules' own doc comments).
//!
//! `jiff::ZonedWith` stays trusted on all three backends — the first
//! type this session where all three real assessments independently
//! landed on trusted, each for its own genuinely different reason,
//! not a shared shortcut. On Kani: `ZonedWith::build()`'s whole
//! purpose is producing a real `Zoned` (confirmed by reading its
//! implementation: it calls `OffsetConflict::resolve`, which consults
//! the receiver's `TimeZone`), so it necessarily hits the same
//! `TimeZone::Repr` pointer-tagging wall documented above for
//! `ZonedSeries`. On Creusot: checked directly (not assumed) that a
//! real `extern_spec!` here would be UNSOUND if stated narrowly —
//! `ZonedWith`'s type alone can't distinguish "fresh from `.with()`,
//! no overrides" from "modified by `.date(..)`/etc." (both are the
//! same type, private fields), so the one real, simple law jiff
//! documents ("no fields set" ⟹ "returns the original unchanged")
//! can't be stated as an unconditional `#[ensures(..)]` on `build()`
//! without also extern-speccing every setter against a tracked ghost
//! override-state — plus `civil::DateTimeWith`'s own calendar-field
//! setters underneath — disproportionate to one type, and it would
//! mean opening up `Zoned`/`civil::DateTime`'s deliberately-opaque
//! surface (see this module's own opening paragraph). On Verus: the
//! same law, restated as a self-contained model, reduces to a bare
//! identity function (`build(original, None) == original`) with no
//! distinguishing computation at all — genuinely as tautological as
//! it gets, the case this codebase's own tautological-model policy
//! says to accept trusted rather than build a thin model for its own
//! sake, arrived at here only after actually trying the richer
//! extern_spec/model and finding the real obstacle above, not assumed
//! upfront.
//!
//! `jiff::civil::Date` gets a real checked property too (see
//! `civil_date.rs`), the first `civil::*` type assessed: unlike
//! `civil::DateTime`/`Zoned` (opaque composites needing `TimeZone`'s
//! `Repr`), `Date` is a pure calendar value with no time zone
//! involved at all — checked directly by reading jiff's real source,
//! not assumed safe just because it's a "calendar type." A real
//! `jiff::Error` Drop-glue wall on `Date::new`'s `Err` arm (the same
//! class this module's own opening paragraphs document for `Offset`)
//! is fixed the identical way: an independent, Drop-free bounds check
//! gates the call.
//!
//! `jiff::civil::DateArithmetic` stays trusted, the identical shape
//! to `TimestampArithmetic`/`ZonedArithmetic`: checked directly
//! against jiff's real source (`src/civil/date.rs`), it has *no*
//! public methods of its own at all — `checked_add`/`checked_neg`/
//! `is_negative` are all private; only the `From` impls (construction,
//! not inspection) are public. Its one field (`duration`) is private
//! with no getter.
//!
//! `jiff::civil::DateDifference` stays trusted for the same
//! builder-only reason as `TimestampDifference`/`ZonedDifference`:
//! checked directly against jiff's real source, its five public
//! methods (`new`/`smallest`/`largest`/`mode`/`increment`) are all
//! plain setters; `rounding_may_change_span`/`since_with_largest_unit`
//! are private, and both fields (`date`/`round`) are private with no
//! getters.
//!
//! `jiff::civil::DateSeries` also stays trusted for Kani specifically,
//! for the SAME real reason as `TimestampSeries` — not `ZonedSeries`'s
//! `TimeZone::Repr` wall (`Date` has no time zone at all): confirmed
//! empirically (not assumed from either resemblance) that a single
//! `.series(period).next()` call times out even for a tiny assumed
//! range, and even with the result immediately `mem::forget`-ed, while
//! bare `Date::new` alone passes instantly — isolating the cost to
//! `DateSeries::next`'s own `checked_mul`/`checked_add` calls, both
//! `Result<_, jiff::Error>`-returning and `.ok()`-converted on every
//! step, the identical recursive-Arc Drop-glue wall this module's own
//! opening paragraphs document for `Offset`/`Error`/`TimestampSeries`.
//! See `gallery::jiff_error_drop_cost`'s own doc comment for the full
//! isolation. Checked on Creusot and Verus instead (`ext_jiff::
//! date_series`/`ext::jiff::date_series`), neither of which shares
//! this Rust-Drop-glue mechanism.
//!
//! `jiff::civil::DateTimeArithmetic` stays trusted, the identical
//! shape to `DateArithmetic`/`TimestampArithmetic`/`ZonedArithmetic`:
//! checked directly against jiff's real source
//! (`src/civil/datetime.rs`), it has *no* public methods of its own
//! at all — `checked_add`/`checked_neg`/`is_negative` are all
//! private; only the `From` impls (construction, not inspection) are
//! public. Its one field (`duration`) is private with no getter.
//!
//! `jiff::civil::DateTimeDifference` stays trusted for the same
//! builder-only reason as `DateDifference`/`TimestampDifference`/
//! `ZonedDifference`: checked directly against jiff's real source,
//! its five public methods (`new`/`smallest`/`largest`/`mode`/
//! `increment`) are all plain setters; `rounding_may_change_span` is
//! private, and both fields (`datetime`/`round`) are private with no
//! getters.
//!
//! `jiff::civil::DateTimeRound` stays trusted for the same
//! builder-only reason as `TimestampRound`/`ZonedRound`: checked
//! directly against jiff's real source, its four public methods
//! (`new`/`smallest`/`mode`/`increment`) are all plain setters; the
//! actual rounding logic (`round`) is private, and all three fields
//! (`smallest`/`mode`/`increment`) are private with no getters.
//!
//! `jiff::civil::DateTimeSeries` also stays trusted for Kani
//! specifically, for the identical real reason as `DateSeries` —
//! `civil::DateTime`, like `civil::Date`, is a pure calendar+clock
//! value with no `TimeZone` at all, so `ZonedSeries`'s `Repr` wall
//! cannot apply here either; `DateTimeSeries::next()` has the exact
//! same `checked_mul`/`checked_add` shape as `DateSeries::next()`,
//! confirmed via `gallery::jiff_error_drop_cost`'s own doc comment to
//! hit the identical `jiff::Error` Drop-glue wall. Checked on Creusot
//! and Verus instead (`ext_jiff::date_time_series`/`ext::jiff::
//! date_time_series`), neither of which shares this Rust-Drop-glue
//! mechanism.
//!
//! `jiff::civil::DateTimeWith` stays trusted on all three backends,
//! for the identical real reason as `ZonedWith` — checked directly,
//! not assumed from resemblance: its type (`{date_with: DateWith,
//! time_with: TimeWith}`) can't distinguish "fresh from `.with()`"
//! from "modified by a setter" any more than `ZonedWith`'s can, so
//! the one real law jiff documents ("no fields set" ⟹ "`build()`
//! returns the original unchanged") can't be stated soundly as an
//! unconditional `#[ensures(..)]`, and an accommodation model of just
//! that case reduces to a bare identity function. See `ZonedWith`'s
//! own doc comment above for the full reasoning.
//!
//! `jiff::civil::DateWith` stays trusted too, the third confirmed
//! instance of this exact pattern: its fields (`original: Date`,
//! `year: Option<DateWithYear>`, `month: Option<i8>`, `day:
//! Option<DateWithDay>`) are private either way, so the extern_spec/
//! model boundary still can't distinguish "no override" from "some
//! override" from the outside — the `Option` wrapping doesn't change
//! that, since private fields are invisible to a real `extern_spec!`
//! regardless of their own internal shape.
//!
//! `jiff::civil::Era` gets a real checked property too (see
//! `civil_era.rs`): though `Era` is a plain fieldless 2-variant enum
//! with no constructor of its own, its only real production path
//! (`Date::era_year() -> (i16, Era)`) has a precisely documented,
//! genuinely checkable law (`year >= 1` ⟺ `(year, Era::CE)`; `year <=
//! 0` ⟺ `(-year + 1, Era::BCE)`) — checked directly, not assumed
//! trusted just because the type itself is fieldless (that would be
//! the wrong lesson from `RoundMode`'s own trusted reasoning, which
//! rested on having literally no public methods, not on having no
//! fields).
//!
//! `jiff::civil::ISOWeekDate` gets a real checked property too (see
//! `civil_iso_week_date.rs`): a real value type with real getters
//! (`year`/`week`/`weekday`), the same shape as `civil::Date` —
//! checked directly by reading jiff's real source. `ISOWeekDate::
//! new`'s validity is more complex than `Date::new`'s (week `53` is
//! only valid for years containing a "leap week"), so this scopes to
//! week `1..=52`, jiff's own documented always-valid sub-range,
//! matching `civil_date.rs`'s own day-range simplification.
//!
//! `jiff::civil::Time` also gets a real checked property (see
//! `civil_time.rs`) — but unlike `Date`/`ISOWeekDate`, its validity
//! is a fully rectangular, unconditional domain with no
//! interdependency between fields at all (checked directly:
//! `Time::MIN`/`MAX` exactly match the same independent per-field
//! bounds, no derived-elsewhere restriction), so this witness states
//! jiff's FULL documented validity condition, not a narrowed
//! sufficient sub-range.

mod civil_date;
mod civil_era;
mod civil_iso_week_date;
mod civil_time;
mod offset;
mod signed_duration;
mod span;
mod span_fieldwise;
mod unit;

use crate::ext::macros::impl_kani_witness_trusted_ext;

impl_kani_witness_trusted_ext!(
    jiff::Timestamp,
    jiff::Zoned,
    jiff::civil::DateTime,
    jiff::Error,
    jiff::RoundMode,
    jiff::SignedDurationRound,
    jiff::SpanArithmetic<'static>,
    jiff::SpanCompare<'static>,
    jiff::SpanRelativeTo<'static>,
    jiff::SpanRound<'static>,
    jiff::SpanTotal<'static>,
    jiff::TimestampArithmetic,
    jiff::TimestampDifference,
    jiff::TimestampDisplayWithOffset,
    jiff::TimestampRound,
    jiff::TimestampSeries,
    jiff::ZonedArithmetic,
    jiff::ZonedDifference<'static>,
    jiff::ZonedRound,
    jiff::ZonedSeries,
    jiff::ZonedWith,
    jiff::civil::DateArithmetic,
    jiff::civil::DateDifference,
    jiff::civil::DateSeries,
    jiff::civil::DateTimeArithmetic,
    jiff::civil::DateTimeDifference,
    jiff::civil::DateTimeRound,
    jiff::civil::DateTimeSeries,
    jiff::civil::DateTimeWith,
    jiff::civil::DateWith
);
