# Exchange Method Proof Derivation Plan (Kani + Creusot)

## Status

Step 0 (the `Sidecar<V>` trait-family redesign) is done and verified —
see "Foundational Trait-Family Redesign" below, including a real
design reversal mid-implementation (an `Evidence<V>` supertrait
approach was fully built, then reverted once it broke unrelated
verifier-agnostic code). Step 1's Kani side is done and verified —
all three `stoplight.rs` edges now carry real, `cargo kani`-confirmed
contracts, with a real Kani tooling limitation discovered and worked
around along the way (see Step 1 below and "Gotchas found directly in
this project"). Step 1's Creusot side is done and verified for all
three edges — a real `cargo creusot prove` pass (`Proved (110 files)
✔`), using a locally-owned accommodation model that genuinely
implements the real `amenable_core` trait family (`Evidence`,
`Witness<V>`, `Sidecar<V>`, `Establish<C, V>`) rather than a flattened
stand-in — a real correction found and applied while starting Step 2,
not assumed from the start (see Step 1 and the now-resolved "Creusot
compilation model" open question). Step 1 is complete. Step 2 is also
complete, but not as originally framed: rather than a `syn`-based
generator deriving the Creusot mirror from the real Kani source (not
well-defined — the two bodies differ by one necessary, documented
transform, not a name substitution), it's a real consistency-check
test (`stoplight_mirror_consistency_test.rs`) that parses both real
sources off disk and asserts they match modulo that one transform,
validated by injecting and reverting drift on both sides. See Step 2
below. Step 3 is also complete, also not as originally framed: the
plan's original sketch put the generated contract directly on
`Exchange::exchange`, but Step 1 already found that has to live on a
plain inherent method instead; and Step 1's Creusot side turned out to
need no new generation at all (it was already minimal, free-function,
`harness!`-wrapped). What landed is `#[amenable_derive::exchange(..)]`
on the Kani-side inherent-method `impl` block, generating the
`Witness<V>`/`ProofRecord`/`Exchange`-impl trio while deliberately
leaving the contract, body, and `harness!` invocation hand-written —
see Step 3 below for why and how it was verified (`cargo expand`
byte-for-byte match, real `cargo kani` re-verification of all three
edges, a real injected-regression check). Step 4 is also complete,
after a real dead end: `std::convert::Infallible` (the edges' original
`Error` type) is uninhabited by design, and `#[kani::stub_verified]`
composition needs `Arbitrary` for the whole return type including the
`Err` side — mathematically impossible for a genuinely uninhabited
type, confirmed empirically across three attempts, not assumed. Fixed
by trading `Infallible` for a real, ordinary, never-actually-
constructed `StoplightError::NotUsed` variant instead, an approach the
user supplied after the dead end was surfaced rather than pushed
through unilaterally — see Step 4 below. Step 5 is also complete:
`impl Amenable for Stoplight` is the first real occupant of
`kani_surface()`/`creusot_surface()`/`verus_surface()`/`audit_surface()`
anywhere in the tree, every method backed by real queried/referenced
data rather than a hand-typed list — including a new, real, feature-
gated Cargo dependency (`amenable_kani`'s own `creusot` feature) added
specifically so `creusot_surface()` isn't a drift-prone hand-typed name
list — see Step 5 below. Step 6 is also complete: all three edges'
DFCC `#[kani::ensures(...)]` closures now call through real,
registered `Ensures<KaniVerifier>` impls (`kani_ensures!`) instead of
restating the boolean inline, closing the same hand-authored-claim-
can-drift gap one level deeper than Step 3 did — see Step 6 below.
Step 7 is also complete: `#[amenable_derive::exchange(..)]` now
generates that same DFCC `ensures` attribute itself, rather than
requiring it hand-typed at each of the three call sites — the actual
predicate stays exactly as hand-authored as before, in the same
`kani_ensures!` invocation, just no longer re-typed as a closure per
edge — see Step 7 below. Step 8 is also complete: Step 5's `creusot_
surface()` had added a real Cargo dependency from `amenable_kani` to
`amenable_creusot` — a genuine violation of verifier-backend
independence, caught by direct correction and fixed by having `amenable
_creusot::stoplight` register its own `ProofRecord`s (`#[cfg(not(
creusot))]`-gated, confirmed to work in an isolated probe) for `creusot
_surface()` to query via the shared `amenable_core` registry instead of
importing across crates — see Step 8 below. Step 9 is also complete:
Step 8's own `#[cfg(not(creusot))]`-gating fix was real but still a
patch around `inventory`'s repeated friction with Creusot's
translator, not a fix for the friction itself — replaced with a real
codegen layer (`amenable::creusot_export`/`emit-creusot-companions`,
mirroring `emit-verus-witnesses`'s own already-shipped architecture)
that generates `amenable_creusot`'s per-edge companions directly from
`amenable_kani`'s real, captured transition bodies, eliminating both
the hand-written mirror and `stoplight_mirror_consistency_test.rs`
entirely — see Step 9 below.

## Motivation

`amenable_core`'s existing derive-witness composition (see
`VERUS_DERIVE_WITNESS_COMPOSITION_PLAN.md`) mechanizes proofs of
**static properties of data**: a struct/enum's own claim follows from
its fields'/variants' own already-proven claims, by structural
conjunction/case-split. That shape has a generic, type-structure-only
composition rule a macro can walk mechanically.

`Exchange<Input, Output>` (`crates/amenable_core/src/exchange.rs`)
proves a categorically different kind of claim: a Hoare triple over a
real method body — `{P(input)} exchange {Q(output)}`. There is no
type-structure-only composition rule for this; correctness depends on
the transition's actual control flow and logic, which a derive macro
cannot discover from field lists the way it can for `Witness`
composition. `docs/AMENABLE_PLAN.md`'s "States Are Roots, Transitions
Are Relations" section names a related split — *runtime* state
occupancy ("the light is currently Green") is asserted `Provenance`,
not derived, since there's no computation behind a bare "we are here"
fact. An earlier draft of this section over-read that as "root states
are categorically unproven" and treated `Established::root()`'s
`Witness`-free bypass as presumptively correct; "Foundational
Trait-Family Redesign" below corrects that — root-ness and whether a
*type's construction* has real content to prove are orthogonal axes,
and every `Evidence` value needs a real `Witness<V>` proof regardless
of which one it is.

**The concrete problem this plan closes:** the one real `Exchange`
implementation in the tree, `crates/amenable_kani/src/stoplight.rs`,
doesn't actually prove its own transition body. Each `Stoplight::
exchange()` impl's real logic is e.g. `Yellow::establish(input.
sidecar())`, but the Kani harness backing `Yellow`'s `Witness<
KaniVerifier>` proof calls a *different*, free-standing function,
`next(color)`, that's documented as mirroring the real transition
logic but is never called by `exchange()` itself. Nothing but a doc
comment keeps them in sync. If `exchange()`'s real branching changed
without a matching edit to `next()`, Kani would keep passing on a
proof that no longer describes the code that runs — the same
hand-authored-claim-can-drift-from-source risk this session's
`Ensures`/`Requires` macro work closed for descriptive text, one level
more dangerous here because what could drift is executable logic, not
a string.

### Prior art: `~/repos/elicitation`

Read directly (not summarized from memory) before writing this plan:
`crates/elicitation_derive/src/formal_method.rs`, `crates/
elicitation_derive/src/derive_vsm.rs`, `crates/elicitation/src/
contracts.rs`, and — the authoritative operational reference,
essential reading before Step 1, not just the macro source —
`KANI_FOR_VSMS.md` at the elicitation repo root. That document records
what was tried and rejected, the current production architecture, and
a large real gotcha catalog; it supersedes parts of what the macro
source alone suggests (see the correction below).

Real, working machinery, structured around three traits — `Prop`,
`Established<P>`, `FormalMethod<In, PIn, Out, POut>` (blanket-impl'd
for any `Fn(In, Established<PIn>) -> (Out, Established<POut>)`) — plus
`VerifiedStateMachine`/`#[derive(VerifiedStateMachine)]` to aggregate a
declared transition list.

**Current production architecture, per `KANI_FOR_VSMS.md` §6** (this
corrects an imprecision in an earlier draft of this plan, which
described "a Kani-contracted wrapper delegating to the real body" in a
way that could be misread as a *call-through* wrapper): contracts
(`#[kani::requires]`/`#[kani::ensures]`) must sit on the function whose
body is actually checked — either the original function directly, or a
companion carrying the exact same body inlined verbatim (elicitation's
own `_kani_contracted` wrapper, generated only to dodge a `cfg`-token-
leak lint issue, not to add indirection). A wrapper that *calls* the
original causes DFCC to inline both the wrapper and the call,
doubling CBMC's work and causing real timeouts — confirmed the hard
way in their own gallery. The harness itself uses "forgive-and-
forget": build a fully symbolic value via `KaniCompose::kani_any()`,
`kani::assume(predicate(&value))` to restrict to valid inputs, `std::
mem::forget(value)` to avoid drop-glue reasoning on the symbolic value,
then rebind a fresh, concrete `KaniCompose::kani_depth0()` shadow to
actually pass into the call. The per-variant/per-depth harness
generation this plan's earlier draft implicitly modeled Step 1 on
(`KaniVariantState`, N×M×3 generated harnesses) is elicitation's own
**superseded, now-diagnostic-only** design — 54× more generated code
for no additional proof coverage than the `proof_for_contract` +
forgive-and-forget architecture their own document recommends
replacing it with. This plan follows the current design, not the
superseded one.

What's genuinely reusable technique, not just scaffolding:

- `#[formal_method]` generates a Kani-contracted wrapper that
  delegates to the **real, unmodified transition body**, carrying real
  `#[kani::requires]`/`#[kani::ensures]` (Kani's native DFCC contract
  mechanism) — not a disconnected proxy function.
- A `#[kani::proof_for_contract]` closure checks that contract once;
  afterward `stub_verified()` lets other proofs treat the transition
  as an axiom instead of re-exploring its body on every call site —
  real modular/compositional verification, standard Kani practice.
- Creusot gets the same real-body-checking property directly: a
  companion function (real body, or a delegation-rewritten clean copy)
  carries real `#[requires]`/`#[ensures]`, checked by `cargo creusot`.

