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

### Naming Raw Requires/Ensures Bounds (elicit_doc-driven)

**Document:** [CONTRACT_BOUND_NAMING_WORKFLOW.md](CONTRACT_BOUND_NAMING_WORKFLOW.md)

**Status:** 🔲 Ongoing — matching mechanism redesigned in an earlier
session (call-shape recognition replaced text matching, closing a real
correctness gap it had); `amenable_creusot` fully cleared under the new
mechanism; `amenable_kani` and `amenable_verus` both actively in
progress, each with many named clusters landed. Per this project's own
convention (see the linked doc's own "Status" note), backlog counts are
intentionally not tracked here or in the linked doc — they drift too
fast to stay trustworthy. Re-run the `elicit_doc quality antipatterns`
scan before picking up work; treat its live checklist as the only
source of truth for what remains.

**Description:** Every `requires`/`ensures` bound should be a named
`amenable_core::{Ensures, Requires}` contract type with one real,
callable predicate, not a raw expression restated per site.
`elicit_doc`'s `ANTIPATTERN-UNNAMED-CONTRACT-BOUND-001` rule scans all
three verifier backends for raw bounds and groups them into duplicate
clusters by clause shape, ranked by size, so the highest-leverage
(most-repeated) bound gets named first. A site is recognized as
compliant only when its clause is a real call to a registered contract's
predicate, never by matching clause text against the registered
fragment's text — the linked document covers why that mattered (a
coincidental Verus text match was hiding real unnamed debt) alongside
the contract-type design pattern, the elicit_doc tooling internals, a
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

### Exchange Method Proof Derivation (Kani + Creusot First)

**Document:** [EXCHANGE_PROOF_DERIVATION_PLAN.md](EXCHANGE_PROOF_DERIVATION_PLAN.md)

**Status:** 🔲 Steps 0 through 3 all done and verified — Kani and
Creusot, all three `stoplight.rs` edges, each carrying real,
tool-confirmed contracts (`cargo kani`, `cargo creusot prove` —
`Proved (110 files) ✔`) with a genuine injected-regression check per
edge per backend, plus a real consistency test keeping the Creusot
mirror honest against the real Kani source (Step 2, verified in both
drift directions), plus the by-hand Kani-side wiring generalized into
a real `#[amenable_derive::exchange(..)]` attribute macro (Step 3),
re-verified against all three edges post-swap. The Creusot mirror
genuinely implements the real `amenable_core` trait family, corrected
from an over-flattened first version while starting Step 2. Kani and
Creusot first, by deliberate sequencing — Verus
support for
`Exchange` remains a real goal, just not yet solvable the same way
(Verus can't check an arbitrary compiled Rust body, only `verus! {}`-
native code), so it gets its own plan once it has a real answer
instead of a weak one bolted on
here.

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

**Status:** 🔲 Planning — initial verifier-facing bounded-construction layer is
being added to the Kani backend and derive surface.

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
