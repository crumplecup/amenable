# A Real jiff-Backed `amenable_time` Backend

## Status

🟡 In progress. Phase 1 done (2026-09-23, commit `3930418e`). Phase 2
done (2026-09-23, commit `2d29ec9f`). This doc is the full-surface map
and checklist; execution proceeds phase by phase per
`docs/PLANNING_INDEX.md`'s "commit between plan steps" convention — no
check-in needed between phases once a phase's own real work is verified
and committed.

**Phase 2 real findings, worth carrying into later phases:** jiff has no
first-class "offset date-time" type of its own — `JiffOffsetDateTime`
is a small new composite (`{ local: jiff::civil::DateTime, offset:
jiff::tz::Offset }`), and later phases building composite carriers
should expect the same. `#[derive(amenable_derive::Evidence)]`'s own
`basis = "Self"` mode needs `Self: Default` UNLESS a `basis_ctor` is
given explicitly (confirmed via the real macro expansion) — `jiff::tz::
Offset` has no `Default` at all, so any carrier wrapping it (directly or
in a composite) needs an explicit `basis_ctor` supplying a real jiff
constant (`Offset::UTC`), not a bare `#[evidence(basis = "Self")]`.
`UtcOffsetDescriptor`'s RFC 9557 unknown-local-offset case has no
`jiff::tz::Offset` representation at all — a real `Unsupported`, not a
silent default. The realize direction's own calendar-date-only scoping
(mirroring the canary's precedent) means Phase 3's own ordinal/week-date
carriers should come back and OPTIONALLY widen `local_date_time_
descriptor_to_jiff_civil_datetime` once they land, rather than
duplicating that conversion logic separately.

**Phase 1 real findings, worth carrying into later phases:** `jiff::Span`'s
setters (`years()`/`months()`/etc) *panic* once a component exceeds
jiff's own representable range (years beyond ±19,998) — any phase
converting a `u32`-or-wider descriptor field into a `Span` component
must use jiff's fallible `try_*` setters throughout, never the panicking
ones. `jiff::Span` derives only `Clone`/`Copy`/`Default` (manual, non-
derived `Debug`, deliberately no `PartialEq`/`Eq`/`Hash`) — every native
carrier wrapping a jiff type needs its own derive list checked against
the real wrapped type's real derives, not assumed. `jiff::Span` has no
fractional representation for anything coarser than seconds — a
descriptor fraction on a coarser component is a real `Unsupported`
error, not silent truncation. Confirmed by compiling `JiffTimeBackend`
with NO trust block first: `DurationSemanticBundle`'s own composition
(via the shared `BackendConversionSemanticBundle` sub-claim every
`*SemanticBundle` carries) reaches the identical 23 machine-checked
contracts `amenable_std::std_time_backend`'s own `canary_trusts!`
already covers — meaning **every** later phase needs the identical
`jiff_backend_trusts!` block already landed in Phase 1's own
`backend.rs`, not a per-phase subset.

## Why this exists

`amenable_time` defines the temporal contract interface: `Temporal*Props`
associated-type families plus `Temporal*NativeBridge`/`*Factory`/
`Parser`/`Formatter` `Exchange<Input, Output, V>` bundles that a concrete
backend implements. Today the only backend is
`amenable_std::StdTimeBackend`, a deliberately narrow **canary**
implementing 6 of the surface's Exchange edges against `std::time`, which
has no calendar, zone, or ISO 8601 parser of its own.

`amenable_ext`'s own plan (`AMENABLE_EXT_PLAN.md`, Phase 3, "the payoff")
and `amenable_time`'s own plan (`AMENABLE_TIME_PLAN.md`, Phase 7's open
follow-on) both name the obvious next step: a real `jiff` backend,
`amenable_ext::jiff::backend`, now buildable because `amenable_ext` just
finished a genuine per-type Kani/Creusot/Verus assessment of jiff's
entire 90-type public surface (`amenable-ext-jiff` coverage checklist,
90/90 complete).

