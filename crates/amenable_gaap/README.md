# amenable_gaap

> GAAP ledger worked example: the `Stoplight`-succeeding worked example
> in the Exchange proof derivation lineage, chosen to exercise a
> genuinely non-trivial, branching invariant (`Stoplight` only ever
> proved `result.is_ok()`).

## What this crate is

The trait-interface and domain-type layer for a two-account
double-entry `Transfer`, mirroring
`~/repos/elicitation/crates/elicit_server::ledger`'s own typestate shape
re-expressed in `amenable_core`'s vocabulary. Crate-hierarchy position
mirrors `amenable_std`'s own real, asymmetric shape: Creusot-side
content will live inside this crate behind a `creusot` feature; the
real Verus proof source will live in `amenable_verus::gallery::ledger`;
Kani-side proofs will live in a new `amenable_kani::ledger` module that
depends on this crate, not the reverse (Rust's orphan rules put
`Witness<KaniVerifier>` impls wherever `KaniVerifier` itself lives).

See [`docs/GAAP_LEDGER_PLAN.md`](../../docs/GAAP_LEDGER_PLAN.md) for
the full design rationale, scope, and step-by-step plan.

## Status

Step 0 only: the type-level skeleton (`TransferPayload`, the four
typestate markers, and the five real contract types) compiles with no
proofs behind it yet. See the plan document for what each later step
adds.
