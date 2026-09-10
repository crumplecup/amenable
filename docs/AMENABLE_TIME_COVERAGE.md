# amenable_time coverage

Tracks the `elicit_temporal` → `amenable_time` migration
(`AMENABLE_TIME_PLAN.md`), worked top-to-bottom. One row per
`elicit_temporal` source module.

Columns:

- **ported** — types exist in `amenable_time` as `Standard` (contracts) /
  `Evidence` (aggregates) / plain (descriptors, traits)
- **establish** — the `ProvableFrom` → `Establish<C, V>` graph is wired
  (Phase 3)
- **kani / creusot / verus** — a real (non-trivial) `Ensures<V>` /
  `Witness<V>` proof exists, not the Phase-3 trivial placeholder (Phase 6)

`n/a` = the module has no provable content (pure citation / descriptors /
capability queries).

## Contracts

**Phase 1 complete — 345/345 citation-only contracts ported as `Standard`s.**

| module (`elicit_temporal`) | count | ported | establish | kani | creusot | verus |
|---|---:|:--:|:--:|:--:|:--:|:--:|
| `contracts/precision` | 5 | ✅ 5/5 | — | — | — | — |
| `contracts/iso_8601` | 109 | ✅ 109/109 | — | 1 | 1 | 1 |
| `contracts/extended` (ISO 8601-2) | 48 | ✅ 48/48 | — | — | — | — |
| `contracts/rfc3339` | 20 | ✅ 20/20 (tier A — verbatim) | — | — | — | — |
| `contracts/rfc9557` | 38 | ✅ 38/38 (tier A — verbatim) | — | — | — | — |
| `contracts/calconnect` | 61 | ✅ 61/61 (tier B — ParaphraseOnly; verbatim upgrade deferred) | — | — | — | — |
| `contracts/interval` | 25 | ✅ 25/25 | — | — | — | — |
| `contracts/zone` | 21 | ✅ 21/21 (tier A — verbatim) | — | — | — | — |
| `contracts/instant` | 6 | ✅ 6/6 | — | — | — | — |
| `contracts/serialization` | 5 | ✅ 5/5 | — | — | — | — |
| `contracts/conversion` | 7 | ✅ 7/7 | — | — | — | — |
| `contracts/proof_composition` (aggregates) | 93 | ✅ 93/93 composite `Evidence`+`Witness` (+ 24 shared branch types) | — | — | — | — |

## Descriptors (`types.rs`)

**Phase 2 complete — 93 descriptor definitions ported** (50 structs + 43
enums) into `src/types/` (13 modules). Structs: private fields +
`derive_getters::Getters` accessors + `derive_new::new` (all-required) or
`derive_builder::Builder` (`FooDescriptorBuilder::default()`, 33 of them)
for optional/multi-field. Closed enums: `strum::EnumIter` +
`derive_more::Display`; data-carrying enums plain. No serde / `JsonSchema`
(house policy). No construction-time validation.

| set | ported as | note |
|---|---|---|
| 50 descriptor structs | private fields + `Getters` + `new`/`Builder`; the 23 top-level ones `#[derive(Default, Evidence)]` (Phase 4 Step 2a) | rest gain `Evidence` in Phase 5 |
| 21 closed enums | `EnumIter` + `Display`; `Default` on all (`#[default]` first variant) | Phase 4 Step 2a |
| 22 data-carrying enums | plain derives + a hand `impl Default` on the first variant | Phase 4 Step 2a |
| 13 `*ProofBranch` enums | ✅ `proof_composition::proof_branches` — `#[derive(Evidence, Witness)]` enums, `Established<T>` → `T`, redundant `evidence:` field dropped | Phase 3 Step 2 |
| 22 `*Bundle` aggregate proof bundles | ✅ `proof_composition::semantic_bundles` — folded `#[derive(Evidence, Witness)]` composites (`Established<X>` → `X`, redundant `*Evidence` dropped, `BackendConversionSemanticBundle` embedded as a field) | Phase 5 Step 1 |
| `ProvenTemporalCarrier<T, STok>` + 14 `Proven*Carrier<T>` aliases + 22 `<Bundle>Token`s | ✅ `src/carrier.rs` — a token-keyed `#[derive(Sidecar)]` (`carrier: T` primary where `T: Evidence`; `semantics: STok` token; proposition = `<STok as ProofToken>::Proposition`). Each `<Bundle>Token` `#[establish]`-swaps from the token that produced it. | Phase 5 Step 2 |
| `*Result` type aliases (116) | → the 24 `Parsed*` sidecar structs (parse) so far | rest in Phase 4/5 |

## Exchange surface (`src/exchange/`)

