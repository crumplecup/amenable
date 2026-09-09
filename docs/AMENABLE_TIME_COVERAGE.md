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
| `*SemanticBundle` (23), `ProvenTemporalCarrier`, `Proven*Carrier` aliases (14) | — | Phase 5 |
| `*Result` type aliases (116) | → the 24 `Parsed*` sidecar structs (parse) so far | rest in Phase 4/5 |

## Exchange surface (`src/exchange/`)

**Phase 4 Steps 1 + 2 done (2026-09-09).** `RawInput` input sidecar
(`Sidecar<V>` for every `V`). `TemporalParser<V>` — a trait whose
supertrait bundle *is* the 24 parse exchanges (`impl TemporalParser<V> ⇔
impl all 24 Exchange<RawInput, ParsedX, V>`), plus 24 `where <Prop>:
Witness<V>` bounds (the honest precondition — a generic consumer must
restate them; a helper macro is a follow-on). `src/exchange/parse.rs` (24
`ParsedX` `#[derive(Sidecar)]` outputs — the `elicit_temporal` return
tuples, named), `parse_props.rs` (12 per-method composite propositions
folding the multi-proof methods' 2–9 sidecars), `establish.rs` (24
`#[amenable_derive::establish]` tokens). **No `Exchange` impl in
`amenable_time`** — the orphan rule keeps those in the backend crate
(`#[capture_exchange_body]` generates them). `amenable_derive::Sidecar`
fixed (restate `Proposition: Witness<V>`; doc the generated `new`).

## Traits (`traits/`, ~35 traits / ~175 methods)

| trait | methods | ported | as |
|---|---:|:--:|---|
| `TemporalReporter` | 7 | ✅ | plain trait (not an `Exchange` — capability query) |
| `TemporalParser<V>` | 24 | ✅ 24/24 | supertrait bundle of `Exchange<RawInput, ParsedX, V>` (`src/exchange/`; the `Exchange` impls are a backend concern) |
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
