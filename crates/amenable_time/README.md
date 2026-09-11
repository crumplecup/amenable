# amenable_time

Temporal contract interface: standards-anchored propositions for date,
time, offset, and timestamp interchange, expressed in `amenable`'s trait
family. A full-scope port of `elicit_temporal`.

Every citation-only contract is an `amenable_core::Standard` carrying a
`TemporalProvenance` record that cites its normative source (ISO 8601-1 /
-2, RFC 3339, RFC 9557, CalConnect CC 18011, IANA TZDB, SI/BIPM, Library
of Congress EDTF). Composed propositions become `Evidence`; the
descriptor-factory trait methods become `Exchange`s.

## The shape

```text
Standard contracts            345 citation-only propositions
  (contracts::*)              "a calendar-date month is 01..12",
                              "an interval's first endpoint is no later
                               than its second", …

  ▼  fold into

Evidence composites           the proof_composition::*Valid aggregates —
  (proof_composition::*)      #[derive(Witness)] structural products of
                              their member contracts

  ▼  ride as the proposition of

Exchange methods              each Temporal*Factory / Temporal*Formatter /
  (traits::*, exchange::*)    Temporal*NativeBridge method is an
                              Exchange<In, Out, V> over a Sidecar pair —
                              a proven input, a proven output

  ▼  implemented by

a backend                     a concrete type providing the Exchange
  (elsewhere — see below)     impls + the Temporal*Props native carriers +
                              a TemporalReporter capability declaration
```

## What is machine-checked

23 of the 345 atomic contracts carry a real, machine-checked proof on
**all three** formal backends (Kani, Creusot, Verus) — the range,
ordering, and Gregorian-arithmetic ones (`CalendarMonthInRangeOneToTwelve`,
`IntervalStartPrecedesEnd`, `GregorianLeapYearUsesDivisibleByFourAndFourHundredException`,
`LeapDayOccursOnlyInLeapYear`, …). The month model and year model are
proven mutually consistent: `Σ days_in_month(y, 1..12) == days_in_year(y)`.

The other 322 are citation-only: they rest on their normative citation
(`amenable_time::structural_witness` gives each a `trusted`/`trivial`
`Witness<V>` so the composites can still compose). All 152
`proof_composition` aggregates resolve as a real `ClassifiedWitness<V>` on
every backend, with a support summary that reports the checked / trusted /
trivial breakdown honestly.

```sh
just temporal-coverage   # the live per-contract table
```

## Using it

**Reference a contract.** Import it from the crate root and read its
provenance:

```rust
use amenable_core::Standard;
use amenable_time::CalendarMonthInRangeOneToTwelve;
let cite = CalendarMonthInRangeOneToTwelve.provenance();
```

**Implement a backend.** Provide the `Exchange` impls for the method
family you support and the `Temporal*Props` native carrier types, against
a concrete `Verifier`. This crate carries no backend of its own —
backends live alongside the type registrations they're built from, in
whichever crate defines the concrete carrier types. `amenable_std::
std_time_backend` is the worked example: a `CanaryVerifier` that runs no
formal tool, `StdTimeBackend` covering the duration / instant /
endpoint-ordering slice `std::time` can honestly back. Its `Exchange`
bodies *execute* the contracts — `order_offset_endpoints` resolves both
endpoints through real Gregorian arithmetic and rejects a reversed
interval. Backends for calendar dates, zones, and ISO 8601 / RFC 3339 text
need a date-time library (`jiff`, `chrono`) and would live in
`amenable_ext` instead (an optional dependency on this crate, gated by
the `jiff`/`chrono` features — see `docs/AMENABLE_EXT_PLAN.md`).

**Report capabilities.** Implement `TemporalReporter`; call
`.capabilities()` for a `Display`-able declaration.

## No checked-in standards corpus

This crate reproduces **no** standards documents. The relevant normative
clause lives embedded in each contract's provenance metadata, governed by
a three-tier redistributability rule:

| Tier | Sources | What is embedded |
|---|---|---|
| A | RFC 3339 / RFC 9557, LoC EDTF, IANA TZDB | the specific clause, verbatim, plus a stable deep link |
| B | CalConnect CC 18011 / 18012 | the single relevant clause, verbatim |
| C | ISO 8601-1 / -2 / Amd 1 (paywalled) | our paraphrase and an exact section pointer only — no ISO prose |

`NormativeQuotation::ParaphraseOnly` marks a tier-C contract.

See `docs/AMENABLE_TIME_PLAN.md` for the full design and migration phases,
and `docs/AMENABLE_TIME_COVERAGE.md` for the coverage prose.
