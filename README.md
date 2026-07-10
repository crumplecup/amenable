# Amenable

> A declarative interface amenable to formal verification.

[![License](https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT-blue.svg)](LICENSE-APACHE)
[![Rust](https://img.shields.io/badge/rust-2024-orange.svg)](https://www.rust-lang.org)

---

## What is Amenable?

`amenable` is the foundational, dependency-light trait family for lawful
proof-carrying software structure. It defines the roles and admissibility
criteria governing a proof economy: which types are permitted to serve as
trusted roots, which types may count as derived evidence, which exchanges
are lawful, and which workflows are closed under those exchanges.

Every claim in the system is backed by exactly one of two lawful things —
never a silent third option:

- a genuine machine-checked proof, emitted through `Witness` and consumed by
  a `Verifier` backend (Kani, Creusot, Verus)
- an explicit `Standard` or `Objective` certification of provenance — a
  structured, auditable record naming the authority, source, and rationale
  for a trust decision that cannot be mathematically derived

`amenable` is upstream of every framework that consumes it. Formal
verification does not depend on any downstream conversational or domain
framework; those frameworks depend on `amenable`.

See [`amenable.md`](amenable.md) for the trait-by-trait design and
[`AMENABLE_PLAN.md`](AMENABLE_PLAN.md) for the full architectural rationale
and phased implementation plan.

## Status

Early design and incubation. The core constitutional trait family
(`Verifier`, `Witness`, `Evidence`, `Standard`, `Objective`, `Sidecar`,
`Establish`, `Exchange`, `StateMachine`, `Amenable`, `Provenance`,
`RustStdType`) is implemented with zero runtime dependencies. Proof-emission
machinery, proof-quality heuristics, and the audit-inversion registry are
not yet built — see `AMENABLE_PLAN.md` for current phase status.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
