# State Machine Derivation Plan

## Status

✅ Steps 0-5, every step this plan scoped, done and verified for real
on all three backends. Step 4 landed in two parts (Creusot, then Verus
— see Step 4's own account below); Step 5 landed out of order, ahead of
Step 4, by direct instruction, once Step 3 surfaced the real gap it
fixes. `KaniCompose` routing for data-bearing carriers (flagged in Step
5's own account) is a real, deliberate follow-on, not part of this
plan's original numbered scope. Step 0: `amenable_core::State<V>`
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

Step 4's Creusot half done too, by direct instruction after Step 5:
"I really don't want creusot to employ any mirrors... macros are how we
keep the generated code faithful." `amenable::creusot_export` (the
generator behind `emit-creusot-companions`) used to stop at a
same-named inherent method — no `Exchange` trait impl at all, the
identical gap Step 5 closed for `capture_exchange_body`, just in a
different generator. It now unconditionally emits a real, concrete
`impl amenable_core::Exchange<Input, Output, CreusotVerifier> for
{self_ty}` alongside the harness-captured method, delegating to it —
concrete, not generic over `V` (unlike `capture_exchange_body`'s own
generated impl for `Ledger`): every edge this generator connects is a
plain, non-generic method on the Creusot mirror. The mirror *types*
(`Green`/`Established`/`Stoplight` itself) still can't go away — the
real transition bodies construct tokens through constructors
deliberately private to `amenable_kani`, and widening them would be a
real, permanent weakening, not a free cleanup — but the generated
`Exchange` impl is now real and faithful, not a lesser same-named
stand-in.

Two real toolchain findings along the way, neither guessed at, both
confirmed the hard way:

- **The generated `exchange()` method needed its own `#[ensures(..)]`,
  not just a body that calls the already-contracted inherent method.**
  Creusot has no mechanical call-through the way Kani's/Verus's
  `Ensures<V>` dispatch does, and only checks what a function's own
  contract actually states — a first version omitted this, and
  `cargo creusot prove`'s own file count went up as expected (new
  functions get their own proof obligation regardless of contract
  content) even with a fabricated `Err(..)` swapped into the body,
  producing zero failures. The injected-bug check on `Stoplight`
  specifically was misleading at first too: every `Stoplight` edge's
  `creusot_ensures` defaults to the literal `"true"`, so a vacuous
  contract there proves nothing either way — the real check moved to
  `Ledger::commit`, which has genuine predicate content, and correctly
  failed (`Goal Coma.vc_exchange_Ledger: ✘`) once the delegation was
  broken, confirming the fix for real. Creusot's proof-transparency
  check then required widening three real mirror fields to `pub`
  (`Amount`'s tuple field, `TransferPayload::from`/`to`/`amount`) — the
  new trait method is necessarily as visible as the (public) `Exchange`
  trait itself, more visible than the private inherent method its
  `#[ensures(..)]` used to sit on alone, the identical constraint
  `GAAP_LEDGER_PLAN.md`'s Step 6 already hit once before.
- **Applying `#[derive(StateMachine)]` to Creusot's own mirror
  `Stoplight` surfaced a real, confirmed `creusot-rustc` ICE** (a
  compiler panic in `naming.rs`'s `ComaNames::get`, "no entry found for
  key", during `translate_function` — not a lint), isolated by
  temporarily emitting only the static assertions with no trait impl
  (still panicked) and then only the trait impl with no assertions
  (compiled clean): the closure-nested-generic-function shape Steps 1-3
  used for the static assertion (`const _: fn() = || { fn assert(..)
  {} assert::<Self>(); };`) is what `creusot-rustc` can't translate,
  not `audit_surface()`'s `inventory` call as first assumed. Fixed by
  restructuring to one shared, top-level generic checker function per
  block plus a plain `const _: fn() = checker::<In, Out, Self>;`
  reference per edge — no closure, no nested function definition,
  still forcing the identical compiler-enforced bound (instantiating
  the reference requires the bound to hold, the same way a direct call
  would) without the construct `creusot-rustc` chokes on.

A real, direct correction along the way, not caught before landing: the
first fix for the ICE (before isolating its real cause) assumed
`audit_surface()`'s ungated `inventory::iter` call was the problem and
baked a `#[cfg(not(creusot))]`/`#[cfg(creusot)]` split into the shared
derive output *unconditionally* — which meant every crate using this
derive, including `amenable_kani`/`amenable_gaap`, needed `cfg(creusot)`
added to their own `Cargo.toml` `check-cfg` lists to silence an
`unexpected_cfgs` warning. Direct pushback: "what the heck is creusot
doing in the kani crate — we have been over this three times." Real,
same-class violation as the Cargo-dependency version of this rule
already caught and reverted twice in `EXCHANGE_PROOF_DERIVATION_PLAN.md`'s
own history, just restated one level down (a cfg *name*, not a
dependency edge). `~/repos/elicitation`'s `UNEXPECTED_CFGS.md` was read
directly at this point — real, relevant prior art (their proc macros
hit the identical `unexpected_cfgs`-from-macro-output problem,
solved with an `#[allow(unexpected_cfgs)]`-wrapped `const`/`mod`) but
solving a broader version of the problem than this one needed: their
macros unconditionally emit `cfg(kani)`/`cfg(creusot)` into *every*
caller, so every downstream consumer needs the suppression. This
derive doesn't have to be that blunt — the fix landed as `translator_cfg
= "creusot"`, a new, *opt-in* per-block argument: `audit_surface()`'s
cfg split is only generated for a block that explicitly asks for it
(only `amenable_creusot`'s own `Stoplight` block does), so no cfg name
ever reaches a crate that doesn't already, legitimately, know about it
— zero `Cargo.toml` changes needed anywhere, zero `#[allow]` needed
anywhere either.

Verified for real: `cargo creusot -- -p amenable_creusot` translates
clean (both fixes confirmed independently — the ICE gone, no
`unexpected_cfgs` warnings anywhere in the workspace), `cargo creusot
prove -- -p amenable_creusot` reports `Proved (153 files) ✔` (up from
149 right after the `Exchange`-impl addition, up from 142 before this
step), full workspace `check`/`clippy -D warnings`/`fmt`/`test` clean,
a real Kani spot-check (`stoplight::verify_green_transitions_only_to_yellow`)
still passing. Verus's own half of Step 4 is still open — confirmed
separately (`amenable_verus` has no dependency on `amenable_derive` or
`inventory` at all, and `verus --crate-type=lib` resolves no external
crate under any circumstances), it needs a hand-built `macro_rules!`
equivalent of this derive, not a reuse of it — not started.

Step 4's Verus half done too, landing far more smoothly than Creusot's
— no ICE, no incorrect first attempt, real on the first try. `verus_
state_machine!` (`amenable_verus::exchange_support`) is the `macro_
rules!` counterpart the derive itself can never be here, matching the
existing `verus_sidecar!`/`verus_ensures!`/`verus_exchange!` family
exactly. One real, structural difference from the proc-macro derive: a
`macro_rules!` macro has no way to look up a declared state's carrier
type by name at expansion time (pattern matching and repetition only,
no lookup), so `edges` names each carrier explicitly per edge rather
than resolving it through a separate `states` table the way `#[derive(
StateMachine)]`'s own `state(..)`/`edge(from, to)` split does —
`states` itself stays a plain name list, since `states()`'s own report
needs no carrier information at all. The static-assertion shape (a
shared top-level generic checker function plus a plain `const _: fn()
= checker::<..>;` reference per edge) carried the Creusot-side
correction over directly rather than risking the same class of
translator trouble a second time — never actually reproduced under
Verus, confirmed by testing the original closure-nested shape wasn't
even attempted here. Deliberately not wrapped in a nested `verus_
builtin_macros::verus! { .. }` invocation the way its three siblings
all are: everything generated (`StateMachine`'s three methods, the
checker function, the assertions) is plain, unannotated Rust with no
`spec`/`open`/`ensures`/named-return-value syntax, so it needs no
Verus-specific macro treatment at all — that trick exists only for
output that itself contains Verus-only syntax. `audit_surface()` is
honestly, permanently empty: `amenable_verus` has no `inventory`-backed
registry to query, the same real gap `verus_ensures!`'s own doc comment
already states.

Applying it to the gallery's own `Stoplight` needed one small,
already-known preparatory step: `amenable_core::state_machine`
(`StateMachine`/`Transition`/`TransitionAudit`) joined the existing
`#[path]`-included trait-family file list in `amenable_verus::lib.rs`,
the same real, unmodified-source-inclusion mechanism every other
`amenable_core` trait already uses there, needing no Cargo dependency.

Verified for real: `verus --crate-type=lib` reports `491 verified, 0
errors` (unchanged from before this step — the new code carries no
`ensures`/`requires` content of its own, matching `states()`/
`transitions()`/`audit_surface()`'s identical shape on the other two
backends, so it adds no new verification obligations, just real
compile-time-checked structure), confirmed non-vacuous by a real
injected-bug check (swapping one edge's declared `to`-carrier for the
wrong state) producing a precise `E0277`-style failure pointing at
`__verus_state_machine_edge_checker`'s own bound, reverted. Full
workspace `check`/`clippy -D warnings`/`fmt`/`test` clean throughout,
including `cargo check -p amenable_std --features verus`/`-p amenable
--features verus`. Step 4 is complete, both backends.

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
- **Step 4** — Extend Creusot and Verus coverage for `Stoplight`. Done,
  both backends. Creusot half landed out of order, after Step 5
  surfaced it needed a real `Exchange`-impl fix first (see its own
  account above for the generator fix, the two real toolchain findings
  — a proof-transparency visibility requirement and a genuine
  `creusot-rustc` ICE from the static assertion's original closure-
  nested-function shape — and the `translator_cfg` correction). Verus
  half confirmed separately that `amenable_verus` cannot use the derive
  at all (`verus --crate-type=lib` resolves no external crate, proc-
  macro or otherwise, under any circumstances), so it's a hand-built
  `verus_state_machine!` `macro_rules!` macro instead, matching the
  existing `verus_sidecar!`/`verus_ensures!`/`verus_exchange!` family —
  landed cleanly on the first real attempt, no ICE.
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