**Phase 4 Steps 1–5 done (2026-09-09).** `RawInput` input sidecar
(`Sidecar<V>` for every `V`). `TemporalParser<V>` / `TemporalFormatter<V>`
/ `TemporalZoneFactory<V>` / `TemporalConversionFactory<V>` /
`TemporalIntervalFactory<V>` — traits whose supertrait bundle *is* the
per-seam exchanges (`impl TemporalParser<V> ⇔ impl all 24
Exchange<RawInput, ParsedX, V>`; the formatter and factory bundles the
same way), plus `where <Prop>: Witness<V>` bounds for every input +
output proposition (the honest precondition — a generic consumer must
restate them; a helper macro is a follow-on).

**Step 4** — the 3 factory seams. Each transition method's descriptors
fold into a `*Request` primary, its `Established<_>` preconditions into a
`*Preconditions` proposition, and its return-tuple proofs into a
`*Established` proposition (the `proof_composition` fold). `*Input` /
`*Output` `#[derive(Sidecar)]` structs; two `#[establish]` edges each
(`*PreconditionsToken` from `TemporalInputToken`, `*EstablishedToken`
from `*PreconditionsToken`), all in
`src/exchange/{zone,conversion,interval,factory_establish}.rs`. The 4
parser-style factory methods (`resolve_named_zone`, `parse_duration`,
`parse_recurring_interval`, `parse_interval`) reuse `RawInput` and the
parser output sidecars.
`NamedTimeZoneDescriptor` / `LocalTimeZoneResolutionAuthorityDescriptor`
/ `PrecisionDescriptor` gained `Evidence`.

- `src/exchange/parse.rs` — 24 + 3 `ParsedX` `#[derive(Sidecar)]` outputs
  (the `elicit_temporal` return tuples, named; the +3 are formatter-only
  inputs — `ParsedDuration` etc.)
- `src/exchange/parse_props.rs` — 12 + 1 per-method composite propositions
  folding the multi-proof parse methods' 2–9 sidecars
- `src/exchange/format.rs` — 36 `Formatted*` `#[derive(Sidecar)]` outputs
  (`FormattedTemporalText` primary + a `<Method>FormattedToken`)
- `src/exchange/format_props.rs` — 36 `<Method>Formatted` emission-proof
  composites (same fold)
- `src/exchange/establish.rs` — 24 parse + 3 input + 36 format
  `#[amenable_derive::establish]` tokens
- `src/exchange/{zone,conversion,interval,factory_establish}.rs` — the 3
  factory seams (Step 4)
- `src/exchange/calconnect.rs` — `TemporalCalConnectFactory` (Step 5),
  self-contained: 6 CalConnect-only parse methods + their 6 emit
  counterparts (folded to `<X>Proof` / `<X>Formatted`), the
  `evaluate_date_time_formula` transition, and their tokens. The 6 shared
  families (qualified value, grouped unit, date-time formula) reuse the
  parser / formatter sidecars.

**No `Exchange` impl in `amenable_time`** — the orphan rule keeps those in
the backend crate (`#[capture_exchange_body]` generates them).
`amenable_derive::Sidecar` fixed (restate `Proposition: Witness<V>`; doc
the generated `new`).

## Traits (`traits/`, ~35 traits / ~175 methods)

| trait | methods | ported | as |
|---|---:|:--:|---|
| `TemporalReporter` | 7 | ✅ | plain trait (not an `Exchange` — capability query) |
| `TemporalParser<V>` | 24 | ✅ 24/24 | supertrait bundle of `Exchange<RawInput, ParsedX, V>` (`src/exchange/`; the `Exchange` impls are a backend concern) |
| `TemporalFormatter<V>` | 36 | ✅ 36/36 | supertrait bundle of `Exchange<ParsedX, FormattedY, V>` — a proven descriptor sidecar in, a `Formatted*` sidecar out (`src/exchange/format*.rs`) |
| `TemporalZoneFactory<V>` | 5 | ✅ 5/5 | supertrait bundle: 1 `Exchange<RawInput, ResolvedNamedTimeZone, V>` + 4 transitions (`*Input` → `*Output`, `src/exchange/zone.rs`) |
| `TemporalConversionFactory<V>` | 4 | ✅ 4/4 | supertrait bundle of 4 transitions (`src/exchange/conversion.rs`) |
| `TemporalIntervalFactory<V>` | 4 | ✅ 4/4 | supertrait bundle: 3 `Exchange<RawInput, Parsed*, V>` (reused parser outputs) + `order_offset_endpoints` transition (`src/exchange/interval.rs`) |
| `TemporalCalConnectFactory<V>` | 19 | ✅ 19/19 | supertrait bundle: 6 new parse + 6 new emit (`Exchange<Parsed*, Formatted*, V>`) + `evaluate_date_time_formula` transition + 6 shared families reusing parser/formatter sidecars (`src/exchange/calconnect.rs`) |
| native-carrier families (16) | — | ✅ 16/16 | `traits/native_props.rs` — associated-type traits, each `type X: Evidence` (so a carrier can be a `ProvenTemporalCarrier` primary); the 3 aggregate traits get blanket impls (Phase 5 Step 3) |
| native `realize_*`/`reflect_*` bridges | 28 | ✅ 28/28 | `traits/native_bridge.rs` — each per-family bridge's supertrait bundle *is* its `Exchange<Reflected<X>, Proven<X>Carrier<Self::X>, V>` + inverse pair; 14 `Reflected<X>` sidecars in `src/exchange/reflect.rs`; elicit's aggregate bridges + blankets (Phase 5 Step 4) |
| native `*_native` factory analogs | 9 | ✅ 9/9 | `traits/native_factory.rs` — carrier→carrier `Exchange`s; multi-input methods fold runtime values into a `<M>NativeRequest<B>` primary (basis = the `NativeCarrierRequest` marker), extra output proofs into a `<M>NativeEstablished` composite (`src/exchange/native.rs`) (Phase 5 Step 4b) |

