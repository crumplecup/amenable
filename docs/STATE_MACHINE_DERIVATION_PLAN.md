# State Machine Derivation Plan

## Status

🔲 Planning — Steps 0-3 and 5 done, Step 4 not yet started (Step 5
landed out of order, by direct instruction, once Step 3 surfaced the
real gap it fixes — see Step 5's own account below). Step 0: `amenable_core::State<V>`
landed exactly as designed — the object-safe facade, blanket-implemented
over `Evidence + Witness<V>`, confirmed via a compile-only test covering
every real state type across both worked examples (`Stoplight`,
`Ledger`), no new impl work needed anywhere. Step 1: `#[derive(
amenable_derive::StateMachine)]` landed exactly as designed too — parses
`#[state_machine(verifier = .., state(..), edge(..))]` and emits one
compiler-enforced static assertion per edge, applied to the real
`Stoplight` canary, confirmed non-vacuous two ways (a fabricated edge
fails with a precise `E0277`; an undeclared state name fails with a
clear macro-level error).

Step 2 done too, with two real corrections found only by implementing,
not foreseeable from the design discussion alone:

- **The audit surface doesn't back onto `ContractRecord`/`ProofRecord`
  the way this doc originally said.** Those registries exist, but
  `ContractRecord.evidence` is a claim-id string (e.g.
  `"amenable_kani::stoplight::Red::yellow_to_red_ensures"`), not a bare
  state name, and `ProofRecord.evidence` for Creusot registrations is a
  fully-qualified function path (e.g.
  `"amenable_creusot::stoplight::green_to_yellow"`) — neither matches a
  declared state name (`"Red"`) by exact string equality the way the
  scoping needed to avoid reintroducing string-prefix fragility.
  `ExchangeEdgeRecord` does: `self_ty`/`evidence` are exact, bare type
  names by construction (`stringify!(#self_ty)`/`stringify!(#evidence)`
  in `#[amenable_derive::exchange(..)]`'s own generated registration).
  `audit_surface()` queries that instead, real captured transition-
  method bodies rather than proof-harness source — a different, still
  real, granularity than the old design's `_SRC` consts.
- **The old `Amenable` trait couldn't be partially preserved.** It was
  declared `Amenable: StateMachine` — a hard supertrait bound — so
  redefining `StateMachine` with a different, generic-over-`V` shape (as
  designed) makes the old `Amenable` trait itself stop compiling, not
  just its impl. There is no clean way to keep `Amenable::
  creusot_surface()` alive for `crates/amenable/tests/
  stoplight_creusot_surface_test.rs` while reclaiming the `StateMachine`
  name. Resolution: delete both together now, rather than contort the
  design to avoid a temporary gap. `ExchangeEdgeRecord` has no
  `verifier` field at all today (every registration comes from
  `amenable_kani`, the only crate whose toolchain can run
  `#[amenable_derive::exchange(..)]`'s generated code safely), so a real
  Creusot-backed `audit_surface()` needs either a new field there or an
  equivalent registry for that backend — genuinely Step 4's work, not a
  Step 2 stopgap. `stoplight_creusot_surface_test.rs` is deleted, not
  weakened to a stub; real Creusot audit content returns in Step 4.

`states()`/`transitions()` echo the parsed `#[state_machine(..)]`
declarations directly (no registry query — the declaration itself is
the source of truth for what was declared). The declared-vs-registered
cross-check landed as a real test (`declared_transitions_match_real_
exchange_edge_registrations_exactly` in `stoplight_amenable_test.rs`),
confirmed non-vacuous by temporarily undeclaring a real edge and
watching it fail with the exact real/declared set diff, reverted.
`TransitionAudit` dropped the `from` field the original sketch implied
`ContractRecord`-backed data would carry — `ExchangeEdgeRecord` has no
reliable, non-fragile way to recover a bare "from" state name from its
`input_ty` field (a full carrier type string, not a bare state name),
so `to`/`method_name`/`body` (all exact matches or real captured
source) is what's actually derivable honestly.

`docs/AMENABLE_PLAN.md`'s own `StateMachine`/`Amenable` references are
untouched — out of scope for this plan, which documents new decisions
in its own file rather than retroactively rewriting an older one, matching
this project's standing practice.

Step 3 done too, but not as originally scoped — the first draft
conflated two things and got corrected twice by direct discussion
before any code was written. First correction: "token-carried `Ensures`/
`Requires` delegation... replacing today's hand-invoked `kani_ensures =
"true"`" was never accurate — `Stoplight` doesn't use that mechanism at
all (`#[exchange(..)]` already generates its contracts fully), and
"attach the contract to the token instead of the evidence type for
multi-edge granularity" has no real, verifiable target anywhere in this
codebase (every state today is reached by exactly one edge). Second,
real correction, from direct clarification: the actual idea isn't about
*where* a contract lives (token vs. evidence type) at all. `Ensures<V>`/
`Requires<V>` already exist and can already be implemented on anything
flowing through a `Sidecar` — that part was never new. What's new is
that a state machine *chains* transitions, so the same declared state is
simultaneously one edge's output (checked via `Ensures<V>`) and the next
edge's input (checked via `Requires<V>`) — two different contract
halves, the same underlying type, connected because the graph guarantees
it. "Sewing together" means those two halves can rest on the same real,
registered atomic claim instead of being independently hand-typed and
silently able to drift apart.

`amenable_gaap::Ledger` is the real canary for this, not `Stoplight` —
confirmed by checking, not assumed: `Stoplight` has no real `Requires<V>`
content anywhere (every edge is a trivial `ensures`-only `result.is_ok()`),
while `Ledger::commit` already had a genuine hand-typed precondition
(`input.primary().amount().value() > 0`) restating what `validate`'s own
postcondition already established once via the registered
`AmountPositive` claim — a real, live instance of exactly the duplication
this step closes. `Validated` (the shared type between `validate`'s
output and `commit`'s input) gained a real `Requires<KaniVerifier>` impl,
via the existing `kani_requires!` macro, delegating through the identical
`AmountPositive::ensures` call `validate`'s own `Ensures<KaniVerifier>`
impl already uses — not a fresh claim, the same one serving both roles.
`capture_exchange_body` gained `kani_requires_evidence = "Type"`
(mutually exclusive with the existing raw-expression `kani_requires`),
generating `<Type as Requires<V>>::requires(input.clone())` the same
mechanical way `kani_ensures = "true"` already generates its own
delegated postcondition — wired onto `commit` in place of the old raw
expression, with a matching `Validated: Requires<V, ..>` bound added to
`commit`'s own `where` clause.

Verified for real, not just compiled: `cargo kani` on `commit`'s own
contract harness (`verify_gaap_commit_always_balances`) still passes,
`0 of 297 failed`, with the precondition now routed through the real
delegated call instead of the inline expression. Two new tests in
`ledger_test.rs` — one confirming `Validated::requires` holds for a
real, lawfully-validated transfer; one confirming the "sewing together"
directly, by querying both `ContractRecord`s (`validate`'s `kind =
"ensures"`, `commit`'s `kind = "requires"`) and asserting both fragment
texts name `AmountPositive` — verified non-vacuous by temporarily
reverting `commit_requires`'s delegation to a hand-typed expression and
watching the fragment-text assertion fail precisely, reverted. A
freestanding "contract surface" query function (originally sketched as
part of this step, keyed by bare state name against `ContractRecord`)
turned out not to be buildable honestly: `ContractRecord.evidence`
carries whatever free-form claim-id string each `kani_ensures!`/
`kani_requires!` call chose to pass (e.g.
`"amenable_kani::ledger::Validated::commit_requires"`), not a bare state
name — exact-matching it against a declared state name doesn't work,
and substring-matching would reintroduce the exact string-prefix
fragility already fixed once for `audit_surface()`. The two new tests
demonstrate the real connection directly instead, against the known
exact evidence-id strings.

Also confirmed, not assumed: `#[derive(StateMachine)]` (Steps 1-2)
cannot be applied to `Ledger` the way it was to `Stoplight` without
further work — `Ledger`'s methods are plain generic-over-`V` inherent
methods registered via `capture_exchange_body`, with no `impl
Exchange<Input, Output, V> for Ledger` anywhere (deliberately: `#[exchange(..)]`
requires a concrete verifier, which `Ledger`'s methods don't have).
The Step 1 static assertion (`T: Exchange<..>`) has nothing to check
against. Applying the full derive to `Ledger` — and deciding what the
static-assertion mechanism should check instead for a
`capture_exchange_body`-shaped edge — stays Step 5's job, not solved
here.

Step 5 done too, but with a direct, firm correction of the framing
above: "what should the static assertion check for a `capture_exchange_
body`-shaped edge" was the wrong question. `Ledger` having no `Exchange`
impl isn't a shape the derive needs to accommodate — it's exactly the
gap the derive exists to catch, surfaced correctly. The real fix: teach
`Ledger`'s methods to actually implement `Exchange`, generically, rather
than special-case the derive around their absence. `#[capture_exchange_
body(..)]` now unconditionally generates a real `impl<V: Verifier>
Exchange<Input, Output, V> for Self` alongside everything it already
did — copying the real method's own `where` clause verbatim (needed
since the generated `exchange()` body calls straight through to the
real method, which needs the identical bounds to resolve), delegating
via `self.method::<V>(input)`. Requires the method be generic over
exactly one type parameter named `V` (validated, clear error
otherwise) — the same hardcoded-name convention `kani_ensures = "true"`
already relies on. `#[exchange(..)]`'s own concrete-verifier bundle
still can't apply here (it needs one fixed backend to also generate a
`Witness<V>` impl and `ProofRecord`); this is additive to
`capture_exchange_body`, not a replacement for `#[exchange(..)]`.
Applied automatically to all four of `Ledger`'s methods the moment the
macro was extended — no per-call-site opt-in, since a missing `Exchange`
impl was the actual defect, not a feature some callers should be able to
decline.

This forced `#[derive(StateMachine)]` itself to grow a second mode.
`Ledger` lives in `amenable_gaap`, a crate that deliberately depends on
no backend — it can never name a concrete verifier type in its own
generated code (that code lands in `amenable_gaap` itself, wherever the
derive was invoked), so the existing `verifier = "KaniVerifier"` mode
literally cannot apply to it. `#[state_machine(generic_over_verifier,
..)]` is the new mode: no concrete verifier named anywhere, a single
blanket `impl<V: Verifier> StateMachine<V> for Self` instead of one per
verifier. The first attempt at its static assertion tried to prove
"`Self: Exchange<A, B, V>` holds for every possible `V: Verifier`" via a
generic function forcing real trait resolution against an unconstrained
type parameter — the compiler correctly rejected it, real errors, not a
bug in the check: `Ledger`'s real `Exchange` impl is only generic over
`V` *conditionally* (bounded by `Witness<V>`/`Ensures<V>`/`Requires<V>`
on whichever evidence types a given edge touches), never for a truly
unconstrained hypothetical verifier with no real proof content backing
it. Resolution: `generic_over_verifier` mode generates no static
assertion at all — `capture_exchange_body`'s own generated impl is
already the complete real compile-time check for its own edge;
declaration-vs-reality correctness (a typo'd or missing edge) is the
runtime `ExchangeEdgeRecord` cross-check's job, which needs no
universal-over-`V` provability to be real, and generalized cleanly to
`Ledger` (proven with the same inject-and-revert check already used for
`Stoplight`).

One more real, unplanned fix along the way: `state(name = "..", carrier
= "..")`/`edge(from = "..", to = "..")`'s key-value syntax (Steps 1-2's
own design) triggers a genuine `clippy::duplicated_attributes` false
positive — confirmed in isolation before concluding it wasn't fixable by
reordering fields — for any state with more than one outgoing edge
(`Ledger`'s `Pending`/`Validated` both have two): the lint compares only
the first key-value pair of a repeated nested meta item, so two
`edge(from = "Pending", to = ..)` entries collide regardless of their
different `to`. Positional syntax (`state("Green", "carrier-type")`,
`edge("Green", "Yellow")`) doesn't trigger it at all, confirmed the same
way, so both `Stoplight`'s and `Ledger`'s declarations — and the
parser — moved to it. A real, external-tooling-forced exception to this
macro family's usual `key = "value"` convention, not a style choice.

Verified for real throughout: full workspace `check`/`clippy -D
warnings`/`fmt`/`test` clean; `cargo kani` re-run on `Ledger`'s own
contract harnesses after the `Exchange` impl addition, still passing.

## Motivation

`amenable_core::state_machine`'s current `StateMachine`/`Amenable` trait
pair is not a foundation to build on. `Stoplight`'s own hand-written
impl — the only implementor anywhere in the tree — self-documents both
halves as vestigial: `Color` ("purely descriptive: nothing in this
module derives it from, or checks it against, the real `Exchange`
graph — that would be exactly the disconnected-proxy mistake this
module used to make," referring to an already-deleted `next()`
function) and `SequentialCycle` ("proven one edge at a time by real
Kani contracts directly on each `Exchange::exchange` body... not, as
an earlier version of this module did, by a disconnected proxy
function nothing here actually calls"). `kani_surface()`/
`creusot_surface()` are also asymmetric in a way that matters: Kani's
filter is `module_path!()`-derived (compiler-checked, can't drift),
Creusot's is a hand-typed string literal (`"amenable_creusot::
stoplight::"`) — the exact drift risk the surrounding doc comments
claim to avoid. This plan does not extend that design. It replaces it.

**Prior art**: `~/repos/elicitation`'s `#[derive(VerifiedStateMachine)]`
/ `#[formal_method]` / `#[derive(KaniVariantState)]`, read directly
before this plan was written. One real technique is kept: per-variant,
bounded-depth compositional proof construction, avoiding one fully
symbolic harness CBMC can't bound in reasonable time. It is not ported
as new machinery — `amenable_derive::KaniCompose` already implements
the same idea (depth0/1/2/any bounded construction) and this plan
reuses it directly. Two things are explicitly rejected: vacuous Verus
companions (`requires true, ensures true`, confirmed in
`formal_method.rs`'s generated output) and `Established::assert()`-
style proof-token construction that bypasses the real `Establish`/
`Sidecar` chain. Also rejected: the two-stage architecture itself
(derive methods return `Vec<TokenStream>` for a `build.rs` to write to
a generated file). `amenable` has no such pipeline anywhere and does
not need one — every derive in this plan expands directly, at compile
time, the same way `capture_exchange_body`/`harness!`/`#[exchange(..)]`
already do.

## Design

### `State<V>`: a deliberately thin, object-safe facade

The bound is `Evidence + Witness<V>` — confirmed precise, not
`Provenance` (a real, distinct trait: `type MetadataIter: Iterator<Item
= MetadataEntry>; fn metadata(&self) -> Self::MetadataIter;`, already
carried separately by `Standard::Provenance`). `Sidecar<V>::
Proposition` is already bounded exactly `Evidence + Witness<V>`, so
this is not a new requirement invented for this plan — it is naming a
bound every proposition flowing through a real `Exchange` today already
satisfies.

Neither `Evidence` nor `Witness<V>` is object-safe as written —
`Witness::proof()` has no `self` receiver at all, `Evidence::basis()`/
`chain()` don't either, and both return associated types. A literal
`trait State<V>: Evidence + Witness<V>` would make `dyn State`
uncompilable. The fix is a facade, not a fix to the underlying traits:

```rust
trait State<V: Verifier> {
    fn metadata(&self) -> Vec<MetadataEntry>;
    fn evidence_chain(&self) -> Vec<&'static str>;
    // owned/self-receiver projections of Witness<V>::proof(), etc.
}

impl<V: Verifier, T: Evidence + Witness<V>> State<V> for T {
    // projects T's Evidence/Witness<V> methods into State's
    // owned, vtable-safe surface
}
```

`dyn State<V>` becomes constructible for real; heterogeneous
collections (`Vec<Box<dyn State<V>>>`) work across otherwise-unrelated
types in the workspace. The blanket impl means every existing state
type — `Green`/`Yellow`/`Red`, `Pending`/`Validated`/`Committed`/
`Rejected<..>`, every `amenable_gaap` contract type — satisfies
`State<V>` the moment the blanket impl exists, with zero new derive
invocations anywhere.

### `#[derive(StateMachine)]`: explicit declarations, compiler-enforced

A derive macro has no type information and cannot read `inventory`
registries at expansion time (those are populated and queried at
runtime, after the macro has already expanded). So it cannot discover
"which methods are transitions" or infer wrapper/token types from
context. Every state and edge is declared explicitly, matching this
codebase's standing preference for explicit args over naming-convention
inference (`#[evidence(basis = "..")]`, `#[standard(basis = "..")]`) —
and matching the user's own direction: concrete types are too binding,
so the macro takes the target as an attribute rather than assuming a
wrapper shape.

```rust
#[derive(StateMachine)]
#[state_machine(
    verifier = "KaniVerifier",
    state(name = "Green", carrier = "Established<Green, GreenToken>"),
    state(name = "Yellow", carrier = "Established<Yellow, YellowToken>"),
    state(name = "Red", carrier = "Established<Red, RedToken>"),
    edge(from = "Green", to = "Yellow"),
    edge(from = "Yellow", to = "Red"),
    edge(from = "Red", to = "Green"),
)]
struct Stoplight;
```

Each `edge(..)` resolves its `from`/`to` against the declared `state(..)`
carriers and emits a static assertion:

```rust
const _: fn() = || {
    fn assert<T: Exchange<Established<Green, GreenToken>, Established<Yellow, YellowToken>, KaniVerifier>>() {}
    assert::<Stoplight>();
};
```

If the real `impl Exchange<..>` doesn't exist, the compiler squawks —
no macro-side introspection required, no runtime dependency. Because
the carrier is an opaque, caller-supplied type expression, the macro
never needs to know or assume anything about `Established` specifically;
any `Sidecar<V>` implementation works identically. Repeat the whole
`#[state_machine(..)]` block once per verifier a machine is proven
under (Kani, Creusot, Verus) rather than having the macro guess which
backends apply.

Because `state(..)`/`edge(..)` are macro args, not registry lookups,
the derive can bake real, static data directly into generated
aggregate methods for free:

```rust
impl StateMachine for Stoplight {
    fn states() -> &'static [&'static str] {
        &["Green", "Yellow", "Red"]
    }

    fn transitions() -> &'static [Transition] {
        &[
            Transition { from: "Green", to: "Yellow", verifier: "KaniVerifier" },
            Transition { from: "Yellow", to: "Red", verifier: "KaniVerifier" },
            Transition { from: "Red", to: "Green", verifier: "KaniVerifier" },
        ]
    }
}
```

**Nested types compose for free.** If a state's carried type itself
contains another type needing its own state-worthiness (the user's
`Green` containing a `BulbKind` enum example), nothing in this derive
needs to check that recursively — `#[derive(Evidence)]`/
`#[derive(Standard)]`'s existing basis-chain composition already
requires every constituent field to satisfy `Evidence` for the outer
type to compile at all. The `StateMachine` derive only needs the
top-level carrier to already be `Evidence + Witness<V>`; if it is, its
own fields already were, transitively, or it wouldn't have compiled in
the first place.

**Declared vs. real, a complementary check — day one, not deferred.**
The static assertion proves the declared edge *type-checks*. It doesn't
prove the declaration is honest — nothing stops someone from declaring
an edge that happens to type-check against a stale or unintended
`Exchange` impl. `ExchangeEdgeRecord` (`self_ty`/`input_ty`/`output_ty`/
`evidence`/`method_name`, populated at runtime by every real
`capture_exchange_body`/`#[exchange(..)]` call) already carries
everything needed to cross-check `StateMachine::transitions()`'s static
list against what was actually registered, catching drift in both
directions. Originally scoped here as a fast-follow; corrected after
review — a plan whose own design section argues this check matters
shouldn't ship the mechanism without it. It's a runtime/test-time check
(a `#[test]` against `inventory::iter::<ExchangeEdgeRecord>()`), not a
compile-time gate, but it lands in the same step as `transitions()`
itself (Step 2), not after.

### Backend auditability and invariant naming, preserved not deferred

Two real, currently-passing test files depend on the trait pair this
plan deletes: `crates/amenable_kani/tests/stoplight_amenable_test.rs`
and `crates/amenable/tests/stoplight_creusot_surface_test.rs`, both
exercising `kani_surface()`/`creusot_surface()`/`verus_surface()`/
`audit_surface()` for real. Deleting `impl Amenable for Stoplight` with
the replacement deferred to the separate, still-unresolved `Audit<V>`/
`Report<V>` design track (see the parallel discussion of `Amenable` as
a bound-selecting derive) would be a real regression, not a refactor —
corrected after review.

The fix doesn't require waiting on that other track to resolve. The
same `state(..)`/`edge(..)` declarations already backing the static
assertions are exactly the data needed to generate a minimal audit
surface honestly — scoped by the real declared type names (not the old
design's `module_path!()`/hand-typed-string-prefix matching, which is
what made `creusot_surface()` driftable in the first place), and
verifier-generic (one method taking `V`, querying `ContractRecord`/
`ProofRecord` filtered by `evidence` and `verifier`) rather than three
near-duplicate methods repeating the same asymmetry bug. This ships as
part of Step 2, alongside `states()`/`transitions()`. Both existing
test files get migrated to call the new generated surface rather than
deleted — same behavior they check today, mechanically generated and
correctly scoped instead of hand-written. If/when the `Audit<V>`/
`Report<V>` design lands, it supersedes this; this isn't blocked on
that landing first.

Invariant naming survives too, but deliberately thinned to what's
actually load-bearing: not a declared type (that's exactly the
`SequentialCycle` disconnected-proxy mistake this plan removes), but an
optional, purely cosmetic label —
`#[state_machine(invariant = "SequentialCycle")]` — carried through to
the generated audit surface as a name only, with no backing type and no
claim of content. Omittable; nothing currently requires it.

### Proof tokens as contract types

Today `Ensures<V>`/`Requires<V>` are implemented on the Evidence/
proposition type (`Validated`, `Committed`, `AmountPositive`), per the
existing `amenable_gaap::ledger.rs` delegation fix. That works because
every worked example so far has exactly one edge landing on each
proposition. It doesn't generalize: `Establish<C, V>`'s own signature
already allows different credential types `C` to mint *different*
token types for the same target proposition, so a state reachable by
more than one edge could need a different claim per edge — which a
contract shared on the proposition type can't express, but a contract
on the *token* minted by that specific edge can.

The `state(name = .., carrier = "Established<S, SToken>")` declaration
already names the token explicitly, giving the derive the exact hook
needed to auto-generate the delegation `capture_exchange_body`'s
`kani_ensures = "true"` mechanism currently requires hand-adding per
transition:

```rust
|result: &Result<Output, Error>| <SToken as Ensures<V>>::ensures(result.clone())
```

**Fail closed, never vacuous.** If no real claim exists for an edge on
a given backend, the derive does not synthesize a placeholder
(rejecting elicitation's `requires true, ensures true` pattern
outright). It either requires the author to supply one explicitly (the
same `kani_requires = ".."` escape hatch `capture_exchange_body`
already has), or leaves that edge's contract unconnected and says so —
matching `Stoplight`'s own honest-empty `verus_surface()` precedent
rather than hiding the gap.

### Reusing `KaniCompose` for non-trivial carriers

Every `Stoplight` edge today is a zero-field marker transition — no
symbolic data, nothing for a splitting strategy to help with. The
moment a transition's carrier wraps real data (an `amenable_gaap`-style
payload with symbolic fields), a single fully-symbolic harness risks
the CBMC blow-up class already catalogued in this project's own Kani
failure-pattern findings. Elicitation's `KaniVariantState` solves this
with bounded per-field construction at fixed depths — `amenable_derive::
KaniCompose` already does the same thing. Auto-generated harnesses for
data-bearing transitions route through `KaniCompose`'s existing
depth0/1/2/any construction rather than a new mechanism.

### Deletion scope

`amenable_core::state_machine.rs` (the current `StateMachine`/
`Amenable` trait pair, `kani_surface`/`creusot_surface`/`verus_surface`/
`audit_surface`) is deleted, not extended. `Stoplight`'s `Color` enum,
`SequentialCycle` marker, and both hand-written impls are deleted and
replaced by applying the new derive to `Stoplight` itself — the design
canary for this whole plan, per direct instruction.

Explicitly out of scope here: the *full* `Audit<V>`/`Report<V>` trait
redesign from the parallel `Amenable`-as-bound-selector discussion. A
minimal, declaration-scoped stand-in ships as part of this plan instead
(see "Backend auditability and invariant naming" above), so nothing
currently working regresses while that other design is still being
decided. `State<V>`/`StateMachine` stand on their own; if/when the
fuller track lands, it supersedes the stand-in rather than this plan
depending on it landing first.

## Steps

- **Step 0** — `State<V>` facade trait + blanket impl in
  `amenable_core`, no behavior change anywhere. A compile-only test
  confirms every existing state type (`Green`/`Yellow`/`Red`,
  `Pending`/`Validated`/`Committed`/`Rejected<..>`, every
  `amenable_gaap` contract type) satisfies it for free.
- **Step 1** — `#[derive(StateMachine)]` skeleton: parse `state(..)`/
  `edge(..)`/`verifier = ..` args, emit only the static assertions.
  Canary: `Stoplight`, Kani only.
- **Step 2** — Generate `states()`/`transitions()` aggregate methods
  from the same parsed declarations, the verifier-generic audit-surface
  method (`ContractRecord`/`ProofRecord` queried by declared `evidence`/
  `verifier`, replacing `kani_surface`/`creusot_surface`/
  `verus_surface`/`audit_surface`), and the runtime cross-check against
  `ExchangeEdgeRecord`. Migrate `stoplight_amenable_test.rs`/
  `stoplight_creusot_surface_test.rs` to the new surface in the same
  step — not a later fast-follow, so nothing currently passing goes
  dark in between.
- **Step 3** — Sew a chained edge's postcondition and the next edge's
  precondition to the same registered atomic claim, on `Ledger`
  (`Stoplight` has no real `Requires<V>` content to demonstrate this
  with). `Validated` gains a real `Requires<KaniVerifier>` impl
  delegating through the same `AmountPositive` claim `validate`'s own
  postcondition already uses; `capture_exchange_body` gains
  `kani_requires_evidence = "Type"`, generating the delegated
  precondition the same mechanical way `kani_ensures = "true"` already
  generates the postcondition; wired onto `commit` in place of its old
  hand-typed inline expression.
- **Step 4** — Extend Creusot and Verus coverage for `Stoplight`
  (second `verifier = ..` blocks), matching the existing per-backend
  precedent.
- **Step 5** — Apply `#[derive(StateMachine)]` to `Ledger` itself. Done
  out of order, ahead of Step 4, by direct instruction: `Ledger` having
  no `Exchange` impl wasn't a shape for the derive to accommodate, it
  was the real gap the derive exists to catch. Fixed at the source —
  `capture_exchange_body` now unconditionally generates a real,
  verifier-generic `impl<V: Verifier> Exchange<Input, Output, V> for
  Self` — rather than special-cased in the derive. `#[derive(
  StateMachine)]` gained a `generic_over_verifier` mode to match (see
  its own account above for the real, compiler-rejected first attempt
  at its static assertion, and why the final design generates none).
  `KaniCompose` routing for data-bearing carriers, which `Stoplight`'s
  zero-field markers never exercise, is still open — `Ledger`'s own
  carriers (`Transfer<S, Token>` wrapping a real payload) are exactly
  that case, not yet exercised by any auto-generated harness (no such
  generation exists yet; today's Kani contracts on `Ledger`'s methods
  are still hand-authored, per Step 3).

## Open, non-blocking implementation questions

- Exact module for `State<V>`'s blanket impl (`amenable_core`,
  alongside `Evidence`/`Witness`, is the natural home) — resolved:
  landed in `amenable_core::state.rs`.
- What `#[derive(StateMachine)]`'s static assertion should check for a
  `capture_exchange_body`-shaped edge with no `Exchange` impl to name —
  resolved: the premise was wrong. `capture_exchange_body` now always
  generates a real `Exchange` impl, so there's nothing left to
  special-case (see Step 5's own account above).