What is **not** being ported, because `amenable_core` already has a
strictly stronger version of the same idea:

- `Established::assert()` is a documented, compiling escape hatch —
  "callers take responsibility for ensuring P genuinely holds."
  `amenable_core::ProofToken` has no equivalent: its implementors'
  fields are private, so the only way to hold one is a lawful
  `Establish::establish()` call, and `Establish<C, V>: Evidence +
  Witness<V>` means that impl cannot exist without a real, matching
  `Witness<V>` proof already existing. This plan must not introduce an
  `assert()`-shaped hole to make codegen easier.
- `Prop::kani_invariant_fn_name()`/`creusot_invariant_fn_name()`
  default to `""`, silently falling back to a trivial `requires true,
  ensures true` / `#[trusted]` contract if a human forgets to name the
  real predicate. `Witness<V>` has no default `proof()` (Phase 4 of
  the Verus plan already eliminated free-boolean/tautological proofs
  project-wide), and this session's `verus_ensures_predicate!`/
  `verus_requires_predicate!` macros made naming the real predicate a
  **mandatory** positional argument with a real `compile_error!` on a
  missing/wrong name, never a silent optional fallback. This plan
  follows that same discipline for Kani/Creusot predicates.

This mirrors a decision already made once for `Establish` itself: see
`PROVABLE_FROM_PLAN.md`, which fixed `Establish::establish` to actually
require `C: ProofToken` (closing a real, live bug where `input.
primary()` — the unguarded payload — could be used in place of `input.
sidecar()` — the real token) and explicitly chose to reuse `ProofToken`
as the credential bound rather than invent a bespoke marker-struct
mechanism like elicitation's `Established<P>`. This plan is the same
kind of move one layer up: reuse `Exchange`/`Witness<V>` rather than
import a parallel trait family.

### Existing amenable hooks this plan is the first real occupant of

- `amenable_core::StateMachine`/`Amenable` (`crates/amenable_core/src/
  state_machine.rs`) already declare `kani_surface()`/
  `creusot_surface()`/`verus_surface()`/`audit_surface()`. Grepping the
  whole tree turns up zero `impl Amenable for` anywhere — this is an
  aspirational surface with no occupant yet.
- `amenable_derive::harness!` (`crates/amenable_derive/src/harness.rs`)
  already captures a Kani proof's verbatim source and registers a
  `KaniProofRegistration`/`inventory` record, but only for a plain
  `#[kani::proof]` function — no `requires`/`ensures`/
  `proof_for_contract` support exists anywhere in the tree today
  (confirmed by grep: zero hits for `kani::requires`, `kani::ensures`,
  `proof_for_contract`, or `stub_verified` in `crates/`). This plan
  introduces Kani function-contract verification to the project for
  the first time.
- `amenable_creusot/src/rust_std.rs` already uses real `#[requires]`/
  `#[ensures]` (via `creusot_std::macros`) on `extern_spec!` blocks
  checked against real std behavior — the Creusot side of this plan
  extends an already-proven pattern rather than introducing a new one.

## Foundational Trait-Family Redesign: `Sidecar<V>` — Implemented

**Status: done, verified, landed** (`cargo check`/`test`/`clippy --all-
targets --all-features -D warnings` all clean across the whole
workspace). This section is written after the fact and describes the
actual shape, including a real design reversal mid-implementation —
kept in, not cleaned up, because the reasoning trail is exactly what a
future reader needs to not repeat the mistake.

Discovered mid-design: `Sidecar::Proposition: Evidence` cannot express
"this is actually backed by a real proof" at all, for *any*
transition, no matter what Kani/Creusot mechanics Steps 1-5 build.
`Evidence` is deliberately verifier-blind (its own doc comment:
"Evidence says nothing about verifier backends... consuming a chain of
evidence to produce a backend-specific proof is `Witness`'s job"), so
`Exchange`'s own doc comment's claim — "an exchange only ever consumes
an input whose precondition is already proven... produces an output
already bundled with the proof of its postcondition" — was
aspirational prose, not something the type system enforced.

### Correcting a premise this plan's own Motivation section overstated

An earlier working draft of this discussion read `AMENABLE_PLAN.md`'s
"States Are Roots, Transitions Are Relations" section as: root states
are categorically asserted `Provenance`, never `Witness`-proven, by
design, forever. That's a conflation, not a real design constraint.
Two different things were being collapsed into one:

- Whether *a running system currently occupies a given state* — "the
  light is currently Green" — is genuinely asserted; there's no
  computation to point to for that runtime fact, and `AMENABLE_PLAN.md`
  is right about that.
- Whether *a given type's constructor has real invariant content to
  check* is a completely different, orthogonal axis. `Green` is a
  zero-field unit struct with an infallible constructor, so a
  `Witness<KaniVerifier>` proof for it, if it existed, would be real
  but trivial — there's nothing to violate, so nothing to falsify. A
  type like `Polygon`, constructed under real constraints (minimum
  vertex count, non-self-intersecting edges), earns a genuinely
  substantive `Witness<V>` proof under the exact same rule — and
  `Polygon` could just as easily be some other machine's root/initial
  state. "Root" and "trivial-vs-substantive" don't track each other at
  all; treating root-ness as a categorical exemption from proof was
  the error.

### Three rejected designs, one fully built then reverted

1. **Add a `Witness<V>` bound only to `Exchange`, leaving `Evidence`
   untouched.** Proposed and retracted before any code was written. It
   treats the symptom where it happened to be noticed rather than the
   actual source, and required bolting a third, ad hoc `V` parameter
   onto `Exchange` specifically — ugly, and wouldn't have protected any
   other future consumer of the same gap.
2. **Carry `V` as an associated type on `Sidecar`** (`type
   BackingVerifier: Verifier;`) instead of a generic parameter. Also
   retracted before code. An associated type is only checked where a
   caller remembers to write an explicit equality constraint — easy to
   omit partway through a longer composition chain, silently permitting
   a pipeline that mixes a Kani-backed step with a Creusot-backed step.
   A shared generic parameter makes that a plain type error everywhere
   instead, with nothing to remember — this part of the reasoning
   carried forward into the design that actually landed.
3. **Add `V: Verifier` directly to `Evidence` itself as a supertrait
   bound** (`Evidence<V>: Witness<V>`), so *every* evidence chain
   requires a real proof, no exceptions. This one was fully implemented
   across `amenable_core` and `amenable_derive` (including new
   `impl_tuple_witness!`/blanket-conditional-impl machinery in the
   `#[derive(Standard)]`/`#[calculation]` macros) before `cargo check
   --workspace` surfaced the real cost: `amenable_std::
   write_rust_std_certificate_artifacts` — a generic provenance/audit-
   report writer with no relationship to any specific verifier's proof
   — could no longer compile, because its generic helper needed
   `RustStdStandard<T>: Witness<V>` for arbitrary `T`, which only ever
   holds for specific concrete `T` in practice. This exposed that
   `Evidence` legitimately has verifier-agnostic uses throughout the
   tree (lineage/audit bookkeeping, provenance reporting) that must
   never be forced to name a backend just to compile — the same root
   mistake as the "roots are categorically unproven" conflation above,
   one level further out: conflating "evidence exists" with "evidence
   is provably correct under some specific backend" is wrong for the
   same reason conflating "root" with "trivial" was. Fully reverted
   (`Evidence`, `Witness::SupportingEvidence`, `Standard`, `cert.rs`,
   the derive macros, and their test fixtures all restored to their
   pre-change shape) in favor of design 4 below, which does not have
   this cost because it states the requirement only where it's actually
   needed.

### The actual design — `Sidecar<V>`, not `Evidence<V>`

`Evidence`/`ProofToken`/`Standard` stay exactly as they were: bare,
unparameterized, verifier-agnostic. The real proof-bearing requirement
is stated as a compound bound at the one place that actually needs
it — `Sidecar`'s own `Proposition` — using the same shape `Establish
<C, V>: Evidence + Witness<V>` already used before any of this work
started (this design didn't invent a new pattern, it just noticed that
one already existed and replicated it one layer up):

```rust
pub trait Sidecar<V: Verifier> {
    type Primary: Evidence;
    type Proposition: Evidence + Witness<V>;
    type SidecarToken: ProofToken<Proposition = Self::Proposition>;
    fn primary(&self) -> &Self::Primary;
    fn sidecar(&self) -> Self::SidecarToken;
}

pub trait Exchange<Input, Output, V: Verifier>
where
    Input: Sidecar<V>,
    Output: Sidecar<V>,
{
    type Error;
    fn exchange(&self, input: Input) -> Result<Output, Self::Error>;
}
```

`V` is still an explicit, shared generic parameter (not an associated
type — design 2's reasoning still holds), so an `Exchange` impl still
can't mix a `Sidecar<KaniVerifier>` input with a `Sidecar<
CreusotVerifier>` output; `V` simply fails to unify. What's different
from design 3: `Primary` stays plain `Evidence` (the raw payload
doesn't need proving, only the proposition carried alongside it does),
and nothing outside `Sidecar`/`Exchange` changes at all — `Evidence`,
`ProofToken`, `Standard`, `Establish` are all textually identical to
before this plan started touching anything.

### Actual blast radius (measured, not estimated)

Landed with a four-file diff: `evidence.rs` (doc comment only, no
functional change), `exchange.rs` (`Sidecar<V>`/`Exchange<Input,
Output, V>`), `roles.rs` (doc comment only), and `stoplight.rs` (the
one real consumer — three `Exchange` impls plus the `Sidecar` impl
gained `KaniVerifier`, and `Established::root()`'s doc comment was
corrected to state precisely what's proven: the type-level `Green:
Witness<KaniVerifier>` bound is satisfied by the *reused* "Red cycles
back to Green" proof, opportunistically, not because anything proves
the root claim itself — the root claim stays exactly what it was,
`Provenance`-backed, not model-checked). No `ProofToken` impl anywhere
in the tree needed touching — confirms design 3's `ProofToken<V>`
change was pure unnecessary blast radius design 4 avoids entirely.
`amenable_std`/`amenable_creusot` needed zero changes; they never used
`Sidecar`/`Exchange` in the first place.

## Scope

Kani and Creusot **first** — a sequencing decision, not a decision to
drop Verus support. Verus support for `Exchange` is a real, standing
goal; it is not covered by *this* document because it does not yet
have a real answer, and bolting a weak one on here (an axiom dressed
up as a proof) would be worse than leaving it explicitly open.

The reason it needed its own separate plan rather than a third column
in this one: Verus cannot check an arbitrary compiled Rust function
body at all — it only checks code written inside its own `verus! { }`
macro — so elicitation's Verus path for transitions is `assume_
specification`/`#[verifier::external]` (an axiom, never checked)
regardless of whether an invariant predicate is named. Closing that
for real looked like it would mean either duplicating real transition
logic into a `verus!{}`-native companion (reintroducing, for executable
logic, the exact drift risk this session eliminated for descriptive
text) or requiring transition bodies be Verus-native from the start —
neither of which is a small extension of the Kani/Creusot design below.

**Resolved.** A real third option neither prediction above considered:
`#[path]` `mod`-inclusion brings `amenable_core`'s real trait-family
source into Verus's own single-file-tree compilation verbatim, with no
Cargo dependency and no duplication — genuinely better than
elicitation's own most mature Verus pattern (`assume_specification`
axiomatizes the real body; this checks it directly). See
[VERUS_EXCHANGE_PROOF_DERIVATION_PLAN.md](VERUS_EXCHANGE_PROOF_DERIVATION_PLAN.md)
for the full story, including two real, distinct Verus limitations
found and fixed along the way and a real methodological mistake caught
and corrected mid-investigation.

