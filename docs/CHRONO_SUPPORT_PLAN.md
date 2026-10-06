# Full chrono support in `amenable_ext`

**Status:** Decisions complete. Implementation not started. Last revised
2026-10-06.

This plan adds full coverage of the chrono 0.4 and chrono-tz 0.10 public
surfaces to `amenable_ext`. It mirrors the jiff effort: each type is registered
with evidence, witnessed on Kani, Creusot, and Verus, and tested through a
proof-chain test. The coverage tool is `cordial coverage`.

Each phase is one unit of work with a gate at its end. Phases run in order,
except where a phase names an independent prerequisite.

## Scope

The 67 rows cover chrono's types. amenable_time's trait surface is larger: the
parser, formatter, and factory traits sit beside the type rows. **Phase 12 brings
them in scope,** after the type rows, so the chrono backend matches the jiff
backend's full surface. Phase 12 adds Exchange impls, not checklist rows.

## Counts

Inventory types (cordial, all features on): **59 distinct**. chrono has 55 rows,
chrono-tz has 5, and `NaiveDateTime` is shared, so it counts once.

Checklist rows: **67** = 59 types + 8 instantiation rows for `DateTime` and
`Date`. The `DateTime` and `Date` rows are aggregates, Complete when their eight
instantiation rows are.

| Phase | Work | Inventory types | Checklist rows |
| --- | --- | ---: | ---: |
| 0 | Wiring | — | — |
| 1 | Registry schema for generic claims | — | — |
| 2 | Value types and civil bridge | 7 | 7 |
| 3 | Offset and zone bridges, zoned types | 7 | 15 |
| 4 | Duration bridge and arithmetic types | 8 | 8 |
| 5 | Enums and aliases | 6 | 6 |
| 6 | Parsing errors | 4 | 4 |
| 7 | `format` module | 12 | 12 |
| 8 | `naive` iterators | 2 | 2 |
| 9 | rkyv archived types, per size | 12 | 12 |
| 10 | Foreign re-export (`Locale`) | 1 | 1 |
| 11 | Final gate | — | — |
| 12 | Parsers, formatters, factories | — | not counted (Exchange impls) |
| **Total** | | **59** | **67** |

Phase 3 covers 7 inventory types: `Local`, `DateTime`, `Date`, `chrono_tz::Tz`,
`TzOffset`, `GapInfo`, and `chrono_tz::ParseError`. Its 8 instantiation rows bring
it to 15.

## Decisions

1. **Complete means complete.** A row is Complete when every item in its
   done-definition holds. Complete is not a proof-strength metric. Every trusted
   witness is labeled with its reason.
2. **Four kinds of claim, per case.**
   - *Proven:* the witness calls the real public API on a domain the backend can
     model. This is the default.
   - *Trusted by construction:* the only path needs a non-public constructor, or
     the behavior is an external side effect the backend cannot observe. Example:
     `jiff::fmt::DefmtWrite`. Still Complete, with the trust stated.
   - *Boundary:* a stated limit on what a claim covers, with approved wording on
     the row. Example: `Local`'s host-supplied offset rules.
   - *Exception:* a cordial exception entry for a gap that is not a stated limit.
     Recorded only after explicit user approval.
3. **Bounds are the source of truth.** Each type's required contracts come from the
   amenable_time bridge its bound names. Nothing is chosen per type.
   - **Civil types** bound on `TemporalCivilNativeBridge<V>`, which requires
     `TemporalCivilProps`.
   - **Offsets** (`Utc`, `FixedOffset`, `Local`) bound on
     `TemporalInstantNativeBridge<V>`, with `OffsetDateTime` as the carrier. Its
     bundle is `OffsetDateTimeSemanticBundle`. `UtcOffset` has no bridge and no
     bundle, so it cannot carry a bound.
   - **Named zones** (`chrono_tz::Tz`) bound on `TemporalZoneNativeBridge<V>`, with
     `NamedTimeZone` as the carrier.
   - **Durations** bound on `TemporalDurationNativeBridge<V>`, which requires
     `TemporalDurationProps`.
4. **`chrono_tz::Tz` is one concrete row.** It is an enum of about 600 zones, so one
   row covers it, with claims over the enum. Its `DateTime` and `Date` instantiations
   are separate rows (decision 6).
