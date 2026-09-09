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
| 50 descriptor structs | private fields + `Getters` + `new`/`Builder` | `#[derive(Evidence)]` deferred to Phase 3/4 |
| 21 closed enums | `EnumIter` + `Display` | `Default` only where `elicit_temporal` had it (`UtcOffsetSign`, `UtcOffsetRelationship`, `DurationDescriptor`) |
| 22 data-carrying enums | `Debug`/`Clone`/`PartialEq`/`Eq`/`Hash`/`Ord` | umbrella + boundary families |
| 13 `*ProofBranch` enums | ✅ `proof_composition::proof_branches` — `#[derive(Evidence, Witness)]` enums, `Established<T>` → `T`, redundant `evidence:` field dropped | Phase 3 Step 2 |
| `*SemanticBundle` (23), `ProvenTemporalCarrier`, `Proven*Carrier` aliases (14) | — | Phase 5 |
| `*Result` type aliases (116) | — | Phase 3/4 |

Deferred `#[derive(Evidence)]`: forcing `Evidence` (hence `Default`) onto
every data-only enum here is meaningless; Phase 3/4's `Sidecar<V>` wiring
shows exactly which descriptors become a `Primary` and need it.

## Exchange surface (`src/exchange/`)

**Phase 4 Step 1 done (2026-09-09)** — the generic carriers: `RawInput`
(`Sidecar<V>` for every `V`, unconditionally), `Proven<D, P>`
(`Sidecar<V>` when `P: Witness<V>`), `Proven::prove` (the honest Phase-4
stand-in for `Establish::establish`), `TemporalInputToken` /
`ProvenToken<P>`. `CalendarDateDescriptor` is the first descriptor with
`#[derive(Evidence)]`.

## Traits (`traits/`, ~35 traits / ~175 methods)

| trait | methods | ported | as |
|---|---:|:--:|---|
| `TemporalReporter` | 7 | ✅ | plain trait (not an `Exchange` — capability query) |
| `TemporalParser` | 24 | — | `Exchange` per method (Phase 4 Step 2) |
| `TemporalFormatter` | 36 | — | `Exchange` per method (Phase 4) |
| `TemporalZoneFactory` | 5 | — | Phase 4 |
| `TemporalConversionFactory` | 4 | — | Phase 4 |
| `TemporalIntervalFactory` | 4 | — | Phase 4 |
| `TemporalCalConnectFactory` | 19 | — | Phase 4 |
| native-carrier families (16) | 0 | — | associated types (Phase 5) |
| native bridges (`realize_*`/`reflect_*`) | ~30 | — | `Exchange` (Phase 5) |

## Licensing gate

Phase 1 exit criterion: a test over the `EvidenceLink` registry asserts
no `StandardsBody::Iso` contract carries `NormativeQuotation::Verbatim`.
Not yet in place (only `contracts/precision` exists).
