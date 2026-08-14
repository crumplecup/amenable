# Planning Documents

This file tracks all planning documents for the amenable project.

## Current Active Plans

### Verus Derive-Witness Composition

**Document:** [VERUS_DERIVE_WITNESS_COMPOSITION_PLAN.md](VERUS_DERIVE_WITNESS_COMPOSITION_PLAN.md)

**Status:** 🔲 In progress — Phases 1–7 implemented and verified.
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
genuine proof. Phase 8 (full rollout) not started.

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
