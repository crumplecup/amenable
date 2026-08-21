# Planning Documents

This file tracks all planning documents for the amenable project.

## Current Active Plans

### Verus Derive-Witness Composition

**Document:** [VERUS_DERIVE_WITNESS_COMPOSITION_PLAN.md](VERUS_DERIVE_WITNESS_COMPOSITION_PLAN.md)

**Status:** ✅ Done — all 8 phases implemented and verified.
Phases 1–3 (`ClassifiedWitness<V>` marker trait and compile-time export
enforcement plus an `#[allow(dead_code)]` fix; owned-`String`
conversion on `VerusCheckedProof`; a real, additive `VerusCallShape`
call-shape registry) each left proof content unchanged. **Phase 4 is
the core deliverable**: the renderer now emits real calls to real Verus
harnesses instead of assumed free booleans — `just verify-verus` went
from `335 verified, 0 errors` to `334 verified, 0 errors`, correctly
(three tautologies replaced by two genuine proofs). Phase 5 (`requires`
propagation) needed no renderer changes, just a real canary with a
genuine precondition — `just verify-verus` is back to `335 verified,
0 errors`, one new genuine proof. **Phase 6 overturned its own
premise**: "mutating/model-method leaves" turned out not to exist —
every real harness, including `RefCell`'s, is a plain value-returning
function; the actual gap was citation text (`result.0`/`!result.1`/
`result.5 as int` don't fit a structured `predicate(args)`-only
representation), fixed by replacing structured citations with
`$placeholder` text templates, plus a second real gap (imports need
their own `module_path`, not the harness's). `just verify-verus` went
to `336 verified, 0 errors`, one new genuine proof. **Phase 7** adds
enum `match`-per-variant composition: a synthetic local selector enum
and result enum, with a real `match selector { ... }` in both the
function body and `ensures`, proving only the selected variant's own
composed claim in its arm. Along the way, found that an artifact
variant's own name can carry a provenance rename (e.g. `fallback`,
lowercase) — a valid audit label but not a valid PascalCase Rust enum
variant identifier — and normalized it before use in the synthetic
types. `just verify-verus` went to `337 verified, 0 errors`, one new
genuine proof. **Phase 8 was broader canary coverage, not a
"rollout"**: `amenable_std` hand-proves each std-lib leaf itself so
derive-witness composition is ready for a downstream user's own custom
composite — the crate never has a real (non-canary) composite of its
own to register, by design. Added three canaries no prior phase's
minimal-coverage approach had exercised: a struct with two independent
checked leaves (the `result.0`/`result.1` tuple path), an enum variant
with two checked leaves (the `r0`/`r1` bind-name path), and a
struct-in-struct (recursion past depth 1). `just verify-verus` went to
`340 verified, 0 errors`, three new genuine proofs. Also added a
permanent `trybuild` regression test for the `ClassifiedWitness`
`E0277` guarantee, verified once by hand in Phase 1 but never locked in
until now — the committed `.stderr` snapshot confirms the error still
names the exact unclassified leaf. **Post-Phase-8** closed a second,
unrelated duplication: every `impl Ensures<VerusVerifier>`/
`impl Requires<VerusVerifier>` in `verus_witness.rs` was a hand-typed
`&'static str` restatement of real carrier source, verified only by
eye — worst case, `NonZero<T>`'s two real clauses split across a
"primary" trait impl and a bolted-on supplementary registration,
repeated by hand across all 12 real widths. Four new
`amenable_derive` macros (`verus_ensures_witness!`/
`verus_requires_witness!` for harness-clause-anchored claims,
`verus_ensures_predicate!`/`verus_requires_predicate!` for claims
anchored to one or more named `spec fn`s directly) now derive
`Bound = &'static [&'static str]` and every `ContractRecord` from the
real source at macro-expansion time. All 34 real sites migrated; none
hand-typed anymore. `just verify-verus` unchanged at `340 verified,
0 errors`; `derived_witness/` regenerates byte-identical.

**Description:** The derive-witness/Verus-export pipeline added in
commits `969b460`..`0a0abd5` renders composite Verus "proofs" that are
tautological (assumed-true free booleans instead of real leaf-proof
calls) and treats `Opaque` (unclassified) leaves identically to `Checked`/
`Trusted` ones — confirmed by a real `just verify-verus` count going from
`332 verified` to `335 verified` with zero new genuine content. This plan
replaces the free-boolean composition with real calls to (or citations
of) each leaf's actual proof, blocks `Opaque` leaves from ever being
exportable via a compile-time `ClassifiedWitness<V>` marker trait
(verified real `E0277`, not a runtime failure or a `const`-eval panic,
with field-level precision), converts the touched structs to owned
`String` fields, and removes a `#[allow(dead_code)]` policy violation
along the way.

### Naming Raw Requires/Ensures Bounds (cordial-driven)

**Document:** [CONTRACT_BOUND_NAMING_WORKFLOW.md](CONTRACT_BOUND_NAMING_WORKFLOW.md)

**Status:** 🔲 Ongoing — matching mechanism redesigned in an earlier
session (call-shape recognition replaced text matching, closing a real
correctness gap it had); `amenable_creusot` fully cleared under the new
mechanism; `amenable_kani` and `amenable_verus` both actively in
progress, each with many named clusters landed. Per this project's own
convention (see the linked doc's own "Status" note), backlog counts are
intentionally not tracked here or in the linked doc — they drift too
fast to stay trustworthy. `elicit_doc` was renamed/rearchitected into
`cordial` (`~/repos/cordial`, 2026-08-20) — re-run `cordial quality
--project /home/erik/repos/amenable` before picking up work; treat its
live checklist (`~/.cordial/amenable/findings/antipatterns.checklist.md`)
as the only source of truth for what remains. A real, crate-wide compile-blocking
bug from this sweep's own past work was found and fixed while
unrelated `KaniCompose` work needed a real `cargo kani` build to
verify against (`amenable_kani`'s "State Machine Derivation" entry,
below, has the full account) — see the linked doc's own new Gotchas
entry: bare named-contract calls become ambiguous, silently, the
moment a second verifier registers a competing impl for the same type,
and the routine check-all-package sweep can never catch it since it
never runs `cargo kani` at all. Worth a real `cargo kani --all-features`
sanity check after any future bare-call rewrite in this sweep, not
just `cargo check`/`clippy`.

**Description:** Every `requires`/`ensures` bound should be a named
`amenable_core::{Ensures, Requires}` contract type with one real,
callable predicate, not a raw expression restated per site.
`cordial`'s `ANTIPATTERN-UNNAMED-CONTRACT-BOUND-001` rule scans all
three verifier backends for raw bounds and groups them into duplicate
clusters by clause shape, ranked by size, so the highest-leverage
(most-repeated) bound gets named first. A site is recognized as
compliant only when its clause is a real call to a registered contract's
predicate, never by matching clause text against the registered
fragment's text — the linked document covers why that mattered (a
coincidental Verus text match was hiding real unnamed debt) alongside
the contract-type design pattern, the cordial tooling internals, a
step-by-step workflow, and every gotcha hit along the way (associated-
type uniqueness, `#[cfg(kani)]` import gating, macro/attribute literal
limitations, `#[logic(open)]` vs `#[logic(opaque)]` visibility rules,
`cargo-expand` as ground truth for auditing macro-generated
registrations). Written so another agent can resume the sweep from the
top of the ranked list without re-deriving any of this.

### Fixing `Establish` to Actually Gate Obligations

**Document:** [PROVABLE_FROM_PLAN.md](PROVABLE_FROM_PLAN.md)

**Status:** ✅ Implemented — `Establish<C, V>` requires `C: ProofToken` and
takes the credential by value, retrofitted across all ~65 sites in
`amenable_kani`. Workspace compiles, clippy is clean, and
`cargo test --workspace` passes with zero failures.

**Description:** `Establish::establish(credential: &C)` did not enforce
that `C`'s value ever demonstrated anything — any value of the credential
type minted a token, which `elicit_doc`'s antipattern scanner correctly
flagged (`unused_underscore_arg`), and which was also live as a real bug
in `Stoplight`'s `exchange` bodies (`input.primary()` used where the real
`.sidecar()` token was available). `Establish` is this codebase's
deliberate rename of `elicitation`'s `ProvableFrom<C>` — the fix lands
inside `Establish` itself, no sibling trait, reusing the already-existing
`ProofToken` trait as the credential bound rather than inventing a
bespoke marker-struct mechanism like `elicitation`'s `Established<P>`.
`AddEvidence`/`calculator.rs` got a reflexive `AddEvidence: ProofToken`
impl (not `Sum`, whose constructor is public and unguarded); `Stoplight`
got its three `Establish` impls retargeted to the real `*Token` types
plus the `.primary()`→`.sidecar()` fix; every accommodation-model site
across `sync_mpsc.rs`, `slice.rs`, `fs.rs`, `io.rs`/`alloc_string.rs`,
`process.rs`, `sync_lock.rs`, `thread.rs`, `path.rs`/`panic.rs`/
`std_panic.rs`, `std_hash.rs`/`std_time.rs`, and the chained UTF-8 buffer
family got a purpose-built `demonstrate_*` witness token (or a reflexive
impl where already gated by construction). See
[PROVABLE_FROM_PLAN.md](PROVABLE_FROM_PLAN.md)'s Resolution section for
the full site list.

### Exchange Method Proof Derivation (Kani + Creusot)

**Document:** [EXCHANGE_PROOF_DERIVATION_PLAN.md](EXCHANGE_PROOF_DERIVATION_PLAN.md)

**Status:** ✅ All eleven steps (0 through 10) done and verified — Kani and
Creusot, all three `stoplight.rs` edges, each carrying real,
tool-confirmed contracts (`cargo kani`, `cargo creusot prove` —
`Proved (112 files) ✔`) with a genuine injected-regression check per
edge per backend, plus the by-hand Kani-side wiring generalized into
a real `#[amenable_derive::exchange(..)]` attribute macro (Step 3),
re-verified against all three edges post-swap, plus all three edges
composed into one `#[kani::stub_verified]` full-cycle harness (Step
4) after a real dead end (`Infallible`'s uninhabitedness blocks
`Arbitrary` reconstruction outright, not just an ownership problem)
resolved by swapping in a real, constructible `StoplightError::
NotUsed` variant, plus `Stoplight`'s real `Amenable` impl (Step 5) —
the first anywhere in the tree, every surface backed by queried or
compiler-checked data (Step 5 originally added a feature-gated
`amenable_kani -> amenable_creusot` dependency to keep `creusot_
surface()` honest; Step 8 later removed it as a real architectural
violation -- see below). Creusot's own companion is now generated
directly from the real Kani body (Step 9, see below) — the mirror
that used to genuinely (if separately) implement the real `amenable_
core` trait family, and the `stoplight_mirror_consistency_test.rs`
that kept it honest against drift (Step 2), are both gone, superseded
by generation rather than hand-duplication. Kani and Creusot
first, by deliberate sequencing — Verus support for `Exchange` was
deferred here rather than bolted on weak (Verus can't check an
arbitrary compiled Rust body directly, only `verus! {}`-native code),
and has since landed with a real answer of its own: see "Verus
Exchange Proof Derivation" below. Step 6, added after the fact: all
three edges' DFCC `#[kani::ensures(...)]` closures now call through
real, registered `Ensures<KaniVerifier>` contract types
(`kani_ensures!`) instead of restating the boolean inline — the same
source-of-truth discipline `rust_std`'s own proofs already apply
everywhere else, closing the same drift risk one level deeper than
Step 3 did (the contract's own content, not just its registration/
delegation machinery). Re-verified via real `cargo kani` on all three
edges plus the Step 4 composition harness, and a real non-vacuous
regression check. Step 7, tackled by explicit direction after
`VERUS_EXCHANGE_PROOF_DERIVATION_PLAN.md`'s own Step 5 ("let's tackle 3
now"): `#[amenable_derive::exchange(..)]` itself now generates that
same DFCC `ensures` attribute rather than requiring it hand-typed at
each call site — the macro clones the parsed method and injects
`#[cfg_attr(#cfg, #cfg::ensures(..))]`, calling through `<Evidence as
Ensures<V>>::ensures(*result)` via fully-qualified syntax (no `use
amenable_core::Ensures;` needed at the call site anymore, so that
import was removed from `stoplight.rs` along with all three
hand-written attributes). The human-authored predicate itself didn't
move — it's still the `kani_ensures!` invocation immediately above
each macro call — only the fully mechanical wiring did. Verified the
authoritative way (`cargo expand` can't render Kani's own attribute
macro output outside the real toolchain): real `cargo kani` on all
three edges plus the composition harness, and a real non-vacuous
regression check pointing at the exact generated closure.

