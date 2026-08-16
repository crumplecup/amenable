# GAAP Ledger Plan

## Status

🔲 Planning — Step 0 done. Crate hierarchy, scope, and initial state
shape are decided; the type-level skeleton compiles and is tested.
Steps 1 onward (the first real proof) not started.

## Motivation

`EXCHANGE_PROOF_DERIVATION_PLAN.md`/`VERUS_EXCHANGE_PROOF_DERIVATION_PLAN.md`'s
`Stoplight` worked example is now fully codegen-driven across all three
verifier backends, but every predicate proven so far is trivial
(`result.is_ok()`). The real remaining test of that design — a genuinely
non-trivial, branching `ensures`/`requires` clause — has never been
exercised on any backend. This plan is that exercise.

The chosen domain is double-entry bookkeeping: familiar enough that
nobody argues about the domain semantics (unlike a unit-conversion
example, which invites overflow/rounding bikeshedding instead of
engaging with the actual claim), with one real, checkable law —
debits must equal credits — that is genuinely branching, not
incidental.

**Prior art**: `~/repos/elicitation/crates/elicit_server::ledger`
(a typestate `Transfer<Pending -> Validated -> Committed>`, with a
`reject()`/`rollback()` branch to `Rejected`, gated by real contracts —
`AmountPositive`, `SufficientFunds`, `AccountsDistinct`, `BalancedEntries`,
composed via `And<P, Q>`/`both()`) and
`elicit_server::gaap::mathematical` (`AccountingEquationHolds` and other
real ASC-cited identities, versus roughly 130 other GAAP props that are
citation-only with empty proof bodies).

Two real strengthenings `amenable` brings over that design, confirmed
against the actual trait definitions rather than asserted:

- `amenable_core::Establish<C, V>: Evidence + Witness<V>` bounds
  `C: ProofToken` — `ProofToken`'s fields are private everywhere, so
  the only way to hold a credential at all is to have already obtained
  one from a prior lawful `establish()` call. A structural, type-system
  guarantee. Elicitation's `ProvableFrom<C>: Prop {}` is an empty
  marker trait; its own docs state the only real protection is that
  credential types are conventionally `pub(crate)` — a visibility
  policy, not a structural one.
- `amenable_core::cert::{Registry, Certificate}` (`issue_standard_certificate`,
  `issued()`, `report()`, per-key `get()`) is a real, iterable,
  queryable audit log. Elicitation's ASC citations
  (`"Source: ASC 230"`) are doc-comment strings with nothing behind
  them.

**A correction made while researching this plan**: elicitation's `Prop`
does not map uniformly onto `amenable_core::Standard`. Only the
citation-only, unprovable props (the ~130 disclosure/policy claims)
are genuinely `Standard`-shaped. The handful with real `kani_proof()`/
`verus_proof()`/`creusot_proof()` bodies — `AmountPositive`,
`SufficientFunds`, `AccountsDistinct`, `BalancedEntries`,
`AccountingEquationHolds` — are proven, not cited, and belong on the
`Evidence`/`Witness<V>`/`Exchange` side instead. Collapsing both into
`derive(Standard)` would blur exactly the proof-vs-audit line this
architecture exists to keep sharp (see `amenable.md`'s "Narrative"
section).

## Scope

**New crate `amenable_gaap`**: the trait-interface + domain-type layer
for the ledger/GAAP domain. Its crate-hierarchy position mirrors
`amenable_std`'s own real (asymmetric) shape exactly — decided
explicitly rather than inventing a new symmetric one (see "Crate
hierarchy" below).

**Initial worked example**: a two-account `Transfer`, mirroring
`elicit_server::ledger::typestate::Transfer<S>`'s shape
(`Pending -> Validated -> Committed`, with `reject()`/`rollback()`
branches to `Rejected`) — not the fuller `Draft -> Posted -> Balanced
-> Closed` cycle discussed earlier in this design conversation.
Deliberately the smaller shape first; scale up later.

**Initial contract set** (5 total, all with real proof content, none
citation-only placeholders):

- `AmountPositive` — `amount > 0`.
- `SufficientFunds` — `balance >= amount`. The first genuinely
  branching, data-dependent predicate this gallery has ever proven —
  the actual point of this whole plan.
- `AccountsDistinct` — `from != to`.
- `BalancedEntries` — `debit + credit == 0`.
- `AccountingEquationHolds` — Assets = Liabilities + Equity (from
  `gaap::mathematical`), the one multi-variable arithmetic identity in
  the initial set.

### Crate hierarchy

Mirrors `amenable_std`'s real, asymmetric shape (confirmed against its
actual `Cargo.toml`/`lib.rs`, not assumed):

- `amenable_gaap` depends on `amenable_core` + `amenable_derive`
  unconditionally.