## Real proofs (Phase 6)

| contract | theorem | kani | creusot | verus |
|---|---|:--:|:--:|:--:|
| `CalendarMonthInRangeOneToTwelve` | the `1..=12` range check agrees, over the whole `u8` domain, with the twelve-way enumeration of the legal calendar months (ISO 8601-1:2019, 3.1.1.2) | ✅ passed | ✅ Proved (150 files) | ✅ 486 verified |
| `HourInRangeZeroToTwentyFour` | `h <= 24` agrees over the whole `u8` domain with `h < 25` (thin boundary guard) | ✅ passed | ✅ Proved (155 files) | ✅ 491 verified |
| `MinuteInRangeZeroToFiftyNine` | `m <= 59` agrees over the whole `u8` domain with `m < 60` | ✅ passed | ✅ Proved (155 files) | ✅ 491 verified |
| `SecondInRangeZeroToSixty` | `s <= 60` (60 admits a leap second) agrees over the whole `u8` domain with `s < 61` | ✅ passed | ✅ Proved (155 files) | ✅ 491 verified |
| `UtcOffsetHourInRangeZeroToTwentyThree` | `h <= 23` agrees over the whole `u8` domain with `h < 24` | ✅ passed | ✅ Proved (155 files) | ✅ 491 verified |
| `UtcOffsetMinuteInRangeZeroToFiftyNine` | `m <= 59` agrees over the whole `u8` domain with `m < 60` | ✅ passed | ✅ Proved (155 files) | ✅ 491 verified |
| `WeekdayInRangeOneToSeven` | the `1..=7` range check agrees, over the whole `u8` domain, with the seven-way enumeration of the ISO weekdays (Mon..Sun) | ✅ passed | ✅ Proved (161 files) | ✅ 497 verified |
| `WeekNumberInRangeOneToFiftyThree` | `1..=53` `contains` agrees over the whole `u8` domain with `1 <= w && w < 54` | ✅ passed | ✅ Proved (161 files) | ✅ 497 verified |
| `OrdinalDayInRangeOneToThreeHundredSixtySix` | `1..=366` `contains` agrees over the whole `u16` domain with `1 <= d && d < 367` | ✅ passed | ✅ Proved (161 files) | ✅ 497 verified |
| `CenturyOrdinalInRangeZeroToNinetyNine` | `o <= 99` agrees over the whole `u8` domain with `o < 100` | ✅ passed | ✅ Proved (161 files) | ✅ 497 verified |
| `DecadeOrdinalInRangeZeroToNineHundredNinetyNine` | `o <= 999` agrees over the whole `u16` domain with `o < 1000` | ✅ passed | ✅ Proved (161 files) | ✅ 497 verified |
| `CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine` | `y <= 9999` agrees over the whole `u16` domain with `y < 10000` | ✅ passed | ✅ Proved (161 files) | ✅ 497 verified |
| `IntervalStartPrecedesEnd` | over every `i32` pair, `start <= end` ⟺ `!(end < start)` ⟺ non-negative `i64` span | ✅ passed | ✅ Proved (164 files) | ✅ 500 verified |
| `IntervalDurationIsNonNegative` | over every `i32` pair, non-negative span ⟺ `start <= end`; span is `0` exactly when endpoints coincide | ✅ passed | ✅ Proved (164 files) | ✅ 500 verified |
| `UtcTimelineOrderingAppliesToFixedInstants` | `<=` on `i32` timeline positions is a total order — reflexive, antisymmetric, total, transitive | ✅ passed | ✅ Proved (164 files) | ✅ 500 verified |
| `GregorianLeapYearUsesDivisibleByFourAndFourHundredException` | Kani: `y%4 && (y%100 \|\| y%400)` agrees over every `i32` with the centennial case split + 6 dated anchors. Verus/Creusot: definitional + `leap(y) ⟹ 4∣y` (+ Verus anchors) | ✅ passed | ✅ Proved (166 files) | ✅ 502 verified |
| `CentennialYearDivisibleByOneHundred` | Kani: `y%100==0` agrees over every `i32` with `y%4==0 && y%25==0` + anchors. Verus: definitional + anchors. Creusot: definitional | ✅ passed | ✅ Proved (166 files) | ✅ 502 verified |
| `LeapYearHasThreeHundredSixtySixCalendarDays` | over `y ≥ 0`: `days_in_year(y) == 366` ⟺ `leap(y)`; a leap year is a common year + 1 day; count always 365/366 (Creusot: leap-predicate biconditional only) | ✅ passed | ✅ Proved (169 files) | ✅ 505 verified |
| `CommonYearHasThreeHundredSixtyFiveCalendarDays` | over `y ≥ 0`: `days_in_year(y) == 365` ⟺ `!leap(y)`; the two lengths are distinct (Creusot: leap-predicate biconditional only) | ✅ passed | ✅ Proved (169 files) | ✅ 505 verified |
| `YearDurationInRangeThreeHundredSixtyFiveToThreeHundredSixtySixCalendarDays` | over `y ≥ 0`: `days_in_year(y)` is always 365 or 366 — the model is well-formed | ✅ passed | ✅ Proved (169 files) | ✅ 505 verified |
| `MonthDurationInRangeTwentyEightToThirtyOneCalendarDays` | Kani/Verus: over every year and month `1..=12`, the duration is always `28..=31`, each length characterised exactly, and the twelve months sum to `days_in_year(y)`. Creusot: always in range | ✅ passed | ✅ Proved (172 files) | ✅ 508 verified |
| `CalendarDayWithinMonthBounds` | Kani/Verus: a valid day is in `1..=31` and a valid February 29 forces a leap year. Creusot: definitional + Feb-29 ⟹ leap | ✅ passed | ✅ Proved (172 files) | ✅ 508 verified |
| `LeapDayOccursOnlyInLeapYear` | February 29 is a valid calendar date exactly when the year is a leap year | ✅ passed | ✅ Proved (172 files) | ✅ 508 verified |