## Non-goals

- No parallel `Prop`/`Established`/`FormalMethod` trait family in
  `amenable_core`. `Exchange`/`Establish`/`Witness<V>`/`ProofToken`
  already occupy that role and stay the single source of truth.
- No weakening of `ProofToken`'s private-field/`establish()`-only
  construction discipline for codegen convenience.
- No optional/defaulted-empty invariant-predicate naming. Naming the
  real predicate is a mandatory macro argument; a missing or
  nonexistent predicate is a real `compile_error!`, never a silent
  trivial contract.

## Design

### Step 0 — land the `Sidecar<V>` trait-family redesign — done

Implemented per "Foundational Trait-Family Redesign" above:
`Sidecar<V: Verifier>` with `Proposition: Evidence + Witness<V>`,
`Exchange<Input, Output, V>`. `Evidence`/`ProofToken`/`Standard`/
`Establish` stay exactly as they were — the design-3 attempt to change
them was built, then reverted, once it broke `amenable_std`'s generic
provenance reporting; see that section for the full story. Migrated:
`stoplight.rs`'s three `Exchange` impls and its one `Sidecar` impl
(the only real consumer in the tree), plus `Established::root()`'s doc
comment, corrected to state precisely what's proven (the type-level
bound is satisfied by the reused Red→Green proof, opportunistically;
the root claim itself stays `Provenance`-backed, not model-checked).
`cargo check`/`test`/`clippy --all-targets --all-features -D
warnings --workspace` all clean.

### Step 1 — one real worked example by hand — Kani side done

Rebuilt all three `stoplight.rs` edges (Green→Yellow, Yellow→Red,
Red→Green — the same mechanical pattern three times by hand, not a
macro; "one real worked example" turned out to mean "one real shape,"
and the whole cycle needed fixing together since Kani compiles the
entire `#[cfg(kani)]` surface regardless of which harness is targeted).

**A real, load-bearing discovery, not anticipated by this plan or by
`KANI_FOR_VSMS.md`:** Kani 0.67.0 cannot place a contract
(`#[kani::requires]`/`#[kani::ensures]`/`#[kani::proof_for_contract]`)
on a trait method when the trait itself is generic. `Exchange<Input,
Output, V>` is generic, so `#[kani::proof_for_contract(<Stoplight as
Exchange<...>>::exchange)]` fails outright with "Kani does not
currently support stubs or function contracts on generic functions in
traits" (a real compiler error, confirmed directly, not inferred —
see the linked upstream issue in the error text). This is a tooling
limitation, not a project convention, and it forced a real design
change: the real logic and its contract now live on a plain inherent
method (`Stoplight::green_to_yellow`, `::yellow_to_red`, `::
red_to_green`), and each `Exchange::exchange` impl is reduced to a
single-expression delegation (`self.green_to_yellow(input)`) — not a
proxy with different logic (the exact failure mode this whole
exercise exists to close), the *same* body, relocated to a shape Kani
can actually contract.

**The contract content is legitimately trivial, confirmed rather than
assumed to be a corner-cut:** every state type here (`Green`,
`Yellow`, `Red`, and their tokens) is a zero-field type with exactly
one possible value, and none of the three bodies branch or can panic.
Once the type system enforces "Green's only lawful successor is
Yellow" (there's exactly one `Exchange` impl for that `Input` type),
there's no nontrivial claim left for Kani to check — the honest
content is `#[kani::ensures(|result| result.is_ok())]`, proving the
body never panics and always returns `Ok`. This is the same "trivial
because construction is infallible, not because it's exempt from
proof" principle Step 0 established for `Green` itself, one level up.

**Removed `Color`/`next()`.** Confirmed by grep before removing, not
assumed: used nowhere except the three now-replaced disconnected
harnesses — not in any test, not in the real `exchange()` bodies. Kept
the `Color` enum alone as `StateMachine::State` (purely descriptive,
backs nothing, explicitly flagged in its own doc comment as still an
open question), dropped `next()` entirely.

**Regression check, run for real:** temporarily added `panic!(...)`
to `Stoplight::green_to_yellow`'s body, re-ran the exact same
`cargo kani` harness — verification failed, pointing at the exact
injected line. Reverted; re-ran; verification passed again. The
`derived_witness/`-style "prove the fix catches a real regression, not
just that it currently passes" discipline from the Verus work carried
over cleanly.

**`-Z function-contracts` wired into the justfile** as a new
`verify-kani-contract` recipe (kept separate from the existing
`verify-kani`, which serves the ~400 pre-existing non-contract
harnesses in this crate — no reason to risk that recipe for this).
`-Z stubbing` included preemptively for when Step 4 needs
`kani::stub_verified` composition. Contract-harness names include
their module path (discovered via `cargo kani list -Z
function-contracts` from inside `crates/amenable_kani` — `--harness`
with a bare function name doesn't resolve; `--exact` needs the full
`stoplight::verify_green_transitions_only_to_yellow` form).

All three edges independently verified with real `cargo kani` runs
(`stoplight::verify_green_transitions_only_to_yellow`, `::
verify_yellow_transitions_only_to_red`, `::
verify_red_transitions_only_to_green` — 0 failures each). Full
workspace `cargo check`/`test`/`clippy --all-targets --all-features -D
warnings`/`fmt` all clean; a pre-existing, unrelated Kani harness
(`rust_std::sync_atomic::verify_atomic_bool`) spot-checked to confirm
nothing else regressed.