- `amenable_gaap` depends on `amenable_creusot` optionally, behind a
  `creusot` feature. Creusot-side content lives *inside* `amenable_gaap`
  under that feature — same as `amenable_std::creusot_witness`/
  `creusot_gallery`.
- `amenable_gaap` has a `verus` feature for export/witness scaffolding
  only — no Cargo edge to `amenable_verus`. The real Verus proof source
  lives in a new `amenable_verus::gallery::ledger` module,
  `include_str!`-linked by relative path — same as
  `amenable_std::verus_witness`.
- `amenable_gaap` has **no** dependency on `amenable_kani`. The
  opposite direction: a new `amenable_kani::ledger` module gains a
  dependency on `amenable_gaap` — the same relationship `amenable_kani`
  already has with `amenable_std` — and hosts the real
  `Witness<KaniVerifier>` impls and harnesses directly, since Rust's
  orphan rules put them wherever `KaniVerifier` itself lives, not
  wherever the trait or type is defined.

No new crate-hierarchy pattern is being invented. This is
`amenable_std`'s exact shape with `amenable_gaap` swapped in for the
domain content, so every future domain crate (`amenable_db`,
`amenable_time`, `amenable_ui`, `amenable_gis`) has one proven template
to follow rather than a fresh decision each time.

## Non-goals

- **Not porting elicitation's full ~150-prop GAAP catalog.** The ~130
  citation-only props (real ASC citations, no real proof body) are
  genuine future `Standard` candidates but are explicitly deferred —
  this plan proves out the small provable core first, matching the
  "one real example by hand first" discipline the whole Exchange
  lineage already runs on.
- **Not adopting `elicitation::contracts::{Prop, VerifiedStateMachine}`
  as a dependency.** `amenable_gaap` re-expresses the same claims in
  `amenable`'s own trait vocabulary (`Evidence`/`Witness<V>`/
  `Sidecar<V>`/`Establish<C, V>`/`Exchange<Input, Output, V>`/
  `Standard`), not a wrapper around elicitation's crate.
- **Not building a cross-backend real-predicate-naming mechanism as
  part of this plan.** Elicitation's `Prop::kani_invariant_fn_name()`/
  `creusot_invariant_fn_name()`/`verus_invariant_fn_name()` is a real,
  working answer to a gap `amenable`'s own Creusot codegen still has
  (`EXCHANGE_PROOF_DERIVATION_PLAN.md`'s Step 9 hardcodes
  `#[requires(true)] #[ensures(true)]`, with no way to name a real
  shared claim). `amenable_gaap`'s five contracts will surface exactly
  where that gap bites, but designing the fix is a later step, once a
  real non-trivial predicate exists to expose the problem for real —
  not before.
- **Not wiring the Exchange-edge codegen layer
  (`amenable_core::ExchangeEdgeRecord`/`emit-creusot-companions`/
  `emit-verus-exchange-companions`) into `amenable_gaap` from day
  one.** Matching `Stoplight`'s own history exactly: hand-build the
  first real edge for Kani, then Creusot, then Verus, then generalize
  into the registry/codegen layer only once the by-hand shape is
  proven — not before.

## Design

### Step 0 — crate skeleton and the Primary/Proposition split — done

New crate `amenable_gaap`, wired per "Crate hierarchy" above.

First real design decision, distinct from `Stoplight`: `Stoplight`'s
states (`Green`/`Yellow`/`Red`) *are* the entire payload —
`Established<T, Token>`'s `Primary` and the sidecar's `Proposition`
are the same type `T`. The ledger's states carry real associated data
alongside the state marker (from/to account, amount, balance captured
at validation time, final balances after commit) — elicitation's own
`StateData<S>` enum captures exactly this shape. This is the first
case in the whole gallery where `Sidecar<V>::Primary` and
`Sidecar<V>::Proposition` need to diverge for real:

- `Proposition`: the state marker (`Pending`/`Validated`/`Committed`/
  `Rejected`), each a real `Evidence` type, `Witness<V>`-backed once
  proven.
- `Primary`: the real transfer payload (`TransferPayload { from:
  AccountId, to: AccountId, amount: Amount, .. }` plus whatever
  state-specific data each transition captures, mirroring
  elicitation's `PendingData`/`ValidatedData`/`CommittedData`/
  `RejectedData`).

Land the crate skeleton, the five contract types (`Evidence`-only, no
`Witness<V>` yet), and the `TransferPayload`/state-marker types with
no proofs at all — confirms the type-level shape compiles and the
Primary/Proposition split is real before any verifier touches it.

