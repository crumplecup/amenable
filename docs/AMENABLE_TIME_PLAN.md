# Amenable Time Plan

## Status

🔲 In progress — plan reviewed, all 7 decisions settled (2026-09-08).
**Phase 0 partially landed**: `crates/amenable_time` exists — error
layer, `provenance_vocab`, `TemporalProvenance`, the `temporal_standard!`
macro, and `contracts::precision` (5 contracts, 6 passing tests) are all
in and `just check-all-package amenable_time` clean. Remaining Phase 0:
the minimal descriptor set, `TemporalReporter`, the one end-to-end
`Exchange` edge with its three-backend proofs, and the coverage
checklist. Design written after a direct read of
`~/repos/elicitation/crates/elicit_temporal` (31 files, ~15.6k lines).

## Why this exists

`amenable_core`'s trait family (`Standard`, `Evidence`, `Provenance`,
`Metadata`, `Witness<V>`, `Sidecar<V>`, `Establish<C, V>`,
`Exchange<Input, Output, V>`, `Ensures<V>`/`Requires<V>`, `Registry`/
`Certificate`) has so far been exercised by:

- `Stoplight` — three edges, every state a zero-field ZST with one
  inhabitant.
- `amenable_gaap` — one `Transfer` typestate, four atomic contracts, one
  arithmetic identity.
- `amenable_std` — a wide but shallow field of one-fact-per-file
  std-library leaves.

None of these puts the interface under *width* stress: many hundreds of
distinct contract types drawn from several independent normative
authorities, dozens of trait methods each carrying two or three proof
sidecars, aggregate "semantic bundle" proofs composed from a dozen
leaves, and a real cross-authority provenance vocabulary. `elicit_temporal`
is exactly that shape and already exists as prior art. Migrating its
**full scope** into a dedicated `amenable_time` crate is the load test:
it will show where `Standard`/`Exchange`/the derives/the registration
story bend or break at scale, under conditions the current worked
examples never create.

The migration is also the first real answer to a question `amenable`'s
own positioning raises (memory `project_amenable_positioning`): *express
program invariants as named types so the compiler does the
invariant-checking*. `elicit_temporal` names ~440 invariants as types
already; the exercise is re-expressing them in `amenable`'s vocabulary
such that **every contract type is a `Standard`** (or, where it is
genuinely provable rather than cited, an `Evidence` — see the split
below) **and every trait method is an `Exchange`**.

This is a multi-session effort. The plan is the whole elephant; the
phases eat it in bites.

## Source material: the `elicit_temporal` inventory

`elicit_temporal` is a **contract-first interface crate** with no
implementation and — today — **no consumers**. `elicit_time`,
`elicit_chrono`, `elicit_jiff` exist but predate it and do not depend on
it; they are prior art for how a backend *could* satisfy the interface,
not part of it. So the migration ports an *interface and its vocabulary*,
not a working multi-crate system.

Four architectural layers (the crate's own `traits/mod.rs` names three;
the contract vocabulary is the fourth and largest):

### 1. Contracts — `src/contracts/`, ~440 ZST proposition types

Every contract is `pub struct Foo;` plus `impl Prop for Foo` (elicitation's
`Prop`: `kani_proof()` / `verus_proof()` / `creusot_proof()` each
returning a `proc_macro2::TokenStream`). A single macro, `structural_prop!`,
generates the near-universal case: all three proof bodies are an empty
`quote! { /* structural: … */ }`. **These are citation-only.** Their
content is the doc comment's `Normative source:` line, not a checkable
predicate.

| File | Count | Normative authority |
|---|---:|---|
| `iso_8601.rs` | 109 | ISO 8601-1:2019 (+ Amd 1:2022) |
| `proof_composition.rs` | 93 | *(aggregates — see layer 3)* |
| `calconnect.rs` | 61 | CalConnect CC 18011:2018 |
| `extended.rs` | 48 | ISO 8601-2:2019 + LoC EDTF |
| `rfc9557.rs` | 38 | RFC 9557 (IXDTF) |
| `interval.rs` | 25 | ISO 8601-1/-2 intervals |
| `zone.rs` | 21 | RFC 9557 §2–4, IANA TZDB |
| `rfc3339.rs` | 20 | RFC 3339 |
| `conversion.rs` | 7 | (cross-authority conversion law) |
| `instant.rs` | 6 | ISO 8601-1 §5.4, RFC 3339 §4.4 |
| `precision.rs` | 5 | ISO 8601-1 Amd 1, ISO 8601-2 |
| `serialization.rs` | 5 | ISO 8601-1/-2, RFC 9557 |
| **total** | **~438** | |

### 2. Descriptors — `src/types.rs`, 259 type definitions

Neutral data carriers used in trait signatures: `CalendarDateDescriptor
{ year: i32, month: u8, day: u8 }`, `UtcOffsetDescriptor`,
`IxdtfTimestampDescriptor`, plus ~30 closed enums (`TemporalComponent`,
`SerializationProfile`, `UtcOffsetSign`, the `*ProofBranch` enums) and
~90 `pub type XResult = TemporalResult<(Descriptor, Established<P>, …)>`
aliases. Most structs already `#[derive(Builder)]` with
`#[builder(pattern = "owned")]` — the house builder convention is
already satisfied upstream.