5. **Kani uses per-instantiation witness types only.** Kani cannot express generics.
   Each concrete instantiation gets its own witness type, and a generic claim's Kani
   coverage is the set of those types.
6. **Eight instantiation rows:** `DateTime<T>` and `Date<T>` for `T` in {`Utc`,
   `FixedOffset`, `Local`, `chrono_tz::Tz`}. The generic claim is proven once over the
   bound. It covers any type that satisfies the bound, including user-defined types.
   `Date` rows depend on `ZonedDateValid` (decision 11). `Date` is deprecated since
   chrono 0.4.23 and stays in scope as public API.
7. **The `case-insensitive` feature stays on.** It means
   `NamedTimeZoneIdentifierIsCaseSensitive` is not guaranteed for `chrono_tz::Tz` on
   that build. The row reads "not satisfied: boundary under `case-insensitive`."
8. **rkyv layout claims are proven per size, all three** (`rkyv-16`, `rkyv-32`,
   `rkyv-64`). Value claims are size-independent and stated once. Layout claims
   (serialized length or bytes, relative-pointer offsets, `size_of`, archived
   `usize`/`isize`) are cfg-gated per size, and each proof name records its size. The
   size features are mutually exclusive, so each size gets its own build and proof run.
9. **All features are on for coverage.** `chrono`, `chrono-tz`, and `jiff` enable
   every optional upstream feature, except jiff's `arbitrary`. It cannot be enabled at
   jiff 0.2.37, because its `jiff-core` dependency lacks the feature.
10. **Premises follow the Exchange pattern.** Each premise gets a proof and a test,
    like any other contract. Premises are amenable_time contracts, not statements
    invented for this plan.
11. **`ZonedDateValid` is added to amenable_time.** It joins `DateValid` and
    `NamedTimeZoneIdentityValid`. Its one stated limit is the offset stored with the
    date, which no contract covers.
12. **`Local` is an offset.** It is bounded on the instant bridge, with the host-rules
    boundary. `NamedTimeZoneUsesIanaIdentifier` does not apply, since `Local` has no
    IANA name.
13. **Boundary wordings.**
    - `Local`: "`Local` takes its offset rules from the host's timezone database, not
      from a fixed zone. Our claims hold for the rules the host supplies. Where the
      host's rules are wrong or absent, our claims don't apply."
    - `case-insensitive`: "Under `case-insensitive`, chrono-tz accepts identifiers
      that differ from the IANA name only in case. The case-sensitivity contract is
      not guaranteed for `chrono_tz::Tz` on this build."
    Whether the `case-insensitive` boundary also needs a cordial exception entry is
    not yet decided.
14. **`pure_rust_locales::Locale` counts under chrono.** It is re-exported under
    `unstable-locales` and goes in chrono's inventory.

## Done-definition (every row)

A row is Complete when all of these hold:
- [ ] Registered in `amenable_ext` under its feature, with `impl_ext_type!` and
      `register_ext_standard_evidence!`, or the generic macro for generic claims.
- [ ] A witness on Kani (per-instantiation type), Creusot, and Verus. Each is proven
      or trusted, and labeled.
- [ ] Every required contract from the bridge its bound names has a proof and a test
      on each backend that can model it. Applies to bridge-backed rows.
- [ ] A proof-chain test in `crates/amenable/tests/proof_chain_test.rs`, so cordial's
      `proof_test` column is true.
- [ ] Any boundary wording or exception is approved and recorded.
- [ ] `cordial coverage` shows the row Complete.

## Working rules

- Propose any exception to the user with its exact axiom and reason. Do not record
  one until it is approved.
- Run `just check-all` before each commit.
- One commit per phase, on `dev`.
- Run heavy cargo commands one at a time, never in parallel.
- Before a phase starts, read the bridge trait it depends on. Count its Exchanges and
  witness bounds from source, and record the count in that phase's notes. Do not
  estimate from memory.

---

## Phase 0: Wiring (done)

Commits `df482c0d` and `bd76fb70`. chrono, chrono-tz, and jiff features enable all
upstream features, and `cordial.toml` declares the three targets. Remaining item
belongs to cordial: the shared dump's feature set must include `chrono` and
`chrono-tz` (see cordial requests).