**Creusot side, also done and verified — the accommodation-model
resolution to "Creusot compilation model" below.** The design question
took real back-and-forth to land: `amenable_creusot` cannot take a
Cargo dependency on `amenable_kani`, `amenable_std`, or `amenable`
itself (any of the three either closes a real dependency cycle —
`amenable_std` already optionally depends back on `amenable_creusot`,
and so does `amenable`, since `amenable_creusot`'s only cycle-safe
consumers are the crates *above* it in the graph, not the reverse — or
would hand `creusot-rustc`'s translator ordinary Rust infrastructure it
has already ICE'd on for real once in this exact codebase, per
`amenable_std::creusot_witness`'s own doc comment: it sweeps *every
local item* in whatever crate it directly compiles, `#[cfg(creusot)]`-
gated or not, and choked on a return-position `impl Trait` and an
`inventory::submit!`-generated `static`). The resolution: reuse the
accommodation-model pattern already established on the Kani side
(`amenable_kani::fs_model`, `::utf8_model`, etc.) — a deliberate,
minimal, *locally-defined* stand-in, not a reference to the real type
at all. `crates/amenable_creusot/src/stoplight.rs` defines sanitized
mirrors (`Green`, `Yellow`, `GreenToken`, `YellowToken`,
`EstablishedGreen`, `EstablishedYellow` — concretely instantiated per
edge, not generic, and carrying none of the `amenable_core` trait-
family machinery, which isn't needed to state the contract) plus one
real contract function, `green_to_yellow`, wrapped in the same
`amenable_derive::harness!` macro the std-type proofs already use.

Verified for real, the same three ways as the Kani side: `just
verify-creusot-translate` and `just verify-creusot` (real `cargo
creusot`/`cargo creusot prove -- -p amenable_creusot`) both succeed —
`Proved (90 files) ✔`, no ICE, no translator crash; a deliberately
injected `panic!()` in `green_to_yellow`'s body made the exact same
`cargo creusot prove` run fail with `Goal Coma.vc_green_to_yellow: ✘`,
confirming the contract is real, not vacuous; reverted, re-verified
clean. Full workspace `check`/`test`/`clippy --all-features -D
warnings`/`fmt` and `just check-all-creusot` all clean afterward.

**All three edges now have a Creusot accommodation model** — `Red`/
`RedToken`/`EstablishedRed` plus `yellow_to_red`/`red_to_green`
followed the identical, now-fully-mechanical pattern (concrete
per-edge mirror types, one `harness!`-wrapped contract function each).
`cargo creusot prove -- -p amenable_creusot` succeeds for the whole
crate — `Proved (92 files) ✔` (up from 90, one new proved file per new
edge) — and both new contracts got the same real injected-`panic!()`
regression check as Green → Yellow, each failing at its own precise
goal (`Goal Coma.vc_yellow_to_red: ✘`, `Goal Coma.vc_red_to_green:
✘`) before being reverted and re-verified clean. Step 1 is complete
for both Kani and Creusot, all three edges, each verified for real.

**Correction found while starting Step 2, applied retroactively to
Step 1's Creusot mirror.** The first version of `amenable_creusot/src/
stoplight.rs` (above) was more flattened than it needed to be: its own
doc comment claimed the mirror types couldn't use any `amenable_core`
trait-family machinery, conflating two different things — "can't
depend on `amenable_kani`/`amenable_std`" (true, real cycle/ICE risk)
with "can't use `amenable_core`'s traits" (false). `amenable_creusot`
already has a real, unconditional Cargo dependency on `amenable_core`,
and `Evidence`/`Witness<V>`/`Sidecar<V>`/`Establish<C, V>`/`ProofToken`
themselves don't contain any of the specific patterns that caused the
real crashes (those were `Provenance`'s `Box<dyn Iterator>` and
`inventory::submit!`, neither of which the mirror needs). Confirmed by
rebuilding the mirror to genuinely implement the real traits — `Green`/
`Yellow`/`Red: Evidence + Witness<CreusotVerifier>`, a real generic
`Established<T, Token>: Sidecar<CreusotVerifier>`, real `Establish<_,
CreusotVerifier>` impls — and re-running the full verification cycle:
`cargo creusot prove` still succeeds (`Proved (110 files) ✔`, more
proof obligations than before since the trait methods themselves now
get checked too), and all three edges still fail their own regression
check when broken. The exchange bodies now use the same call shape as
the real Kani bodies (`Yellow::establish(input.sidecar())`,
`Established::new(Yellow, token)`), differing only in which concrete
types they close over — closer to genuine extraction-with-substitution
territory for Step 2's tool than the flattened version was, and the
reason this correction is documented under Step 2 rather than quietly
folded into Step 1's own history.

### Step 2 — keep the Creusot mirror honest against the real Kani body — done

**Original framing (superseded).** The plan as first drafted assumed
Step 2 would be a `syn`-based extractor that *derives* the Creusot
predicate/body from the real Kani source — i.e. a code generator.
Once both bodies actually existed side by side (Step 1), that framing
turned out not to be well-defined: the real body
(`amenable_kani::stoplight::Stoplight::green_to_yellow`, `Result`-
returning, `&self`-taking) and the mirror body
(`amenable_creusot::stoplight::green_to_yellow`, bare return, free
function) are not the same tokens with names substituted. They
necessarily differ in one specific, documented way — the real body's
trailing `Ok(...)` wrapper, which the mirror's intentionally
un-modeled `Result` has no equivalent for. "Derive B from A" isn't a
well-defined operation when A and B have different shapes by
necessity; Verus's verbatim predicate-text extraction (`amenable_core
::verus_carrier`) works because there the extracted text *is* the
predicate, unmodified.

**What was actually built instead: a consistency check, not a
generator.** `crates/amenable_creusot/tests/
stoplight_mirror_consistency_test.rs` reads
`amenable_kani/src/stoplight.rs`'s real source directly off disk
(`fs::read_to_string` against a path built from
`env!("CARGO_MANIFEST_DIR")` — no Cargo dependency, for the same
reason `amenable_creusot` can't depend on `amenable_kani` at all; see
"Gotchas found directly in this project"), parses it with `syn`,
extracts the real `impl Stoplight { fn NAME }` body, and applies the
one documented transform (strip the trailing `Ok(...)` call — panics
with a clear message identifying the exact shape mismatch if the real
body ever stops matching that shape, rather than silently producing a
wrong comparison). It then parses the mirror's own already-exported
`harness!`-captured verbatim source constants
(`VERIFY_GREEN_TO_YELLOW_EXCHANGE_SRC` et al. — the same mechanism
`amenable_std::creusot_witness` already uses to keep a *reported
claim* honest) back into a function body, and asserts token-stream
equality (`quote!(#block).to_string()`) between the two. Three tests,
one per edge, all passing.

**Validated as real, not vacuous**, by injecting drift on both sides
and confirming a precise failure each time, then reverting:

- Mirror-side drift (`amenable_creusot/src/stoplight.rs`,
  `green_to_yellow`: added a dead `let _drift_check = input.token;`)
  → precise left/right mismatch reported, reverted, re-verified clean.
- Real-side drift (`amenable_kani/src/stoplight.rs`,
  `Stoplight::green_to_yellow`: added a dead
  `let _drift_check = input.primary();`) → same, reverted, re-verified
  clean.

Full-workspace re-verification after landing: `cargo fmt --all
--check` clean; `cargo check --workspace` clean; `cargo clippy
--workspace --all-targets --all-features -- -D warnings` clean;
`cargo test --workspace` 61/61 (up from 60, the +1 test binary being
this file); `just verify-creusot` (real `cargo creusot prove -- -p
amenable_creusot`) still `Proved (110 files) ✔` — confirming the new
`[dev-dependencies]` (`syn`, `quote`, `proc-macro2`, test-only) and
new test file don't disturb the real Creusot toolchain invocation,
since they never enter the `#[cfg(creusot)]`-compiled lib surface.
Step 2 is complete.

### Step 3 — generalize into an attribute macro — done

**Original framing (superseded on two points, both discovered
empirically, not assumed).** The plan as first drafted imagined
`#[amenable_exchange(kani = "predicate_name", creusot =
"predicate_name")]` sitting directly on `fn exchange(&self, input) ->
Result<Output, Error>` and generating the contract itself. Two things
Step 1 already established make that exact shape wrong:

1. The contract and the real body can't live on `Exchange::exchange`
   at all — Kani 0.67.0 rejects contracts on a trait method when the
   trait itself is generic. They live on a plain inherent method
   instead (`Stoplight::green_to_yellow` et al.), with `exchange`
   reduced to delegation. So the macro has to attach to that inherent
   method's `impl SelfType { .. }` block, not to the trait impl.
2. On the Creusot side, the by-hand pattern already **is** about as
   terse as generation could make it: `amenable_creusot::stoplight`'s
   harness functions are free functions with `#[requires(true)]
   #[ensures(true)]` wrapped directly in `harness!` — no per-edge
   `Witness`/`ProofRecord`/`Exchange`-impl scaffolding exists on that
   side to mechanize, because the mirror doesn't implement `Exchange`
   at all (see Step 1's Creusot writeup — it's a proof function over
   the mirror types, not a method on a mirror `Stoplight`). So Step
   3's real scope narrowed to the Kani side; no new Creusot macro was
   needed or built.

**What was actually built:** `#[amenable_derive::exchange(cfg = ..,
verifier = .., evidence = .., proof_artifact = .., harness_fn = ..,
harness_const = .., evidence_id = ..)]` (`crates/amenable_derive/src/
exchange.rs`), applied to the inherent `impl Stoplight { fn
method(&self, input: Input) -> Result<Output, Error> { .. } }` block.
It deliberately does **not** touch two things that already carry
their own real guarantees:

- The `#[cfg_attr(kani, kani::ensures(..))]` contract and the method
  body — real proof content, stays hand-written.
- The `amenable_derive::harness! { .. }` invocation — its verbatim-
  source capture (`Span::source_text()`) only works when the braced
  item is written directly at the call site (see `harness.rs`'s own
  doc comment). Splicing a macro-argument setup block through this
  macro's `quote!` output would silently degrade that capture to a
  token-reconstructed, whitespace-losing fallback — a real, if minor,
  guarantee this macro must not weaken. So `harness!` stays a separate
  invocation immediately below the attribute, unchanged from Step 1.

What it *does* generate, since none of it carries either guarantee
above and all of it was previously identical boilerplate repeated
once per edge: the `Witness<V>` impl for the transition's target
evidence (naming the harness by function/const identifier, closing
the loop into the existing derive-witness composition pipeline the
same way `#[calculation]`-backed leaves already do), the `ProofRecord`
registration backing it, and the `Exchange<Input, Output, V>` impl
that delegates to the real method — `Input`/`Output`/`Error` are
extracted from the method's own signature via `syn`, not re-typed as
macro arguments.

Applied to all three `Stoplight` edges, replacing their by-hand
`Witness`/`ProofRecord`/`Exchange`-impl trio (the cycle-back edge's
`"::cycle_back"` `ProofRecord` id suffix preserved via an optional
`evidence_id` argument). Verified for real, not just "it compiles":
`cargo expand -p amenable_kani stoplight` confirmed the generated code
is byte-for-byte identical to the prior hand-written expansion,
including the `harness!` verbatim-source constant (still capturing
real multi-line source, confirming the harness!-stays-untouched
design choice actually preserves the property it was meant to);
`just verify-kani-contract` re-run on all three harnesses, all still
`VERIFICATION:- SUCCESSFUL`; a real injected-`panic!()` regression
check on `green_to_yellow` failed at the exact injected line
(confirming the macro-generated `Exchange::exchange` delegation still
routes through the real body), then reverted and re-verified clean.
Full workspace `fmt --check`/`check`/`clippy --all-targets
--all-features -D warnings`/`test` all clean (61/61, unchanged from
Step 2 — no test count regression), plus the Step 2 consistency test
re-run unaffected (it reads the method body's tokens, which this
macro never touches). Net effect on `stoplight.rs`: 161 lines changed,
a real ~114-line reduction against a ~24-line addition to
`amenable_derive`, not counting the doc-comment prose added to explain
what moved where. Step 3 is complete.

### Step 4 — modular composition via `stub_verified` — done

Composes all three `Stoplight` edges into one full-cycle harness via
`#[kani::stub_verified]`, which replaces each stubbed call's body
with its already-proven contract instead of re-exploring it — the
first real use of compositional/modular Kani verification in this
codebase; every other `#[kani::proof]` harness here is direct
symbolic execution with no stubbing.

**A real dead end hit and resolved along the way, not smoothed over.**
`stub_verified` needs `ReturnType: kani::Arbitrary` for every stubbed
call, since stubbing reconstructs a symbolic stand-in for whatever the
real call would have returned. `Established<T, Token>: kani::
Arbitrary` was straightforward (own type, own fields — `Established::
new(T::any(), Token::any())`). But every edge's real return type is
`Result<Established<T, Token>, std::convert::Infallible>`, and
`Infallible` is uninhabited by design (the type-level expression of
"this can never fail"). Three attempts to reconcile that with
`Arbitrary`, each a real, confirmed failure, not a hypothetical:

1. Implement `Arbitrary` for the whole `Result<..., Infallible>`
   directly — blocked twice by the orphan rules: first as a generic
   impl over `T, Token` (real `E0117`, "uncovered type parameter"),
   then narrowed to three concrete edge types — still blocked (`Result`
   is foreign and not a `#[fundamental]` type, so no downstream crate
   may implement a foreign trait for it, generic or concrete, full
   stop).
2. Swap `Infallible` for a local uninhabited `enum Never {}`, so
   `impl kani::Arbitrary for Never` becomes legal (local type). It
   compiled, and Kani's own conditional `Result<T, E>: Arbitrary`
   impl (which lives in Kani's crate, not ours, and is why option 1
   was even conceivable) picked it up automatically. But real `cargo
   kani` still failed: Kani's `Result` reconstruction unconditionally
   calls `E::any()` while exploring the stub's symbolic return-value
   space, and CBMC actually executes that call. The only body a
   genuinely uninhabited type's `any()` can have is `unreachable!()`
   (no safe-Rust value of an uninhabited type exists to return), and
   that panics for real under verification — confirmed by the exact
   `internal error: entered unreachable code` failure, not assumed.
   Constructing a value of a truly uninhabited type is impossible, and
   no amount of type-juggling changes that.
3. At this point the honest options seemed to be: accept a real,
   documented limitation and skip to Step 5 with no composition demo,
   or redesign `Exchange`'s core signature (fixed in Step 0) to use a
   locally-owned `Result`-analog whose `Arbitrary` impl this crate
   could actually write — a change disproportionate to what Step 4
   asked for, rippling into the trait family itself. Presented to the
   user as a real fork rather than picked unilaterally.

**The actual fix, from the user:** the root problem was never *which*
crate owns the uninhabited type — it's that `Infallible`'s
*uninhabitedness itself* is what `Arbitrary` reconstruction cannot
survive. So stop using an uninhabited type. `StoplightError` (an enum
with one variant, `NotUsed`) replaces `Infallible` as every edge's
`Error` type: ordinary, safely constructible data, never actually
returned by any edge (each edge's own `#[kani::ensures]` contract,
already proven per-edge in Step 1/3, is what establishes that — not
the type system). This trades a compile-time "impossible to
construct" guarantee for a runtime "never actually happens" one,
specifically so `kani::Arbitrary` can be honest (`NotUsed`, no
`unreachable!()`, no unsafe). Real, not hypothetical: `cargo kani`
verifies the composition harness clean once this lands.

**Verified for real**, including the one check unique to this step —
does `stub_verified` actually skip the body, not just accept a
trivial one? A `panic!()` injected into `green_to_yellow`'s real body
made its own `proof_for_contract` harness fail at the exact injected
line (confirming the panic is live, reachable code), while the
*composition* harness — which stubs that same function — stayed
`VERIFICATION:- SUCCESSFUL` throughout, since stubbing never executes
the body at all. Reverted and re-verified both clean afterward. Full
workspace `fmt --check`/`check`/`clippy --all-targets --all-features
-D warnings`/`test` all clean (61/61, unchanged — the composition
harness is Kani-only, not a `cargo test` target); the Step 2
consistency test and the real `stoplight_test.rs` integration tests
(updated to use `StoplightError` in place of `Infallible`) both still
pass. Step 4 is complete.

### Step 5 — wire into `Amenable` — done

`impl Amenable for Stoplight` (`amenable_kani/src/stoplight.rs`) is the
first real occupant of `kani_surface()`/`creusot_surface()`/
`verus_surface()`/`audit_surface()` anywhere in the tree — grepping the
whole tree before this found zero `impl Amenable for`. Not elicitation's
`VerifiedStateMachine::transition_harnesses()`/`vsm_kani_proof()` shape
(a `Vec<proc_macro2::TokenStream>` a `build.rs` writes out to a generated
`.rs` file) — amenable doesn't do proof-source codegen anywhere, so that
shape doesn't map onto anything real here. Instead, each method either
queries data that already exists (no new registration mechanism
invented) or references real, compiler-checked items:

- `type ProofSurface = Vec<String>;` — a plain list of proof
  identifiers, reusing `KaniProof.id`'s own format
  (`crate::module::path::harness_name`) rather than inventing a second,
  near-identical struct just for this trait.
- `kani_surface()` queries the same real `KaniProofRegistration`
  `inventory` catalog `harness!` already populates — the exact
  mechanism `amenable::kani::registered_proofs` (the CLI's own harness
  listing) already uses — filtered to this module's own entries via
  `module_path!()`. Evaluated inside `stoplight.rs` itself, so it's the
  literal same string `harness!`'s own `id: concat!(module_path!(), "::",
  ..)` computed when each harness registered; no hand-typed module name
  on either side to drift apart.
- `creusot_surface()` — at the time Step 5 landed, `amenable_creusot::
  stoplight` had no `inventory`-backed registry of its own (deliberately:
  see that module's own doc comment on why `inventory::submit!` was
  off-limits there), so nothing was queryable the way `kani_surface()`
  queries. This method instead referenced the real exported harness-
  source constants directly (`let _: &str = amenable_creusot::
  VERIFY_GREEN_TO_YELLOW_EXCHANGE_SRC;` et al.) — confirmed for real, not
  assumed, that this actually caught drift: renaming one of those
  constants in `amenable_creusot/src/stoplight.rs` broke `cargo check -p
  amenable_kani --features creusot` immediately (`E0432`, unresolved
  import), reverted after confirming. Gated behind a **new** `creusot`
  feature on `amenable_kani` itself (`amenable_creusot = { workspace =
  true, optional = true }` + `creusot = ["dep:amenable_creusot"]`),
  mirroring `amenable_std`'s identical dependency for the identical
  reason. **Superseded by Step 8, and for a real, substantive reason, not
  just a style preference**: this Cargo edge was a genuine violation of
  "verifier backend crates never depend on each other" — `amenable_kani`
  linking directly to `amenable_creusot` — caught and fixed after direct
  correction. See Step 8 below for the real fix (a shared, `amenable_
  core`-owned registry query, no cross-crate import at all) and why the
  original justification here ("the alternative would have reintroduced
  the drift risk this plan exists to close") turned out to have a third
  option neither predicted.
- `verus_surface()` returned `Vec::new()` at the time Step 5 landed —
  honest, not aspirational: no Verus `Exchange` proof existed for
  `Stoplight` yet (Verus was out of scope for this plan at the time;
  see Motivation). **Superseded by Step 6/`VERUS_EXCHANGE_PROOF_
  DERIVATION_PLAN.md`**: a real Verus proof of this same cycle now
  exists (`amenable_verus::gallery::stoplight_exchange`), but in a
  separate crate with no `inventory`-backed registry yet, so
  `verus_surface()` still returns `Vec::new()` — still honest, just for
  a different reason now (nothing to query, not nothing to report).
- `audit_surface()` returns the real `harness!`-captured verbatim
  source of all four Kani harnesses (the three edges plus the Step 4
  composition harness) — literal source, not identifiers, the same
  constants the `Witness::proof()` impls already rely on.

**Verified for real**, not just "it compiles": a new test file
(`amenable_kani/tests/stoplight_amenable_test.rs`) asserts `kani_surface()`
returns exactly the four expected, module-scoped harness ids;
`creusot_surface()` returns the three expected edges under the `creusot`
feature and an honestly-empty list without it; `verus_surface()` is
empty; `audit_surface()` contains four entries whose real source text
names every edge plus `stub_verified`. All pass under both feature
configurations (`cargo test -p amenable_kani --test
stoplight_amenable_test` with and without `--features creusot`). Full
workspace `fmt --check`/`check`/`clippy --all-targets --all-features -D
warnings` clean under both configurations, `cargo test --workspace`
62/62 (up from 61 — the new test binary), `just verify-creusot` still
`Proved (110 files) ✔`. Step 5 is complete.

### Step 6 — route DFCC `ensures` closures through registered `Ensures<KaniVerifier>` impls — done

Not part of the plan's original five steps — added after the fact,
prompted by the same source-of-truth discipline `rust_std`'s own
proofs already apply everywhere else in the tree (`kani_ensures!`/
`kani_requires!`, dozens of real call sites) but that this plan's own
Step 1 never retrofitted onto the three `Stoplight` edges it created:
each `#[kani::ensures(...)]` closure restated its claim inline
(`|result: &Result<..>| result.is_ok()`) instead of calling through a
registered contract type the way every other proof in this codebase
does. That's the same hand-authored-claim-can-drift risk the
Motivation section already names for `next()` vs. `exchange()`, just
recurring one level deeper — this time in the contract's own content,
not the surrounding registration/delegation machinery Step 3 already
closed.

Fixed by wiring all three edges' postconditions through real, `kani_
ensures!`-registered `Ensures<KaniVerifier>` impls (`Yellow::ensures`,
`Red::ensures`, `Green::ensures`, one per edge, each with its own
`ContractRecord` evidence-string literal — `Green`'s cycle-back edge
keeps the existing `::cycle_back` suffix convention so it stays
distinct from any future root-case registration). Each DFCC closure
now reads `|result: &Result<..>| Yellow::ensures(*result)` (etc.)
instead of restating the boolean. `Established<T, Token>` gained
`#[derive(Clone, Copy)]` so the closure's `&Result<..>` can be
dereferenced into the owned `Ensures::Input` these contract types take.

The `Ensures` import needed its own `#[cfg(kani)] use amenable_core::
Ensures;` line, not a plain unconditional one (mirroring the existing
convention already in `primitives.rs`) — the same "compiles clean
under plain `cargo check` doesn't mean the real path was checked"
lesson `VERUS_EXCHANGE_PROOF_DERIVATION_PLAN.md` hit independently:
the whole `#[kani::ensures(...)]` closure body is inert under ordinary
compilation, so an unconditional import looked unused there and only
surfaced as a real `E0599` under actual `cargo kani`.

**Verified for real**: `cargo kani` re-verified all three edges
individually (`VERIFICATION:- SUCCESSFUL` each) plus the Step 4
`stub_verified` composition harness (0 of 49 failed). A real,
non-vacuous regression check on `green_to_yellow` (swapped `is_ok()`
for `is_err()` inside the `kani_ensures!` invocation) produced a
precise failure pointing at the exact `Yellow::ensures(*result)` call
site, then reverted and re-verified clean. Full workspace `fmt
--check`/`check`/`clippy --all-targets --all-features -D warnings`
clean, `cargo test --workspace` 62/62 unaffected,
`stoplight_mirror_consistency_test` still 3/3 (confirms Step 2 is
unaffected — it only compares the method body, not the surrounding
attribute), `stoplight_amenable_test` still 6/6, `just verify-creusot`
still `Proved (110 files) ✔`. Step 6 is complete.

Left for future direction, not started here: the `Input`/`Output`
sidecar's `Requires<KaniVerifier>` (precondition) side — the current
edges have no real precondition beyond type-safety (each `Input` is
already a proven `Sidecar`), so there was nothing non-trivial to wire;
and generating this `kani_ensures!`/closure-through-contract pattern
from `#[amenable_derive::exchange(..)]` itself, rather than hand-
written per edge — deliberately not done here, matching this plan's
"one real example by hand first, generalize later" discipline used at
every other step.

### Step 7 — generate the DFCC `ensures` attribute from `#[amenable_derive::exchange(..)]` — done

The last piece of "one real example by hand first, generalize later"
this lineage had left: Step 6 proved the `Ensures<KaniVerifier>`-
routing pattern by hand at all three call sites; Step 7 moves the
mechanical half of that pattern into the macro itself, tackled by
explicit direction after `VERUS_EXCHANGE_PROOF_DERIVATION_PLAN.md`'s
own Step 5 landed the Verus counterpart ("let's tackle 3 now," where 3
was this step, done deliberately after 2 — see that doc's Status).

**What moved, and why it doesn't weaken "real proof content stays
hand-authored."** `#[amenable_derive::exchange(..)]`'s own doc comment
had explicitly listed the DFCC contract as one of two things the macro
"deliberately does NOT touch," alongside the method body — both named
as real proof content that must stay hand-authored. That was true when
written, and is still true of the method body. It stopped being true
of the *contract attribute* once Step 6 moved the actual predicate
into a separately-registered `Ensures<KaniVerifier>` impl
(`kani_ensures!`): the attribute the macro used to require hand-typed
at each call site --- `#[cfg_attr(kani, kani::ensures(|result: &Result
<Output, Error>| Evidence::ensures(*result)))]` --- had become 100%
mechanical, derivable entirely from information the macro already
parses (`evidence`, the method's own `Result<Output, Error>` return
type). Generating it doesn't synthesize a new claim; the human-authored
predicate it calls through to is exactly as hand-written as it was
before, in exactly the same place (the `kani_ensures!` invocation,
still directly preceding the macro invocation, still untouched by it).

**Implementation.** `expand_exchange` (`crates/amenable_derive/src/
exchange.rs`) now clones the parsed `ItemImpl`, locates its one method,
and pushes the generated `#[cfg_attr(#cfg, #cfg::ensures(..))]`
attribute onto the clone before re-emitting it — `#cfg` doubles as both
the `cfg_attr` predicate and the attribute macro's crate path (`kani`
in both positions), reusing the identical `Ident` already required for
the `ProofRecord`'s `verifier` field, not a new argument. The generated
closure uses fully-qualified syntax (`<#evidence as ::amenable_core::
Ensures<#verifier>>::ensures(*result)`), not a bare `Evidence::ensures`
call, so the generated code needs no `use amenable_core::Ensures;` in
scope at the call site at all — `stoplight.rs`'s own `#[cfg(kani)] use
amenable_core::Ensures;` (added in Step 6) was removed as a result,
along with all three hand-written `#[cfg_attr(kani, kani::ensures(..))]`
attributes on `green_to_yellow`/`yellow_to_red`/`red_to_green`.
Unconditional, not an opt-in flag: every real edge today wants this
bound, and a hypothetical future edge with no meaningful postcondition
is not this macro's problem to design around in advance.

**Verified for real**, the same discipline as every other step: `cargo
expand` can't render the generated `#[kani::ensures(..)]` content
directly (the real `kani` crate's attribute macro only resolves inside
the actual Kani toolchain, not plain `cargo expand`), so this was
checked the authoritative way instead — real `cargo kani` on all three
edges individually plus the `stub_verified` composition harness, all
`VERIFICATION:- SUCCESSFUL`. A real, non-vacuous regression check
(swapping `is_ok()` for `is_err()` inside `green_to_yellow`'s
`kani_ensures!` invocation) produced a precise failure pointing at the
exact macro-generated closure body and its real source line, reverted
and re-verified clean. Full workspace `fmt --check`/`check`/`clippy
--all-targets --all-features -D warnings` clean, `cargo test
--workspace` clean (zero failures), `stoplight_mirror_consistency_test`
still 3/3 (Step 2 unaffected — it only compares the method body,
untouched here), `stoplight_amenable_test` still 6/6, `just
verify-creusot` still `Proved (110 files) ✔`. Step 7 is complete.

Left for future direction, not started here: the same generation for
`Requires<KaniVerifier>` once a real precondition exists to wire.
The Verus-side counterpart landed separately, by explicit direction
("let's tackle the verus side macro next, so we maintain our
trifecta"), and then extended further by direct pushback on an
architecture opinion the user asked for ("hand-written is not the
goal here"): `VERUS_EXCHANGE_PROOF_DERIVATION_PLAN.md`'s own Step 6
(`exchange_support::verus_ensures!`, the `Ensures<V>`-wiring
counterpart) and Step 7 (`exchange_support::verus_exchange!`, the
`Witness<V>`/`Exchange<V>` counterpart) — necessarily `macro_rules!`
macros rather than a proc-macro crate like `amenable_derive`, since
`verus --crate-type=lib` never resolves any extern crate beyond what
the real `verus` binary itself bakes in.

### Step 8 — remove the `amenable_kani -> amenable_creusot` Cargo edge — done

A direct, firm correction, not a self-initiated cleanup: "kani, creusot
and verus never depend on each other. Rather we need to move shared
types to core, simple as that. I don't know when or how you decided to
link kani to creusot, but that also needs to be fixed." Step 5's own
`creusot_surface()` (see above) had added exactly that link —
`amenable_kani = { ..., amenable_creusot = { workspace = true, optional
= true } }`, gated behind a new `creusot` feature — to get a real,
compiler-checked reference to `amenable_creusot`'s harness-source
constants instead of a hand-typed, drift-prone name list. Solving one
real problem (drift) by creating another (backend crates linking to each
other) — verifier backend crates are supposed to be, and mostly already
were, independent; `amenable_core` is where shared types live.

**The real fix needed a real question answered first, not just the
dependency deleted and the list re-hand-typed**: why did `creusot_
surface()` need a cross-crate import in the first place, when `kani_
surface()` (right above it) queries a shared `inventory` registry with
no cross-crate import at all? Because `amenable_creusot` itself was
believed unable to call `inventory::submit!` — `amenable_std::
creusot_witness`'s own doc comment documents a real, confirmed ICE from
exactly that pattern, which is why `amenable_creusot`'s entire witness-
bridge/registry surface for ~90 std carriers was relocated into
`amenable_std` instead, years before this plan started.

**Checked directly with the real toolchain, not assumed to still be the
only fix** (in an isolated, throwaway probe crate — `creusot_ice_probe`,
deleted after confirming the finding, never kept live in the real
workspace, matching this whole plan's own "hypothesis, trial, error,
resolution" gallery discipline): `#[cfg(not(creusot))]`-gating `inventory
::collect!`/`inventory::submit!` *in place*, precisely, avoids the
translator error entirely — confirmed clean under real `cargo creusot --
-p <probe>`. Two real, previously-undocumented refinements found in the
same probe, not assumed from the existing gallery case:

1. Gating only `submit!` and leaving `collect!` ungated still fails —
   `collect!` independently trips its own, differently-worded translator
   error (`Unsupported constant value: Scalar(alloc1) of type &'?2
   inventory::Registry`). Both need gating. (Not relevant to `amenable_
   creusot::stoplight`'s own fix below, since `amenable_core` already
   owns the one `inventory::collect!(ProofRecord)` call site, outside
   the crate `cargo creusot -p amenable_creusot` actually translates —
   only `submit!` calls, added locally in `amenable_creusot`, needed
   gating there.)
2. `Box<dyn Iterator<Item = ..>>` as a concrete associated-type value —
   the pattern `amenable_kani::stoplight`'s real `Green`/`Yellow`/`Red`
   `Provenance` impls use — is a real, independent translator error
   (`forbidden dyn type ... dyn support is currently minimal`), *not*
   the same thing as the already-fixed, differently-shaped
   `rpitit_panics_intrinsics_gathering` case (return-position `impl
   Trait`, a compiler-synthesized opaque type; `Box<dyn ..>` is an
   ordinary, named trait-object type, no RPITIT desugaring at all).
   Confirmed real by testing it ungated first (it failed on its own),
   then gated (it translated clean) — not assumed to be the same known
   issue by proximity in an old summary.

Documented permanently as a new `amenable_std::creusot_gallery` case
(`cfg_not_creusot_gating_avoids_the_inventory_and_dyn_iterator_errors`,
`BestPractice`/`Proved`) — the gallery is exactly where a finding like
this belongs, per its own stated purpose (demonstrating hypothesis,
trial, error, and resolution), not just applied silently.

**The real fix, landed**: `amenable_creusot::stoplight` now registers
its own three `ProofRecord`s (`#[cfg(not(creusot))]`-gated `inventory::
submit!` calls, referencing its own already-ungated `VERIFY_*_SRC`
constants), and `amenable_kani::stoplight::creusot_surface()` queries
the shared registry (`inventory::iter::<amenable_core::ProofRecord>()`,
filtered by `verifier == "creusot"` and an `evidence` string prefix) —
the identical shape `kani_surface()` already used. No Cargo dependency
in either direction: `inventory` only requires both crates to be linked
into the *same final binary* (e.g. `amenable`'s own CLI/tests, which
already depend on both), not a direct edge between them. This also
simplified the code: no `#[cfg(feature = "creusot")]`/`#[cfg(not(feature
= "creusot"))]` split needed anymore — one unconditional implementation,
honestly empty when `amenable_creusot` isn't linked in, real when it is.
`amenable_kani`'s `creusot` feature and its `amenable_creusot` Cargo
dependency were removed entirely; `amenable`'s own `creusot` feature
updated to drop the now-nonexistent `amenable_kani/creusot` unification.

**Verified for real**: full workspace `fmt --check`/`check`/`clippy
--all-targets --all-features -D warnings`/`test` clean, `just
verify-creusot` still `Proved (110 files) ✔` (the new `#[cfg(not(
creusot))]`-gated registrations never reach `cargo creusot -p
amenable_creusot`'s translator at all), `cargo check -p amenable_kani`
confirmed to no longer resolve `amenable_creusot` by any path. Step 8
is complete.

**Deliberately not done here, a separate and much larger question**:
whether `amenable_std::creusot_witness`'s own, much larger witness-
bridge/registry surface (~90 real std-carrier registrations, the
original reason for that whole crate split) should be migrated the same
way. The new finding may undermine part of that split's original
justification too, but unwinding ~90 existing, real, working
registrations is a different scale of change than three `Stoplight`
edges, and wasn't part of what was asked for here — flagged for
explicit future direction, not started.

### Step 9 — a real codegen layer replaces the hand-written Creusot mirror — done

Direct, pointed pushback on Step 8 itself, not a new request: "why is
codegen deliberately been ignored, in spite of multiple issues with
inventory at every step? Isn't the solution staring us in the face?"
Step 8's `#[cfg(not(creusot))]`-gating fix was real and correctly
removed the `amenable_kani -> amenable_creusot` Cargo edge, but it was
still a patch *around* `inventory`'s repeated friction with Creusot's
whole-crate translator, not a fix that made the friction stop
recurring. The user pointed at `amenable`'s own `emit-verus-witnesses`
(`crates/amenable/src/verus_export.rs`, ~950 lines, real and already
shipped) as proof a better answer already existed in this exact
codebase: read a real registry from inside an ordinary, never-
translated binary, and *generate* real, checked-in, `inventory`-free
source the verifier just compiles as ordinary static code. `inventory`
never has to survive into the translated crate at all — not gated out
item by item, simply never present.

