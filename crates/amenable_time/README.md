# amenable_time

> Standards-anchored temporal contracts — every date, time, zone, and
> duration rule that date-time code quietly assumes, turned into a
> named, citable type.

## Why this exists

Date and time handling is where a disproportionate share of real bugs
live, precisely because the rules involved are both numerous and
scattered across documents nobody re-reads before writing code: a leap
year follows a divisible-by-4 rule with a divisible-by-100 exception
that itself has a divisible-by-400 exception; an RFC 3339 fractional
second uses a dot, never a comma; a named time zone identifier can't
contain a `.`-segment; an interval's second endpoint must not precede
its first. Every date-time library enforces rules like these somewhere
in its source — but the rule itself, the actual normative sentence
being implemented, rarely survives as anything a caller can see, cite,
or check. It gets absorbed into the implementation and forgotten.

`amenable_time` inverts that. Each rule becomes its own named Rust
type, carrying the standard and section it comes from (and, where
redistribution permits, the verbatim clause). Code that depends on "a
calendar month is `1..=12`" names `CalendarMonthInRangeOneToTwelve`
instead of writing a bare range check — and that name is now something
you can look up, reference from a type signature, and, for the rules
that are genuinely computable properties rather than textual
convention, back with a real machine-checked proof.

## A contract, up close

```rust
use amenable_core::Standard;
use amenable_time::CalendarMonthInRangeOneToTwelve;

let facts = CalendarMonthInRangeOneToTwelve.provenance();
println!("{} §{}", facts.document(), facts.section());
// ISO/WD 8601-1:2016(E) §3.2.1, 4.1.2.1
```

That's the whole contract: a zero-sized type, and a provenance record
you can query instead of a comment you have to go find. All 345 of
this crate's contracts are declared the same compact way, through one
internal macro — a `document / section / summary / citation` block per
rule, not generated code you'd need to go hunting through.

## From rule to running code

A single rule like "a month is `1..=12`" rarely stands alone — it
combines with dozens of siblings into something like "this is a valid
calendar date," and *that* combines further into "parsing this string
yields a valid calendar date." `amenable_time` models that layering
directly, through four stages:

```text
Standard contracts     345 atomic, citation-grounded propositions —
                        "a month is 1..=12", "an interval's start
                        precedes its end", …

  ▼ fold into

Evidence composites    structural products of their member contracts —
                        "this is a valid CalendarDate" is the Evidence
                        folded over every rule a calendar date must
                        satisfy, derived rather than hand-assembled

  ▼ ride as the proposition of

Exchange methods       each parse / format / native-bridge operation
                        is an Exchange<In, Out, V>: hand it unproven
                        input, get back a value that carries proof the
                        composite rule holds — not just a result

  ▼ implemented by

a backend               a concrete type's Exchange impls, its native
                        carrier types, and a capability declaration
```

This crate stops at the trait and contract layer, deliberately — it
stays dependency-light so more than one backend can depend on it
without any one date-time library being dragged in by default. The
real implementation, where these traits meet actual calendar
arithmetic, lives in [`amenable_ext`](../amenable_ext/README.md): its
jiff-backed `JiffTimeBackend` is the place to start if you want
working code rather than the contract layer on its own.

## What's actually proven

Most of the 345 contracts describe a lexical or structural convention
— "a duration uses the `P` designator," "an offset's sign encodes
direction relative to UTC" — the kind of rule a given piece of text
either follows or doesn't, with no further computation to verify
beyond checking it. Those rest on their citation alone, each wearing a
`trusted`/`trivial` witness so composites can still fold over them
honestly without pretending they're something they're not.

A smaller set state an actual computable property, and those get a
real proof: 23 of the 345 contracts carry a machine-checked proof on
**all three** formal backends (Kani, Creusot, Verus) — calendar-month
range, interval ordering, the Gregorian leap-year rule with its
divisible-by-400 exception, the fact that a leap day can only occur in
a leap year. The month model and year model are checked mutually
consistent — `Σ days_in_month(y, 1..=12) == days_in_year(y)` — proven,
not assumed.

Composites are honest about the mix, not just the leaves: all 152
`proof_composition` aggregates resolve as a real classified witness on
every backend, and each one's support summary reports exactly how many
of its member contracts are checked versus cited. See the live table
yourself:

```sh
just temporal-coverage
```

## Citing standards without reproducing them

This crate ships **no standards text**. What it carries is a
structured citation per contract — document, section, and (where
licensing allows) the verbatim clause — governed by a three-tier rule:

| Tier | Sources | What's embedded |
|---|---|---|
| A | RFC 3339 / RFC 9557, Library of Congress EDTF, IANA TZDB | the clause, verbatim, plus a stable deep link |
| B | CalConnect CC 18011 / 18012 | the one relevant clause, verbatim |
| C | ISO 8601-1 / -2 (paywalled) | our paraphrase and an exact section pointer — no ISO prose |

A tier-C contract's citation is `ParaphraseOnly`, visible directly on
the type through the same `provenance()` call shown above.

## Using it

On its own, this crate gets you the *contracts*: look one up, read its
citation, fold your own composite over a set of them. For something
that actually parses, formats, and manipulates real temporal values
under proof, go to [`amenable_ext`](../amenable_ext/README.md) — its
jiff-backed backend implements the majority of this crate's trait
surface against real calendar arithmetic, with working code and tests
you can read.

Implementing a different backend means providing this trait family's
`Exchange` impls, plus the native carrier types each `Temporal*Props`
trait names, for a concrete verifier. Implement `TemporalReporter` to
declare what your backend actually supports — leap seconds, named-zone
round trips, a current TZDB revision — so callers can query
capabilities instead of guessing them.

## See also

- [Root README](../../README.md) for the project-wide overview.
- [`amenable_ext`](../amenable_ext/README.md) for the jiff-backed
  implementation of this crate's trait surface.
- [`amenable_core`](../amenable_core/README.md) for the underlying
  `Standard` / `Evidence` / `Witness` / `Exchange` trait family this
  crate is built from.
