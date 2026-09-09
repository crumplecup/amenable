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
| `contracts/iso_8601` | 109 | ✅ 109/109 | — | — | — | — |
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
| native `*_native` factory analogs | 8 | — | carrier→carrier `Exchange` (Phase 5 Step 4b) |

## Licensing gate

Phase 1 exit criterion: a test over the `EvidenceLink` registry asserts
no `StandardsBody::Iso` contract carries `NormativeQuotation::Verbatim`.
Not yet in place (only `contracts/precision` exists).