### 3. Proof composition — `src/contracts/proof_composition.rs`

93 aggregate props (`CalendarDateValid`, `Rfc3339TimestampValid`,
`IntervalEndpointsOrdered`, …), each paired with an `*Evidence` bundle
type, wired by ~98 `impl ProvableFrom<XEvidence> for XValid {}` lines.
Several `XValid` types are provable from more than one distinct evidence
(three `CompleteInterval…SubstitutionEvidence` → one
`CompleteIntervalSubstitutionSemanticsValid`). This is the layer that is
*not* citation-only: a `*Valid` proposition is minted from a credential,
which is exactly `Establish<C, V>`.

### 4. Runtime seams — `src/traits/`, ~35 traits, ~175 methods

- **Descriptor factories** (role 1a — "leaf factory"): `TemporalParser`
  (24 methods), `TemporalFormatter` (36), `TemporalZoneFactory` (5),
  `TemporalConversionFactory` (4), `TemporalIntervalFactory` (4),
  `TemporalCalConnectFactory` (19). Each method:
  `raw input → TemporalResult<(Descriptor, Established<P>, …proof sidecars)>`.
- **Section factory** (role 1b): the `ProvableFrom` graph itself.
- **Reporter** (role 2): `TemporalReporter` (10 methods) — capability
  queries, mints no proofs, returns plain values.
- **Native associated-type families** (16 traits, `native_props.rs`):
  `TemporalCivilProps { type CalendarDate; type LocalDateTime; … }` etc.
  — backend-owned carrier types, no methods.
- **Native bridges** (`native_bridge.rs`, `native_span.rs`,
  `native_zone.rs`, `native_conversion.rs`, `native_extension.rs`,
  `native_interval.rs`): `realize_*` / `reflect_*` method pairs moving a
  descriptor + a `SemanticBundle` to/from a `ProvenTemporalCarrier<T, S>`
  (`{ carrier: T, semantics: S }`).
- **`SemanticBundle` types** (`types.rs`, ~25): aggregate
  `Established<P>` + `*Evidence` fields, `#[derive(Builder)]`. The
  `Sidecar`-composite shape.

### 5. Error — `src/error.rs`

`TemporalError { kind, file: &'static str, line: u32 }` +
`TemporalErrorKind` (7 variants). Uses `&'static str` fields and `pub`
fields — **does not** follow this workspace's error conventions (owned
`String`, private fields + `derive_getters`); the port fixes that.

## The core mapping

| `elicit_temporal` / `elicitation` | `amenable` | Notes |
|---|---|---|
| `Prop` (ZST + 3 empty proof `TokenStream`s) — citation-only | **`Standard`** | ~345 of them. Provenance record cites the normative source. |
| `Prop` — the `proof_composition` aggregates | **`Evidence`** (`#[derive(Evidence)] #[evidence(basis = …)]`) | ~95. Per the GAAP lesson (memory `project_amenable_ensures_requires_contract_types`, and `amenable_gaap/src/contracts.rs`'s own doc): a claim that gets *proven* is `Evidence`, not an asserted root. |
| `Established<P>` (proof token) | a `ProofToken` whose `Proposition = P` | minted only through a lawful path |
| `impl ProvableFrom<C> for P {}` | `impl Establish<C, V> for P` | `Establish` is `amenable`'s deliberate rename of `ProvableFrom` (memory `project_...`, `PROVABLE_FROM_PLAN.md`); `C: ProofToken`, consumed by value |
| `proof_credential! { C => P; }` | `#[derive(ProofToken)]` + an `establish` wiring | |
| descriptor struct (`CalendarDateDescriptor`) | `Sidecar::Primary` payload; also `#[derive(Evidence)] #[evidence(basis = "Self")]` so it can flow through a sidecar | already `#[derive(Builder)]` |
| trait method `fn parse_x(&self, s: &str) -> Result<(Desc, Established<P>)>` | **`Exchange<InRaw, OutProven, V>`** where `InRaw: Sidecar<V>` (payload `&str`/an `Unvalidated` newtype, trivial precondition) and `OutProven: Sidecar<V>` (payload `Desc`, proposition `P`) | "every trait method is an Exchange" |
| `SemanticBundle { validity: Established<A>, …, local_semantics: Established<B>, … }` | a composed `Witness<V>` via the derive-witness composition machinery (`VERUS_DERIVE_WITNESS_COMPOSITION_PLAN.md`), surfaced as one `Sidecar<V>::Proposition` | the conjunction-of-leaves case the composition derive exists for |
| `ProvenTemporalCarrier<T, S> { carrier, semantics }` | `Sidecar<V>` with `Primary = T`, `Proposition` = the composed bundle | |
| `TemporalCivilProps { type CalendarDate; … }` | unchanged — associated types on the `amenable_time` backend trait | native carriers stay backend-owned |
| `TemporalReporter` | plain trait; feeds an `Amenable`-style capability surface, **not** an `Exchange` (mints no proof) | the one documented exception to "every method is an Exchange" |
| `TemporalError` (`&'static str`, pub fields) | rebuilt per CLAUDE.md error patterns 2+3 | owned `String`, private, `derive_getters`, `#[track_caller]` |
| the `structural_prop!` macro | a new `temporal_standard!` macro in `amenable_time` | one line per contract → ZST + `#[derive(Standard)]` + provenance + `inventory` registration |

### The one deliberate deviation from "every contract type is a `Standard`"

The user's framing is "every contract type is a `Standard`." The GAAP
worked example already found the sharp edge here
(`amenable_gaap/src/contracts.rs`, and memory
`project_amenable_ensures_requires_contract_types`): elicitation's `Prop`
maps cleanly to `Standard` **only for its citation-only, unprovable
members**. A proposition with a real proof body is `Evidence` — it
terminates its chain on something other than itself, and it does not
carry a `Provenance` record the way a cited standard does.

`elicit_temporal` splits almost perfectly along this line already:

- **`contracts/` layers 1 (the ~345 structural props)** — citation-only.
  Every one is `impl Prop` with an *empty* proof body and a
  `Normative source:` doc line. → **`Standard`**, provenance = the
  citation.
- **`contracts/proof_composition.rs` (the ~95 `*Valid` / `*Evidence`
  aggregates)** — minted from a credential via `ProvableFrom`. →
  **`Evidence`**, wired with `Establish<C, V>`, and the target of the
  real per-backend proofs in Phase 6.

Recommendation: adopt the split, name it explicitly in
`amenable_time`'s own module docs, and treat "contract type" in the
user's phrasing as covering both halves (a `Standard` *and* an `Evidence`
are both "named-type invariants" — the amenable thesis doesn't require
them to be the same trait).