Step 8, a direct, firm correction, not a self-initiated cleanup:
"kani, creusot and verus never depend on each other... shared types
live in core." Step 5's `amenable_kani -> amenable_creusot` Cargo
dependency was exactly that violation. The real fix needed a real
question answered first: why did `creusot_surface()` need a cross-
crate import at all, when `kani_surface()` queries a shared registry
with none? Because `amenable_creusot` was believed unable to call
`inventory::submit!` at all (a real, confirmed ICE documented in
`amenable_std::creusot_witness`, which is why that crate's own ~90-
carrier witness-bridge surface was relocated there in the first
place, years earlier). Checked directly with the real toolchain, not
assumed still true: in an isolated, throwaway probe crate (deleted
after confirming, never kept live), `#[cfg(not(creusot))]`-gating
`inventory::collect!`/`inventory::submit!` *in place* avoids the
translator error entirely -- with two real refinements found in the
same probe, not assumed from the existing gallery case: `collect!`
needs its own gate too (a different, independent translator error,
not just `submit!`'s), and `Box<dyn Iterator<..>>` as a concrete
associated-type value (the exact pattern `amenable_kani::stoplight`'s
real `Provenance` impls use) is a real, separate translation error
("forbidden dyn type"), confirmed distinct from the already-fixed
RPITIT case by testing it ungated first. Documented permanently as a
new `amenable_std::creusot_gallery` case before applying it for real,
matching the gallery's own stated purpose (hypothesis, trial, error,
resolution). `amenable_creusot::stoplight` now registers its own three
`ProofRecord`s this way; `creusot_surface()` queries the shared
registry instead of importing across crates; `amenable_kani`'s
`creusot` feature and Cargo dependency on `amenable_creusot` are gone
entirely, with no `#[cfg(feature = ..)]` split needed anymore either.
Full workspace clean, `just verify-creusot` still `Proved (110 files)
✔`. Deliberately not done here: migrating `amenable_std::creusot_
witness`'s own much larger (~90-registration) surface the same way --
a real, much bigger undertaking, flagged for explicit future
direction, not started.

