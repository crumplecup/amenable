# Planning Documents

This file tracks all planning documents for the amenable project.

## Current Active Plans

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
