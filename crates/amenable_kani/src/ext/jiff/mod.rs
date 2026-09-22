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
//!
//! `jiff::civil::TimeArithmetic` stays trusted, the identical shape
//! to `DateArithmetic`/`DateTimeArithmetic`/`TimestampArithmetic`/
//! `ZonedArithmetic`: checked directly against jiff's real source
//! (`src/civil/time.rs`), it has *no* public methods of its own at
//! all — `wrapping_add`/`wrapping_sub`/`checked_add`/`checked_neg`/
//! `is_negative` are all private; only the `From` impls (construction,
//! not inspection) are public. Its one field (`duration`) is private
//! with no getter.
//!
//! `jiff::civil::TimeDifference` stays trusted for the same
//! builder-only reason as `DateDifference`/`DateTimeDifference`/
//! `TimestampDifference`/`ZonedDifference`: checked directly against
//! jiff's real source, its five public methods (`new`/`smallest`/
//! `largest`/`mode`/`increment`) are all plain setters;
//! `rounding_may_change_span`/`until_with_largest_unit` are private,
//! and both fields (`time`/`round`) are private with no getters.
//!
//! `jiff::civil::TimeRound` stays trusted for the same builder-only
//! reason as `DateTimeRound`/`TimestampRound`/`ZonedRound`: checked
//! directly against jiff's real source, its four public methods
//! (`new`/`smallest`/`mode`/`increment`) are all plain setters; the
//! actual rounding logic (`round`) is private, and all three fields
//! (`smallest`/`mode`/`increment`) are private with no getters.
//!
//! `jiff::civil::TimeSeries` also stays trusted for Kani specifically,
//! for the identical real reason as `DateSeries`/`DateTimeSeries` —
//! `civil::Time`, like `civil::Date`/`civil::DateTime`, is a pure
//! clock value with no `TimeZone` at all, so `ZonedSeries`'s `Repr`
//! wall cannot apply here either; `TimeSeries::next()` has the exact
//! same `checked_mul`/`checked_add` shape as `DateSeries::next()`/
//! `DateTimeSeries::next()`, confirmed via `gallery::
//! jiff_error_drop_cost`'s own doc comment to hit the identical
//! `jiff::Error` Drop-glue wall. Checked on Creusot and Verus instead
//! (`ext_jiff::civil_time_series`/`ext::jiff::civil_time_series`),
//! neither of which shares this Rust-Drop-glue mechanism.
//!
//! `jiff::civil::TimeWith` stays trusted too, the fourth confirmed
//! instance of the `ZonedWith`/`DateTimeWith`/`DateWith` pattern:
//! its fields (`original: Time`, `hour: Option<i8>`, `minute:
//! Option<i8>`, `second: Option<i8>`, `millisecond: Option<i16>`,
//! `microsecond: Option<i16>`, `nanosecond: Option<i16>`,
//! `subsec_nanosecond: Option<i32>`) are private either way, so the
//! extern_spec/model boundary still can't distinguish "no override"
//! from "some override" from the outside — the identical soundness
//! obstacle, checked directly against jiff's real source rather than
//! assumed from the earlier three.
//!
//! `jiff::civil::Weekday` gets a real checked property too (see
//! `civil_weekday.rs`) — checked directly, not assumed trusted just
//! because it's a fieldless enum (the same lesson `Era` already
//! established): a real round-trip law over
//! `from_monday_one_offset`/`to_monday_one_offset`, the identical
//! claim already extern-spec'd once for `ISOWeekDate`'s own witness.
//!
//! `jiff::civil::WeekdaysForward` gets a real checked property too
//! (see `civil_weekdays_forward.rs`) — checked directly on Kani,
//! genuinely different from the `*Series` family: its `Iterator::
//! next()` has no `Result`/`jiff::Error` anywhere in its call chain
//! at all (confirmed by reading jiff's real source — it delegates to
//! an infallible `jcore` cycle plus a plain `const fn` match), so
//! the recursive-Arc Drop-glue wall that makes `TimestampSeries`/
//! `DateSeries`/`DateTimeSeries`/`TimeSeries` Kani-uncheckable never
//! applies here — not assumed trusted-and-why by resemblance to that
//! family just because it's also an "iterator over a jiff type."
//!
//! `jiff::civil::WeekdaysReverse` gets a real checked property too
//! (see `civil_weekdays_reverse.rs`) — re-verified, not assumed from
//! `WeekdaysForward`'s own confirmed shape: checked directly against
//! jiff's real source, `WeekdaysReverse::next()` has the identical
//! infallible `jcore`-delegation shape, no `Result`/`jiff::Error`
//! anywhere.
//!
//! `jiff::fmt::StdFmtWrite<String>` gets a real checked property too
//! (see `fmt_std_fmt_write.rs`): a real newtype wrapper adapting any
//! `core::fmt::Write` value to jiff's own public `jiff::fmt::Write`
//! trait — checked directly against jiff's real source. Scoped to
//! `W = String`, whose `core::fmt::Write` implementation never fails,
//! so the `Err` arm (the only path that would construct a real
//! `jiff::Error`) is never reached, avoiding the recursive-Arc
//! Drop-glue wall this module's own opening paragraphs document
//! rather than needing to work around it.
//!
//! `jiff::fmt::StdIoWrite<Vec<u8>>` gets a real checked property too
//! (see `fmt_std_io_write.rs`), the identical shape to `StdFmtWrite<
//! String>`'s own: a real newtype wrapper adapting any `std::io::
//! Write` value to jiff's `jiff::fmt::Write` trait, scoped to
//! `W = Vec<u8>`, whose `write_all` is documented to never fail (it
//! just extends the vector), so the `Err` arm is never reached.
//!
//! `jiff::fmt::friendly::Designator` stays trusted, the first
//! `jiff::fmt::*` type to land here rather than get a checked
//! property: checked directly against jiff's real source
//! (`src/fmt/friendly/printer.rs`), it's a fieldless, `#[non_exhaustive]`
//! config-marker enum (`Verbose`/`Short`/`Compact`/`HumanTime`)
//! deriving only `Clone`/`Copy`/`Debug` — no `impl Designator` block
//! at all, only consumed by `SpanPrinter::designator(self, Designator)
//! -> SpanPrinter`. The identical "no public methods beyond the
//! standard derives" shape `RoundMode`'s own trusted reasoning rests
//! on, not the "fieldless enum" shortcut `Era`/`Weekday` already
//! showed is the wrong lesson to draw.
//!
//! `jiff::fmt::friendly::Direction` stays trusted, the identical
//! shape to `Designator`: checked directly against jiff's real
//! source, it's a fieldless, `#[non_exhaustive]` config-marker enum
//! (`Auto`/`Sign`/`ForceSign`/`Suffix`) deriving only `Clone`/`Copy`/
//! `Debug`; its one `impl Direction` method (`sign`) is private.
//!
//! `jiff::fmt::friendly::Spacing` stays trusted, the identical shape
//! to `Designator`/`Direction`: checked directly against jiff's real
//! source, it's a fieldless, `#[non_exhaustive]` config-marker enum
//! (`None`/`BetweenUnits`/`BetweenUnitsAndDesignators`) deriving only
//! `Clone`/`Copy`/`Debug`; both `impl Spacing` methods
//! (`between_units`/`between_units_and_designators`) are private.
//!
//! `jiff::fmt::friendly::SpanParser` stays trusted, a genuinely
//! different reason from every other type in this file: unlike the
//! builder/config-marker shapes above, it has real, substantial
//! documented behavior (`parse_span`/`parse_duration`/
//! `parse_unsigned_duration`, jiff's real friendly-duration parsing
//! grammar) — but checked directly against jiff's real source, the
//! struct itself carries zero configurable state (`_private: ()`,
//! and jiff's own doc comment says outright "there are no available
//! configuration options for this parser"). Checking its exact
//! parsing behavior would mean reproducing jiff's entire grammar,
//! the same disproportionate-reproduction reason `jiff::
//! TimestampDisplayWithOffset` already established for formatting
//! output.
//!
//! `jiff::fmt::friendly::SpanPrinter` stays trusted too, combining
//! BOTH real reasons already established elsewhere in this file:
//! checked directly against jiff's real source, its nine public
//! methods (`designator`/`spacing`/`direction`/`fractional`/
//! `comma_after_designator`/`hours_minutes_seconds`/`padding`/
//! `precision`/`zero_unit`) are all plain setters over its nine
//! private fields — no getters at all, the same builder-only shape
//! `SpanRound`/`TimestampRound`/etc. share — AND its real formatted-
//! output methods (`span_to_string`/`print_span`/etc.) have the same
//! disproportionate-reproduction shape `SpanParser`/
//! `TimestampDisplayWithOffset` already established.
//!
//! `jiff::fmt::rfc2822::DateTimeParser` stays trusted, the identical
//! combined shape to `SpanParser`/`SpanPrinter`: checked directly
//! against jiff's real source (`src/fmt/rfc2822.rs`), its one public
//! method (`relaxed_weekday(self, bool) -> Self`) is a plain setter
//! over its one private field, with no getter — and its real parsing
//! methods (`parse_zoned`/`parse_timestamp`) have the same
//! disproportionate-reproduction shape `SpanParser` already
//! established, reproducing jiff's real RFC 2822 grammar this time
//! rather than the friendly-duration one.
//!
//! `jiff::fmt::rfc2822::DateTimePrinter` stays trusted, the identical
//! shape to `SpanParser`: checked directly against jiff's real
//! source, its `_private: ()` field carries zero configurable state
//! (jiff's own inline comment says "the RFC 2822 printer has no
//! configuration at present") — and its real formatting methods
//! (`zoned_to_string`/`timestamp_to_string`/
//! `timestamp_to_rfc9110_string`/etc.) have the same disproportionate-
//! reproduction shape.
//!
//! `jiff::fmt::strtime::BrokenDownTime` gets a real checked property
//! too (see `fmt_strtime_broken_down_time.rs`), the biggest single
//! type assessed since `Span` itself: 12 numeric fields, each with a
//! real, independently-validated `Result`-returning setter/getter
//! pair, checked directly against jiff's real source and `jiff-core`'s
//! own bounds table (not assumed) — no cross-field interdependency at
//! all, confirmed by reading `set_day`'s own doc comment: "setting a
//! day to a value that is legal in any context is always valid, even
//! if it isn't valid for the year, month." Deliberately scoped to
//! these 12 numeric round trips, not the 5 reference-type fields
//! (`offset`/`weekday`/`meridiem`/`timestamp`/`iana_time_zone`),
//! whose setters are plain unconditional field assignments with no
//! validation logic at all.
//!
//! `jiff::fmt::strtime::Config<DefaultCustom>` stays trusted, checked
//! directly against jiff's real source: a pure builder — both fields
//! (`custom`/`lenient`) are private with no getters at all, and its
//! two public methods (`custom<U: Custom>`/`lenient(bool)`) are plain
//! setters, the same shape as `SpanRound`/`TimestampRound`/etc.
//!
//! `jiff::fmt::strtime::DefaultCustom` stays trusted, even more
//! opaque than `RoundMode`: checked directly against jiff's real
//! source, it's `DefaultCustom(())` — a single private zero-sized
//! field — with one constructor (`new()`) and an empty `impl Custom
//! for DefaultCustom {}` using only the trait's own inherited
//! defaults, no overrides at all. Nothing publicly inspectable to
//! state a property about.
//!
//! `jiff::fmt::strtime::Display<'static>` stays trusted, the same
//! reason as `TimestampDisplayWithOffset`: checked directly against
//! jiff's real source, both fields (`fmt`/`tm`) are `pub(crate)`
//! (invisible outside jiff), with only `impl core::fmt::Display`/
//! `Debug` — checking its exact formatted output would mean
//! reproducing jiff's whole `strtime` formatting algorithm, the same
//! disproportionate-reproduction reason already established.
//!
//! `jiff::fmt::strtime::Extension` stays trusted, checked directly
//! against jiff's real source: three private fields, zero `pub`
//! methods at all (`parse_flag`/`parse_width`/`parse_colons` are all
//! private), only `#[derive(Clone, Debug)]` — jiff's own doc comment
//! confirms this is deliberate: "if you have use cases for
//! introspecting this type, please open an issue."
//!
//! `jiff::fmt::strtime::PosixCustom` stays trusted too: checked
//! directly against jiff's real source, `PosixCustom(())` mirrors
//! `DefaultCustom`'s own zero-field shape (its own value type has no
//! methods beyond `new()`) — but unlike `DefaultCustom`, its `impl
//! Custom for PosixCustom` DOES override `format_datetime`/
//! `format_date`/`format_time`/`format_12hour_time`, each just
//! delegating to `BrokenDownTime::format_with_config` with a fixed
//! POSIX format string. Checking that exactly would mean reproducing
//! the same `strtime` formatting engine `SpanParser`/`Display`
//! already established as disproportionate, one level removed.
//!
//! `jiff::fmt::strtime::Meridiem` gets a real checked property too
//! (see `fmt_strtime_meridiem.rs`), the "small enums aren't
//! automatically trusted" lesson `civil::Era`/`civil::Weekday`
//! already established recurring here: checked directly against
//! jiff's real source, it has one real public conversion, `impl
//! From<civil::Time> for Meridiem`, a documented threshold (`AM` for
//! `hour < 12`, `PM` otherwise).
//!
//! `jiff::fmt::friendly::FractionalUnit` gets a real checked property
//! too (see `fmt_friendly_fractional_unit.rs`), genuinely different
//! from `Designator`/`Direction`: checked directly against jiff's
//! real source, it has no `impl FractionalUnit` block at all, but it
//! DOES have one real public conversion, `impl From<FractionalUnit>
//! for Unit`, a documented per-variant mapping — checked by exhaustive
//! enumeration of all 5 variants.
//!
//! `jiff::fmt::temporal::DateTimeParser` stays trusted, the identical
//! combined shape to `SpanParser`/`rfc2822::DateTimeParser`: checked
//! directly against jiff's real source, its two public methods
//! (`offset_conflict`/`disambiguation`) are plain setters over three
//! private fields, no getters — and its real parsing methods
//! (`parse_zoned`/`parse_timestamp`/`parse_datetime`/etc.) have the
//! same disproportionate-reproduction shape, reproducing jiff's real
//! ISO 8601/RFC 9557 temporal grammar this time.
//!
//! `jiff::fmt::temporal::DateTimePrinter` stays trusted, the identical
//! combined shape to `SpanPrinter`/`rfc2822::DateTimePrinter`: checked
//! directly against jiff's real source, its three public methods
//! (`lowercase`/`separator`/`precision`) are plain setters over one
//! private field, no getters — and its real formatting methods
//! (`zoned_to_string`/`timestamp_to_string`/`print_zoned`/etc.) have
//! the same disproportionate-reproduction shape.
//!
//! `jiff::fmt::temporal::Pieces<'static>` gets a real checked
//! property too (see `fmt_temporal_pieces.rs`), genuinely different
//! from every other `fmt::*` type assessed so far: a real,
//! substantial data-carrying type (four private fields), checked
//! directly against jiff's real source — `with_date`/`with_time` are
//! real, unconditional setters (`Pieces { date, ..self }`/`Pieces {
//! time: Some(time), ..self }`) round-tripping through their matching
//! `date()`/`time()` getters. Scoped to those two fields
//! deliberately, not `offset`/`time_zone_annotation` — their own
//! types are separate, not-yet-assessed checklist entries.
//!
//! `jiff::fmt::temporal::PiecesNumericOffset` gets a real checked
//! property too (see `fmt_temporal_pieces_numeric_offset.rs`): a real
//! data-carrying type (`offset: Offset`, `is_negative: bool`), not a
//! parser/printer — `From<Offset>` round-trips the wrapped offset's
//! seconds and sets `is_negative` to whether it's negative, and
//! `with_negative_zero` preserves the wrapped offset while forcing
//! `is_negative` to `true`.
//!
//! `jiff::fmt::temporal::PiecesOffset` gets a real checked property
//! too (see `fmt_temporal_pieces_offset.rs`): a real, `#[non_
//! exhaustive]` two-variant enum (`Zulu`, `Numeric(
//! PiecesNumericOffset)`) — `Zulu.to_numeric_offset()` is always
//! `Offset::UTC`, and `From<Offset>` always builds `Numeric`,
//! round-tripping through `to_numeric_offset()`.
//!
//! `jiff::fmt::temporal::SpanParser` stays trusted, checked directly
//! against jiff's real source: a DIFFERENT type from `fmt::friendly::
//! SpanParser` despite the same name (verified independently, not
//! assumed) — one private field (`p: parser::SpanParser`, the
//! internal engine), zero setters at all this time, and its real
//! parsing methods (`parse_span`/`parse_duration`/
//! `parse_unsigned_duration`) have the same disproportionate-
//! reproduction shape, reproducing jiff's real ISO 8601 duration
//! grammar.
//!
//! `jiff::fmt::temporal::SpanPrinter` stays trusted, the identical
//! combined shape to every other `*Printer` in this checklist:
//! checked directly against jiff's real source, its one public
//! method (`lowercase`) is a plain setter over a private field, no
//! getter — and its real formatting methods (`span_to_string`/
//! `duration_to_string`/`unsigned_duration_to_string`/`print_span`/
//! etc.) have the same disproportionate-reproduction shape.
//!
//! `jiff::fmt::temporal::TimeZoneAnnotation<'static>` gets a real
//! checked property too (see `fmt_temporal_time_zone_annotation.rs`):
//! a real data-carrying type (`kind`, `critical`, both `pub(crate)`
//! to jiff) — `From<&str>` always builds `Named` with
//! `is_critical() == false`, `From<Offset>` always builds `Offset`
//! (carrying the wrapped offset's own seconds) with
//! `is_critical() == false`.
//!
//! `jiff::fmt::temporal::TimeZoneAnnotationKind<'static>` gets a real
//! checked property too (see
//! `fmt_temporal_time_zone_annotation_kind.rs`): unlike
//! `TimeZoneAnnotation<'static>`, this `#[non_exhaustive]` enum's own
//! variants are public (no private-field indirection), so
//! `From<&str>`'s exact name content is checked directly by
//! matching — `From<Offset>` always builds `Offset` carrying the
//! wrapped offset's own seconds.
//!
//! `jiff::fmt::temporal::TimeZoneAnnotationName<'static>` gets a real
//! checked property too (see
//! `fmt_temporal_time_zone_annotation_name.rs`): wraps one private
//! field, with a real, checkable round trip —
//! `TimeZoneAnnotationName::from(s).as_str() == s` for every `&str`.
//!
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