## "Every trait method is an `Exchange`"

Each descriptor-factory method becomes an `Exchange` impl. Concretely,
for `TemporalParser::parse_calendar_date`:

```text
// payload newtype — the raw, not-yet-lawful input
struct UnvalidatedCalendarDate(String);   // Evidence (basis = Self), trivial

// input sidecar: raw text + a trivial "this is well-formed UTF-8 we were handed" token
struct RawInput<T>          : Sidecar<V> { Primary = T; Proposition = InputReceived; }

// output sidecar: the descriptor + the real proposition
struct Proven<D, P>         : Sidecar<V> { Primary = D; Proposition = P; }

impl Exchange<RawInput<UnvalidatedCalendarDate>,
              Proven<CalendarDateDescriptor, CalendarDateValid>,
              V>
    for TemporalParserImpl { … }
```

Methods returning *several* `Established<P>` sidecars
(`parse_local_date_time` returns three) map to an `Output` whose
`Proposition` is a composed `Witness<V>` (the derive-witness conjunction),
so `Exchange`'s single-`Output`-sidecar shape still holds.

`TemporalFormatter` methods run the other direction (descriptor + its
validity proof in, wire `String` + an emission-conformance proof out) —
still an `Exchange`, just `Proven<Descriptor, Valid>` → `Proven<String,
Iso8601ExtendedFormUsesSeparators>`.

The `realize_*` / `reflect_*` native-bridge pairs are `Exchange`s
between a descriptor sidecar and a `ProvenTemporalCarrier` sidecar.

`TemporalReporter` is the exception: capability reporting mints nothing,
so it stays a plain trait and feeds `Amenable::*_surface()` /
`RegistryReport` instead.

## Provenance sources to accommodate

The user asked for the total count up front. From a full sweep of every
`Normative source:` / `Informative cross-check:` / `Open-text
cross-check:` line:

**Normative authorities (9):**

1. ISO 8601-1:2019 (incl. Amd 1:2022)
2. ISO 8601-2:2019
3. RFC 3339
4. RFC 9557 (IXDTF)
5. CalConnect CC 18011:2018
6. IANA TZDB (referenced by the zone contracts)
7. SI / BIPM — the second as SI base unit, leap-second rules
8. ISO/WD 8601-1:2016(E) — the "open-text" pre-publication draft, cited
   as a cross-check where the published standard is paywalled
9. Library of Congress EDTF (Extended Date/Time Format) — informative,
   for the `extended.rs` level-1/level-2 forms

**Producer / carrier provenance (3), for the eventual backend impls —
bringing the total to 12:**

1. `time` crate
2. `chrono` crate
3. `jiff` crate

These 12 are the closed set. (The three producer libraries mirror
`amenable_std`'s own
`RustStdProvenance` / `RustLanguageProvenance` split — a carrier library
is a provenance authority for the "this value came through backend X"
claim.)

## Metadata approach — the user's "single vs. per-source" question

**Recommendation: one shared vocabulary, one provenance *record type*,
the existing `Registry`/`Certificate` as the umbrella.** Not a separate
metadata struct per authority.