## Phase 1: Registry schema for generic claims

**Why first:** cordial cannot tell a generic claim from a concrete one until the
dump carries `bounds` and `premises`. The cordial request closes at the end of this
phase, for the generic half.

**Files and changes:**
1. `crates/amenable_core/src/link.rs`: `EvidenceLink` gains `bounds:
   &'static [&'static str]` and `premises: &'static [Premise]`.
   - `EvidenceLink::new(name, basis, index)` keeps its signature and sets both to
     empty, so existing callers do not change.
   - Add `EvidenceLink::generic(name, basis, index, bounds, premises)`.
2. `crates/amenable_core`: new `Premise { id: &'static str, statement: &'static str }`.
3. `crates/amenable/src/registry_dump.rs`: `EvidenceLinkDump` gains `bounds:
   Vec<String>` and `premises: Vec<PremiseDump>`. Concrete links serialize both as
   empty.
4. `crates/amenable_ext/src/macros.rs`: `register_ext_generic_evidence!(ty, bounds =
   [...], premises = [...])`. It emits a link named `amenable_ext::ExtGeneric<ty>`.
   The wrapper differs from `ExtStandard<T>`, so a generic name can never equal a
   concrete one.

**Verify before starting:** count the call sites of `EvidenceLink::new` across the
workspace, including the derive macros in `amenable_derive`, so the signature stays
stable.

**Tests:** a dump round-trip test covering one concrete link and one generic link.
Existing dump tests must still pass unchanged.

**Gate:** cordial reads `bounds` and `premises` from a regenerated dump.

## Phase 2: Value types and civil bridge (7 rows)

**Types:** `NaiveDate`, `NaiveTime`, `NaiveDateTime`, `IsoWeek`, `NaiveWeek`,
`Utc`, `FixedOffset`. `NaiveDateTime` is also chrono-tz's type, and it counts once.

**Work:**
1. **Civil bridge** (`TemporalCivilNativeBridge<V>`, the subject is the chrono
   backend). Its props trait `TemporalCivilProps` names these associated types:
   - `CalendarDate` → `NaiveDate`
   - `LocalTime` → `NaiveTime`
   - `LocalDateTime` → `NaiveDateTime`
   - `WeekDate` → `IsoWeek`
   - `OrdinalDate` → `NaiveDate`, reached through `from_yo` and `ordinal`, the same
     route jiff uses. Verify this in chrono's source first.
   - `ReducedCalendarDate` and `ReducedLocalTime` have no chrono type. Use thin
     wrapper types, the same approach the jiff backend used.
2. **Props impl:** `TemporalCivilProps` on the chrono backend type.
3. **Exchanges and witnesses:** the bridge's realize and reflect Exchanges and its
   semantic-bundle witnesses, per verifier. Count these from the trait source before
   writing anything (see working rules).
4. **Registrations:** `impl_ext_type!` and `register_ext_standard_evidence!` for
   the seven types, under the `chrono` feature.
5. **Witnesses per type:** Kani, Creusot, and Verus, following the jiff witness
   files.
6. **Proof-chain tests:** one per type, in `proof_chain_test.rs`.
7. **`Utc` and `FixedOffset`** are registered as values here. Their bridges are
   Phase 3.

**Gate:** the seven rows are Complete in `cordial coverage`. The civil bridge tests
pass on the chrono backend for every civil carrier.

## Phase 3: Offset and zone bridges, zoned types (15 rows)

**Types and bridges:**
- **Instant bridge** (`TemporalInstantNativeBridge<V>`, carrier `OffsetDateTime`)
  for `Utc`, `FixedOffset`, and `Local`. `TemporalInstantProps` names `UtcOffset`,
  `OffsetDateTime`, and `Instant`. Map `Instant` to `chrono::DateTime<Utc>`, and
  `OffsetDateTime` to `chrono::DateTime<FixedOffset>`. Verify both mappings before
  writing the impls.
- **Zone bridge** (`TemporalZoneNativeBridge<V>`, carrier `NamedTimeZone`) for
  `chrono_tz::Tz`. Its `ZonedDateTime` carrier is `DateTime<Tz>`.
