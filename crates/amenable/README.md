# amenable

> Top-level facade and CLI for the `amenable` constitutional trait family.

## What this crate is

`amenable` re-exports the core constitutional roles from `amenable_core`
alongside sibling crates such as `amenable_std`, so most users depend on
this one crate rather than assembling the family themselves. This is the
single sanctioned exception to the workspace's "no re-exports between
crates" rule — see [`CLAUDE.md`](../../CLAUDE.md)'s Workspace
Organization section for why that rule exists and why this crate is the
one place it's waived.

Crates that are themselves part of the family (`amenable_kani`,
`amenable_creusot`, `amenable_std` itself) depend on `amenable_core`
directly, never on this facade, to avoid a circular dependency.
`amenable_verus` is the one exception to that: Verus never resolves
`Cargo.toml`, so it depends on nothing from this workspace at all (not
even `amenable_core`) — see `amenable_std::verus_witness`'s doc comment
for the full split rationale. This facade re-exports Verus's witness
types from `amenable_std` (where they now live) instead.

## The CLI

This crate also builds the `amenable` binary, which inspects and
exercises the proof registry directly:

- `amenable audit` — write the registered proof chain for one evidence
  name
- `amenable assess` — record and report structured assessments of
  registered proof harnesses
- `amenable gallery` — run and inspect non-production Kani proof-gallery
  experiments
- `amenable dump-registry` — write the full evidence and proof registry
  as JSON
- `amenable verify` — run registered proof harnesses through a verifier
  backend

Run `amenable --help` (or `amenable <subcommand> --help`) for the full
option set. Errors surface through `miette`, behind the default `cli`
feature — the library surface itself (`AmenableError`/`AmenableResult`)
has no `miette` dependency; only the binary and its tests do.

## See also

- [Root README](../../README.md) for the project-wide overview,
  verifier summary, and current coverage status.
- [`amenable.md`](../../amenable.md) for the trait-by-trait design.
- [`AMENABLE_PLAN.md`](../../docs/AMENABLE_PLAN.md) for the original
  architectural rationale.