**What landed.** New crate `amenable_gaap`
(`amenable_core` + `amenable_derive` + `inventory` dependencies only —
`creusot`/`verus` Cargo features deferred to Steps 3/4 rather than
declared now with no content behind them, a deliberate narrowing of
"wire its `Cargo.toml`" to what Step 0 actually needs). `contracts.rs`:
the five contract types, each a bare `#[amenable_derive::evidence]`
root (`Basis = Self`) — deliberately *not* `#[derive(Standard)]`, per
this plan's own "Motivation" correction (they get proven, not cited).
`transfer.rs`: `AccountId`/`Amount` (unvalidated newtypes —
`AmountPositive`'s real check lands as a proof, not a constructor
guard) and `TransferPayload` (the `Primary` shape, itself a bare
`Evidence` root since `Sidecar<V>::Primary` only requires `Evidence`,
not `Witness<V>`); `Pending`/`Validated`/`Committed`/`Rejected`
(the `Proposition` shape), each `#[derive(Standard)]` with a real
`Provenance` impl, mirroring `amenable_kani::stoplight`'s
`Green`/`Yellow`/`Red` exactly — every state is a root asserted claim
regardless of reachability, matching `amenable.md`'s "States Are Roots,
Transitions Are Relations." Per-state associated data
(`ValidatedData`'s captured balance, etc., mirroring elicitation's
`StateData<S>`) deliberately not added yet — belongs with the real
transition logic that captures it, not speculated on before one
exists.

Verified for real: `cargo check -p amenable_gaap`, `cargo clippy -p
amenable_gaap --all-targets -- -D warnings`, and `cargo fmt -p
amenable_gaap --check` all clean; a new `tests/ledger_skeleton_test.rs`
(5 tests) confirms every state and contract type is a real,
inspectable root (`is_root()`, `Evidence::chain()`,
`Standard::report()` all exercised) rather than merely "it compiles."
Full workspace `cargo check --workspace`/`cargo fmt --check` clean.
Step 0 is complete.

### Step 1 — first real Kani edge, by hand — not started

Pick the single edge with the most payoff for exercising a genuinely
branching claim: `Pending -> Validated`, gated on `SufficientFunds`
(and `AmountPositive`, `AccountsDistinct`). Hand-build it directly in
a new `amenable_kani::ledger` module — `Witness<KaniVerifier>` impls
for the state markers, a real `#[kani::proof_for_contract]` harness
proving `SufficientFunds` fails exactly when `balance < amount` —
matching this whole lineage's "one real example by hand first"
discipline (`EXCHANGE_PROOF_DERIVATION_PLAN.md`'s own Step 1/Step 3
split). No macro, no registry, no codegen yet.

### Step 2 — remaining Kani edges — not started

Extend to `Validated -> Committed` (carrying `BalancedEntries`) and
the `reject()`/`rollback()` branches to `Rejected`. Decide then
whether `AccountingEquationHolds` belongs on `Committed` itself or as
a separate period-close edge — deferred until the smaller edges are
real and proven, not decided speculatively now.

### Step 3 — Creusot — not started

Mirror Steps 1/2 inside `amenable_gaap` itself under its `creusot`
feature, following `amenable_creusot::stoplight`'s own established
shape (concrete local mirror types, `harness!`-captured Pearlite
bodies).

### Step 4 — Verus — not started

Mirror Steps 1/2 inside a new `amenable_verus::gallery::ledger`
module, following `stoplight_exchange.rs`'s own shape
(`verus_ensures!`/`verus_exchange!`, real `ensures` clauses).

### Step 5+ — generalize, only once the by-hand shape is proven — not started, not scoped in detail yet

Once all three backends prove the same real edges by hand, revisit
whether/how the existing `ExchangeEdgeRecord`/codegen layer extends to
`amenable_gaap`, and whether the Creusot placeholder-predicate gap
needs a real fix (informed by, not copied from, elicitation's
`Prop::*_invariant_fn_name()` pattern) now that a genuinely
non-trivial predicate exists to expose it for real.

## Open questions

- Whether `AccountingEquationHolds` attaches to `Committed` directly
  or needs its own period-close state — deferred to Step 2.
- Whether the ~130 citation-only GAAP props become a later
  `Standard`-only slice of `amenable_gaap` (no proof, `derive(Standard)`
  only) once the provable core lands — explicitly out of scope for
  this plan, noted for later.
- Whether `Objective` (described in `amenable.md`'s "Trait Objective"
  section but not yet implemented anywhere in `amenable_core`) needs
  to exist before any GAAP claim without a third-party citation shows
  up. Not expected to matter for this initial scope — every claim here
  traces to either real math or a real ASC citation — but worth
  flagging since it's the one described-but-unbuilt piece of the trait
  family this plan might eventually need.

## Next step

Step 1: hand-build the first real Kani edge (`Pending -> Validated`,
gated on `SufficientFunds`/`AmountPositive`/`AccountsDistinct`) in a
new `amenable_kani::ledger` module. Not started.