Rationale: the authorities differ only in *which values* fill the same
handful of slots — document name, section, clause, normative status,
plus zero-or-more cross-checks. That is the exact situation the
`METADATA_TRAIT_PLAN.md` work built for: a shared `#[derive(Entry)]`
vocabulary (`amenable_std::provenance_vocab`) whose keys can't drift,
composed into a record with `#[entry(flatten)]` / `#[entry(nested)]`.

Concretely, a new `amenable_time::provenance_vocab` (decision 1) with:

| Entry type | key | example value |
|---|---|---|
| `NormativeDocument` (string newtype) | `normative_document` | `"ISO 8601-1:2019"` |
| `NormativeSection` (string newtype) | `normative_section` | `"5.2.2"` |
| `NormativeStatus` (closed enum) | `normative_status` | `Normative` / `Informative` / `OpenTextCrossCheck` |
| `StandardsBody` (closed enum) | `standards_body` | `Iso` / `Ietf` / `CalConnect` / `Iana` / `Bipm` / `LibraryOfCongress` |
| `NormativeQuotation` (closed enum) | `normative_quotation` | `Verbatim(String)` / `ParaphraseOnly` / `Unavailable` — merges the "what text" and "what status" concerns into one type; `Display` is meaningful for all three variants |
| `CrossCheck` (struct) | `cross_check_{n}` | one secondary citation `{ document, section, status }` |

plus reuse of `SourceUrl` / `SemanticSummary` from `amenable_std`
(`Authority` turned out unused). A clause *title* (`NormativeClause`)
was dropped for now — the `SemanticSummary` paraphrase covers it;
revisit if a real need appears.

Then one record type (as-built, Phase 0):

```rust
#[derive(Debug, Clone, PartialEq, Eq, derive_getters::Getters)]
pub struct TemporalProvenance {
    document: NormativeDocument,
    section: NormativeSection,
    status: NormativeStatus,
    body: StandardsBody,
    summary: SemanticSummary,        // our paraphrase, every contract
    quotation: NormativeQuotation,   // Verbatim | ParaphraseOnly | Unavailable
    url: Option<SourceUrl>,          // stable deep link where one exists
    cross_checks: Vec<CrossCheck>,   // 0..n secondary citations
}
// hand-written `new` + `with_url` / `with_cross_check` (chain by value);
// hand-written `impl Metadata` (skips absent url, indexes cross-checks);
// `impl Provenance {}`. Not `#[derive(Provenance)]` — the derive is for
// the all-fields-always case; this record has real optionality.
```

### No checked-in source dump — embed the clause, not the corpus

`elicit_temporal` carries `crates/elicit_temporal/standards/` — ~4.5 MB
of RFC/EDTF/TZDB text, CalConnect XML, ISO working-draft PDFs, **and a
`standards/licensed/` subtree with paywalled ISO 8601 "sample" PDFs and
multi-page `.txt` extracts**. `amenable_time` carries **none of this**.
The relevant normative text lives *in the provenance metadata* — the
specific clause, next to the one type it justifies — governed by the
source's redistributability:

| Tier | Sources | `quotation` variant | What the record carries |
|---|---|---|---|
| **A — freely redistributable** | RFC 3339, RFC 9557 (IETF Trust / BCP 78), LoC EDTF, IANA TZDB (public domain) | `Verbatim(..)` | `SourceUrl` (stable deep link) **+** the specific sentence/clause, never the whole section |
| **B — public, copyrighted** | CalConnect CC 18011 / 18012 | `Verbatim(..)` | `SourceUrl` if stable + the single relevant clause |
| **C — paywalled** | ISO 8601-1:2019, -2:2019, Amd 1:2022; the ISO WD drafts | `ParaphraseOnly` | `SemanticSummary` + exact `NormativeSection` + catalog `SourceUrl` only. **No verbatim ISO text anywhere in the repo.** |

`SemanticSummary` (our paraphrase) is present on *every* contract
regardless of tier. Tier C carries `NormativeQuotation::ParaphraseOnly`,
which `Display`s as a fixed marker string, never ISO prose. **A future
review gate / cordial rule** asserts no `StandardsBody::Iso` contract
ever carries `Verbatim(..)` — the plan Phase 1 exit criterion. (The
`temporal_standard!` macro does not yet enforce this at expansion; a
decl macro can't compare a runtime enum variant against `body:`. The
gate is a test / lint, added in Phase 1.)

Excerpts are string literals in the `temporal_standard!` call itself, so
the type, our summary, and (tier A/B) their verbatim clause all sit in
one place — each contract type is self-contained for audit with no
cross-reference to an external corpus. Distributed cost: ~200 tier-A/B
contracts × 1–3 sentences ≈ 40–80 KB spread across the source tree,
versus `elicit_temporal`'s 4.5 MB dump.

Every `Standard` contract's `provenance()` returns a `TemporalProvenance`.
The **umbrella cert** is not a new type: a `TemporalBackend` implementor
calls `contract.certification(&mut registry, subject)` per supported
contract, and a workspace report (`RegistryReport` / a
`just temporal-coverage` recipe) aggregates — the same shape
`amenable_std` already uses for its std-library coverage.

The `*SemanticBundle` aggregates carry *proof* composition, not
*provenance* composition — those become composed `Witness<V>`s (Phase 5),
separate from this.

## Registration strategy — keeping ~440 types orderly

1. **Module layout mirrors `elicit_temporal/src/contracts/` one-to-one:**
   `amenable_time/src/contracts/{iso_8601,iso_8601_2,rfc3339,rfc9557,
   calconnect,interval,zone,instant,precision,serialization,conversion}.rs`
   alongside `proof_composition.rs`. `lib.rs` stays `mod` + `pub use` only (house
   rule); each contract module re-exports its own set; `lib.rs` re-exports
   from the modules. (`elicit_temporal`'s giant flat `pub use contracts::{…}`
   block in `lib.rs` is ~300 lines — we split the re-export burden across
   the module `mod.rs` files.)

2. **A `temporal_standard!` macro** (in `amenable_time`, the
   `structural_prop!` analog) — one line per citation-only contract:

   As-built syntax (named fields, not positional — less error-prone
   across hundreds of entries):

   ```rust
   use crate::NormativeQuotation;

   temporal_standard! {
       /// A complete ISO 8601 calendar date carries year, month, and day.
       CalendarDateHasYearMonthDay => (
           document: "ISO 8601-1:2019",
           section: "5.2.2",
           body: Iso,
           status: Normative,
           summary: "a complete calendar date names year, month, and day",
           quotation: NormativeQuotation::ParaphraseOnly,       // tier C
           url: "https://www.iso.org/standard/70907.html",
       );

       /// An RFC 3339 timestamp uses a four-digit year.
       Rfc3339UsesFourDigitYear => (
           document: "RFC 3339",
           section: "5.6",
           body: Ietf,
           status: Normative,
           summary: "the year field is exactly four digits",
           quotation: NormativeQuotation::verbatim("date-fullyear   = 4DIGIT"),  // tier A
           url: "https://www.rfc-editor.org/rfc/rfc3339#section-5.6",
       );
   }
   ```

   Each entry expands to: the ZST + `#[derive(amenable_derive::Standard)]`
   (`basis = "Self"`, `provenance = "Self::facts()"`,
   `provenance_type = "crate::TemporalProvenance"`), a private `facts()`
   building the `TemporalProvenance` by value-chained `let`, and (from
   the `Standard` derive) an `EvidenceLink` self-registration. `url:` and
   `cross_check: (doc, section, status)` repeat 0..n and are optional.
   The macro is crate-internal (`#[macro_use]`), so it can use
   `#[derive(Standard)]` with a `crate::`-qualified provenance type
   rather than hand-rolling the trait impls; it becomes `#[macro_export]`
   only when a real second caller appears. The tier-C-never-`Verbatim`
   check is a Phase 1 test/lint, not macro-enforced (a decl macro can't
   inspect an `expr` value against `body:`).

