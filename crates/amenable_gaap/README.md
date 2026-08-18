# amenable_gaap

> GAAP ledger worked example: a two-account double-entry `Transfer`,
> proven three independent ways — the `Stoplight`-succeeding worked
> example in the Exchange proof derivation lineage, chosen to exercise
> a genuinely non-trivial, branching invariant (`Stoplight` only ever
> proved `result.is_ok()`).

## What this crate is

The full trait-interface, domain-type, *and* transition-logic layer for
a lawful ledger transfer — `Pending -> Validated -> Committed`, with a
`reject()`/`rollback()` branch to `Rejected<T>` — mirroring
`~/repos/elicitation/crates/elicit_server::ledger`'s own typestate shape,
re-expressed in `amenable_core`'s vocabulary. Every type and every
method here is real and backend-neutral: `Ledger`, `Transfer<S, Token>`,
`TransferError`, and all six of `Ledger`'s own methods
(`validate`/`commit`/`reject`/`rollback`/`check_amount_positive`/
`check_sufficient_funds`) live in this one crate, generic over `V:
Verifier`. No per-backend duplicate of any of them exists anywhere in
the workspace — `amenable_kani`/`amenable_creusot`/`amenable_verus`
each attach their own proof to the real methods here (directly, for
Kani; via a generated companion reading the real body verbatim, for
Creusot/Verus), never a hand-copied second implementation.

See [`docs/GAAP_LEDGER_PLAN.md`](../../docs/GAAP_LEDGER_PLAN.md) for
the full design rationale and the step-by-step history of how this got
here (including the real dead ends and the Kani 0.67.0 DFCC bug that
initially looked like it would block this crate's own existence).

## A guided tour

Read the source in this order — it's also the order a new worked
example would be built in (see
[`amenable_derive`](../amenable_derive/README.md#onboarding-building-a-new-worked-example)'s
own onboarding walkthrough for the general recipe this crate follows):

1. **[`transfer.rs`](src/transfer.rs)** — `AccountId`, `Amount`,
   `TransferPayload` (the real data), and the four typestate markers
   (`Pending`/`Validated`/`Committed`/`Rejected<T>`), each a
   `#[derive(Standard)]` root claim — asserted by construction, not
   proven, the same way a `Stoplight` color is. `Rejected<T>` is
   parameterized by the state it was rejected *from* (`Rejected<
   Pending>`/`Rejected<Validated>` are genuinely distinct concrete
   types), not flat — see its own doc comment for the real `E0119`
   this avoids.
2. **[`contracts.rs`](src/contracts.rs)** — `AmountPositive`/
   `SufficientFunds`/`AccountsDistinct`/`BalancedEntries`: the four
   atomic claims `Ledger`'s own methods actually check, each a
   `#[amenable_derive::evidence]`-decorated root (provable, not
   asserted — the real split `#[derive(Standard)]` vs. `#[evidence]`
   exists for). `AccountingEquationHolds` is defined but not yet wired
   to any proof — a deferred, open question (`GAAP_LEDGER_PLAN.md`'s
   own "Open questions" section), not a bug.
3. **[`tokens.rs`](src/tokens.rs)** — `PendingToken`/`ValidatedToken`/
   `CommittedToken`/`RejectedFromPendingToken`/
   `RejectedFromValidatedToken`, each `#[derive(ProofToken)]` plus a
   verifier-less `#[amenable_derive::establish(..)]` — one real,
   backend-generic `impl<V: Verifier> Establish<C, V> for Y` per edge,
   gated only on whichever backend has actually registered a real
   `Witness<V>` proof for `Y`.
4. **[`ledger.rs`](src/ledger.rs)** — `Transfer<S, Token>` (a
   `#[derive(Sidecar)]` carrier), `TransferError`, `Ledger`, and all
   six real methods — the actual logic, each generic over `V`, each
   decorated with `#[amenable_derive::capture_exchange_body(..)]` so
   Creusot's/Verus's own codegen can read the real body, and each
   carrying its own real Kani contract directly (no delegating
   wrapper — see the module's own doc comment for why that's a hard
   requirement, not a style choice).

## The three backends

- **Kani** (`amenable_kani::gaap_ledger`) — a real `#[cfg_attr(kani,
  kani::requires/ensures(..))]` contract sits directly on each of
  `Ledger`'s six methods in *this* crate. `amenable_kani` itself holds
  only what's genuinely Kani-specific: each atomic contract's own
  `Ensures<KaniVerifier>` predicate (`kani_ensures!`), and the
  `Witness<KaniVerifier>` impls the removed
  `#[amenable_derive::exchange(..)]` used to generate for free. All six
  harnesses verify clean; see `amenable_kani::gallery::
  ledger_gaap_free_function_contract` for the full, four-case
  investigation into the DFCC bug this pattern works around.
- **Creusot** (`amenable_creusot::ledger`) — a real Cargo dependency on
  this crate (no accommodation-model mirror needed for `Pending`/
  `Validated`/`Committed`/`Rejected<T>` — `creusot-rustc` sweeps an
  ordinary dependency's own items fine, only *local* items risk the ICE
  classes documented in `amenable_creusot`'s own README). `Ledger`/
  `Transfer<S, Token>`/`TransferError` still need a hand-written mirror
  (their real constructors are deliberately private to this crate), fed
  by a real generated companion per method
  (`amenable_creusot/src/generated/*.rs`, regenerated with `just
  generate-creusot` from this crate's own `capture_exchange_body`
  registrations — never hand-edited). `cargo creusot prove -- -p
  amenable_creusot` reports every ledger goal proved.
- **Verus** (`amenable_verus::gallery::ledger_exchange`) — Verus can
  never resolve this crate at all (`verus --crate-type=lib` never reads
  `Cargo.toml`), so every type here is mirrored by hand, permanently —
  `Rejected<T>` included. Real, generated companions the same way
  Creusot's are (`just generate-verus-exchange`/`just
  generate-verus-gaap-tokens`). `verus --crate-type=lib
  crates/amenable_verus/src/lib.rs` verifies every ledger goal clean.

Every claim above has been reconfirmed non-vacuous with a real
injected-bug regression check at least once — see
`docs/GAAP_LEDGER_PLAN.md`'s Step 6/Step 7 for the specific bugs
injected and the precise failures each produced before being reverted.

## Status

All seven steps in the plan are done, on all three backends, including
the `reject`/`rollback` edges (deliberately deferred at first as a real
scope call, later revisited and closed). Nothing scoped in the plan
document is currently open; `AccountingEquationHolds` (above) is the
one named-but-unconnected type, tracked as an open question rather than
a gap.

## See also

- [Root README](../../README.md) for the project-wide overview.
- [`docs/GAAP_LEDGER_PLAN.md`](../../docs/GAAP_LEDGER_PLAN.md) for the
  full step-by-step history.
- [`amenable_derive`](../amenable_derive/README.md) for the macros this
  crate is built entirely out of, and an onboarding walkthrough for
  building the next worked example the same way.
- [`amenable_core`](../amenable_core/README.md) for the trait family
  itself (`Evidence`, `Standard`, `Sidecar`, `Establish`, `Witness`,
  `Ensures`, `Verifier`).
- [`amenable_kani`](../amenable_kani/README.md),
  [`amenable_creusot`](../amenable_creusot/README.md),
  [`amenable_verus`](../amenable_verus/README.md) for how each backend
  attaches its own proof to this crate's real types.