- **`Local`** uses the instant bridge, with the host-rules boundary (decision 12).

**Work:**
1. Props impls: `TemporalInstantProps` and `TemporalZoneProps`, on the chrono backend
   type.
2. Instant bridge Exchanges: 2 per verifier, plus 1 witness per verifier. Count from
   source first.
3. Zone bridge Exchanges: 4 per verifier, plus 2 witnesses per verifier. Count from
   source first.
4. **`ZonedDateValid`** (decision 11), in amenable_time:
   - New file `crates/amenable_time/src/proof_composition/composites/zone/zoned_date.rs`.
   - Fields `date: DateValid` and `zone: NamedTimeZoneIdentityValid`. Derives match
     `identity.rs`.
   - Doc comment states the single limit: the offset stored with the date is not
     covered. No contract says it matches the zone's rules for that date.
     `OffsetConsistentWithNamedZone` is about a fixed instant, which a date does not
     denote.
   - Export from `zone/mod.rs`, and from the crate root in `lib.rs`. Both `DateValid`
     and `NamedTimeZoneIdentityValid` are already re-exported at the root.
5. **Registrations:**
   - `DateTime<T>` and `Date<T>` for the four zone types, eight in all, as instantiation
     rows (decision 6).
   - The generic claims as `ExtGeneric<DateTime<Tz>>` and `ExtGeneric<Date<Tz>>`,
     using the Phase 1 macro. Each lists its bound and premises.
6. **chrono-tz rows:** `Tz` (zone bridge), `TzOffset`, `GapInfo`, and
   `chrono_tz::ParseError`. Decide each row's claim kind from its role before writing
   its witness. This is not settled yet, so read each type's source before Phase 3
   starts.
7. **Contract table:** rebuild `scratchpad/zone-contract-table.md` for the offset
   bridge and the zone bridge, derived from the bounds. The current version
   treats `Local` as a named zone and must not be used.

**chrono facts (verified in source):**
- `Date<Tz>` stores a `NaiveDate` and a `Tz::Offset` fixed at construction
  (`chrono-0.4.45/src/date.rs:58`).
- `and_time` resolves a `DateTime` through `from_local_datetime(...).single()`, so
  ambiguity and gap cases occur there, not in `Date`.
- `chrono-tz`'s `case-insensitive` feature adds the `uncased` crate to its identifier
  map (`chrono-tz-0.10.4/Cargo.toml`).

**Gate:** the 15 rows are Complete. `Date` rows depend on `ZonedDateValid` existing.

## Phase 4: Duration bridge and arithmetic types (8 rows)

**Types:** `TimeDelta` (also `Duration`, its alias), `Days`, `Months`,
`WeekdaySet`, `RoundingError`, `OutOfRange`, `OutOfRangeError`.

**Work:**
1. **Duration bridge** (`TemporalDurationNativeBridge<V>`). Props: `TemporalDurationProps`,
   with `Duration` as the carrier. Map it to `TimeDelta`. Count Exchanges from source.
2. **`TimeDelta`** is the only type with a carrier in this phase.
3. **The other seven** have no carrier in the Props traits. For each, decide the
   claim kind (proven, trusted, or boundary) from its role. That decision is made at
   the start of this phase and recorded in its notes. Any boundary needs approved
   wording before it is recorded.

**Gate:** the 8 rows are Complete, and each non-carrier row's claim kind is recorded.

## Phase 5: Enums and aliases (6 rows)

**Types:** `Month`, `Weekday`, `SecondsFormat`, `MappedLocalTime`,
`offset::LocalResult`, `ParseResult`.

**Work:**
1. None of these has a carrier in the Props traits. Each is a plain enum or alias.
2. For each, record its claim kind. `MappedLocalTime` and `LocalResult` describe the
   zero, one, or two mappings of a local time, which is the same ambiguity and gap
   behavior the zone contracts describe. Their witnesses should reference those
   contracts (`ZoneTransitionAmbiguityDeclared` and `ZoneTransitionGapDeclared`),
   not be trusted by fiat.
3. `Month` and `Weekday` relate to the civil carriers and can reference them. Confirm
   this from chrono's conversions before writing the witnesses.

**Gate:** the 6 rows are Complete.