3. **A `temporal_evidence!` macro** for the `proof_composition` layer —
   emits `#[derive(Evidence)]` + the `Establish<C, V>` wiring stub +
   `EvidenceLink`. The per-backend `Ensures<V>` bodies are added later
   (Phase 6) and live in the backend crates.

4. **Per-backend impl crates.** Contract types + descriptors + trait
   *interfaces* live in `amenable_time`. `Witness<V>` / `Ensures<V>` /
   `Requires<V>` / `Establish<C, V>` impls live in `amenable_kani::time`,
   `amenable_creusot::time`, `amenable_verus::time` — orphan rules force
   it there (the verifier marker type lives there), and
   backend-to-backend Cargo deps stay forbidden (memory
   `feedback_verifier_backends_never_depend_on_each_other`). Shared
   data crosses through `amenable_core` types, the `inventory` registry,
   and the `amenable` facade, exactly as `amenable_gaap` → `amenable_kani::
   ledger` already does.

5. **Codegen for the proof companions**, reusing the existing pipeline
   (`emit-creusot-companions` / `emit-verus-exchange-companions`, driven
   by `ExchangeEdgeRecord`) rather than hand-writing per-backend mirrors
   (memory `feedback_codegen_over_registry_patching`,
   `feedback_use_codegen_not_hand_mirror`). Each `Exchange` edge (a
   ported trait method) captures its body and registers an
   `ExchangeEdgeRecord`; the Creusot/Verus companions are generated.

6. **A coverage checklist** — `cordial`-style or a plain generated
   `docs/` table — tracking, per contract module: ported as `Standard` ✓,
   `Establish` wired ✓, real `Ensures<KaniVerifier>` ✓ / Creusot ✓ /
   Verus ✓. ~440 rows; the phases work it top-to-bottom (memory
   `feedback_pull_from_top_of_list`).

## Crate hierarchy

