# GAAP Ledger Plan

## Status

🔲 Planning — Steps 0, 1, and 2 done. Step 3 (Creusot) has real,
non-trivial predicates proven for all four contracts (`AmountPositive`/
`SufficientFunds`/`AccountsDistinct`/`BalancedEntries`, `Proved (118
files) ✔`), implemented directly on the real `Validated`/`Committed`
types — no accommodation-model mirror, a real correction to this plan's
own original design (see Step 3 below). `Ledger::validate`/`::commit`
themselves aren't connected to those predicates yet. Along the way,
two new derive macros — `#[derive(ProofToken)]` and `#[amenable_
derive::establish(..)]` — were built and retrofitted onto every
hand-written `ProofToken`/`Establish` impl in both `stoplight.rs` and
`ledger.rs`, closing a real gap: this whole worked-example lineage
exists to dogfood `amenable`'s own derives. `reject()`/`rollback()`'s
structural wrinkle (both targeting one flat `Rejected` would collide
with `#[amenable_derive::exchange]`'s one-`Witness`-impl-per-evidence
assumption) is resolved by making `Rejected<T>` generic over the state
it was rejected from. Getting Step 3 to compile also forced a real
workspace-wide dependency-tree restructuring, recorded in
`EXCHANGE_PROOF_DERIVATION_PLAN.md`'s Step 10: `amenable_std`'s
optional edge to `amenable_creusot` flipped for real (closing a Cargo
cycle that blocked `amenable_kani` from being tried as a direct
dependency too), and `Stoplight`'s `Green`/`Yellow`/`Red` moved from
`amenable_kani` to `amenable_core` after a real, caught-and-reverted
attempt to depend `amenable_creusot` directly on `amenable_kani`.

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