This doc exists because the job is bigger than a narrow slice: the whole
`amenable_time` trait surface is ~129 distinct Exchange edges. The goal
is a **complete map** — every edge classified as real-and-buildable with
jiff, real-but-needs-custom-logic, or genuinely out of scope — worked
through phase by phase with the same "slow but sure, one real piece at a
time, one commit per piece" discipline the 90-type jiff coverage
checklist proved out, on a different axis: trait *implementation*
completeness against jiff, not per-type *witness* coverage.

## Full surface inventory

| Family | File | Traits | Exchange edges |
|---|---|---|---|
| Props (assoc. types only) | `native_props.rs` | 16 traits, 18 assoc. types | 0 |
| Native bridges | `native_bridge.rs` | 13 realize/reflect pairs | 28 |
| Native factories | `native_factory.rs` | 4 | 9 |
| `TemporalIntervalFactory` | `interval.rs` | 1 | 4 |
| `TemporalZoneFactory` | `zone.rs` | 1 | 5 |
| `TemporalConversionFactory` | `conversion.rs` | 1 | 4 |
| `TemporalParser` | `parse.rs` | 1 | 24 |
| `TemporalFormatter` | `format.rs` | 1 | 36 |
| `TemporalCalConnectFactory` | `calconnect.rs` | 1 | 19 |
| `TemporalReporter` | `report.rs` | 1 | 0 (7 plain capability queries) |

**Total: ~129 distinct `Exchange<Input,Output,V>` edges.** The std::time
canary implements 6 of them.

## Feasibility classification

### Props associated types → proposed jiff native carrier