Step 9, direct pointed pushback on Step 8 itself: "why is codegen
deliberately been ignored, in spite of multiple issues with inventory
at every step? Isn't the solution staring us in the face?" Step 8's
own `#[cfg(not(creusot))]`-gating fix was real but still a patch
*around* `inventory`'s repeated friction with Creusot's translator,
not a fix for the friction itself. Pointed at real, already-shipped
proof this codebase already had a better answer:
`amenable::verus_export`/`emit-verus-witnesses` (~950 lines, real,
already shipped) reads a registry from inside an ordinary, never-
translated binary and *generates* real, checked-in, `inventory`-free
source the verifier just compiles as static code -- `inventory` never
has to survive into the translated crate at all, not gated out item by
item, simply never present. `#[amenable_derive::exchange(..)]` now
captures each real edge's transition body verbatim (`Span::
source_text()` on the method's own block, the same technique
`harness!` already uses) and registers it, with its real type names,
as a new `amenable_core::ExchangeEdgeRecord` -- safe unconditionally,
since `amenable_kani` is never translated by anything. A new
`amenable::creusot_export`/`emit-creusot-companions` (mirroring
`verus_export`'s own architecture exactly) reads that registry and
writes real `harness! { creusot, .. }` + `ProofRecord` files into
`amenable_creusot/src/generated/*.rs`, `include!`d into `stoplight.rs`
directly (no `mod`, so no imports needed -- also means `cargo fmt -p
<crate>` never discovers them, a real, confirmed gap fixed by running
`rustfmt` directly on the generated files instead). Deliberately
narrow: generates only the per-edge transition body, not the
surrounding state/token/sidecar type definitions, which stay hand-
written, stable, one-time infrastructure.

Five real bugs found and fixed while building this, none assumed away:
capturing the real `Ok(..)`-wrapped body meant the generated signature
needed the real `Result<Output, Error>` return type (not the old
mirror's silently-simplified bare `Output`), which meant `StoplightError`
itself needed a real Creusot-local counterpart; `rustfmt` doesn't
reformat inside an opaque macro invocation by default, confirmed
directly (a real generated file's `stringify!`-produced type spacing
survived a real `cargo fmt` run untouched), fixed with a targeted
text-cleanup pass; the captured body's own first line loses its
original indentation to `.trim()` while every other line keeps it, a
real confirmed dedent bug, fixed with a real dedent-then-reindent
pass; a real chicken-and-egg bootstrap problem (the generator needs
`amenable_creusot` to compile, which needs the generated files to
exist), broken by committing minimal placeholders once; and the
generated header's `//!` caused a real `E0753` once `include!`d
mid-file, fixed with plain `//`. `stoplight_mirror_consistency_test.rs`
is gone entirely, not superseded by a freshness check -- with the
body generated directly from the real source, there's nothing left to
guard drift *between*. `amenable_creusot`'s now-unused `proc-macro2`/
`quote`/`syn` dev-dependencies (only needed by that test) removed too.

Verified for real at every step: `just verify-creusot` -- `Proved (112
files) ✔` (up from 110). Full workspace clean, `just check-all-creusot`/
`test-creusot` clean end to end, `cargo test --workspace` clean
(matching this project's own per-package/per-feature testing
convention -- a real, confirmed nuance found along the way: a blanket
`--all-features` sweep transitively links `amenable_creusot` into
`amenable_kani`'s own test binary via `amenable_std`'s `creusot`
feature, even with no edge `amenable_kani` itself declares, which its
own test's doc comment now states precisely rather than overclaiming).
`just generate-creusot` wired into every Creusot recipe that checks
anything, matching `emit-verus-witnesses`'s own "regenerate before
checking" placement exactly.

Step 10, grown out of the GAAP ledger worked example
(`GAAP_LEDGER_PLAN.md`'s Step 3) rather than a `Stoplight`-specific
request, but changing `Stoplight`'s own architecture along the way:
confirming `amenable_creusot -> amenable_gaap` as a real dependency
worked (zero ICE, `Witness<CreusotVerifier>` directly on real
`Validated`/`Committed`, no mirror needed) led to trying the same for
`amenable_kani`, which first hit a genuine Cargo cycle through
`amenable_std`'s own optional edge back to `amenable_creusot`. Fixed
by flipping that edge for real -- `amenable_creusot` now depends on
`amenable_std` unconditionally, and `amenable_std`'s entire
~90-carrier Creusot witness-bridge surface moved wholesale into
`amenable_creusot::rust_std_witness` (closing Step 9's own deferred
migration item). With the cycle gone, `amenable_creusot ->
amenable_kani` *did* compile clean too -- but adding it was caught
immediately as violating Step 8's own rule a second time: "what the
heck are you trying to share from Kani to creusot and why isn't it in
core?" The real fix: `Green`/`Yellow`/`Red` moved to `amenable_core`,
a neutral crate both backends already depend on independently,
matching `amenable_gaap`'s own split exactly. `amenable_creusot`'s
Cargo dependency on `amenable_kani` was removed entirely. Verified for
real: full workspace `check`/`fmt`/`clippy -D warnings`/`test --all-
features` clean, `cargo creusot prove -- -p amenable_creusot` --
`Proved (119 files) ✔`, `cargo tree -p amenable_creusot` confirms no
`amenable_kani` edge remains.

**Description:** `Exchange<Input, Output>` proves a Hoare-triple-shaped
claim over a real method body, not a static structural fact — the
derive-witness composition machinery's generic conjunction/case-split
rule has no analog here, since correctness depends on the transition's
actual logic. The one real `Exchange` impl in the tree
(`amenable_kani::stoplight`) doesn't prove its own body: the Kani
harness backing each transition's `Witness` proof calls a disconnected
free function (`next()`) that's supposed to mirror the real `exchange()`
logic, kept in sync only by a doc comment — the same hand-authored-
claim-can-drift-from-source risk this session's `Ensures`/`Requires`
macro work closed for descriptive text, one level more dangerous since
what could drift here is executable logic. Real prior art exists in
`~/repos/elicitation` (`#[formal_method]`/`#[derive(
VerifiedStateMachine)]` plus the operational reference `KANI_FOR_VSMS.
md`, all read directly before writing this plan, not summarized from
memory): real `#[kani::requires]`/`#[kani::ensures]` on the actual
transition body (never a call-through wrapper — that doubles CBMC's
work under DFCC), checked via `#[kani::proof_for_contract]` using a
forgive-and-forget construction, reusable via `stub_verified()` for
modular composition; a real Creusot `requires`/`ensures` companion
against the real body. Checking amenable's own source (not assuming
from the elicitation read) found `KaniCompose` already ported,
including the `kani_any()` method the forgive-and-forget pattern
needs, with a genuinely different — not yet empirically confirmed
equivalent — approach to symbolic enum construction than
elicitation's; `KaniVariantState` was not ported, and per elicitation's
own current-architecture notes is likely unnecessary since it only
supports their now-legacy per-variant harness path. `amenable_core`'s
own trait family (`Establish`/`ProofToken`/`Witness<V>`) is already
stricter than elicitation's `Established`/`Prop` equivalents (no
`assert()` escape hatch; no defaulted-empty/silently-trivial invariant
naming), so this plan ports the *technique*, not the trait scaffolding,
and keeps that extra strictness rather than loosening it for codegen
convenience. Kani function contracts (`requires`/`ensures`/
`proof_for_contract`/`stub_verified`) are entirely unused in this
codebase today — this is the first real occupant of `Amenable::
kani_surface()`/`creusot_surface()`, which exist in `amenable_core::
state_machine` but have zero implementors anywhere in the tree.

Design converged on a deeper fix during this discussion, now landed as
Step 0: `Sidecar::Proposition: Evidence` alone can never guarantee a
real proof exists, for any transition, because `Evidence` is
deliberately verifier-blind. The fix that shipped is `Sidecar<V:
Verifier>` with `Proposition: Evidence + Witness<V>` — a compound
bound on the one trait that actually needs it, using the exact shape
`Establish<C, V>: Evidence + Witness<V>` already used. `V` is a shared
generic parameter (not an associated type: an associated type is only
checked where a caller remembers an explicit equality bound, a shared
parameter makes a verifier-mixing pipeline a plain type error
everywhere). Also corrected along the way: an earlier read of
`AMENABLE_PLAN.md`'s root-state discussion over-generalized "the light
is currently Green is asserted, not derived" (a real claim about
*runtime* state occupancy) into "root states are categorically
unproven" (false) — root-ness and whether a *type's construction* has
real invariant content are orthogonal axes; `Green`'s proof is trivial
because its constructor is infallible, not because it's a root.
Notably, the first fix attempted — adding `V` to `Evidence` itself as
a supertrait (`Evidence<V>: Witness<V>`) — was fully built (including
new macro machinery in `amenable_derive`) before `cargo check
--workspace` showed it broke `amenable_std`'s generic provenance/
audit-report writer, which has no relationship to any verifier's proof
and shouldn't need one to compile. Reverted in full once that surfaced;
`Sidecar<V>` doesn't have that cost, because it states the requirement
only where it's needed. `stoplight.rs`'s three `Exchange` impls and one
`Sidecar` impl were the only real migration; no `ProofToken` impl
anywhere needed touching. `cargo check`/`test`/`clippy --all-targets
--all-features -D warnings --workspace` all clean.

Step 1's Kani side then landed for all three `stoplight.rs` edges, with
a real, previously-undocumented Kani 0.67.0 limitation discovered
along the way: contracts can't target a trait method when the trait
itself is generic (`Exchange<Input, Output, V>` is) — a real compiler
error, not a syntax mistake, confirmed by trying the direct approach
first and getting "Kani does not currently support stubs or function
contracts on generic functions in traits." Fix: real logic and its
contract moved to plain inherent methods (`Stoplight::green_to_yellow`
etc.), with each `Exchange::exchange` impl reduced to a single-
expression delegation — the same body, not a proxy with different
logic. Contract content is legitimately trivial (every state type here
is zero-field with exactly one possible value, no body branches or can
panic) — proving "never panics, always `Ok`" once the type system
itself already enforces which transitions are legal. Verified for
real three ways: all three harnesses pass under real `cargo kani`
(`-Z function-contracts`, wired into a new `verify-kani-contract`
justfile recipe); a deliberately injected `panic!` in one body made
the same harness fail at the exact injected line, then verified clean
again after reverting; an unrelated pre-existing Kani harness spot-
checked to confirm nothing else regressed.

The Creusot side landed too, after a real design detour: the obvious
approach (`amenable_creusot` depends on whatever crate owns `Stoplight`)
is a dead end no matter which crate that is — `amenable_std` and
`amenable` both already optionally depend back on `amenable_creusot`
(a direct cycle either way), and even a cycle-safe new crate would hand
`creusot-rustc`'s translator ordinary Rust infrastructure it has
already ICE'd on for real in this exact codebase (confirmed via
`amenable_std::creusot_witness`'s own doc comment: it sweeps *every*
local item in whatever crate it directly compiles, gated or not, and
crashed on a return-position `impl Trait` and an `inventory::submit!`
static). Resolution: reuse the accommodation-model pattern already
proven on the Kani side — `amenable_creusot/src/stoplight.rs` defines
sanitized, locally-owned mirror types (no Cargo dependency on the real
ones at all) plus one real contract function per edge, now covering
all three (`green_to_yellow`, `yellow_to_red`, `red_to_green`).
Verified for real: `cargo creusot prove -- -p amenable_creusot`
succeeds; an injected `panic!()` in each of the three functions in
turn made the exact same run fail at that function's own goal (`Goal
Coma.vc_green_to_yellow: ✘`, `::vc_yellow_to_red: ✘`, `::
vc_red_to_green: ✘`), confirming every contract is real, not vacuous;
each reverted and re-verified clean.

Then, while starting Step 2 (deriving the contract from real source
instead of hand-typing it), a real correction: the mirror's own doc
comment claimed it couldn't use *any* `amenable_core` trait-family
machinery, conflating "can't depend on `amenable_kani`/`amenable_std`"
(true) with "can't use `amenable_core`'s traits" (false) —
`amenable_creusot` already has a real, unconditional dependency on
`amenable_core`, and none of `Evidence`/`Witness<V>`/`Sidecar<V>`/
`Establish<C, V>` contain the specific patterns that caused the real
ICEs (those were `Provenance`'s `Box<dyn Iterator>` and `inventory::
submit!`, not the trait family itself). Rebuilt the mirror to
genuinely implement the real traits — a real generic `Established<T,
Token>: Sidecar<CreusotVerifier>`, real `Establish<_, CreusotVerifier>`
impls — so the exchange bodies now use the same call shape as the real
Kani bodies (`Yellow::establish(input.sidecar())`), differing only in
which concrete types they close over. Re-verified the full cycle
afterward: `cargo creusot prove` still succeeds (`Proved (110 files)
✔`, more proof obligations than before since the trait methods
themselves now get checked too), all three edges still fail their own
regression check when broken, and full workspace `check`/`test`/
`clippy --all-features -D warnings`/`fmt` and `just check-all-creusot`
all clean afterward. Step 1 is complete for both backends, all three
edges.

Step 2 landed too, but not as originally framed. The plan's original
text described a `syn`-based generator deriving the Creusot mirror
body from the real Kani source; once both bodies existed side by side
it became clear that framing was ill-defined — the real body
(`Result`-returning, `&self`-taking) and the mirror body (bare return,
free function) necessarily differ by one specific transform (stripping
the real body's trailing `Ok(...)` wrapper), not a literal token
substitution, so "derive B from A" isn't well-defined the way Verus's
verbatim predicate-text extraction is. What got built instead:
`amenable_creusot/tests/stoplight_mirror_consistency_test.rs`, a real
consistency check — reads `amenable_kani/src/stoplight.rs` directly
off disk (`fs::read_to_string`, no Cargo dependency, same constraint
as the mirror itself), parses both the real body and the mirror's own
already-exported `harness!`-captured source constants with `syn`,
applies the one documented `Ok(...)`-unwrap transform, and asserts
token-stream equality — three tests, one per edge, all passing.
Verified as real (not vacuous) by injecting drift on both the mirror
side and the real Kani side separately and confirming a precise
failure each time, then reverting both. Full re-verification after
landing: `cargo fmt --all --check`, `cargo check --workspace`, `cargo
clippy --workspace --all-targets --all-features -- -D warnings`, and
`cargo test --workspace` (61/61, up from 60) all clean; `just
verify-creusot` still `Proved (110 files) ✔`, confirming the new
test-only `syn`/`quote`/`proc-macro2` dev-dependencies don't disturb
the real Creusot toolchain invocation. Step 2 is complete.

Step 3 landed too, generalizing the by-hand pattern into an attribute
macro -- also not quite as originally framed, on two points the plan's
own earlier steps had already settled empirically. First: the plan's
sketch put the generated contract on `Exchange::exchange` itself, but
Step 1 already found that has to live on a plain inherent method
instead (Kani's generic-trait-method limitation), so the macro attaches
to that inherent method's `impl SelfType { .. }` block. Second: Step
1's Creusot side turned out to need no new generated scaffolding at
all -- its harness functions are free functions wrapped directly in
`harness!`, with no per-edge `Witness`/`ProofRecord`/`Exchange`-impl
trio to mechanize, so Step 3's real scope narrowed to Kani only. What
got built: `#[amenable_derive::exchange(cfg = .., verifier = ..,
evidence = .., proof_artifact = .., harness_fn = .., harness_const =
.., evidence_id = ..)]` (`crates/amenable_derive/src/exchange.rs`),
generating the `Witness<V>` impl, its `ProofRecord` registration, and
the `Exchange` trait-impl delegation -- deliberately *not* touching the
contract, the method body, or the `harness!` invocation, since the
latter's verbatim-source capture (`Span::source_text()`) only works
when its braced item is written directly at the call site; splicing it
through this macro would have silently degraded that capture to a
token-reconstructed fallback. `Input`/`Output`/`Error` are extracted
from the method's own signature via `syn`, not re-typed as macro
arguments. Applied to all three `Stoplight` edges in place of their
by-hand trio (the cycle-back edge's `"::cycle_back"` id preserved via
an `evidence_id` argument). Verified for real: `cargo expand -p
amenable_kani stoplight` confirmed byte-for-byte identical output to
the prior hand-written expansion, including the `harness!` constant
still capturing real multi-line source (not the degraded fallback);
`just verify-kani-contract` re-run on all three harnesses, all still
`VERIFICATION:- SUCCESSFUL`; a real injected `panic!()` in
`green_to_yellow` failed at the exact injected line under `cargo kani`
(confirming the generated delegation still routes through the real
body), reverted and re-verified clean. Full workspace `fmt --check`/
`check`/`clippy --all-targets --all-features -D warnings`/`test` all
clean (61/61, unchanged), and the Step 2 consistency test unaffected
(it never touches macro-generated code). Net: `stoplight.rs` shed
about 114 lines against a roughly 24-line addition to
`amenable_derive`. Step 3 is complete.

Step 4 landed too, composing all three edges into one full-cycle
`#[kani::stub_verified]` harness -- but only after a real dead end,
surfaced to the user rather than pushed through. `stub_verified`
needs `Arbitrary` for a stubbed call's whole return type, since
stubbing reconstructs a symbolic stand-in for it. `Established<T,
Token>: Arbitrary` was easy (own type, own fields). But every edge
returned `Result<Established<T, Token>, std::convert::Infallible>`,
and `Infallible` is uninhabited by design. Three real attempts, each
a confirmed failure: implementing `Arbitrary` for the whole `Result`
directly (blocked by the orphan rules twice over -- `Result` is
foreign and not `#[fundamental]`, so no downstream crate may
implement a foreign trait for it at all, generic or concrete);
swapping `Infallible` for a local uninhabited `enum Never {}` so a
local `Arbitrary` impl became legal (compiled, but Kani's `Result`
reconstruction unconditionally calls `E::any()` while exploring the
stub's return-value space, and the only body an uninhabited type's
`any()` can have -- `unreachable!()` -- panics for real under `cargo
kani`, confirmed by the exact failure, not assumed). At that point the
honest options were a documented limitation with no composition demo,
or redesigning `Exchange`'s core signature (fixed in Step 0) to use a
locally-owned `Result`-analog -- disproportionate to what Step 4
asked for. Surfaced to the user as a real fork instead of decided
unilaterally.

The user's fix: the problem was never which crate owns the
uninhabited type, it's the uninhabitedness itself that `Arbitrary`
reconstruction can't survive -- so stop using an uninhabited type.
`StoplightError` (one variant, `NotUsed`) replaces `Infallible` as
every edge's `Error`: ordinary, safely constructible, never actually
returned by any edge (each edge's own already-proven `#[kani::
ensures]` contract is what establishes that, not the type system).
Verified for real, including the check unique to this step: a
`panic!()` injected into `green_to_yellow`'s real body failed its own
`proof_for_contract` harness at the exact line, while the composition
harness -- which stubs that same function -- stayed `VERIFICATION:-
SUCCESSFUL` throughout, confirming stubbing genuinely never executes
the body. Reverted and re-verified clean. Full workspace `fmt
--check`/`check`/`clippy --all-targets --all-features -D
warnings`/`test` all clean (61/61, unchanged), plus the real
`stoplight_test.rs` integration tests (updated to use `StoplightError`
in place of `Infallible`) and the Step 2 consistency test both still
passing. Step 4 is complete.

Step 5 landed last, wiring `Stoplight` into `Amenable` for real -- the
first `impl Amenable for` anywhere in the tree. Not elicitation's
`VerifiedStateMachine::transition_harnesses()`/`vsm_kani_proof()` shape
(`Vec<proc_macro2::TokenStream>` for a `build.rs` to write out) --
amenable has no proof-source codegen pipeline anywhere, so that shape
doesn't map onto anything real here. Every method instead queries
already-existing data or references real, compiler-checked items:
`kani_surface()` queries the same `KaniProofRegistration` inventory
catalog `harness!` already populates (the same mechanism the CLI's own
harness listing uses), filtered to this module's own entries via
`module_path!()` evaluated in the same file `harness!`'s own ids were
built from -- no hand-typed module name on either side.
`creusot_surface()` originally (Step 5) referenced `amenable_creusot::
stoplight`'s real exported harness-source constants directly, gated
behind a **new** `creusot` feature added to `amenable_kani` itself
(mirroring `amenable_std`'s identical dependency) -- a real Cargo
dependency from `amenable_kani` to `amenable_creusot`, later caught as
a genuine architectural violation and fixed in Step 8 (see below):
`creusot_surface()` now queries the same shared `inventory` registry
`kani_surface()` already does, with `amenable_creusot::stoplight`
registering its own `ProofRecord`s instead of `amenable_kani` importing
its constants across a Cargo edge. `verus_surface()` returns an honest
empty list (no Verus `Exchange` proof exists yet). `audit_surface()`
returns the real captured
verbatim source of all four Kani harnesses. A new test file
(`stoplight_amenable_test.rs`) asserts all four surfaces for real under
both feature configurations, not just that the code compiles. Full
workspace `fmt`/`check`/`clippy --all-features -D warnings` clean under
both configurations, `cargo test --workspace` 62/62 (up from 61). Step
5 is complete -- every step this plan originally scoped now is.

### Verus Exchange Proof Derivation

**Document:** [VERUS_EXCHANGE_PROOF_DERIVATION_PLAN.md](VERUS_EXCHANGE_PROOF_DERIVATION_PLAN.md)

**Status:** ✅ Steps 0 through 8 done and verified — the real, unmodified
`amenable_core` trait family (`Evidence` including a genuine
self-referential root, `Witness<V>`, `ProofToken`, `Sidecar<V>`,
`Establish<C, V>`, `Exchange<Input, Output, V>`, and now `Ensures<V>`)
compiles and verifies under real Verus via `#[path]` mod-inclusion,
with the complete `Stoplight` three-edge cycle proven as a worked
example. Every mechanical piece of each edge is now macro-generated,
including the call sites themselves as of Step 8: the `ensures(...)`
clauses route through registered `Ensures<GalleryVerifier>` contract
types, those contract types are generated by `exchange_support::
verus_ensures!` (Step 6), and the `Witness<GalleryVerifier>`/
`Exchange<..>` impl *definitions* are generated by `exchange_support::
verus_exchange!` (Step 7) — Step 8 then closed the last hand-typed
piece, generating the three `verus_exchange!(..)` *call sites*
themselves (`amenable::verus_exchange_export`/`emit-verus-exchange-
companions`, reading the same `amenable_core::ExchangeEdgeRecord`
registry `emit-creusot-companions` reads) directly from Kani's own
captured transition bodies, the identical class of fix `EXCHANGE_
PROOF_DERIVATION_PLAN.md`'s own Step 9 landed for Creusot. Only each
edge's real transition body stays hand-authored: `verus
--crate-type=lib` reports `419 verified, 0 errors`. Closes the Verus
deferral `EXCHANGE_PROOF_DERIVATION_PLAN.md`'s own "Scope" section
left explicitly open, via a third option neither that section nor
elicitation's own real Verus prior art (`VERUS_FOR_VSMS.md`'s "V13"
`assume_specification` pattern, which *axiomatizes* the real body
rather than checking it) considered: real source brought in verbatim,
not duplicated and not axiomatized.

**Description:** Two real, distinct Verus limitations blocked the real
trait family before this landed. `Evidence`'s deliberate "a root is its
own basis" idiom (`type Basis = Self`) unconditionally tripped Verus's
static cyclic-self-reference checker, with no per-item escape hatch
(`#[verifier::external_body]` doesn't apply to trait impls at all;
`#[verifier::external]` compiles but crashes Verus's own AIR backend).
A real methodological mistake happened and was caught here: an initial
fix that cfg-gated only `Evidence::chain()`'s recursive default method
looked sufficient in testing, but was a false positive -- Verus stops
at the *first* cyclic definition it finds in a compilation, and the
real, un-fixed trait was still failing elsewhere in the same test file
the whole time, masking the real signal. Only a fully isolated,
single-variable re-test caught it: the actual cause is the `type
Basis: Evidence` bound itself, independent of `chain()`. The real fix:
`amenable_core::evidence` now declares `Evidence` twice,
`#[cfg(..verus_keep_ghost)]`-exclusive -- the real, unchanged shape for
every ordinary toolchain (Verus's own driver unconditionally sets that
cfg; no ordinary toolchain ever does), an unbounded, `chain()`-free
variant only under Verus. `Sidecar<V>`'s generic `ProofToken<
Proposition = T>` associated-type-equality bound separately crashed
Verus's AIR backend outright (not just a warning) -- fixed via four
`#[verifier::external_trait_specification]` companion traits
(`ExVerifier`, `ExEvidence`, `ExProofToken`, `ExWitness`), each needed
for a distinct, real reason found empirically (a bound mismatch;
Verus's internal trait-conflict checker needing names resolvable in its
own generated code, cascading through `Witness<V: Verifier>`'s own
bound to `Verifier`; and `'static` belonging on the specification trait
itself, not the mirrored associated type).

Landed as a real, permanent `amenable_verus::gallery` (mirroring
`amenable_kani::gallery`'s role) rather than scratch files, adapted for
a real structural difference: Verus verifies the whole file tree in one
pass with no per-case selection, unlike Kani's per-harness `cargo kani
--harness` selection, so nothing here is `#[cfg(kani)]`-shaped and
every case has to stay genuinely `pub`-reachable to avoid real
dead-code lint errors. Extended to the full `Stoplight` cycle
(`amenable_kani::stoplight`/`amenable_creusot::stoplight`'s own shape,
not a simplified stand-in), surfacing a real, new, checked-not-assumed
finding: Kani 0.67.0 can't place a contract on `Exchange::exchange`
because the trait is generic, forcing Kani's real proofs onto inherent
methods with the trait impl reduced to delegation -- that limitation
does not carry over to Verus, whose `ensures` clauses work directly on
the real, generic trait method for all three edges. Verified non-vacuous
via a real injected bug (`Err(())` swapped for a real `Ok(..)` body)
producing a precise `postcondition not satisfied` failure at the exact
line, reverted and re-verified clean; `full_cycle` chains all three real
`Exchange::exchange` calls together, matching the real `Stoplight`'s own
cycle. Full workspace `fmt`/`check`/`clippy --all-features -D
warnings`/`test` clean throughout, `just check-all-verus` clean,
`just verify-creusot` unaffected (`Proved (110 files) ✔`). Landed in
three commits: `ce77446` (the two trait-family fixes plus gallery
infrastructure), `5eb3566` (the full `Stoplight` cycle), and Step 5.

Step 5, tackled by explicit direction ("let's tackle 2 before 3" —
Verus contract-routing before extending the pattern into a macro
layer): `amenable_core::contract` (`Ensures<V>`, `Requires<V>`) mod-
included for the first time, closing the same gap `EXCHANGE_PROOF_
DERIVATION_PLAN.md`'s own Step 6 closed for Kani. A real finding,
checked rather than assumed: `contract.rs`'s own doc comment predicted
Verus would need the weaker `Bound = &'static str` shape (description
text, not a checked value), since Kani's `Bound = bool`/`ensures()`
-*is*-the-check pattern seemingly can't cross the exec/spec boundary.
That's only true for bounds needing genuinely spec-only constructs —
`Bound = bool` works for Verus too, via `#[verifier::
when_used_as_spec]`: each `Ensures<GalleryVerifier>` impl pairs a real
exec `ensures()` body with a private `spec fn` companion of identical
logic, the same mechanism `vstd::std_specs::result::is_ok` itself uses
to make `Result::is_ok()` usable from spec position. Isolated first in
a dedicated gallery case (`gallery::ensures_contract_bound`) before
touching the real `Stoplight` cycle. Two real sub-fixes needed: the
`when_used_as_spec` attribute has to live on the *impl*, not the trait
declaration (the trait can't carry Verus attributes at all, having no
dependency on `verus_builtin_macros`) — confirmed legal per-impl via
Verus's own test suite and confirmed load-bearing by removing it and
getting a real "cannot call function ... with mode exec" rejection;
and the spec companion needs at least the same visibility as the exec
method it bridges, or a real "more private" rejection follows. Verified
non-vacuous the same way as every other case (a real injected bug
producing a precise `postcondition not satisfied` failure, reverted).
`amenable_core::contract`'s own doc comment updated to describe both
shapes accurately instead of only the Kani case it was originally
written against.

Step 6, tackled by explicit direction ("let's tackle the verus side
macro next, so we maintain our trifecta"): generalized Step 5's
by-hand `Ensures<GalleryVerifier>` wiring into a real macro, the Verus
counterpart to `EXCHANGE_PROOF_DERIVATION_PLAN.md`'s own Step 7
(`#[amenable_derive::exchange(..)]`). Necessarily a different kind of
macro, checked rather than assumed: `verus --crate-type=lib` never
consults `Cargo.toml`, so a separate proc-macro crate like
`amenable_derive` cannot be linked at all under the invocation that
actually matters, regardless of what `Cargo.toml` declares — only a
`macro_rules!` macro works, since it needs no extern crate resolution.
A real, rejected first attempt, isolated in a new gallery case
(`gallery::ensures_macro_generated`) before touching the real
`Stoplight` cycle: a `macro_rules!` macro invoked *inside* the
surrounding `verus! {}` block, expanding directly to `spec fn`/`open`
syntax, fails immediately — ordinary `rustc` macro expansion checks a
`macro_rules!` macro's *output* against plain Rust item grammar before
Verus's own bespoke token-stream processing (which only runs once, on
its own literal input) is ever reachable, and `spec`/`open` aren't
plain Rust syntax. Fixed by having the macro's own expansion wrap its
content in a *fresh, nested* `verus! { .. }` invocation and calling it
from outside any enclosing one — a `verus! {}` call is opaque token
soup to plain `rustc`, not something checked against item grammar up
front. Landed as `exchange_support::verus_ensures!`, real permanent
infrastructure alongside the four `external_trait_specification`
traits already there; applied to the real `Stoplight` cycle, replacing
all three hand-written `Ensures<GalleryVerifier>` impls. One further
real fix needed: `Ensures` became a genuine "unused import" under
plain `cargo clippy` once its only reference moved inside an
`ensures(...)` clause (content plain compilation erases entirely) —
fixed with the identical `#[cfg(verus_keep_ghost)]`-gated import
pattern `EXCHANGE_PROOF_DERIVATION_PLAN.md`'s own Step 6 used for the
same masking on the Kani side. Verified non-vacuous twice (the
isolated gallery case and the real `Stoplight` cycle), each via a real
injected-bug regression check producing a precise failure, reverted.
Together, Kani's proc-macro-generated contract wiring, Verus's
`macro_rules!`-generated contract wiring, and Creusot's consistency
test (Step 2 of the Kani/Creusot plan) complete what the user called
the "trifecta."

Step 7, tackled by direct pushback on an architecture opinion the user
asked for: "hand-written is not the goal here" — Step 6 closed the
`Ensures<V>`-wiring slice, but every edge's `Witness<GalleryVerifier>`
impl (a fixed, always-identical three lines) and `Exchange<..>` impl
scaffold were still hand-copied per edge. Isolated first in a new
gallery case (`gallery::exchange_macro_generated`, a second,
independent minimal state pair built entirely through the new macro)
before touching the real `Stoplight` cycle. A real, rejected first
attempt found a genuine macro-hygiene bug, not a Verus-specific one:
hardcoding the generated method's parameter name (`fn exchange(&self,
input: ..)`) inside the macro's own template meant the caller's own
body block couldn't reference `input` — real, immediate rejection (``
cannot find value `input` in this scope ``, rustc's own diagnostic
naming "macro hygiene" as the cause), since an identifier written
literally in a macro's template is a different syntactic identifier
from anything the caller writes, even spelled identically. Fixed by
taking the parameter name itself as a macro argument. Landed as
`exchange_support::verus_exchange!`, generating both the `Witness<V>`
impl and the `Exchange<..>` impl (its `ensures` clause calling through
`<Evidence as crate::Ensures<V>>::ensures(result)` via fully-qualified
syntax, needing no import at the call site at all — an improvement
over Step 6's own version, which still needed a `#[cfg(verus_keep_
ghost)]`-gated import). Kept deliberately separate from `verus_
ensures!`, mirroring the real split `amenable_kani::stoplight` uses
between `kani_ensures!` and `#[amenable_derive::exchange(..)]`: the
actual postcondition stays visibly hand-authored in its own adjacent
call, not swallowed into the bigger macro. Applied to the real
`Stoplight` cycle, replacing all three remaining hand-written
`Witness`/`Exchange` impls. Verified non-vacuous twice more (the
isolated gallery case and the real cycle), each via a real
injected-bug regression check producing a precise failure pointing at
the macro-generated `ensures` clause, reverted. `verus
--crate-type=lib`: `418 verified, 0 errors` (up from `402`). On both
Kani and Verus, only each edge's real transition body remains
hand-authored now — every mechanical piece around it is generated.

Step 8, direct follow-on to `EXCHANGE_PROOF_DERIVATION_PLAN.md`'s own
Step 9 (the Creusot codegen layer): "does this lesson on codegen apply
to verus or Kani exchange derives?" Kani needed nothing — its own
scaffold was real proc-macro codegen from the start. Verus was the
real gap: Step 7 generated the `Witness<V>`/`Exchange<V>` *macro
definitions*, but every real edge's own `verus_exchange!(..)` *call
site* was still hand-typed and hand-copied, with nothing guarding
drift against Kani's own real body the way Creusot's former mirror at
least had a consistency test for. A new `amenable::
verus_exchange_export`/`emit-verus-exchange-companions` (mirroring
`creusot_export.rs`) reads the same `amenable_core::ExchangeEdgeRecord`
registry `emit-creusot-companions` reads and writes the three real
`verus_exchange!(..)` call sites into `amenable_verus/src/gallery/
generated/stoplight_exchange/*.rs`, `include!`d from `gallery::
stoplight_exchange`. Simpler than the Creusot generator in one real
respect: `verus_exchange!`'s own definition already threads the real
predicate through automatically via the registered `Ensures<V>` impl,
so there is no trivial-contract placeholder to hardcode at all. No
Creusot-style bootstrap chicken-and-egg problem either, since
`amenable_verus` is never a Cargo dependency of `amenable` in the
first place. Same `stringify!`-spacing, `rustfmt`-doesn't-see-
`include!`d-files, and captured-body-dedent fixes carried over
verbatim from Step 9; one new fix specific to this side — the
gallery's `StoplightError` mirror needed `#[derive(Debug, ..)]` too,
since capturing the real `Ok(..)`-wrapped body (not a hand-simplified
one) means `full_cycle`'s `.unwrap()` calls need it. Verified for
real: `verus --crate-type=lib` reports `419 verified, 0 errors`, and a
real non-vacuous regression check (swapping the generated `Green ->
Yellow` edge's body for `Err(..)` directly in the generated file)
reproduced the identical class of precise failure Step 7's own check
found, reverted by regenerating rather than a manual `git checkout`.
`just generate-verus-exchange` (new recipe) wired into `check-verus`/
`clippy-verus`/`test-verus`/`verify-verus`, matching `generate-
creusot`'s identical wiring on the Creusot side.

### Kani Filesystem Accommodation Model

**Document:** [KANI_FILESYSTEM_MODEL_PLAN.md](KANI_FILESYSTEM_MODEL_PLAN.md)

**Status:** ✅ Implemented — the full `std::fs` production proof queue (10
proofs: `DirBuilder`, `DirEntry`, `File`, `FileTimes`, `FileType`, `Metadata`,
`OpenOptions`, `Permissions`, `ReadDir`, `TryLockError`) has been migrated off
the direct real-tempdir path onto narrow, proof-specific Amenable-owned
observation models. Several of the later migrations (`FileTimes`,
`Metadata`, `OpenOptions`, `Permissions`, `TryLockError`) were assessed as
`strengthen`: their models are close to tautological identity checks rather
than independently derived laws, and are queued for a follow-up pass.

**Description:** Introduced a small verifier-facing filesystem model in
`amenable_kani` (`fs_model.rs`: `KaniFileSystem`, `KaniFsPath`, plus
per-proof observation types), migrated the `std::fs` queue in registry
order starting with recursive directory creation and directory entries,
preserved the direct real-filesystem timeout path in the gallery, and
validated the result with scoped checks plus native Kani runs.

### Kani UTF-8 Accommodation Model

**Document:** [KANI_UTF8_MODEL_PLAN.md](KANI_UTF8_MODEL_PLAN.md)

**Status:** ✅ Implemented — the `FromUtf8Error` proof uses the bounded
byte-recovery model, and three further proofs (`str::Utf8Error`'s
`valid_up_to`/`error_len`, `primitives::String`'s length/emptiness
consistency, `std_ffi::OsStr`'s `to_str()` round trip) were migrated in a
later pass. The `String`/`OsStr` migrations required a second model,
`KaniUtf8Buffer<MAX_LEN>`, lifted directly from `elicitation`'s
`verification::types::Utf8Bytes<MAX_LEN>`: it assumes UTF-8 validity as a
symbolic fact under Kani rather than running the validation algorithm,
since the algorithm itself (not its input representation) was confirmed to
time out even for two fixed valid bytes when the loop must run to
completion — see `gallery::utf8_validation_algorithm_cost`.

**Description:** Lifted the bounded UTF-8 modeling pattern from
`elicitation` into `amenable_kani` (`utf8_model.rs`: `KaniUtf8`,
`KaniUtf8String`, `KaniFromUtf8Error`, `KaniUtf8PositionError`,
`KaniUtf8Buffer`), migrated the `FromUtf8Error`, `Utf8Error`, `String`, and
`OsStr` proofs onto it, preserved the direct `String::from_utf8` /
full-validation-algorithm timeouts in the gallery, and validated the
resulting laws with scoped checks plus native Kani runs.

### Kani BTree Accommodation Model

**Document:** [KANI_BTREE_MODEL_PLAN.md](KANI_BTREE_MODEL_PLAN.md)

**Status:** 🔲 Planning — initial Kani-only ordered-BTree semantics are being
added so `BTreeMap` / `BTreeSet` proofs can move from std traversal blow-up to
explicit Amenable-owned ordering and removal laws.

**Description:** Introduce a small verifier-facing BTree model in
`amenable_kani`, migrate the ordered `BTreeMap` / `BTreeSet` proofs to that
model, preserve the direct symbolic std iteration timeout path in the gallery,
and validate the model with scoped checks plus native Kani runs.

### Kani Fmt Accommodation Model

**Document:** [KANI_FMT_MODEL_PLAN.md](KANI_FMT_MODEL_PLAN.md)

**Status:** 🔲 Planning — initial Kani-only formatter semantics are being
added so formatting-builder proofs can move from std rendering blow-up to
explicit Amenable-owned punctuation and ordering laws.

**Description:** Introduce a small verifier-facing formatter model in
`amenable_kani`, migrate the replace-marked `Arguments` / `Debug*` proofs to
that model, preserve the direct formatting timeout path in the gallery, and
validate the model with scoped checks plus native Kani runs.

### Kani Backtrace Accommodation Model

**Document:** [KANI_BACKTRACE_MODEL_PLAN.md](KANI_BACKTRACE_MODEL_PLAN.md)

**Status:** 🔲 Planning — initial Kani-only backtrace semantics are being
added so forced-capture proofs can move from Kani's unsupported unwinding
boundary to explicit Amenable-owned status laws.

**Description:** Introduce a small verifier-facing backtrace model in
`amenable_kani`, migrate the `Backtrace` / `BacktraceStatus` proofs to that
model, preserve the direct unwinding path in the gallery, and validate the
model with scoped checks plus native Kani runs.

### Kani Argv Accommodation Model

**Document:** [KANI_ARGV_MODEL_PLAN.md](KANI_ARGV_MODEL_PLAN.md)

**Status:** 🔲 Planning — initial Kani-only argv semantics are being added so
process-argument proofs can move from Kani's synthetic-process mismatch to
explicit Amenable-owned non-empty-argv laws.

**Description:** Introduce a small verifier-facing argv model in
`amenable_kani`, migrate the `Args` / `ArgsOs` proofs to that model, preserve
the direct synthetic-process mismatch in the gallery, and validate the model
with scoped checks plus native Kani runs.

### Kani Env Path Accommodation Model

**Document:** [KANI_ENV_PATH_MODEL_PLAN.md](KANI_ENV_PATH_MODEL_PLAN.md)

**Status:** 🔲 Planning — initial Kani-only PATH-style helper semantics are
being added so `join_paths` / `split_paths` proofs can move from std helper
timeout to explicit Amenable-owned separator and error laws.

**Description:** Introduce a small verifier-facing env-path model in
`amenable_kani`, migrate the `JoinPathsError` / `SplitPaths<'static>` proofs
to that model, preserve the direct std timeout path in the gallery, and
validate the result with scoped checks plus native Kani runs.

### Kani LinkedList ExtractIf Accommodation Model

**Document:** [KANI_LINKED_LIST_EXTRACT_MODEL_PLAN.md](KANI_LINKED_LIST_EXTRACT_MODEL_PLAN.md)

**Status:** 🔲 Planning — initial Kani-only `extract_if` semantics are being
added so `LinkedList::extract_if` proofs can move from std traversal timeout to
explicit Amenable-owned partition and early-drop laws.

**Description:** Introduce a small verifier-facing `LinkedList::extract_if`
model in `amenable_kani`, migrate the production proof to that model, keep the
direct std timeout path in the gallery, and validate the result with scoped
checks plus native Kani runs.

### Kani Pipe Accommodation Model

**Document:** [KANI_PIPE_MODEL_PLAN.md](KANI_PIPE_MODEL_PLAN.md)

**Status:** 🔲 Planning — initial Kani-only pipe semantics are being added so
anonymous-pipe proofs can move from unsupported `pipe2` boundaries to explicit
Amenable-owned byte-channel laws.

**Description:** Introduce a small verifier-facing anonymous-pipe model in
`amenable_kani`, migrate the `PipeReader` / `PipeWriter` proofs to that model,
preserve the direct `pipe2` path in the gallery, and validate the model with
scoped checks plus native Kani runs.

### Kani FD Accommodation Model

**Document:** [KANI_FD_MODEL_PLAN.md](KANI_FD_MODEL_PLAN.md)

**Status:** 🔲 Planning — initial Kani-only fd semantics are being added so
Unix handle proofs can move from unsupported libc boundaries to explicit
Amenable-owned accommodation laws.

**Description:** Introduce a small verifier-facing Unix fd model in
`amenable_kani`, migrate the `OwnedFd` proof to that model, preserve the
direct `fcntl` path in the gallery, and give the repo a minimal `justfile`
for canonical scoped validation.

### KaniCompose for Amenable

**Document:** [KANI_COMPOSE_PLAN.md](KANI_COMPOSE_PLAN.md)

**Status:** ✅ Done, including the scope correction. Initial scope (trait,
~30 built-in impls, `#[derive(KaniCompose)]`, hand-written `Ledger`/
`AccountId`/etc. impls) landed across several follow-ons tracked
elsewhere in this index. The trait's own doc always called it Kani-only,
but `symbolic_any`/`kani_assume` carried a `#[cfg(not(kani))]` panic
branch, kept alive only because four ordinary `#[test]`s reached into
`kani_depth0/1/2()` for convenience -- fixed by rewriting those tests
(commits `c4b0fad`/`362cd75`) and then gating the trait, all ~30 impls,
the derive's generated impl, and `amenable/src/lib.rs`'s re-export as
genuinely `#[cfg(kani)]`-only, no panic branch anywhere in the chain
(commit `5ecae3b`). See the doc's "Scope Correction" section.

**Description:** Introduce a Kani-specific bounded-construction trait and
derive support so Amenable can model heap-backed and recursive carriers without
defaulting to unconstrained symbolic collections.

### Kani Proof Gallery

**Document:** [PROOF_GALLERY_PLAN.md](PROOF_GALLERY_PLAN.md)

**Status:** 🔲 Planning — gallery architecture being added so verifier-pattern
experiments stop leaking into the production proof queue.

**Description:** Maintain a separate, inventory-backed proof gallery for Kani
experiments that document best practices, false trails, and open hypotheses.
Gallery cases are executable harnesses with an expected verifier outcome, but
they are not production proof claims and do not share the production proof
ledger.

### Proof Assessment Rubric and Review Ledger

**Document:** [PROOF_ASSESSMENT_PLAN.md](PROOF_ASSESSMENT_PLAN.md)

**Status:** ✅ Implemented — `amenable assess` records append-only rubric
assessments, reports score distributions, queues unassessed proofs, and is
being used to drive compiled-registry-order Kani proof review and refinement,
including individually recorded native verifier timeouts and passing
refinements.

**Description:** Let developers and agents record structured, reviewer-owned
assessments of a proof's evidentiary quality. Every assessment scores the
same orthogonal rubric, carries a recommendation and unrestricted supporting
text, and is retained as historical review data. Assessment neither changes
the inventory catalog nor substitutes for a verifier result.

### Kani Proof Runner and Result Ledger

**Document:** [KANI_PROOF_RUNNER_PLAN.md](KANI_PROOF_RUNNER_PLAN.md)

**Status:** ✅ Implemented — inventory-backed Kani selection, native timeout,
CSV result tracking, and a representative passing proof are validated.

**Description:** Register every executable Kani harness through `inventory`,
run selected harnesses through the `amenable` CLI with Kani's own per-harness
timeout, and track the latest `passed`, `failed`, or `timeout` result per
stable proof ID in a CSV ledger. Static registration describes what can run;
the ledger records what did run.

### Constitutional Trait Family and Proof-Emission Upgrade

**Document:** [AMENABLE_PLAN.md](AMENABLE_PLAN.md)

**Status:** 🔲 Planning — core trait family implemented, certification
architecture now split between abstract core traits and concrete std-backed
registrations, proof-emission machinery not yet started

**Description:** `amenable` is the foundational, dependency-light crate
defining the trait family for lawful proof-carrying software structure.
Formal verification does not depend on elicitation or any other downstream
framework; those frameworks depend on `amenable`. The core constitutional
traits (`Verifier`, `Witness`, `Evidence`, `Standard`, `Provenance`,
`Certificate`, `Registry`, `Sidecar`, `Establish`, `Exchange`,
`StateMachine`, `Amenable`) have been relocated here from an incubation
module inside `elicitation`. The current design direction is: abstract trait
interfaces in `amenable_core`, concrete std-backed provenance/certification
registrations in `amenable_std`, and explicit wrapper carriers for lawful
std-lib `Standard` registrations. Remaining work: proof-quality heuristics
on `Witness`, full certification artifact plumbing, and a from-scratch
upgrade of the proof-emission machinery (the successor to `elicitation`'s
`Prop`/`Established<P>`/`ProvableFrom<C>`/`VerifiedStateMachine`), after
which `elicitation` becomes a consumer of this crate rather than an
independent proof-carrying framework.

**Architecture principle:** `amenable` defines upstream architectural law.
Every `Evidence`-bearing claim is backed by either a genuine machine-checked
proof or an explicit provenance-backed `Standard` certification — never a
blanket impl that grants trust for free.

### State Machine Derivation

**Document:** [STATE_MACHINE_DERIVATION_PLAN.md](STATE_MACHINE_DERIVATION_PLAN.md)

**Status:** ✅ Steps 0-5, every step this plan scoped, done and verified
for real on all three backends (see the plan doc's own Status
section for the full account): `State<V>` facade landed; `#[derive(
StateMachine)]` generates compiler-enforced static assertions per
declared edge plus a real `impl StateMachine<V> for Self`
(`states()`/`transitions()`/`audit_surface()`), applied to `Stoplight`
as the canary and confirmed non-vacuous by injected-bug checks in both
directions (a fabricated edge fails to compile; an undeclared real edge
fails a runtime cross-check test). The old `Amenable` trait and its
`Stoplight` impl are deleted, not adapted — a hard `Amenable:
StateMachine` supertrait bound made partial preservation impossible
once `StateMachine` itself changed shape, so `crates/amenable/tests/
stoplight_creusot_surface_test.rs` is deleted too; real Creusot-backed
audit content is Step 4's job, not reintroduced as a stopgap here.
Step 3, corrected twice by direct discussion before landing: a chained
edge's postcondition and the next edge's precondition can rest on the
same registered atomic claim instead of independently hand-typed
copies — demonstrated on `amenable_gaap::Ledger` (`Stoplight` has no
real `Requires<V>` content to show this with), giving `Validated` a
real `Requires<KaniVerifier>` impl that delegates through the same
`AmountPositive` claim `validate`'s own postcondition already uses,
wired via a new `capture_exchange_body(kani_requires_evidence = ..)`
arg onto `commit` in place of its old hand-typed precondition. Verified
for real: `cargo kani` on `commit`'s own contract harness still passes
(`0 of 297 failed`), plus two new tests confirmed non-vacuous by
temporarily breaking the delegation and watching the exact assertion
fail. Also surfaced a real finding, resolved directly rather than
special-cased: `Ledger` had no `Exchange` trait impl at all. Step 5
fixed it at the source instead of accommodating its absence —
`capture_exchange_body` now unconditionally generates a real,
verifier-generic `impl<V: Verifier> Exchange<Input, Output, V> for
Self` (copying the real method's own `where` clause, delegating via
`self.method::<V>(input)`), applied automatically to all four of
`Ledger`'s methods. `#[derive(StateMachine)]` gained a
`generic_over_verifier` mode to match, since `Ledger` lives in a
backend-neutral crate that can never name a concrete verifier type; its
first design (an unconstrained "works for every `V`" static assertion)
was correctly rejected by the compiler — `Ledger`'s real impl is only
generic over `V` conditionally, bounded by real `Witness`/`Ensures`/
`Requires` impls per edge — so the final design generates no static
assertion in that mode at all, relying on `capture_exchange_body`'s own
impl as the real compile-time check and the runtime `ExchangeEdgeRecord`
cross-check for declaration correctness. Also fixed along the way: a
real `clippy::duplicated_attributes` false positive on the `state(name
= .., carrier = ..)`/`edge(from = .., to = ..)` key-value syntax
(triggers on any state with more than one outgoing edge), moved to
positional `state(name, carrier)`/`edge(from, to)` instead. Verified
for real: `cargo kani` re-run on `Ledger`'s `validate`/`commit`
contract harnesses after the `Exchange` impl addition, both still
passing (`0 of 501`/`297 failed`).

Step 4's Creusot half landed too, by direct instruction: "I really
don't want creusot to employ any mirrors... macros are how we keep the
generated code faithful." `amenable::creusot_export` now unconditionally
emits a real, concrete `impl Exchange<Input, Output, CreusotVerifier>`
per edge alongside its harness-captured inherent method — the identical
gap Step 5 closed for `capture_exchange_body`, just in a different
generator; the mirror *types* still can't go away (real transition
bodies construct tokens through constructors deliberately private to
`amenable_kani`), but the generated impl is now real, not a lesser
same-named stand-in. Two real toolchain findings: the generated
`exchange()` method needed its own `#[ensures(..)]` (Creusot has no
mechanical call-through the way Kani's/Verus's `Ensures<V>` dispatch
does; a first version without one let a fabricated `Err(..)` through
silently, caught properly once tested against `Ledger::commit`'s real
predicate rather than `Stoplight`'s vacuous-by-default one), needing
three real mirror fields widened to `pub` for Creusot's proof-
transparency check; and applying the derive to Creusot's own mirror
`Stoplight` hit a genuine `creusot-rustc` ICE (a compiler panic, not a
lint) from the static assertion's closure-nested-generic-function
shape, isolated by binary search and fixed by restructuring to a
top-level shared checker function plus plain `const` references, no
closure or nesting. A real, direct correction along the way, caught
immediately: the first ICE fix assumed `audit_surface()`'s `inventory`
call was the cause and baked a `cfg(creusot)` split unconditionally
into the shared derive output, leaking `cfg(creusot)` into `amenable_
kani`/`amenable_gaap`'s own `Cargo.toml` `check-cfg` lists to silence
the resulting `unexpected_cfgs` warning — "what the heck is creusot
doing in the kani crate, we have been over this three times," the same
class of violation as the Cargo-dependency version of this rule already
caught and reverted twice before, one level down. `~/repos/elicitation`'s
`UNEXPECTED_CFGS.md` was read directly at this point: real, relevant
prior art solving a broader version of the same problem (their macros
unconditionally emit cfg tokens into every caller), but this derive
didn't need that blunt a fix — landed as `translator_cfg = "creusot"`,
an opt-in per-block argument, so no cfg name reaches a crate that
doesn't already, legitimately, know about it. Verified for real: `cargo
creusot -- -p amenable_creusot` translates clean, `cargo creusot prove`
reports `Proved (153 files) ✔` (up from 149, up from 142 before this
step), full workspace clean, a real Kani spot-check still passing.
Verus's own half of Step 4 landed too, much more smoothly than
Creusot's — no ICE, no incorrect first attempt. `verus_state_machine!`
(`amenable_verus::exchange_support`) is the `macro_rules!` counterpart
the derive can never be there (`verus --crate-type=lib` resolves no
external crate under any circumstances), matching the existing `verus_
sidecar!`/`verus_ensures!`/`verus_exchange!` family. One real
structural difference: `macro_rules!` can't look up a declared state's
carrier by name, so `edges` names each carrier explicitly per edge
rather than through a `states` table; `states` itself stays a plain
name list. Carried the Creusot-side static-assertion fix (a top-level
checker function plus plain `const` references, no closure or nesting)
over directly as a precaution, though the closure-based shape was never
actually tried under Verus. `amenable_core::state_machine` joined the
existing `#[path]`-included trait-family file list in `amenable_verus::
lib.rs`, the same real-source-inclusion mechanism already used for
every other `amenable_core` trait there. Verified for real: `verus
--crate-type=lib` reports `491 verified, 0 errors` (unchanged -- the
new code carries no `ensures`/`requires` content, so it adds no new
verification obligations, just real compile-time-checked structure),
confirmed non-vacuous by an injected-bug check producing a precise
failure at the checker function's own bound, reverted. Step 4 is
complete, both backends -- every step this plan scoped is now done.

A real follow-on landed after that, outside this plan's original
scope: a "homecoming" downstream consumer, dogfooding the design,
reported that `StateMachine`'s surface can't distinguish a state that
is *only* reachable via a declared edge from one that is also
root-enterable with no prior credential at all -- `Stoplight::Green` is
both (root via `Established::<Green, GreenToken>::root()`, and the
real target of the `Red -> Green` cycle-back edge), and nothing in the
derived surface said so; a graph built from edges alone could infer a
false dependency and miss the real entry point. `inventory` genuinely
keeps both of `GreenToken`'s registrations already (a bare
`#[derive(ProofToken)]` one and an `#[establish(..)]` one) -- not a
data-loss bug, contrary to the report's first hypothesis -- but neither
one names a callable root. Fixed with the narrowest of three proposed
options: a new `amenable_core::RootEntry { state, constructor }`, a
new `StateMachine<V>::root_entries()` method (default-implemented as
`&[]`, non-breaking for every existing implementor), and an optional
third positional `root` argument on `state(name, carrier, root)`,
checked at compile time by the same flat `const _: fn() -> Carrier = #root;`
shape Step 4 already established, confirmed non-vacuous by
pointing it at a nonexistent function and observing a precise `E0599`,
then reverted. `StateDecl` keeps the root's parsed `syn::Path` (for the
assertion) and its original literal text (for the emitted
`constructor` string) as a pair, rather than re-stringifying the
parsed path via `quote!`, which normalizes token spacing into an uglier
string than the one actually written. Applied to exactly one site --
`Green` in `crates/amenable_kani/src/stoplight.rs` -- with `Ledger`'s
`Pending` state deliberately left out (its real constructor takes a
real argument, not a fit for a zero-argument mechanism). Verified for
real: full workspace check/clippy/fmt/test clean, a real `cargo kani`
re-run on the harness that itself calls `Established::<Green,
GreenToken>::root()` still verifies clean, a new test asserts the
exact declared `root_entries()` entry. See `docs/
STATE_MACHINE_DERIVATION_PLAN.md`'s own "Follow-on: root entries"
section for the full account.

**A second follow-on landed after that**: `KaniCompose` routing for
`Ledger`'s data-bearing carriers, the gap Step 5's own account flagged
("not yet exercised by any auto-generated harness"). Auto-generating
the harness itself turned out not to fit the derive at all -- `Ledger`
is `#[state_machine(generic_over_verifier, ..)]` precisely so
`amenable_gaap` stays neutral, but a `#[kani::proof_for_contract(..)]`
harness has to name `KaniVerifier` concretely, which the derive
expanding inside `amenable_gaap` structurally cannot do without the
same neutral-crate violation this project has already caught twice.
Scoped down instead: hand-written `KaniCompose` impls for `AccountId`/
`Amount`/`TransferPayload`/`Transfer<Pending, PendingToken>`/
`Transfer<Validated, ValidatedToken>` in `amenable_kani::ledger` (can't
be `#[derive(KaniCompose)]` on the struct definitions themselves, same
neutral-crate reason), plus `uuid::Uuid` coverage in `amenable_kani::
compose`; the existing hand-authored `gaap_ledger` harnesses rewired to
call them instead of `kani::any()` plus two literal accounts that never
varied, each harness still holding exactly the property it's named for
(the "accepts a lawful transfer"/"rejects the same account" pair keep
their distinct-vs-same-account assumptions; `reject`/`rollback`, with
no precondition at all, collapsed to one `kani_any()` call each). A
real second CBMC-cost finding, root-caused the same disciplined way as
this worked example's Step 1: `AccountId`'s `name: String` field's
construction itself (not comparison -- `PartialEq` never touches
`name`) was expensive; fixed by pinning `name` to a constant empty
string at every `KaniCompose` depth in `AccountId`'s own impl, since
`name` has zero causal effect on any claim checked here, not a
weakened proof. A second, much larger, wholly unrelated blocker
surfaced getting a real `cargo kani` run at all: every `cargo kani -p
amenable_kani --all-features` build was already broken crate-wide
(~28 ambiguous bare `Type::requires(..)`/`::ensures(..)` call sites
across ~10 files, present since 2026-08-11, invisible to the routine
check-all-package sweep since it never runs `cargo kani`) -- see
`CONTRACT_BOUND_NAMING_WORKFLOW.md`'s own new Gotchas entry; fixed
first, as a precondition, by qualifying each with `<Type as
Requires<crate::KaniVerifier>>::requires(..)` (or `Ensures`). Verified
for real: the crate-wide fix confirmed via a clean re-run of the
untouched `stoplight::verify_green_transitions_only_to_yellow` harness
(`0 of 38 failed`), then all five rewired `gaap_ledger` harnesses
individually under real `cargo kani`, all clean; full workspace
check/clippy -D warnings/fmt/test clean throughout. See `docs/
STATE_MACHINE_DERIVATION_PLAN.md`'s own "Follow-on: `KaniCompose`
routing" section for the full account.

**A third follow-on landed after that, prompted directly by homecoming's
own follow-up feedback** on the root-entries fix: it confirmed
`root_entries()` correctly handles zero-argument roots, and named the
honest remaining gap directly — `Ledger` still declares no root for
`Pending`, a "root with payload" case root entries explicitly didn't
cover. Raised by direct suggestion: an associated type, `Seed`, `()`
for stateless roots but a real type otherwise. A literal Rust
associated type can't live on `RootEntry` itself (`root_entries()`
returns a homogeneous `&'static [RootEntry]`, the same reason
`constructor` is already a stringified path, not a real callable), so
`Seed` shows up in two places instead: a real type at the compile-time
check (`state(name, carrier, root, seed)` gained an optional fourth
positional argument; the generated assertion widens from `fn() ->
Carrier` to `fn(Seed) -> Carrier` when declared, confirmed non-vacuous
by pointing `Ledger`'s new `Pending` seed at the wrong real type
(`TransferError` instead of `TransferPayload`) and observing a precise
`E0308`, then reverting), and a stringified `seed: &'static str` field
on `RootEntry` for the audit surface (`"()"` for every existing
zero-argument declaration, unchanged and non-breaking). Applied to
exactly the site homecoming named: `Ledger`'s `Pending`, now declaring
`Transfer::pending`/`TransferPayload`. Verified for real: full
workspace check/clippy -D warnings/fmt/test clean; two real `cargo
kani` re-runs (the untouched `stoplight` harness, confirming the
crate-wide ambiguity fix above still holds, and a `gaap_ledger` harness
that itself calls `Transfer::pending`) both still verify clean; a new
test asserts `Ledger`'s exact declared root entry. See `docs/
STATE_MACHINE_DERIVATION_PLAN.md`'s own "Follow-on: `Seed`" section
for the full account.

**Description:** Replaces (not extends) `amenable_core::state_machine`'s
current `StateMachine`/`Amenable` trait pair, which `Stoplight`'s own
hand-written impl already self-documents as a disconnected proxy
(`Color`/`SequentialCycle` back nothing real, and `creusot_surface()`'s
hand-typed string filter repeats the exact drift risk the surrounding
code claims to avoid). New design: a thin, object-safe `State<V>`
facade (`Evidence + Witness<V>`, via blanket impl — every existing
state type qualifies for free) and a `#[derive(StateMachine)]` that
takes explicit `state(name = .., carrier = ..)`/`edge(from = .., to =
..)` declarations (concrete carrier types are caller-supplied, never
assumed, since a macro has no type information to infer them) and
emits compiler-enforced static assertions against real `Exchange`
impls. Real prior art from `~/repos/elicitation`'s `VerifiedStateMachine`
/`formal_method`/`KaniVariantState` (read directly, not summarized):
keeps the per-variant bounded-depth splitting technique (already ported
as `amenable_derive::KaniCompose`), rejects vacuous Verus contracts and
`Established::assert()`-style non-lawful proof construction, and
rejects the two-stage `TokenStream`/`build.rs` codegen architecture in
favor of `amenable`'s existing direct compile-time macro expansion.
`Stoplight` is the design canary; `amenable_gaap::Ledger` is the
planned second worked example, chosen to stress data-bearing carriers
and multi-edge contract granularity `Stoplight`'s zero-field markers
can't exercise.

### GAAP Ledger

**Document:** [GAAP_LEDGER_PLAN.md](GAAP_LEDGER_PLAN.md)

**Status:** 🔲 Planning — Steps 0 and 1 done: the crate skeleton
compiles and is tested, and the first real Kani edge (`Pending ->
Validated`) is proven via genuine `#[kani::ensures]` DFCC
contract-checking (`0 of 492 failed`). A real CBMC timeout on the
first version — comparing two `String`s for equality inside a
`#[kani::ensures]` closure is expensive regardless of content or
length — was root-caused via a 21-experiment gallery investigation and
fixed by giving `AccountId` a `Uuid` identity (`PartialEq` compares
`id` only, never the human-readable `name`), not by weakening the
postcondition. A second real bug (an under-specified helper contract,
exposed only once the timeout stopped masking it) was also found and
fixed. Step 2's `Validated -> Committed` edge (`BalancedEntries`) is
also done (`0 of 297 failed`), with its own, unrelated CBMC timeout
(symbolic `i64::MIN` negation overflow) root-caused and fixed via a
genuine `#[kani::requires]` precondition. Two new derive macros,
`#[derive(ProofToken)]` and `#[amenable_derive::establish(..)]`, were
built and retrofitted onto every hand-written `ProofToken`/`Establish`
impl in both `stoplight.rs` and `ledger.rs` — closing a real gap where
this dogfooding lineage had never actually derived those two trivial-
but-universal shapes. Step 2 is now fully done: `reject()`/`rollback()`
to `Rejected` are proven (`0 of 287 failed` each) by making `Rejected<T>`
generic over the state it was rejected from (`Rejected<Pending>`/
`Rejected<Validated>`, resolving the same-evidence `#[amenable_derive::
exchange]` collision by making the two edges target genuinely distinct
concrete types, a deliberate divergence from elicitation's flat
`Rejected` + runtime-reason design). Step 3 (Creusot) has all four
contracts (`AmountPositive`/`SufficientFunds`/`AccountsDistinct`/
`BalancedEntries`) proven as real, non-trivial Pearlite predicates
(`Proved (118 files) ✔`) directly on the real `Validated`/`Committed`
types — a real correction to this plan's original design: `amenable_
creusot` can take an ordinary Cargo dependency on `amenable_gaap`
(confirmed empirically, no ICE), so no accommodation-model mirror was
needed at all, unlike `Stoplight`'s own shape. Getting there also
forced a real, workspace-wide dependency-tree restructuring (see
"Exchange Method Proof Derivation"'s own Step 10 above): `amenable_std`'s
optional edge to `amenable_creusot` flipped for real, and `Stoplight`'s
own `Green`/`Yellow`/`Red` moved from `amenable_kani` to `amenable_core`
after a real, caught-and-reverted attempt to depend `amenable_creusot`
directly on `amenable_kani`. `Ledger::validate`/`::commit` themselves
were connected to those four Creusot predicates in Step 6 (below) — not
via a direct `amenable_creusot -> amenable_kani` dependency, which is a
settled no (same architectural rule, not an open question), but via the
existing `ExchangeEdgeRecord`/codegen layer, generalized. Step 4 (Verus) is
done, redirected mid-flight by explicit correction ("by hand is the
wrong approach, use the derives") before any hand-written proof content
existed: `amenable_verus::gallery::ledger_exchange` proves the same
`Validated`/`Committed` biconditionals a third way, with `Ledger::
validate`'s/`::commit`'s real bodies captured verbatim by the same
codegen mechanism Creusot's own `Stoplight` companions use, generalized
to a second `self_ty` group for the first time (`amenable::
verus_exchange_export` now routes by an explicit `(self_ty,
method_name)` table, not a single hardcoded directory). Surfaced four
real, novel findings no `Stoplight` edge ever exercised: a `self`
mixed-site macro-hygiene gap in `verus_exchange!` (fixed the same way
`$input_param` was); two real Verus-translator gaps in the captured
body itself (a bare tuple-variant constructor used as a function value,
and a destructuring closure parameter) fixed at the source in
`amenable_kani::ledger` with two small real helper methods, re-verified
clean against real `cargo kani` afterward; `Result::map_err`'s own
`vstd` spec requiring every chained accessor/constructor to carry its
own real `ensures` before anything downstream could be proven; and
`Committed`'s claim needing `i128` widening (not `int`, not `wrapping_
*`) to avoid a real overflow obligation with no precondition at all.
Verified for real: `verus --crate-type=lib` -- `458 verified, 0
errors`, plus a real injected-bug regression check. `reject`/`rollback`
deliberately still not connected on either backend (legitimately
trivial, no new proof content). Step 5 is done, all three backends: a
standing correction repeated multiple times ("mirrors if used must be
derived," "contract based bounds... should be derived from contract
types") closed two gaps everywhere they appeared, not just on Kani.
`#[derive(amenable_derive::Sidecar)]` (Kani/Creusot) plus a `verus_
sidecar!` macro_rules! counterpart (Verus, which resolves no proc-macro
crate at all) now generate the `Established<T, Token>`/`Transfer<S,
Token>` carrier shape everywhere it appears -- Kani's own real types
included, not just mirrors -- replacing four hand-written copies.
`AmountPositive`/`SufficientFunds`/`AccountsDistinct`/`BalancedEntries`
(real `Evidence` types since Step 0, dead code until now) got real
per-backend `Ensures<V>` impls: Kani's via the already-generic `kani_
ensures!` macro (`Bound = bool`, a real checked predicate);
Creusot's with `Bound = &'static str` (Pearlite has no exec
representation at all, matching `amenable_core::contract`'s own
documented fallback -- audit-only, since the real checking already ran
through the existing `_holds` Pearlite functions and continues to);
Verus's as real local mirror types (this backend resolves no real
`amenable_gaap` either) with their own `bool`-checked `verus_ensures!`
calls. Every atomic check's own contract and `Validated`'s/`Committed`'s
combined claims, on every backend, now call through these instead of
restating the arithmetic. A generic composition derive for the *outer*
match/control-flow shape was considered and rejected, on all three --
genuine short-circuiting domain logic (`TransferError`'s three
variants), not a mechanically-derivable pattern. A third real "no
general patterns" translator gap surfaced finishing the Verus half:
`SufficientFunds`'s/`AccountsDistinct`'s own `verus_ensures!` calls
first used destructuring parameters, hit the identical error `Step 4`'s
own `Ledger::insufficient_funds` already found for a closure parameter,
fixed the same way (index a plain tuple parameter instead). Re-verified
for real throughout: 8 total real `cargo kani` harness checks (all `0
of N failed`), `cargo creusot prove` (`Proved 119 files`, unchanged --
the new Creusot impls are `#[cfg(not(creusot))]`-only, invisible to
real translation), `verus --crate-type=lib` (`478 verified, 0 errors`,
up from 458), plus a second real injected-bug regression check. Step 6
connects `Ledger::validate`/`::commit`'s real bodies to genuine Creusot
predicates too, closing the last unconnected corner across all three
backends: generalized the existing `emit-creusot-companions` codegen
(the `self_ty == "Stoplight"` filter widened to an explicit `(self_ty,
method_name) -> module` table, matching Step 4's own Verus precedent)
rather than hand-mirroring a third time, and added a new
`creusot_ensures: &'static str` field to `ExchangeEdgeRecord` — a real
Pearlite predicate expression spliced into the generated companion's
own `#[ensures(..)]`, needed because Creusot (unlike Kani/Verus's DFCC-
style call-through) has no mechanical way to route a postcondition
through `Ensures<V>`. Surfaced four real, previously unexercised
Creusot toolchain findings: `#[ensures(..)]` cannot call ordinary
methods, only `#[logic]` functions or direct field access; `Result::
map_err` has zero contract anywhere in `creusot-std` itself (confirmed
by reading the real source), fixed with a real `extern_spec!` block;
`cargo creusot` sets `--cfg creusot` across its *whole* dependency
graph, not just the crate it translates (broke `amenable_kani`
transitively until the `#[derive(Sidecar)]`-generated ensures clauses
were gated on `verifier`'s own stringified text instead of `cfg_attr`);
and two rounds of Creusot's proof-transparency visibility check forcing
private fields to `pub`. `cargo creusot prove` reports `Proved (143
files) ✔` (up from 119), confirmed non-vacuous via a real injected-bug
regression check; full workspace check/fmt/clippy/test clean; all
three real `cargo kani` `Ledger::validate`/`::commit` harnesses
re-verified clean; `verus --crate-type=lib` reconfirmed clean (`478
verified, 0 errors`, unchanged). Step 7 moves `Ledger`/`Transfer`/
`TransferError` and all six of `Ledger`'s own methods to `amenable_gaap`
for real, on all three backends, closing the same asymmetry
`Stoplight`'s own edges never had: prompted by direct pushback on an
unexamined assumption ("you make a shared model in gaap... how is that
pattern not apply here?"). Blocked initially by a real Kani 0.67.0 DFCC
bug — any contracted function whose body *delegates* to a separate
function (any shape, any contract content) fails CBMC's `free.frees.1`
at a builtin location; root-caused via four registered `gallery::
ledger_gaap_free_function_contract` cases, fix confirmed: attach the
contract directly to the real, generic-over-`V` method, zero
delegation. `#[derive(Sidecar)]` gained a verifier-less generic form to
support it (matching `#[amenable_derive::establish]`'s existing one).
`amenable_kani`'s own duplicate `Ledger`/`Transfer`/`TransferError`/
reject-tokens retired entirely; all call sites (`tests/ledger_test.rs`,
both gallery investigation files) repointed at `amenable_gaap`, with
`::<KaniVerifier>` turbofish added to every real generic method call.
Re-verified clean: all six real harnesses pass; full workspace
check/clippy -D warnings/fmt/test clean.

Creusot's/Verus's own generated companions (`generated/validate.rs`/
`commit.rs`, `ledger_exchange`'s own) went accurate-but-orphaned when
`#[exchange(..)]` came off the moved methods (no concrete verifier left
for that macro's own bundle to name) — fixed with a new, narrower
macro, `#[amenable_derive::capture_exchange_body(evidence = ..,
creusot_ensures = .., method_generics = ..)]`: re-registers a real
`ExchangeEdgeRecord` from the method's own body verbatim, generating
nothing else at all (no contract, no `Witness<V>`/`Exchange<..>` impl —
Kani's own contract already lives hand-written directly on the method).
`validate`/`commit` each moved into their own single-method `impl
Ledger { .. }` block in `amenable_gaap::ledger.rs`, matching `#[exchange
(..)]`'s own long-standing "one method, captured at its own call site"
shape. One real new wrinkle: `validate`'s captured body calls `Self::
check_amount_positive::<V>(amount)` with an explicit turbofish, needing
*something* named `V` in the generated companion's own scope —
`ExchangeEdgeRecord` gained a `method_generics: &'static str` field for
this. Creusot's fix: splice `<V>` onto the generated function and give
the mirror's own `check_amount_positive`/`check_sufficient_funds` a
matching unconstrained `<V>` (no bound — Creusot's `Ensures<
CreusotVerifier>` has a structurally different shape, `Input = ()`, so
mirroring the real bound would be a type mismatch). Verus needed a
different fix: `Exchange::exchange` is fixed-arity, and Rust rejects an
impl adding a generic parameter to a trait method (confirmed: E0049) —
so `gallery::ledger_exchange` instead declares a plain module-level
`type V = GalleryVerifier;` alias. Both mirrors' `<V>` also needed a
trivial `let _phantom: PhantomData<V> = PhantomData;` body line to
satisfy a real, un-`#[allow]`-able clippy lint
(`clippy::extra_unused_type_parameters`) — only Verus's mirror actually
hit it (Creusot's is `#[cfg(creusot)]`-gated, never reached by ordinary
clippy at all). Re-verified clean on both: `cargo creusot prove`
reports `Proved (138 files) ✔`; `verus --crate-type=lib` reports `478
verified, 0 errors` (unchanged); both reconfirmed non-vacuous via a
real injected-bug regression check (a loosened `AmountPositive` claim
produced precise, real failures on each backend before being reverted).
Kani spot-checked unaffected by the `amenable_gaap::ledger.rs`
restructuring. Full workspace check/clippy -D warnings/fmt/test clean.

**Revisited by direct request ("time to revisit"): `reject`/`rollback`
connected on Creusot/Verus too**, closing the one deliberate scope
boundary Step 4/Step 6 had left standing. Cheap on the codegen side
(both moved into their own `#[capture_exchange_body(..)]`-decorated
`impl Ledger` block, no `method_generics` needed), but neither backend
had ever mirrored `Rejected<T>` or its two tokens (only `Pending`/
`Validated`/`Committed` were). Creusot: real type, real dependency, no
mirror needed — just a trivial `Witness<CreusotVerifier>` impl per
instantiation plus two new `extern_spec!` entries for the external
`establish` calls. Verus: a real local mirror (`pub struct Rejected<T>
{ _marker: PhantomData<T> }`, blanket `impl<T> Evidence`), and a real
discovery — `verus_exchange!`'s template routes every edge through
`Ensures<V>` with no `"true"` fallback the way Creusot's `creusot_
ensures` has, so both edges needed a real `verus_ensures!(.., |result|
result.is_ok())` claim, not an omission. Tokens: `RejectedFromPendingToken`/
`RejectedFromValidatedToken` already had real registrations, just
needed adding to `verus_gaap_tokens_export`'s allowlist — surfaced a
real latent bug there (never applied the `stringify!`-spacing tidy the
other two generators already carry, silently fine until a generic
`proposition` arrived). Also closed a real justfile gap found along the
way: `emit-verus-gaap-tokens` never had a `just` recipe at all. Re-verified
clean, both with a real injected-bug check: `cargo creusot prove` →
`Proved (142 files) ✔`; `verus --crate-type=lib` → `491 verified, 0
errors`. Kani spot-checked unaffected. Full workspace check/clippy -D
warnings/fmt/test clean. Nothing scoped for this plan is open now.

**Description:** The next worked example after `Stoplight`, chosen to
exercise the one thing the whole Exchange proof derivation lineage has
never tested on any backend: a genuinely non-trivial, branching
predicate (every claim proven so far is `result.is_ok()`). Domain:
double-entry bookkeeping — familiar enough that nobody argues over the
domain semantics, with one real checkable law (debits must equal
credits) that is genuinely branching, not incidental. Direct prior art
in `~/repos/elicitation/crates/elicit_server::ledger` (a typestate
`Transfer<Pending -> Validated -> Committed>` with real contracts —
`AmountPositive`, `SufficientFunds`, `AccountsDistinct`,
`BalancedEntries`) and `elicit_server::gaap::mathematical`
(`AccountingEquationHolds` and other real ASC-cited identities, versus
~130 other GAAP props that are citation-only). Confirmed two real
strengthenings `amenable` has over elicitation's design:
`Establish<C, V>` structurally requires `C: ProofToken` (private
fields, only mintable via a prior lawful `establish()` call) where
elicitation's `ProvableFrom<C>: Prop {}` relies only on `pub(crate)`
convention; and `amenable_core::cert::{Registry, Certificate}` is a
real, queryable, iterable audit log where elicitation's ASC citations
are doc-comment strings only. New crate `amenable_gaap` mirrors
`amenable_std`'s own real, asymmetric crate-hierarchy shape exactly
(Creusot content lives inside it behind a `creusot` feature with a
real optional Cargo dependency; Verus gets export scaffolding only,
file-path-linked to real proof source in a new
`amenable_verus::gallery::ledger`; a new `amenable_kani::ledger`
module depends on `amenable_gaap` rather than the reverse, since
orphan rules put `Witness<KaniVerifier>` impls wherever `KaniVerifier`
itself lives). Initial scope is five real contracts (not elicitation's
full ~150-prop catalog) proven by hand, one backend at a time, before
any codegen — matching `Stoplight`'s own history exactly.
