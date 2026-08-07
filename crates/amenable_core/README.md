# amenable_core

> Constitutional trait family for lawful proof-carrying software structure.

## What this crate is

`amenable_core` defines the roles and admissibility criteria governing a
proof economy: which types are permitted to serve as trusted roots, which
types may count as derived evidence, which exchanges are lawful, and
which workflows are closed under those exchanges. It is dependency-light
by design and does not depend on any downstream proof-carrying framework.

Every claim in the system is backed by exactly one of two lawful things —
never a silent third option:

- a genuine machine-checked proof, emitted through `Witness` and consumed
  by a `Verifier` backend (Kani, Creusot, Verus)
- an explicit `Standard` or `Objective` certification of provenance — a
  structured, auditable record naming the authority, source, and
  rationale for a trust decision that cannot be mathematically derived

## The role family

- `Verifier` — a formal-verification backend marker (Kani, Creusot, Verus
  each define their own, in their own crate).
- `Witness` — the bridge from a concrete type to a `Verifier`'s proof
  artifact.
- `Evidence` — a chain-derived provenance basis a claim can rest on.
- `Standard` / `AsStandard` — an explicit, non-derived certification of
  provenance.
- `Sidecar`, `Establish`, `Exchange` — the lawful-exchange machinery: what
  it means for one proof-carrying value to hand off to another without
  losing what's been established about it.
- `StateMachine` / `Amenable` — states are roots, transitions are
  relations; this is the trait pair that makes that concrete.
- `Provenance` — structured, chain-derived metadata every claim carries
  regardless of which of the two lawful paths produced it.
- `Certificate` / `Registry` / `EvidenceLink` / `ProofRecord` — the
  audit-facing types: how a claim's chain gets recorded and looked back
  up.

## Why this split

Traits that must be implemented directly on foreign standard-library
types (which Rust's orphan rules require to live in the crate that
defines the trait) live in dedicated sibling crates instead —
`amenable_std` for `RustStdType`. Users should generally depend on the
top-level `amenable` facade crate, which re-exports this crate's family
alongside its siblings, rather than depending on `amenable_core`
directly.

## See also

- [Root README](../../README.md) for the project-wide overview and
  current coverage status.
- [`amenable.md`](../../amenable.md) for the trait-by-trait design.
- [`AMENABLE_PLAN.md`](../../docs/AMENABLE_PLAN.md) for the original
  architectural rationale.