## Phase 6: Parsing errors (4 rows)

**Types:** `chrono::ParseError`, `ParseMonthError`, `ParseWeekdayError`,
`format::ParseErrorKind`. Note that `chrono_tz::ParseError` is a separate type,
handled in Phase 3.

**Work:**
1. Parse errors map to amenable_time's `TemporalError` family, where that family
   already has an equivalent. Check for one before writing a witness.
2. Where no equivalent exists, record the claim kind per row.

**Gate:** the 4 rows are Complete.

## Phase 7: `format` module (12 rows)

**Types:** `Colons`, `DelayedFormat`, `Fixed`, `InternalFixed`, `InternalNumeric`,
`Item`, `Numeric`, `OffsetFormat`, `OffsetPrecision`, `Pad`, `Parsed`, `StrftimeItems`.

**Work:**
1. These are chrono's formatting machinery. They are the chrono side of amenable_time's
   `TemporalFormatter` and `TemporalParser` traits, which Phase 12 implements.
2. Order: Phase 12's parser and formatter Exchanges for these descriptors come first.
   Phase 7's rows then reference those Exchanges as their witnesses, so Phase 7 cannot
   close before the matching Phase 12 work.
3. **Gate:** the 12 rows are Complete, with witnesses that reference the Phase 12
   Exchanges.

## Phase 8: `naive` iterators (2 rows)

**Types:** `NaiveDateDaysIterator`, `NaiveDateWeeksIterator`.

**Work:** no carrier. Record the claim kind per row, and reference the civil
contracts (`CalendarDateValid`) where the iterators yield calendar dates.

**Gate:** the 2 rows are Complete.

## Phase 9: rkyv archived types, per size (12 rows)

**Types:** `rkyv::ArchivedDateTime`, `ArchivedDuration`, `ArchivedFixedOffset`,
`ArchivedIsoWeek`, `ArchivedLocal`, `ArchivedMonth`, `ArchivedNaiveDate`,
`ArchivedNaiveDateTime`, `ArchivedNaiveTime`, `ArchivedTimeDelta`, `ArchivedUtc`,
`ArchivedWeekday`.

**Work:**
1. Add per-size features in `amenable_ext`: `rkyv-16`, `rkyv-32`, and `rkyv-64`. They
   cannot be enabled together. Each is a separate build target.
2. Register value claims once. Their names are the same for all three sizes.
3. Register layout claims per size, cfg-gated, with the size in the proof name.
4. Run each size's proofs and record the results per size, so a Complete names its
   size.
5. Confirm, from `rkyv` 0.7.46's `pick_size_type!` macro, which archived integer types
   each size changes. Those are the types whose layout claims need per-size gates.

**Gate:** the 12 rows are Complete for every size. `check-features` covers the
per-size features, or a per-size CI job exists.

## Phase 10: Foreign re-export, `Locale` (1 row)

**Type:** `pure_rust_locales::Locale`, re-exported by chrono under `unstable-locales`.

**Work:** register it under chrono, with its owning crate recorded in the evidence.
Decision 14 settles that it counts.

**Gate:** the row is Complete.

## Phase 11: Final gate

- `cordial coverage` shows 59 types and 67 rows Complete.
- `cordial quality --deny-open` reports zero.
- `just check-all` and `just cordial-gate` pass.
- Every boundary and approved exception is listed, with its reason, in the checklist.

## Phase 12: Parsers, formatters, and factories

Adds Exchange impls to the chrono backend. It adds no checklist rows.

**Traits and edges (measured from `crates/amenable_time/src/traits/`):**

| Trait | File | Distinct Exchange edges | Notes |
| --- | --- | ---: | --- |
| `TemporalParser` | `parse.rs` | 25 | One pair appears as `ParsedX`, a likely grep artifact. Verify at start. |
| `TemporalFormatter` | `format.rs` | 31 | Basic and extended forms are separate edges. |
| `TemporalIntervalFactory` | `interval.rs` | 4 | |
| `TemporalZoneFactory` | `zone.rs` | 4 | |
| `TemporalConversionFactory` | `conversion.rs` | 3 | |
| `TemporalCalConnectFactory` | `calconnect.rs` | 17 | Out of scope, see below. |
| `TemporalNativeConversionFactory`, `TemporalNativeZoneFactory`, `TemporalNativeIntervalFactory`, `TemporalNativeDateTimeFormulaFactory` | `native_factory.rs` | not measured | The regex missed these. Count from source at start. |
| `TemporalReporter` | `report.rs` | 0 | Capability queries, not Exchanges. Implement directly. |