```text
amenable_time                     (interface + vocabulary + contracts + descriptors + Exchange traits)
  ├── depends on: amenable_core, amenable_derive, amenable_std (for provenance_vocab)
  └── NO dependency on any verifier backend

amenable_kani/src/time/…          (Witness<KaniVerifier>, Ensures<KaniVerifier>, Exchange proofs)
  └── depends on: amenable_time, amenable_kani(self)

amenable_creusot/src/time/…       (Creusot companions — generated)
amenable_verus/src/time/…         (Verus companions — generated; #[path]-included, no Cargo.toml)

amenable (facade)                 re-exports amenable_time
```

Mirrors `amenable_gaap`'s hierarchy decision exactly
(`GAAP_LEDGER_PLAN.md` "Crate hierarchy"). Producer crates
(`amenable_time_jiff` etc.) are **out of scope** — a follow-on, noted
below.

## Phased plan

Each phase commits per file / per contract module (memory
`feedback_commit_between_plan_steps`). "All three backends re-verified"
means `just verify-kani` sample + `just verify-creusot` + `just
verify-verus`, per the METADATA plan's own cadence.

### Phase 0 — skeleton + one module end-to-end

- [x] Create `crates/amenable_time` (`Cargo.toml`, `build.rs` declaring
      `cfg(kani)`, `lib.rs` = `mod` + `pub use`, `#![forbid(unsafe_code)]`,
      `#![warn(missing_docs)]`, README). Added to the workspace and the
      facade is left untouched for now (matches `amenable_gaap` — worked
      examples aren't facade-exported; revisit in Phase 7).
- [x] Port `error.rs` → house error patterns (owned `String` location,
      private fields + `derive_getters`, `derive_more::Display`/`Error`,
      `#[track_caller]`, `#[instrument]`, no `PartialEq`/`Eq` on the
      wrapper). `ParseRejected`'s profile is a `String` until
      `SerializationProfile` lands.
- [x] Build `amenable_time::provenance_vocab` — `NormativeDocument` /
      `NormativeSection` (string newtypes, `#[derive(Entry)]`),
      `NormativeStatus` / `StandardsBody` / `NormativeQuotation` (closed
      enums, `#[derive(Entry)]`), `CrossCheck` (struct). `NormativeQuotation`
      merges "what text" + "what status" into one enum. `NormativeClause`
      dropped. Reuses `SourceUrl` / `SemanticSummary` from `amenable_std`.
- [x] Build the `temporal_standard!` `macro_rules!` macro (crate-internal
      via `#[macro_use]`; `#[derive(Standard)]`-backed). `temporal_evidence!`
      deferred to Phase 3 (nothing needs it until `proof_composition`).
- [x] Port `contracts/precision.rs` (5 types) as the pattern-setter: 4
      tier-C ISO contracts (`ParaphraseOnly`), 1 tier-A RFC 3339 contract
      with the verbatim `time-secfrac` ABNF + deep link. 6 tests
      (`tests/precision_contracts_test.rs`) cover the Standard/Evidence
      wiring, the Metadata query surface, both tiers, cross-check
      projection, and `EvidenceLink` self-registration. `just
      check-all-package amenable_time` clean.
- [ ] Port `types.rs`'s `SerializationProfile`, `TemporalComponent`,
      `PrecisionDescriptor` (the minimal descriptor set `precision`
      needs) as `Evidence` payloads.
- [ ] Port `TemporalReporter` as a plain trait (no proofs — cheapest
      real trait to stand up).
- [ ] One `Exchange` edge end-to-end: pick the single simplest parser
      method (`parse_reduced_local_time` or similar), wire
      `RawInput` → `Proven` sidecars, an `ExchangeEdgeRecord`, and a
      real Kani `Witness<KaniVerifier>` proof for its output proposition.
- [ ] Generate the Creusot + Verus companions for that one edge; verify
      all three backends.
- [x] `docs/PLANNING_INDEX.md` entry.
- [ ] Coverage checklist scaffold.

Phase 0 exit: the full vertical slice works for one contract module and
one exchange. Everything after is width.

### Phase 1 — all citation-only contracts as `Standard`s

- [ ] `contracts/iso_8601.rs` (109) — commit per logical group
      (date / time / duration / week / ordinal / …).
- [ ] `contracts/iso_8601_2.rs` + `extended.rs` (48).
- [ ] `contracts/rfc3339.rs` (20).
- [ ] `contracts/rfc9557.rs` (38).
- [ ] `contracts/calconnect.rs` (61).
- [ ] `contracts/interval.rs` (25).
- [ ] `contracts/zone.rs` (21).
- [ ] `contracts/instant.rs` (6).
- [ ] `contracts/serialization.rs` (5).
- [ ] `contracts/conversion.rs` (7).
- [ ] cordial gate clean after each file; `just check-all amenable_time`.

Phase 1 exit: ~345 `Standard`s registered, each with a real
`TemporalProvenance`. No proofs yet. `just temporal-coverage` shows the
`Standard ✓` column full.

### Phase 2 — descriptors as `Evidence` payloads

- [ ] Port all 259 `types.rs` definitions. Structs already
      `#[derive(Builder)]`; add `#[derive(Evidence)] #[evidence(basis =
      "Self")]`. Closed enums: plain derives + `strum` per house policy.
- [ ] Port the `*Result` type aliases (rename to `amenable`'s
      `Sidecar`-shaped output types where they carry proofs; keep as
      plain `Result` aliases where they don't).
- [ ] `*ProofBranch` enums → keep as enums; they become
      `Sidecar::Proposition` discriminants in Phase 5.

### Phase 3 — the `ProvableFrom` graph → `Establish` + `Evidence`

- [ ] Port `proof_composition.rs`'s 93 `*Valid` / `*Evidence` types via
      `temporal_evidence!` — `#[derive(Evidence)]`, `EvidenceLink`.
- [ ] Port the ~98 `impl ProvableFrom<C> for P {}` lines as one
      `impl<V: Verifier> Establish<C, V> for P where Self: Witness<V>` per
      edge — generic over `V`, not per-backend (decision 3). `C` becomes
      a `#[derive(ProofToken)]` credential; `establish()` is the lawful
      mint path. Multi-credential `*Valid` types get one `Establish` impl
      per credential.
- [ ] Stand up `Witness<V>` for every `*Valid` / `*Evidence` prop in all
      three backend crates (`amenable_{kani,creusot,verus}::time`) —
      deliberately trivial / `#[trusted]`-shielded here, the honest
      "cited, not yet checked" representation. This is the cost decision 3
      accepts: `Establish<C, V>`'s `Self: Witness<V>` bound must be
      satisfiable for all `V` from Phase 3, not Phase 6.

### Phase 4 — trait methods → `Exchange`

Per trait, commit per method-group:

- [ ] `TemporalParser` (24 methods).
- [ ] `TemporalFormatter` (36).
- [ ] `TemporalZoneFactory` (5), `TemporalConversionFactory` (4),
      `TemporalIntervalFactory` (4).
- [ ] `TemporalCalConnectFactory` (19).
- [ ] `TemporalBackend` aggregate supertrait + its blanket impl.
- [ ] Each method → an `Exchange<In, Out, V>` impl + an
      `ExchangeEdgeRecord`. Methods with multiple proof sidecars get a
      composed `Out::Proposition`.

### Phase 5 — semantic bundles + native carriers

- [ ] The ~25 `*SemanticBundle` types → composed `Witness<V>` via the
      derive-witness composition machinery
      (`VERUS_DERIVE_WITNESS_COMPOSITION_PLAN.md`).
- [ ] `ProvenTemporalCarrier<T, S>` → a generic `Sidecar<V>`.
- [ ] The 16 `native_props.rs` associated-type families → the
      `amenable_time` backend trait's associated types (unchanged shape).
- [ ] The `realize_*` / `reflect_*` bridge method pairs → `Exchange`
      impls between descriptor and carrier sidecars.

### Phase 6 — real proofs for the genuinely-checkable contracts

Replace the trivial Phase 3 `Witness<V>` bodies with real proofs where a
checkable predicate exists. Not every contract has one; many are
structural ("uses a hyphen separator") and stay trivial-by-design — that
is the honest representation, not a gap (memory
`feedback_tautological_model_policy`). Target the ones that do,
single-source per the established design (Kani: `bool` predicate;
Creusot/Verus: shared `#[logic]` / `spec fn` via `harness!` / codegen —
memory `project_amenable_ensures_requires_contract_types`):

- [ ] Range contracts: `CalendarMonthInRangeOneToTwelve`,
      `HourInRangeZeroToTwentyFour`, `MinuteInRangeZeroToFiftyNine`,
      `SecondInRangeZeroToSixty`, `WeekNumberInRangeOneToFiftyThree`,
      `WeekdayInRangeOneToSeven`, `OrdinalDayInRangeOneToThreeHundredSixtySix`,
      `UtcOffsetHourInRangeZeroToTwentyThree`, `CenturyOrdinalInRangeZeroToNinetyNine`,
      `DecadeOrdinalInRange…`, `CalendarYearInRangeZeroToNine…`.
- [ ] Ordering contracts: `IntervalStartPrecedesEnd`,
      `IntervalEndpointsOrdered`, `IntervalDurationIsNonNegative`,
      `UtcTimelineOrderingAppliesToFixedInstants`.
- [ ] Arithmetic-rule contracts: `GregorianLeapYearUsesDivisibleByFourAndFourHundredException`,
      `LeapYearHasThreeHundredSixtySixCalendarDays`,
      `CommonYearHasThreeHundredSixtyFiveCalendarDays`,
      `CalendarDayWithinMonthBounds`, `LeapDayOccursOnlyInLeapYear`,
      `CentennialYearDivisibleByOneHundred`.
- [ ] The `proof_composition` aggregates whose leaves are now all proven
      → composed `Witness<V>` conjunctions (real, not tautological).
- [ ] Each real proof: injected-regression check per backend (memory
      `feedback_use_gallery_for_kani_investigations` — findings/timeouts
      go in `amenable_kani::gallery`).
- [ ] Structural contracts stay `Standard`-only with a documented
      `#[trusted]`-equivalent citation — that is the honest
      representation, not a gap (memory `feedback_tautological_model_policy`).

### Phase 7 — surfaces, certificates, one reference backend

- [ ] `TemporalReporter` → `Amenable::*_surface()` + `RegistryReport`.
- [ ] `just temporal-coverage` — the umbrella report: per contract,
      which backends prove it, which are citation-only.
- [ ] A minimal in-crate reference `TemporalBackend` impl (a test
      double, not a real datetime library) exercising every `Exchange`
      edge — the load test's actual assertion surface.
- [ ] `tests/` coverage (house rule: no inline `#[cfg(test)]`).
- [ ] README + module docs as user guide (memory
      `feedback_follow_claudemd_while_writing_not_after` — docs as we go,
      not a retro pass).

## Resolved decisions (2026-09-08 review)

1. **Vocabulary home — new module in `amenable_time`.** A fresh
   `amenable_time::provenance_vocab` for the normative-reference types
   (`NormativeDocument` / `NormativeSection` / `NormativeClause` /
   `NormativeStatus` / `StandardsBody` / `NormativeQuotation` /
   `QuotationStatus` / `CrossCheckList`), reusing only `Authority` /
   `SourceUrl` / `SemanticSummary` from `amenable_std::provenance_vocab`.
   `amenable_std`'s vocabulary is about carrier libraries; this one is
   about standards documents — different concerns, so no extension.
2. **Macro home — `macro_rules!` in `amenable_time`.** `temporal_standard!`
   / `temporal_evidence!` as declarative macros (like
   `amenable_std::string_vocab_entry!`), not proc-macros. The expansion
   is mechanical and a decl macro stays inspectable at the call site.
3. **`Establish`'s `V` — one impl, generic over `V`.** The `ProvableFrom`
   graph genuinely does not vary with the backend, so `amenable_time`
   carries a single `impl<V: Verifier> Establish<C, V> for P where Self:
   Witness<V>` per edge, not per-backend duplicates. The cost this
   imposes, accepted deliberately: **every temporal `Evidence`
   proposition needs a `Witness<V>` impl for all three backends from
   Phase 3 onward**, not just from Phase 6. Phase 3 stands those up as
   deliberately trivial / `#[trusted]`-shielded witnesses (the honest
   representation of "cited, not yet checked"); Phase 6 replaces the
   bodies of the genuinely-checkable ones with real proofs, leaving the
   structural ones trivial-by-design. `Establish<C, V>` never changes.
4. **The `Standard` / `Evidence` split — adopted.** Citation-only
   structural contracts → `Standard`; the `proof_composition` aggregates
   → `Evidence`. Named in `amenable_time`'s module docs.
5. **Descriptor invariants — none (follow GAAP).** `CalendarDateDescriptor`
   etc. stay validation-free; a `month = 13` is rejected by a *proof*
   (`CalendarMonthInRangeOneToTwelve`'s `Ensures<V>`), never by a
   constructor guard. The point is to prove, not to guard at runtime.
6. **Scale cutoff — full scope.** Phase 1 targets *all* ~345 citation-only
   contracts across every normative authority, not a subset. The
   ISO 8601-1 / RFC 3339 / RFC 9557 fault line is noted only as a natural
   commit-grouping boundary, not a stopping point.
7. **No checked-in standards corpus.** Unlike `elicit_temporal`'s
   ~4.5 MB `standards/` tree (including a `standards/licensed/` subtree of
   paywalled ISO extracts), `amenable_time` embeds only the specific
   normative clause in each contract's provenance metadata, governed by a
   three-tier redistributability rule (see "No checked-in source dump"
   above): RFC / EDTF / TZDB verbatim, CalConnect single-clause, ISO
   paraphrase-only. Stable deep link wherever one exists.
   Tier-C-never-`Verbatim` is enforced by a Phase 1 test/lint over the
   `EvidenceLink` registry, not by the macro (a decl macro can't inspect
   an `expr` against `body:`).

## Non-goals

- **Producer crates.** `amenable_time_jiff` / `_chrono` / `_time`
  wrapping real datetime libraries — a follow-on effort once the
  interface is proven under load. The Phase 7 reference backend is a
  test double only.
- **Runtime datetime arithmetic.** `amenable_time` is an interface +
  contract crate, like `elicit_temporal`. No calendar math
  implementation.
- **A checked-in standards corpus.** No `standards/` directory, no PDFs,
  no HTML/XML shells, and above all no paywalled-ISO extracts (decision
  7). Normative text lives embedded in provenance metadata, minimal and
  redistributability-governed, or as a stable URL.
- **New verifier capability.** This exercises the existing trait family
  and codegen; it does not add verifier features. If something in the
  family genuinely can't express a temporal pattern, that is a finding
  for a separate plan, not scope creep here (memory
  `feedback_no_dead_ends_talk_first`).
- **`elicitation` compatibility.** No attempt to keep `amenable_time`
  drop-in for `elicit_temporal`'s consumers (it has none).
