# amenable_time

Temporal contract interface: standards-anchored propositions for date,
time, offset, and timestamp interchange, expressed in `amenable`'s trait
family.

Every citation-only contract is an `amenable_core::Standard` carrying a
`TemporalProvenance` record that cites its normative source (ISO 8601-1 /
-2, RFC 3339, RFC 9557, CalConnect CC 18011, IANA TZDB, SI/BIPM, Library
of Congress EDTF). Composed / provable propositions become `Evidence`;
the descriptor-factory trait methods become `Exchange`s.

Ported from `elicit_temporal`. See `docs/AMENABLE_TIME_PLAN.md` for the
full design and migration phases.

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

## Status

Under construction — plan Phase 0 (skeleton + `contracts::precision`).