The earlier figures (24 parser edges, 36 formatter edges, 19 CalConnect) came
from the jiff plan and do not match the source. The table above replaces them.

**Which edges chrono can back.** chrono implements ISO 8601-1 and RFC 3339 forms,
plus strftime-style parsing and formatting. It has no ISO 8601-2 or CalConnect
forms. Expected split, to confirm per edge at the start of the phase:
- *In scope, chrono has an API:* calendar date, ordinal date (`%j`), week date
  (`%G-W%V-%u`), local time, local date-time, UTC offset, offset date-time, RFC 3339
  timestamp, and the time interval forms that compose those.
- *Out of scope, no chrono form:* qualified temporal values, seasonal expressions,
  sub-year groupings, unspecified components, temporal sets, date-time formulas,
  date and time with shift, extended years, decades, centuries, grouped time-scale
  units, selection expressions, repeat rules, and recurring intervals with repeat
  rules. These are ISO 8601-2 and CalConnect constructs. The jiff plan excluded the
  same set, and this plan follows that precedent.
- *CalConnect factory (17 edges):* out of scope for the same reason.

Out-of-scope edges are recorded as permanently out of scope, with the reason, in
the plan's checklist. They are not left as "future work." Adding any of them later
would need a new decision.

**Work, per in-scope edge:**
1. Map the edge to a chrono API. Candidates: `DateTime::parse_from_rfc3339`,
   `NaiveDate::parse_from_str` with `%Y-%m-%d`, `%j` for ordinal, `%G-W%V-%u` for week
   date, `NaiveTime::parse_from_str`, and `to_rfc3339` for formatting. Verify each
   against chrono 0.4.45 before writing its impl.
2. Write the parser Exchange, and the formatter Exchange, for each verifier: three
   verifiers, so three impls per edge per direction.
3. Where chrono's API cannot produce the exact descriptor form, record the claim kind
   (trusted or boundary) with its reason, before writing the witness.
4. Each Exchange's witness comes from the contract its descriptor names in
   `contracts/`. Do not trust by fiat where a contract exists.

**Work, per factory:** implement each factory trait's Exchanges for the chrono
backend, following the jiff backend's zone factory and conversion factory as the
model. Native factories are counted from source at the start.

**Work, reporter:** implement `TemporalReporter` directly. Its capability values
(maximum fractional digits, leap-second support, IANA revision, and similar) come
from chrono's documented behavior and are checked by test.

**Gate:** every in-scope edge has a Exchange impl on all three verifiers, with its
witness and a test. Out-of-scope edges are listed in the plan with their reasons.

## Decisions carried into Phase 12

- Parser, formatter, and factory traits: **in scope**, per the user.
- CalConnect and ISO 8601-2 edges: **out of scope**, with reasons, following the jiff
  precedent. This is a precedent, not a new user decision. Confirm if you want it
  revisited.

## Cordial requests (owned by the cordial agent)

1. Report the eight instantiation rows as checklist rows, with the `DateTime` and
   `Date` rows as aggregates derived from them. Needed at the end of Phase 3.
2. Read `bounds` and `premises` from the registry dump. Needed at the end of
   Phase 1, which closes the generic half of the request.
3. Confirm the shared dump's feature set includes `chrono` and `chrono-tz`.
4. Record per-size rkyv results, so a Complete names its size. Needed in Phase 9.
5. Normalize whitespace in evidence names before matching.

## When the cordial request closes

- **Per-instantiation rows:** closed at the end of Phase 3.
- **Generic claims readable from the dump:** closed at the end of Phase 1.

## Exit criteria

67 checklist rows Complete. Every trusted witness, boundary, and approved exception
is recorded with its reason. Per-size rkyv results are recorded. Phase 12's in-scope
edges each have a witness and a test on all three verifiers, and its out-of-scope
edges are listed with their reasons.