**What got built.** `#[amenable_derive::exchange(..)]` now captures
each real edge's transition body verbatim (the same `Span::
source_text()` technique `harness!` already uses, applied to the
method's own `block` this time) and registers it, alongside its real
type names (`self_ty`/`input_ty`/`output_ty`/`error_ty`/`evidence`/
`method_name`), as a new `amenable_core::ExchangeEdgeRecord` —
unconditionally, regardless of `#cfg`, since `amenable_kani` is
ordinary Cargo-built and never translated by anything, so this
`inventory::submit!` carries none of the risk a translator-based
crate's own would. A new `amenable::creusot_export` module (mirroring
`verus_export.rs`'s own architecture) plus `emit-creusot-companions`
CLI command queries that registry from inside the safe `amenable`
binary and writes real `amenable_derive::harness! { creusot, .. }` +
`ProofRecord`-registration files into `amenable_creusot/src/
generated/*.rs`, `include!`d (not `mod`-declared, so the generated
files share `stoplight.rs`'s own scope with no imports needed) from
`amenable_creusot/src/stoplight.rs`. Deliberately narrow scope, one
real example first: generates only the per-edge transition-body
content, not the surrounding state/token/sidecar type definitions
(`Green`/`Yellow`/`Established<T, Token>`), which stay hand-written —
stable, one-time accommodation-model infrastructure with far lower
drift risk than a transition body's own evolving logic, and not
something a different backend's real source can be captured verbatim
*from* in the first place (Creusot's own `Green` is a different,
locally-defined concrete type from Kani's, matching by name/shape
convention, not by shared source).

**Real bugs found and fixed while building this, not assumed away:**

1. Capturing the real body verbatim, `Ok(..)` wrapper included, meant
   the generated function's return type had to be the real `Result<
   Output, Error>`, not the bare `Output` type the old hand-written
   mirror used (which had silently simplified the signature and
   dropped the wrapper). This meant `StoplightError` itself needed a
   real Creusot-local counterpart — one more small, stable, hand-
   written accommodation-model type, matching `Green`/`Yellow`/`Red`'s
   own treatment, not something codegen could avoid needing.
2. `rustfmt` does not reformat inside an opaque macro invocation's own
   token tree (`harness! { .. }`) by default — confirmed directly, not
   assumed: a real generated file's `stringify!`-produced type text
   (`Established < Green , GreenToken >`, from `stringify!`'s one-
   space-per-token join) survived a real `cargo fmt -p amenable_creusot`
   run untouched. Fixed with a narrow, targeted text-cleanup pass
   (`tidy_stringified_type`) rather than relying on a later format
   pass that never actually reaches the text — and confirmed `cargo
   fmt -p <crate>` doesn't even discover `include!`d (not `mod`-
   declared) files at all, fixed by invoking `rustfmt` directly on the
   generated files in the `generate-creusot` recipe instead.
3. The captured body's own first line loses its original indentation
   to `.trim()` (which only strips the very start/end of the *whole*
   captured string), while every other line keeps its real, original
   column — a real, confirmed dedent bug (visibly double-indented
   non-first lines in an actual generated file), not a hypothetical
   one, fixed with a real dedent-then-reindent pass, not a uniform
   re-indent alone.
4. A real chicken-and-egg bootstrap problem: `emit-creusot-companions`
   itself needs `amenable_creusot` to compile successfully to run at
   all (it's linked into the `amenable` binary), but `amenable_creusot`
   needs the generated files to exist to compile. Broken by committing
   minimal placeholder generated files (matching `pub const NAME: &str
   = "";` shape) once, the same bootstrap step `elicitation`'s own
   generator docs describe for an identical reason.
5. The generated file's own header used `//!` (inner doc comment) —
   valid for a real module, a real `E0753` compile error once
   `include!`d mid-file into `stoplight.rs`'s own scope instead of its
   own module. Fixed with a plain `//` comment.

**`stoplight_mirror_consistency_test.rs` is gone**, not superseded by
a regeneration-freshness check: with the transition bodies now
generated directly from the real source (not a hand-kept-in-sync
copy), there is nothing left for a consistency test to guard against
drift *between* — the single source of truth and its Creusot
companion are mechanically the same by construction. `amenable_creusot`'s
now-unused `proc-macro2`/`quote`/`syn` dev-dependencies (only ever
needed by that test's own source-parsing) were removed too.

**Verified for real, against the actual toolchain at every step, not
assumed**: `just verify-creusot` — `Proved (112 files) ✔` (up from
110; the generated-file split changes proof-unit granularity slightly,
still a full clean proof). Full workspace `fmt --check`/`check`/
`clippy --all-targets --all-features -D warnings` clean, `just
check-all-creusot`/`just test-creusot` clean end to end, `cargo test
--workspace` clean (matching this project's own per-package/per-
feature testing convention, not a blanket `--all-features` sweep — a
real, confirmed nuance found along the way: workspace-wide feature
unification transitively links `amenable_creusot` into `amenable_
kani`'s own test binary via `amenable_std`'s own `creusot` feature,
even though `amenable_kani` itself declares no such edge, which is why
`creusot_surface_is_honestly_empty_in_this_crate_s_own_test_binary`'s
own doc comment now states that caveat precisely rather than
overclaiming). `just generate-creusot` (the new recipe) wired into
`check-creusot`/`clippy-creusot`/`test-creusot`/`verify-creusot`/
`verify-creusot-translate`, matching `emit-verus-witnesses`'s own
"regenerate before checking" placement exactly. Step 9 is complete.

**Left for explicit future direction, not started here**: migrating
`amenable_std::creusot_witness`'s own much larger (~90-registration)
surface the same way — Step 8's own deferred question, now with an
even stronger case for it (real codegen, not just cfg-gating, is
proven out for this exact class of problem) but still a much bigger
undertaking than this step; and a `creusot_ensures!`-style mechanism
for a genuinely non-trivial Pearlite predicate, once one exists to
generate for (every real edge today is still trivially `#[requires(
true)] #[ensures(true)]`, hardcoded in the generator rather than
designed speculatively for a case that doesn't exist yet).

## Open questions

- **Creusot compilation model — resolved, landed in Step 1.** Not
  elicitation's `elicitation_kani`/`elicitation_creusot`/`elicit_proofs`
  split (that requires a real dependency edge into the crate owning the
  types, and every candidate edge here closes a cycle — see Step 1's
  own writeup). Resolved instead by treating it as the same problem the
  Kani side already has a real, working answer to: an accommodation
  model — genuinely implementing the real `amenable_core` trait family
  (`Evidence`/`Witness<V>`/`Sidecar<V>`/`Establish<C, V>`), not a
  flattened stand-in, once an over-broad early version of this fix was
  corrected. `amenable_creusot/src/stoplight.rs` defines sanitized,
  locally-owned mirror types with no Cargo dependency on anything that
  owns the real ones, verified for real with `cargo creusot prove`
  (`Proved (110 files) ✔`, all three edges) and a real injected-
  regression check per edge.
- **Root/init-state handling — resolved, landed in Step 0.** This
  plan's Motivation section originally read `AMENABLE_PLAN.md`'s
  root-state discussion as "roots are categorically asserted, never
  proven" and treated `Established::root()`'s `Establish`/`Witness`-
  free bypass as presumptively correct. That conflated root-ness with
  trivial-vs-substantive proof content, which are orthogonal (see
  "Foundational Trait-Family Redesign"). What actually landed:
  `Established::root()`'s doc comment now states precisely what's
  proven and what isn't. `GreenToken::new` still mints without calling
  `Establish::establish()` — the root claim isn't derived from a prior
  state, so there's no credential to present, and that stays true.
  Separately, `Sidecar<KaniVerifier>`'s own `Green: Witness<
  KaniVerifier>` bound does apply here (it applies to every `Sidecar`
  proposition, root or not), and happens to already be satisfied by
  the real proof backing `Red`'s cycle-back edge, reused
  opportunistically because it's the only `Witness<KaniVerifier>`
  `Green` has — not because that proof is "about" the root claim. The
  root claim itself remains exactly what `AMENABLE_PLAN.md` says it is:
  `Provenance`-backed, not model-checked. No further open question
  here; a *certified*-root-claim type distinct from this would be new
  scope, not something this plan needs.
- **Symbolic state construction — corrected after checking real
  source, not assumed.** `KaniCompose` is already ported
  (`amenable_kani::compose`, `#[derive(KaniCompose)]` in
  `amenable_derive::kani_compose`) and already includes `kani_any()`,
  not just `kani_depth0/1/2` — the exact method `KANI_FOR_VSMS.md`
  §6.2's forgive-and-forget pattern needs. For enums specifically,
  amenable's derive takes a genuinely different approach from
  elicitation's: instead of N generated per-variant harness functions,
  it builds a bounded symbolic selector (`kani::any::<usize>()` +
  `kani::assume(selector < variant_count)`) and `match`es on it, so
  each of CBMC's explored paths constructs one concrete variant. This
  may or may not dodge the same symbolic-enum-drop explosion
  `KANI_FOR_VSMS.md` §1 documents at length for `kani::any::
  <StateEnum>()` directly — it's a different code shape (assumed-
  bounded integer branching to a concrete constructor per arm, not a
  literal `impl kani::Arbitrary for StateEnum`), but that difference
  has not been confirmed against a real, heap-field-bearing multi-
  variant enum under actual `cargo kani`. Step 1 should include this as
  a real empirical check, in the graduated-synthetic-ladder style
  `KANI_FOR_VSMS.md` §4/§8 itself uses to isolate one variable at a
  time, before trusting it at any larger scale.
- **`KaniVariantState` is very likely not needed at all.**
  `KANI_FOR_VSMS.md` §6/§10 is explicit that its own per-variant/
  per-depth harness generation (the thing `KaniVariantState` supports)
  is now legacy/diagnostic-only — the production path is
  `proof_for_contract` + forgive-and-forget, which only needs
  `KaniCompose::kani_any()`/`kani_depth0()`, already present. Don't
  port `KaniVariantState` speculatively; only reconsider if Step 1
  surfaces a real need for isolating a failure to one specific variant.
- **`Established<P>: kani::Arbitrary` has no direct amenable
  analog.** Elicitation's `stub_verified` composition (§6.4/§6.5) needs
  Kani to reconstruct a stubbed function's return type, so `Established
  <P>` (a bare `PhantomData` ZST) gets a trivial `kani::Arbitrary` impl.
  Amenable's `ProofToken` implementors are concrete per-transition
  types with private fields (`GreenToken(())`, `YellowToken(())`, ...),
  not one generic `Established<P>` — whether `stub_verified` composition
  (Step 4) needs an equivalent `kani::Arbitrary` impl per token type,
  and whether that's even sound to add given the private-field/
  `establish()`-only construction discipline this plan is protecting,
  is unresolved. Real question for Step 4, not Step 1.
- **Exact macro attribute shape** for Step 3 — deferred until Step 1/2
  reveal what a real transition macro actually needs, rather than
  guessed in advance.

## Known operational gotchas (from `KANI_FOR_VSMS.md`, not yet hit here)

Real failure modes documented there, worth checking against before
concluding something in Step 1+ is a novel problem:

- `#[instrument]`/inline `tracing::debug!` etc. must be gated
  `cfg_attr(not(kani), ...)` — tracing closures capture arguments
  symbolically, recreating the drop-explosion problem even for a
  one-line transition body (§6.3).
- `HashMap::new()` calls `getrandom` — Kani can't model it. Use
  `BTreeMap` in any state type touched by a Kani proof.
- Under DFCC, symbolic `String` shadows must be `String::new()`, never
  a non-empty literal — `String::from(...)` allocates a heap buffer
  that trips DFCC's freeable-pointer check on drop (§6.2).
- A large "live arm" struct sharing a union with another variant's
  `BTreeMap`/heap-collection field can hang even with a concrete
  discriminant and no recursion — CBMC reasons about whether the live
  arm's raw bytes could alias a valid pointer in the dead arm. Fix:
  `Box<T>` the live arm to shrink the union footprint (§4).
- Self-recursive fields (`Vec<Self>`/`Box<Self>`) defeat depth-bounded
  construction entirely — CBMC's destructor model is type-driven, not
  value-driven, so even `Vec::new()` triggers infinite unrolling. Fix
  is an arena/index refactor (`Vec<Self>` → `Vec<usize>` + a flat
  wrapper), not more `KaniCompose` depth (§3).
- Symbolic `usize` arithmetic (`idx + 1`) can overflow at
  `usize::MAX`, caught by real symbolic proofs but not unit tests —
  use `saturating_add`/`saturating_sub` in any index math a Kani proof
  can reach (§8).

## Gotchas found directly in this project (not in `KANI_FOR_VSMS.md`)

- **Kani 0.67.0 cannot contract a trait method when the trait itself
  is generic.** `#[kani::proof_for_contract(<Type as SomeGenericTrait
  <Args>>::method)]` fails outright ("Kani does not currently support
  stubs or function contracts on generic functions in traits"), even
  when every generic argument is fully concrete at the impl site.
  `Exchange<Input, Output, V>` is generic, so this hits every `Exchange
  ::exchange` impl. Fix: put the real logic and its contract on a
  plain inherent method instead, and reduce the trait impl to a
  single-expression delegation to it — see `Stoplight::green_to_yellow`
  et al. in `stoplight.rs`. This is now the standard shape Step 3's
  macro needs to generate, not an incidental workaround.
- **`cargo kani --harness NAME --exact` needs the module-qualified
  name**, not the bare function name, for contract harnesses —
  discover the real name with `cargo kani list -Z function-contracts`
  run from inside the crate directory (`cargo kani list` doesn't take
  `-p`/`--lib`/`--all-features`, unlike the main `cargo kani` command).
- **`amenable_creusot` cannot take a Cargo dependency on any crate
  that owns real workspace-local types it needs to prove things about**
  — every candidate (`amenable_kani`, `amenable_std`, `amenable`)
  either closes a real cycle (`amenable_std`/`amenable` both already
  optionally depend back on `amenable_creusot`) or would hand
  `creusot-rustc`'s whole-crate-sweep translator ordinary Rust
  infrastructure already confirmed to ICE it (see `amenable_std::
  creusot_witness`'s doc comment). Fix: a locally-defined accommodation
  model — sanitized mirror types owned entirely by `amenable_creusot`
  itself. **Do not over-apply this and strip out `amenable_core`'s own
  trait family too** — that was a real mistake made and corrected in
  this exact file (see Step 1's "Correction found while starting Step
  2"). `amenable_creusot` already has a real, unconditional dependency
  on `amenable_core`, and `Evidence`/`Witness<V>`/`Sidecar<V>`/
  `Establish<C, V>`/`ProofToken` don't contain the specific patterns
  that caused the real ICEs (those were `Provenance`'s `Box<dyn
  Iterator>` and `inventory::submit!` specifically, not the trait
  family as a whole). The mirror types should genuinely implement the
  real traits; only the registry-facing machinery (`Provenance`,
  `inventory::submit!`) needs to stay out. Confirmed working for real
  (`cargo creusot prove`, real injected-regression check per edge) in
  `amenable_creusot/src/stoplight.rs`.

## Next step

Steps 0 through 5 — every step this plan originally scoped — plus
Steps 6 through 9 (added after the fact) are now complete: the
`Sidecar<V>` trait-family fix; real Kani and Creusot bodies for all
three `Stoplight` edges; the by-hand Kani-side pattern generalized
into `#[amenable_derive::exchange(..)]`; all three edges composed into
one full-cycle `#[kani::stub_verified]` harness; `Stoplight`'s real
`Amenable` impl, the first anywhere in the tree; all three edges' DFCC
postconditions routed through registered `Ensures<KaniVerifier>`
contract types instead of restated inline, that routing generated by
the macro itself rather than hand-typed per edge (Steps 6/7); the
`amenable_kani -> amenable_creusot` Cargo dependency Step 5 introduced
removed as a real architectural violation (Step 8); and the hand-
written Creusot mirror — plus `stoplight_mirror_consistency_test.rs`,
the drift-guard it needed — replaced entirely by a real codegen layer
generating Creusot's companions directly from Kani's own captured
transition bodies (Step 9). The Verus counterpart of Steps 6/7 has
since landed too, in two parts — `VERUS_EXCHANGE_PROOF_DERIVATION_
PLAN.md`'s own Step 6 (`exchange_support::verus_ensures!`) and Step 7
(`exchange_support::verus_exchange!`, added after direct pushback —
"hand-written is not the goal here") — completing what the user called
the "trifecta": Kani's contract/scaffold generated by a proc-macro
attribute, Verus's by `macro_rules!` macros (a proc-macro crate
structurally cannot resolve under `verus --crate-type=lib` at all),
and Creusot's now generated too, by a real codegen tool reading a safe
registry (mirroring `emit-verus-witnesses`'s own architecture) rather
than either hand-copying or embedding `inventory` in the translated
crate itself. On all three backends, only each edge's real transition
body remains hand-authored; every mechanical piece around it is
generated.

Nothing further is queued — extending this pattern to more `Exchange`
edges beyond `Stoplight`; wiring `Requires<KaniVerifier>`/`Requires<V>`
for a real (non-trivial) precondition once one exists; a `creusot_
ensures!`-style mechanism for a genuinely non-trivial Pearlite
predicate (Step 9's own generator still hardcodes a trivial `requires`/
`ensures` pair, honestly, since nothing non-trivial exists yet);
migrating `amenable_std::creusot_witness`'s own much larger (~90-
registration) witness-bridge surface to the same codegen pattern (Step
8/9's own deferred question, now with a stronger case for it); or
generating `Evidence`/`ProofToken`/`Establish` themselves, or the
coarser-grained per-state-machine carrier shape (`Established<T,
Token>`/`Sidecar<V>` on the Verus side; there is no Kani analog since
`amenable_kani::stoplight`'s own `Established<T, Token>` already plays
that role) are all real future directions, but none should be started
without explicit new direction, matching this plan's pacing
throughout.
