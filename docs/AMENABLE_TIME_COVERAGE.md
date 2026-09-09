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

| module (`elicit_temporal`) | count | ported | establish | kani | creusot | verus |
|---|---:|:--:|:--:|:--:|:--:|:--:|
| `contracts/precision` | 5 | ✅ 5/5 | — | — | — | — |
| `contracts/iso_8601` | 109 | ✅ 109/109 | — | — | — | — |
| `contracts/extended` (ISO 8601-2) | 48 | ✅ 48/48 | — | — | — | — |
| `contracts/rfc3339` | 20 | ✅ 20/20 (tier A — verbatim) | — | — | — | — |
| `contracts/rfc9557` | 38 | — | — | — | — | — |
| `contracts/calconnect` | 61 | — | — | — | — | — |
| `contracts/interval` | 25 | — | — | — | — | — |
| `contracts/zone` | 21 | — | — | — | — | — |
| `contracts/instant` | 6 | — | — | — | — | — |
| `contracts/serialization` | 5 | — | — | — | — | — |
| `contracts/conversion` | 7 | — | — | — | — | — |
| `contracts/proof_composition` | 93 | — | — | — | — | — |

## Descriptors (`types.rs`, 259 defs)

| set | ported as | note |
|---|---|---|
| `TemporalComponent`, `SerializationProfile` | plain enums | Phase 0 minimal set |
| everything else | — | Phase 2; structs gain `#[derive(Evidence)]` then |

## Traits (`traits/`, ~35 traits / ~175 methods)

| trait | methods | ported | as |
|---|---:|:--:|---|
| `TemporalReporter` | 7 | ✅ | plain trait (not an `Exchange` — capability query) |
| `TemporalParser` | 24 | — | `Exchange` per method (Phase 4) |
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