| Props family | Assoc. type | Jiff carrier | Status |
|---|---|---|---|
| `TemporalDurationProps` | `Duration` | `jiff::Span` (checked, all 3 backends) | **Real** |
| `TemporalInstantProps` | `Instant` | `jiff::Timestamp` (checked) | **Real** |
| | `UtcOffset` | `jiff::tz::Offset` (checked) | **Real** |
| | `OffsetDateTime` | new composite: `{ local: jiff::civil::DateTime, offset: jiff::tz::Offset }` | **Real**, small carrier struct needed |
| `TemporalZoneProps` | `NamedTimeZone` | `jiff::tz::TimeZone` (checked Creusot+Verus, trusted-Kani) | **Real** |
| | `ZonedDateTime` | `jiff::Zoned` (trusted, opaque) | **Real** |
| `TemporalCivilProps` | `CalendarDate` | `jiff::civil::Date` (checked) | **Real** |
| | `LocalTime` | `jiff::civil::Time` (checked) | **Real** |
| | `LocalDateTime` | `jiff::civil::DateTime` (trusted, opaque) | **Real** |
| | `WeekDate` | `jiff::civil::ISOWeekDate` (checked, all 3) | **Real** |
| | `OrdinalDate` | `jiff::civil::Date` via real `Date::day_of_year()`/`.day_of_year(n)` | **Real**, no direct `FromStr` for the ordinal string form — parse edge needs int-splitting, not a jiff gap |
| | `ReducedCalendarDate` | none | **Needs a thin custom wrapper** (jiff has no year-only/year-month-only partial date form) |
| | `ReducedLocalTime` | none | **Needs a thin custom wrapper** (jiff's `Time` has no partial hour-only/no-seconds form) |
| `TemporalTimeIntervalProps` | `TimeInterval` | new composite over `TimeIntervalRepresentation`'s 3 real forms (StartEnd/StartDuration/DurationEnd) | **Real**, buildable from the carriers above |
| `TemporalRecurringIntervalProps` | `RecurringInterval` | wraps `TimeInterval` + repeat count (ISO 8601's simple `R[n]/` form) | **Real**, thin wrapper |
| `TemporalExtensionProps` (`QualifiedTemporalValue`, `ExplicitTemporalForm`, `ExplicitDuration`, `ExplicitTimeInterval`, `GroupedTimeScaleUnit`, `TemporalSet`, `DateTimeFormula`) | — | none | **Out of scope** — ISO 8601-2 Level 2/3 constructs (masked digits, seasons, sub-year groupings, temporal sets, date-time formulas, explicit time-shifts) a Level-1 library like jiff has zero concept of. `native_props.rs`'s own doc comment sanctions leaving these with no native carrier ("Descriptor-only forms should remain in the neutral accord"). |

### Trait-level feasibility

**Real, buildable (dependency order):**

1. `TemporalDurationProps` + `TemporalDurationNativeBridge` (2 edges)
2. `TemporalInstantProps` + `TemporalInstantNativeBridge` (2 edges)
3. `TemporalCivilProps` (5 real assoc. types) + `TemporalCivilNativeBridge` (2 edges) — the two reduced-precision wrappers land first
4. `TemporalZoneProps` + `TemporalZoneNativeBridge` (4) + `TemporalZoneFactory` (5) + `TemporalNativeZoneFactory` (3)
5. `TemporalNativeConversionFactory` (4) + `TemporalConversionFactory` (4) — presupposes 2-4
6. `TemporalReporter` — real capability declaration
7. `TemporalIntervalFactory` (4) — duration parse real via `Span: FromStr`; interval/recurring-interval parse via hand-rolled `/`-split over the carriers above; `order_offset_endpoints` becomes real jiff arithmetic
8. `TemporalTimeIntervalProps`/`RecurringIntervalProps` + `NativeBridge`s (4) + `TemporalNativeIntervalFactory` (1)
9. `TemporalParser` (~9 of 24 real edges: CalendarDate, WeekDate, LocalTime, LocalDateTime, UtcOffset, OffsetDateTime, OrdinalDate, Rfc3339Timestamp, IxdtfTimestamp) — each via a real jiff `FromStr`/`DateTimeParser::parse_*` method, checked individually
10. `TemporalFormatter` (~9+ matching real edges via jiff's `*Printer` types)

`jiff::tz::Offset` has **no direct `FromStr`** (confirmed by source read
— only reachable as part of a larger parse); a standalone offset-string
parse edge needs a small hand-rolled parser, not a jiff gap.
`ExtendedYear`/`Decade`/`Century` need custom integer parsing (year forms
beyond jiff's ±9999 range or coarser than a full date).

**Genuinely out of scope (no jiff representation, not deferred work):**

- `TemporalExtensionProps` + its 7 `NativeBridge`s (14 edges)
- `TemporalNativeDateTimeFormulaFactory` (1 edge)
- `TemporalCalConnectFactory` (19 edges)
- The `TemporalParser`/`TemporalFormatter` edges keyed to
  `QualifiedTemporalValue`, `SeasonalTemporalExpression`,
  `SubYearGroupingExpression`, `UnspecifiedComponentExpression`,
  `TemporalSet`, `GroupedTimeScaleUnit`, `DateTimeFormula`,
  `DateWithShift`, `TimeOfDayWithShift` (~18 of the 60 parse+format edges)

Roughly **95 of ~129 edges are real and buildable with jiff; ~34 are
permanently out of scope** (ISO 8601-2 / CalConnect). The 95-edge real
surface is the actual checklist below.

## Proposed architecture

- New module `crates/amenable_ext/src/jiff/backend/` (single file until
  size forces a split, per CLAUDE.md's ~500-1000 line guideline).
- `JiffVerifier` + `JiffVerifierMetadata` — same `Verifier`/`Metadata`
  shape as `CanaryVerifier`, but honestly described as a real (if
  partial) backend: its `Exchange` bodies call jiff's real parser/
  calendar/zone code, not a runtime-oracle stand-in.
- A `jiff_backend_trusts!` macro (mirroring `canary_trusts!`) for the
  ~23 machine-checkable range/ordering contracts `JiffVerifier` needs a
  trusted citation for, plus `TemporalInputReceived`.
- Thin `#[derive(amenable_derive::Evidence)] #[evidence(basis = "Self")]`
  native-carrier newtypes per real Props assoc. type (`JiffSpan`,
  `JiffTimestamp`, `JiffOffset`, `JiffDate`, `JiffTime`, `JiffDateTime`,
  `JiffISOWeekDate`, `JiffTimeZone`, `JiffZoned`, the `JiffOffsetDateTime`/
  `JiffTimeInterval`/`JiffRecurringInterval` composites, and the two
  reduced-precision wrappers) — **deliberately separate** from
  `amenable_ext::ExtStandard<T>` (confirmed via source read: `ExtStandard<T>`
  is a `PhantomData<*const T>` marker with no runtime value — the
  witness-registration role; these new types are real value holders for
  the Exchange bodies, the same separation `StdDuration`/
  `RustStdStandard<Duration>` already keep: `std::time::Duration` isn't
  even registered in `rust_std/std_time.rs`, confirming the two axes are
  independent by design).
- One `JiffTimeBackend` struct throughout; `Temporal*Props`/
  `NativeBridge`/`Factory` impls added incrementally, one family per
  commit.
- `TemporalReporter`: `max_fractional_second_digits` → `Some(9)`;
  `supports_leap_seconds` → `false`; `supports_named_zone_round_trip` →
  `true` (real IANA tzdb, a genuine capability upgrade over the canary);
  `current_tzdb_revision` — check `jiff::tz::TimeZoneDatabase`'s real API
  before committing to `Some`/`None`; `supports_unknown_local_offset`/
  `supports_end_of_day_twenty_four` need a real source check before
  either value is asserted.

## Phased checklist

- [x] **Phase 1 — Duration.** `TemporalDurationProps`/`NativeBridge` over
      `JiffSpan`. Real round trip via `jiff::Span`'s own descriptor↔span
      conversion (richer than `std::time::Duration`'s whole-seconds-only
      shape).
- [x] **Phase 2 — Instant.** `TemporalInstantProps`/`NativeBridge` over
      `JiffTimestamp`/`JiffOffset`/`JiffOffsetDateTime`. Real jiff
      arithmetic replaces the canary's own hand-rolled `days_from_civil`
      for the endpoint-ordering exchange.
- [ ] **Phase 3 — Civil.** `TemporalCivilProps`/`NativeBridge` over
      `JiffDate`/`JiffTime`/`JiffDateTime`/`JiffISOWeekDate`, plus the two
      reduced-precision wrappers and the real ordinal-date round trip via
      `Date::day_of_year`.
- [ ] **Phase 4 — Zone.** `TemporalZoneProps`/`NativeBridge`/`Factory`/
      `NativeZoneFactory` over `JiffTimeZone`/`JiffZoned` — the biggest
      genuine capability jump (real IANA tzdb, DST resolution).
- [ ] **Phase 5 — Conversion.** `NativeConversionFactory`/
      `ConversionFactory` (UTC normalize, zone-strip, precision adjust).
- [ ] **Phase 6 — Reporter.** Real capability declaration; verify the two
      flagged unknowns against jiff source before committing.
- [ ] **Phase 7 — Interval factory.** `TemporalIntervalFactory` — real
      duration/order-endpoints edges; interval/recurring-interval
      text-parse via hand-rolled `/`-split over the Phase 1-2 carriers.
- [ ] **Phase 8 — Time interval / recurring interval.**
      `TemporalTimeIntervalProps`/`RecurringIntervalProps` +
      `NativeBridge`s + `NativeIntervalFactory`, over the composite
      carriers.
- [ ] **Phase 9 — Parser.** The ~9 real `TemporalParser` edges, each
      checked individually against jiff's real parser method before
      being marked done — no batch shortcut.
- [ ] **Phase 10 — Formatter.** The matching ~9+ real `TemporalFormatter`
      edges via jiff's `*Printer` types.

Each phase: its own real implementation, its own test file (mirroring
`amenable_std/tests/std_backend_test.rs`'s trait-bound-assertion +
real-round-trip pattern), its own commit, full workspace `cargo check`/
`clippy`/`fmt --check` before committing.

## Explicitly out of scope

`TemporalExtensionProps` and its 7 `NativeBridge`s, `TemporalCalConnectFactory`,
`TemporalNativeDateTimeFormulaFactory`, and the CalConnect-keyed
`Parser`/`Formatter` edges (~34 edges total) genuinely have no jiff
representation. jiff is a Level-1 ISO 8601 calendar/time-zone library;
these are ISO 8601-2/CalConnect Level 2/3 constructs (masked digits,
seasons, temporal sets, recurrence rules, date-time formulas, explicit
time-shifts) that no general-purpose date-time library models. This
matches `native_props.rs`'s own doc-comment guidance to leave such types
in "the neutral accord" rather than force a synthetic wrapper — marked
**out of scope**, not TODO.
