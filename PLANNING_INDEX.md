# Planning Documents

This file tracks all planning documents for the amenable project.

## Current Active Plans

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