Some contracts prove different depths on different backends — the
number theory that is nonlinear for an SMT solver (Verus, Creusot) is
carried bit-precisely by Kani, and the two SMT backends carry the
definitional check plus whatever linear/propositional facts survive.

Wiring: `amenable_kani::time` / `amenable_creusot::time` take an
`amenable_time` dep; Verus's real `verus! {}` proof lives in
`amenable_verus/src/time/`, with `amenable_time`'s `verus` feature
gating `src/verus_witness.rs`.

## The live report

`just temporal-coverage` (`amenable temporal-coverage --features
creusot,verus`) prints this table from the registries: **345 atomic
contracts, 23 machine-checked on all three backends, 322 citation-only.**
This file is the prose companion; the recipe is the source of truth.

## Composed aggregates

All 152 `proof_composition` `#[derive(Witness)]` aggregates
(`CalendarDateValid`, `LocalTimeValid`, `Rfc3339TimestampValid`,
`IxdtfTimestampValid`, …) resolve as real `ClassifiedWitness<V>` on Kani,
Creusot and Verus. A composite's derived proof is a product of its
members': the 23 machine-checked leaves contribute `checked` support, the
322 citation-only structural leaves (`amenable_time::structural_witness`,
`trivial`/`trusted` by citation tier) contribute the rest, and the
composite's `WitnessSupportSummary` reports the breakdown. Asserted by
`temporal_composition_test` in each backend crate.

## Canary backend

`amenable_time::backends::std_time` implements the temporal trait surface
against real `std::time` types — an interface-drift tripwire *and* a
runtime oracle over the slice it covers, not a usable backend.
`CanaryVerifier` runs no formal tool, but the `Exchange` bodies execute
the contracts: `order_offset_endpoints` resolves both endpoints to epoch
seconds through real Gregorian calendar arithmetic and returns `Err` for a
reversed interval; the `TemporalDurationNativeBridge` realize/reflect pair
round-trips a span through an actual `std::time::Duration` and rejects
year/month components. Coverage: `TemporalDurationProps` /
`TemporalInstantProps` / `TemporalReporter`,
`TemporalIntervalFactory<CanaryVerifier>` (real `order_offset_endpoints`;
the three ISO-8601-parse edges report unsupported), and
`TemporalDurationNativeBridge<CanaryVerifier>`. Calendar/week/ordinal
dates, zones, parsing and formatting are out of scope — those need a
date-time library (`jiff` / `chrono`).

## Licensing gate

Phase 1 exit criterion: a test over the `EvidenceLink` registry asserts
no `StandardsBody::Iso` contract carries `NormativeQuotation::Verbatim`.
Not yet in place (only `contracts/precision` exists).
