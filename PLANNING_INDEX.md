# Planning Documents

This file tracks all planning documents for the amenable project.

## Current Active Plans

### Constitutional Trait Family and Proof-Emission Upgrade

**Document:** [AMENABLE_PLAN.md](AMENABLE_PLAN.md)

**Status:** 🔲 Planning — core trait family implemented, proof-emission
machinery not yet started

**Description:** `amenable` is the foundational, dependency-light crate
defining the trait family for lawful proof-carrying software structure.
Formal verification does not depend on elicitation or any other downstream
framework; those frameworks depend on `amenable`. The core constitutional
traits (`Verifier`, `Witness`, `Evidence`, `Standard`, `Objective`,
`Sidecar`, `Establish`, `Exchange`, `StateMachine`, `Amenable`) have been
relocated here from an incubation module inside `elicitation`. Remaining
work: proof-quality heuristics on `Witness` to catch corner-cut proofs,
certification-of-provenance discipline for `Standard`/`Objective`, and a
from-scratch upgrade of the proof-emission machinery (the successor to
`elicitation`'s `Prop`/`Established<P>`/`ProvableFrom<C>`/
`VerifiedStateMachine`), after which `elicitation` becomes a consumer of this
crate rather than an independent proof-carrying framework.

**Architecture principle:** `amenable` defines upstream architectural law.
Every `Evidence`-bearing claim is backed by either a genuine machine-checked
proof or an explicit `Standard`/`Objective` certification of provenance —
never a blanket impl that grants trust for free.