mod civil_date;
mod civil_era;
mod civil_iso_week_date;
mod civil_time;
mod civil_weekday;
mod civil_weekdays_forward;
mod civil_weekdays_reverse;
mod fmt_friendly_fractional_unit;
mod fmt_std_fmt_write;
mod fmt_std_io_write;
mod fmt_strtime_broken_down_time;
mod fmt_strtime_meridiem;
mod fmt_temporal_pieces;
mod fmt_temporal_pieces_numeric_offset;
mod fmt_temporal_pieces_offset;
mod fmt_temporal_time_zone_annotation;
mod fmt_temporal_time_zone_annotation_kind;
mod fmt_temporal_time_zone_annotation_name;
mod offset;
mod signed_duration;
mod span;
mod span_fieldwise;
mod tz_dst;
mod tz_time_zone_database;
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
    jiff::civil::DateWith,
    jiff::civil::TimeArithmetic,
    jiff::civil::TimeDifference,
    jiff::civil::TimeRound,
    jiff::civil::TimeSeries,
    jiff::civil::TimeWith,
    jiff::fmt::friendly::Designator,
    jiff::fmt::friendly::Direction,
    jiff::fmt::friendly::Spacing,
    jiff::fmt::friendly::SpanParser,
    jiff::fmt::friendly::SpanPrinter,
    jiff::fmt::rfc2822::DateTimeParser,
    jiff::fmt::rfc2822::DateTimePrinter,
    jiff::fmt::strtime::Config<jiff::fmt::strtime::DefaultCustom>,
    jiff::fmt::strtime::DefaultCustom,
    jiff::fmt::strtime::Display<'static>,
    jiff::fmt::strtime::Extension,
    jiff::fmt::strtime::PosixCustom,
    jiff::fmt::temporal::DateTimeParser,
    jiff::fmt::temporal::DateTimePrinter,
    jiff::fmt::temporal::SpanParser,
    jiff::fmt::temporal::SpanPrinter,
    jiff::tz::AmbiguousOffset,
    jiff::tz::AmbiguousTimestamp,
    jiff::tz::AmbiguousZoned,
    jiff::tz::Disambiguation,
    jiff::tz::OffsetArithmetic,
    jiff::tz::OffsetConflict,
    jiff::tz::OffsetRound,
    jiff::tz::TimeZone
);