**Superseded by Step 3's own real correction (see below) — this is the
actual, current shape, not the plan's original sketch.** `amenable_gaap`
is a genuinely neutral domain crate: both verifier backends depend on
it, it depends on neither of them, matching how `amenable_core` now
hosts `Stoplight`'s own `Green`/`Yellow`/`Red` for the identical reason
(`EXCHANGE_PROOF_DERIVATION_PLAN.md`'s Step 10).

- `amenable_gaap` depends on `amenable_core` + `amenable_derive` +
  `inventory` + `uuid` unconditionally. **No** dependency on
  `amenable_creusot`, `amenable_kani`, or `amenable_verus`, optional or
  otherwise — `amenable_gaap` itself carries no verifier-specific proof
  content at all, only the evidence markers (`Pending`/`Validated`/
  `Committed`/`Rejected<T>`) and domain data.
- `amenable_creusot` depends on `amenable_gaap` unconditionally and
  implements `Witness<CreusotVerifier>` directly on the real
  `Validated`/`Committed` types in `amenable_creusot::ledger` — no
  accommodation-model mirror, confirmed empirically to compile and
  prove clean (see Step 3 below for the full story of how this
  displaced the plan's original `creusot`-feature-inside-`amenable_gaap`
  design).
- `amenable_kani` depends on `amenable_gaap` unconditionally — the same
  relationship `amenable_kani` already has with `amenable_std` — and
  hosts the real `Witness<KaniVerifier>` impls and harnesses directly
  in `amenable_kani::ledger`, since Rust's orphan rules put them
  wherever `KaniVerifier` itself lives, not wherever the trait or type
  is defined.
- `amenable_gaap` has a `verus` feature for export/witness scaffolding
  only — no Cargo edge to `amenable_verus`. The real Verus proof source
  lives in a new `amenable_verus::gallery::ledger` module,
  `include_str!`-linked by relative path — same as
  `amenable_std::verus_witness`.

No new crate-hierarchy pattern is being invented. Both backends
depending independently on one neutral domain crate, with no edge
between the backends themselves, is the same template `amenable_core`
now uses for `Stoplight`'s evidence markers — so every future domain
crate (`amenable_db`, `amenable_time`, `amenable_ui`, `amenable_gis`)
has one proven template to follow rather than a fresh decision each
time.

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

### Step 1 — first real Kani edge, by hand — done

Hand-built the single edge with the most payoff for exercising a
genuinely branching claim: `Pending -> Validated`, gated on
`AmountPositive`/`SufficientFunds`/`AccountsDistinct`, directly in a
new `amenable_kani::ledger` module — matching this whole lineage's
"one real example by hand first" discipline
(`EXCHANGE_PROOF_DERIVATION_PLAN.md`'s own Step 1/Step 3 split). No
macro, no registry, no codegen. `Transfer<S, Token>`/`Sidecar<
KaniVerifier>` mirror `stoplight.rs`'s own shape, with the first real
divergence Step 0 anticipated: `Sidecar::Primary` is `TransferPayload`
(real data), `::Proposition` is the state marker — genuinely different
types, unlike `Established<T, Token>`'s `T` playing both roles for
`Stoplight`.

**A real CBMC timeout, fully root-caused, not worked around.** The
first version of `validate`'s postcondition (`kani_ensures!`,
biconditional over `amount > 0`, `balance >= amount`, and `payload.
from() != payload.to()`) timed out under `just verify-kani-contract`
with `amount`/`balance` symbolic. Root-caused via an extensive gallery
investigation (`amenable_kani::gallery::ledger_account_id_comparison`,
21 real experiments — read that module's own doc comment for the full
blow-by-blow) down to one precise fact: comparing two independently-
constructed `String`s for equality *inside a `#[kani::ensures]`
closure* is expensive for CBMC, regardless of content, length, or
whether the strings are symbolic or concrete — while the identical
comparison is cheap everywhere else (a bare standalone check, an
ordinary `if` in `validate`'s own function body). Along the way, two
real, second-order findings surfaced and were fixed at the shared
macro level (benefiting `Stoplight` too, not just this edge):
`#[amenable_derive::exchange]`'s generated `#[kani::ensures]` closure
used to dereference `result` (`*result`, requiring `Output: Copy`,
impossible once `Output` carries a `String`) — fixed to `result.
clone()`, `Clone` being a strict superset of `Copy`.

**What comparisons work inside a `#[kani::ensures]` closure, checked
directly rather than assumed**, all against `validate`'s exact real
body: a fixed 2-variant enum (0.12s), a bare `u64` (0.12s), a `[u8;
16]` UUID-shaped byte array (0.42s), and a `{ id, name }` hybrid with
`PartialEq` on `id` only (0.83s) all verify fast. A genuinely important
correction found along the way: a *fixed-capacity* string (`{ bytes:
[u8; 24], len: u8 }`, stack-allocated, comparing only `bytes[..len]`)
is exactly as expensive as `String` — still times out. "No heap
allocation" was never the real dividing line; what matters is whether
the *comparison itself* has a length fixed at compile time. `len`
being symbolic makes the fixed-capacity string's comparison just as
variable-length as `String`'s own, the identical shape this project's
own catalogued "symbolic-length memcmp" timeout class already names —
now confirmed to bite specifically inside a `#[kani::ensures]` closure,
not only in direct iteration.

**The fix**: `amenable_gaap::AccountId` now carries a `Uuid` identity
(chosen over a bare integer as "a real, dedicated identity type, not a
proxy repurposing an arbitrary numeric type" — the user's own call) —
`{ id: Uuid, name: String }`, with `PartialEq`/`Eq`/`Hash`/`Ord` all
comparing `id` only. The `name` field is unchanged in kind (still a
real, heap-allocated `String`, still constructed, still carried
through every transfer) — it's simply never what equality checks.
`id` is supplied explicitly at construction (`AccountId::new(id,
name)`), not generated fresh per call: the same real-world account
referenced twice must compare equal, which only holds if its id is
stable across reconstructions — the same reason a real chart of
accounts assigns an id once, at account creation, not per lookup. This
needed no change to `#[amenable_derive::exchange]`, `kani_ensures!`,
or any other shared macro — the generated `#[kani::ensures]` wiring
was never the problem, confirmed across the whole investigation.
`uuid` landed as a new workspace dependency (`amenable_gaap`,
`amenable_kani`); the ~52 call sites across `amenable_gaap`'s own
tests, `amenable_kani::ledger`, its tests, and the gallery
investigation file were updated mechanically (`AccountId::new("Alice")`
→ `AccountId::new(uuid::Uuid::from_u128(1), "Alice")`, consistently
per name, preserving every existing test's same-account/distinct-
account semantics).

**A second real bug, found only once the timeout stopped masking
it.** With the CBMC cost gone, `verify_validate_accepts_a_lawful_
transfer` (`#[kani::proof_for_contract(Ledger::validate)]` composed
via `#[kani::stub_verified]` over `check_amount_positive`/`check_
sufficient_funds`) ran to completion in ~110s and reported a genuine
verification failure — not a timeout. Root cause: `check_amount_
positive`'s own `#[kani::ensures]` was under-specified (`Err(bad) =>
*bad == amount`, missing `&& amount <= 0`). A too-weak postcondition
still passes that function's own isolated `#[kani::proof_for_
contract]` check trivially (the real body's actual behavior is a
strict subset of what a weak contract allows), but `#[kani::
stub_verified]` treats the contract as the *complete* story when
composing it into a caller — it was free to substitute `Err` even when
`amount > 0`, breaking `validate`'s own downstream claim that
`Err(NegativeAmount(amount))` implies `amount <= 0`. Fixed by making
the `Err` arm a full biconditional. `check_sufficient_funds`'s own
contract was already tight and needed no change.

**Verified for real, against the actual toolchain, not assumed**: both
`just verify-kani-contract` harnesses —
`verify_validate_accepts_a_lawful_transfer` and `verify_validate_
rejects_the_same_account` — report `VERIFICATION:- SUCCESSFUL`,
`0 of 492 failed`, in 111s and 122s respectively. The real, non-trivial
biconditional postcondition is intact throughout — never weakened,
never dropped, never replaced with a hand-written `assert!`-based
harness, matching the two hard constraints set for this fix. Full
workspace `cargo check`/`clippy --all-targets -D warnings`/`fmt
--check`/`test` clean for `amenable_gaap` and `amenable_kani`; all 4
functional tests in `tests/ledger_test.rs` and all 6 in `amenable_gaap`'s
own `ledger_skeleton_test.rs` (including a new one confirming `AccountId`
equality compares `id`, not `name`) still pass. Step 1 is complete.

### Step 2 — remaining Kani edges — done

**`Validated -> Committed` (`BalancedEntries`) — done.** Infallible,
like every `Stoplight` edge: a transfer that already passed `validate`'s
checks has nothing left to reject at commit time (this worked example
doesn't model concurrent modification between validation and commit).
`BalancedEntries`'s real claim (`debit + credit == 0`) is honestly
tautological by construction here (`debit` is literally `-payload.
amount().value()`) — the same kind of triviality `Stoplight`'s own
zero-field edges document rather than hide; a future refinement
building separate debit/credit `JournalEntry` postings from `commit`'s
own body would make it non-tautological, deliberately deferred.

**A second real CBMC timeout, unrelated to Step 1's, fully root-caused
via a three-experiment gallery investigation**
(`amenable_kani::gallery::ledger_commit_contract_timeout`): negating a
fully unconstrained symbolic `i64` (`-payload.amount().value()`)
overflows at `i64::MIN`, and Kani's default overflow-checking reasoning
over the full symbolic range is expensive enough to time out on its
own — independent of `AccountId`/`Uuid` comparison (not involved here
at all) and independent of the real `Sidecar`/`Establish`/`Transfer::
new` construction chain in the harness's own setup code (real, but not
the bottleneck: a fully-concrete-values control case with that same
chain passed in 143s). Fixed with a genuine `#[kani::requires(input.
primary().amount().value() > 0)]` precondition on `commit` itself — the
real, true precondition (`commit` is only ever meant to be called on an
already-`validate`d transfer, which already established
`AmountPositive`), not an artifact of what a harness happens to assume.

**Verified for real**: `just verify-kani-contract
ledger::verify_commit_always_balances` reports `VERIFICATION:-
SUCCESSFUL`, `0 of 297 failed`, 122.7s.

**Two new derive macros, built to close a real gap the user flagged
directly**: every `ProofToken`/`Establish` impl in this worked example
(and in `stoplight.rs`) was hand-written, despite this whole lineage
existing specifically to dogfood `amenable`'s own derives (`Standard`/
`Witness`/`Exchange` all get real exercise; `ProofToken`/`Establish`
never had a derive to exercise at all). `#[derive(ProofToken)]`
(`amenable_derive::proof_token`) generates `impl ProofToken for X {
type Proposition = Y; }` from `#[proof_token(proposition = "Y")]` —
an ordinary derive, since the impl targets `Self`, matching
`#[derive(Standard)]`'s own precedent. `#[amenable_derive::establish(
credential = .., verifier = .., proposition = ..)]` generates the
trivial-token-minting half of `impl Establish<C, V> for Y` — an
*attribute* macro, not a derive, because the generated impl targets
the *proposition* (a different type, usually defined in a different,
upstream crate) rather than `Self`, matching `#[amenable_derive::
exchange]`'s own precedent of an attribute wherever the generated impl
targets something other than the annotated type. Both retrofitted onto
every existing hand-written impl in `stoplight.rs` and `ledger.rs`; all
seven affected Kani harnesses (`verify_green_transitions_only_to_yellow`,
`verify_yellow_transitions_only_to_red`,
`verify_red_transitions_only_to_green`, `verify_full_cycle_composes`,
`verify_validate_accepts_a_lawful_transfer`,
`verify_validate_rejects_the_same_account`,
`verify_commit_always_balances`) re-verified successful after the
retrofit, confirming the derive-generated impls are behaviorally
identical to what they replaced.

**`Pending -> Rejected<Pending>` and `Validated -> Rejected<Validated>`
(`reject()`/`rollback()`) — done.** Both edges target logically the
same "rejected" outcome, which collides with `#[amenable_derive::
exchange]`'s one-`Witness`-impl-per-evidence-type assumption if
`Rejected` stays the flat marker Step 0 gave it: two macro invocations
naming the same concrete `evidence` would each generate `impl
Witness<KaniVerifier> for Rejected`, a hard `E0119` conflicting-
implementation error, not a style choice to avoid. Resolved by making
`Rejected<T>` generic over the state it was rejected from —
`amenable_gaap::transfer::Rejected<T>`, parameterized rather than flat.
A deliberate divergence from `~/repos/elicitation/crates/
elicit_server::ledger::typestate::Rejected` (flat, with a runtime
`RejectedData { reason }` field distinguishing why): elicitation has no
per-evidence-type "exactly one `Witness` impl" constraint, so
collapsing both edges into one flat state costs it nothing there.
`amenable` does have that constraint, for a real reason — `Witness<V>`
means "this state is provably reached, backed by *this specific*
proof," and `reject()`'s claim (mirrors `validate`'s own failure
conditions) isn't obviously the same claim as `rollback()`'s (whatever
it ends up checking post-validation). `Rejected<Pending>`/`Rejected<
Validated>` being genuinely distinct concrete types means each edge
earns its own honest proof rather than an artificial unification.

Both edges are infallible, like `commit` — rejecting a still-`Pending`
or still-`Validated` transfer has no failure mode of its own in this
worked example, so the real claim is legitimately trivial
(`result.is_ok()`), matching every `Stoplight` edge; `validate`/`commit`
are where this worked example's non-trivial claims live. No
rejection-reason payload (elicitation's `RejectionReason`) is threaded
through yet — deliberately out of scope for this pass, per this plan's
own "one real example by hand first" discipline; a future refinement
could add reason data to `Rejected<T>`'s own state.

A necessary correction along the way, not scope creep: `#[amenable_
derive::establish(..)]`'s and `#[amenable_derive::exchange]`'s
`evidence`/`credential`/`verifier`/`proposition` fields moved from bare
identifiers to string literals re-parsed as a `Path` (matching
`#[derive(Standard)]`'s own `#[standard(basis = "..")]` convention).
`proposition = Rejected<Pending>` is genuinely ambiguous as a bare
attribute expression (`<`/`>` parse as comparison operators, not
generic-argument delimiters, inside `Expr` position) — the fix applies
uniformly to every existing call site in `stoplight.rs`/`ledger.rs`,
not just the two new ones.

`Ledger::exchange`, called generically, becomes ambiguous wherever two
edges share an input type (`Transfer<Pending, PendingToken>` now goes
to either `Validated` or `Rejected<Pending>`; `Transfer<Validated,
ValidatedToken>` now goes to either `Committed` or `Rejected<
Validated>`) — every existing call site (`ledger_test.rs`, the
`ledger_account_id_comparison` gallery investigation) needed an
explicit `Output` type annotation. A real, permanent property of this
design once a state has more than one outgoing edge, not a one-time
migration cost.

**A real gap found in `#[derive(Standard)]` itself, exactly the kind
this dogfooding lineage exists to surface.** `Rejected<T>` is the
first *generic* `Standard` anywhere in the workspace. First attempt:
plain `#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Standard)]`
failed with `T: Clone`/`T: Default` unsatisfied — misdiagnosed at
first as "the built-in derives over-constrain a `PhantomData<T>`-only
parameter" and briefly hand-written around. The real cause was
narrower and squarely in `amenable_derive::standard`: its generated
`Evidence`/`Standard` impls call `Clone::clone(self)` (the default
`provenance_expr`) and `<Self as Default>::default()` (the default
`basis_ctor`) unconditionally inside the impl body, but never added
`Self: Clone`/`Self: Default` to that impl's own where-clause. Every
non-generic `Standard` before `Rejected<T>` made this invisible — a
concrete type either has `Clone`/`Default` or it doesn't, and every one
so far does — so the missing bound never bit until a generic type
actually needed it stated explicitly. Fixed in the macro itself
(`standard.rs` now adds `Self: Clone`/`#basis_ty: Default` to the
generated where-clause whenever the corresponding default expression
is used, a no-op for every existing non-generic call site): `Rejected<
T>` derives `Debug`/`Clone`/`Copy`/`PartialEq`/`Eq`/`Default` plainly,
each conditional on `T: Trait` the ordinary way, which the two real
instantiations (`T = Pending`/`Validated`) already satisfy.

**Verified for real, against the actual toolchain**: `just
verify-kani-contract ledger::verify_reject_always_succeeds` and
`ledger::verify_rollback_always_succeeds` both report `VERIFICATION:-
SUCCESSFUL`, `0 of 287 failed`, ~57s each. `verify_commit_always_
balances` and `verify_validate_accepts_a_lawful_transfer` re-verified
successful after the retrofit (`0 of 297 failed`/`0 of 492 failed`,
matching prior numbers exactly — no regression from the shared file
changes), plus a Stoplight spot-check
(`verify_green_transitions_only_to_yellow`).

Decide then whether `AccountingEquationHolds` belongs on `Committed`
itself or as a separate period-close edge — deferred until Step 3/4
give a reason to revisit, not decided speculatively now.

### Step 3 — Creusot — `AmountPositive`/`SufficientFunds`/`AccountsDistinct`/`BalancedEntries` done, `Ledger::validate`/`::commit` themselves not yet connected

**A real architectural correction, not the design this plan originally
sketched.** The plan as first drafted (and this section's own original
text) assumed GAAP's Creusot content would mirror `Stoplight`'s own
shape: hand-written accommodation-model types living *inside*
`amenable_gaap` under a `creusot` feature, with the real `#[requires]`/
`#[ensures]` Pearlite content generated later, once the by-hand shape
was proven (`GAAP_LEDGER_PLAN.md`'s own "Non-goals": "hand-build the
first real edge for Kani, then Creusot, then Verus, then generalize...
not before"). Building that revealed the real, better answer instead:
`amenable_gaap`'s evidence types have no dependency back on
`amenable_creusot` at all (unlike `Stoplight`'s own `Green`/`Yellow`/
`Red`, which did — `amenable_kani`, where they originally lived,
depends on `amenable_std`, which at the time optionally depended back
on `amenable_creusot`), so `amenable_creusot` can take a real, ordinary
Cargo dependency on `amenable_gaap` and implement `Witness<
CreusotVerifier>` **directly on the real `Validated`/`Committed` types**
— no accommodation-model mirror needed, ever, for this pair. Confirmed
empirically before committing to it, not assumed: `cargo creusot -- -p
amenable_creusot` and `cargo creusot prove -- -p amenable_creusot` both
succeed with `amenable_gaap` as a real dependency; the earlier
assumption that *any* dependency beyond `amenable_core` risked the same
translator ICE a *local* item can hit (per `amenable_std::creusot_
witness`'s own doc comment) turned out not to generalize to an ordinary
dependency crate's own items — only to items local to whatever crate
`cargo creusot` is actually translating. (This same finding later let
`amenable_std`'s own optional edge back to `amenable_creusot` be
flipped for real, and even let `amenable_creusot -> amenable_kani`
compile clean with zero ICE when tried directly — that specific edge
was still reverted, but for an unrelated, more fundamental reason:
verifier backend crates never depend on each other, full stop. See
`EXCHANGE_PROOF_DERIVATION_PLAN.md`'s Step 10 for the full account —
`Stoplight`'s `Green`/`Yellow`/`Red` moved to `amenable_core` instead,
the same neutral-crate split this section describes for GAAP.)

**A real, load-bearing counter-example found along the way.** The
`Witness<CreusotVerifier>` impls themselves still have to stay
`#[cfg(creusot)]`-gated (matching `stoplight.rs`'s own precedent), and
a first attempt gave them a real, descriptive `ProofArtifact`
(`Vec<(String, String)>` plus a `Display` impl) left *ungated* so an
ordinary, non-translated build could also use it — `cargo creusot
prove` hit a real internal compiler panic (a genuine rustc ICE, not a
proof failure). `creusot-rustc`'s translator sweeps every *local* item
regardless of whether `#[cfg(creusot)]`'s own condition is satisfied
elsewhere in the same crate; the `Vec`/`Display` machinery is exactly
the kind of ordinary Rust infrastructure the doc comment already warned
about. Fixed by keeping the `#[cfg(creusot)]`-gated `Witness` impls
trivial (`ProofArtifact = ()`, matching `Green`/`Yellow`/`Red`'s own
shape exactly — their only role during actual translation is
satisfying `Establish`'s `Witness<V>` bound) and moving the real,
descriptive `CheckedProof` to the `#[cfg(not(creusot))]` side, where
the `ProofRecord` registration already needed it.

**Four real Pearlite predicates, the first genuinely non-trivial
Creusot claims anywhere in this workspace** (every `Stoplight` claim is
`result.is_ok()`) — `amount_positive_holds`, `sufficient_funds_holds`,
`accounts_distinct_holds`, `balanced_entries_holds`, each backing an
isolated proof function (`check_amount_positive`, `::check_sufficient_
funds`, `::check_accounts_distinct`, `::check_commit_balances`),
mirroring `Ledger::check_amount_positive`/`::check_sufficient_funds`
being proven separately on the Kani side. The proof functions' own
signatures are still a sanitized mirror, not a byte-for-byte copy:
`amount`/`balance` stay plain `i64`; an account identity is mirrored as
a plain `u64` rather than `AccountId`'s real `Uuid`-backed struct
(Creusot support for a hand-rolled `Uuid`-backed equality type is
untested territory, not worth risking for this pass). `check_commit_
balances` carries the same real `amount@ > 0` precondition Kani's own
`Ledger::commit` needed (`i64` negation still overflows in the ordinary
function body even though Pearlite's `@`-lifted `Int` arithmetic in the
*claim itself* has no such overflow).

**Verified for real, against the actual toolchain, with real
non-vacuity checks**: `cargo creusot prove -- -p amenable_creusot`
reports `Proved (118 files) ✔` (up from 110 before this step). Two
real injected bugs — `check_amount_positive`'s body loosened to
`amount >= 0`, and `check_commit_balances`'s credit shifted by one —
each produced a precise, real failure (`vc_check_amount_positive`
failed, then `vc_check_commit_balances` failed 2/4 goals) before being
reverted and re-verified clean. `just check-all-creusot` (fmt/clippy/
test across `amenable_creusot`, `amenable_std`, and `amenable`, each
with their `creusot` feature) clean; full workspace `cargo check
--workspace --all-features` clean.

**A real, recurring papercut fixed along the way, not scope creep**:
`amenable emit-creusot-companions` reads the shared `ExchangeEdgeRecord`
registry, which `#[amenable_derive::exchange(..)]` populates
unconditionally — so it was also picking up `amenable_kani::ledger`'s
own edges (`validate`/`commit`/`reject`/`rollback`) and writing dead,
uncompilable companion files for them (verbatim-copying the real Kani
body, which references `Self::check_amount_positive`/`TransferError`/
etc. that exist in no mirror namespace) every time `just generate-
creusot` ran. Filtered to `self_ty == "Stoplight"` in `amenable::
creusot_export`, with a doc comment explaining why — this generator's
whole model (free function spliced into a matching-named mirror scope,
hardcoded trivial `#[ensures(true)]`) is specific to `Stoplight`'s own
shape and doesn't apply to how GAAP's real claims are proven.

**Not yet done**: `Ledger::validate`/`::commit` themselves (the real
functions, living only in `amenable_kani`) are not yet connected to
these Creusot predicates — the four checks above are proven in
isolation, the same way `Ledger::check_amount_positive`/`::check_
sufficient_funds` were proven in isolation before `validate`'s own
Kani contract was written. `Ledger`/`Transfer<S, Token>` becoming a
real Cargo dependency of `amenable_creusot` is **not** on the table —
that would be a direct `amenable_creusot -> amenable_kani` edge, ruled
out on architectural grounds regardless of translator-ICE risk (see
`EXCHANGE_PROOF_DERIVATION_PLAN.md`'s Step 10: this exact edge was
tried, compiled clean, and was reverted anyway for violating
"verifier backends never depend on each other"). Whatever connects
`Ledger::validate`/`::commit` to these predicates will need either its
own neutral-crate marker types (matching `Stoplight`'s `Green`/
`Yellow`/`Red` -> `amenable_core` move) or a different mechanism
entirely — an open question for whenever that connection is actually
built.

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

Step 3's remaining piece: connect `Ledger::validate`/`::commit`
themselves (real functions, living only in `amenable_kani`) to the
four proven Creusot predicates. **Not** via a real Cargo dependency
from `amenable_creusot` to `amenable_kani` — that question is now
settled, not open: tried directly, compiled clean (translator-ICE risk
was never the blocker), and reverted anyway for violating "verifier
backends never depend on each other"
(`EXCHANGE_PROOF_DERIVATION_PLAN.md`'s Step 10). Whatever connects
`Ledger`/`Transfer<S, Token>` to these predicates needs a different
treatment — most likely its own neutral-crate marker types, matching
`Stoplight`'s `Green`/`Yellow`/`Red` -> `amenable_core` move.
