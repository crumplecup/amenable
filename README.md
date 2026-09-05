# Amenable

> A declarative interface amenable to formal verification.

[![CI](https://github.com/crumplecup/amenable/actions/workflows/ci.yml/badge.svg)](https://github.com/crumplecup/amenable/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT-blue.svg)](LICENSE-APACHE)
[![Rust](https://img.shields.io/badge/rust-2024-orange.svg)](https://www.rust-lang.org)

---

## What is Amenable?

Software that claims to be "verified" is usually trusted on faith: a
comment says so, a test suite is green, and a reviewer has no way to
check what was actually established or how much confidence it deserves.
`amenable` makes that distinction explicit and load-bearing. It is the
foundational, dependency-light trait family for lawful proof-carrying
software structure: it defines the roles and admissibility criteria
governing a proof economy — which types are permitted to serve as
trusted roots, which types may count as derived evidence, which
exchanges are lawful, and which workflows are closed under those
exchanges.

Every claim in the system is backed by exactly one of two lawful things —
never a silent third option:

- a genuine machine-checked proof, emitted through `Witness` and consumed by
  a `Verifier` backend (Kani, Creusot, Verus)
- an explicit `Standard` certification of provenance — a structured,
  auditable record naming the authority, source, and rationale for a
  trust decision that cannot be mathematically derived, whether that
  authority is a third-party citation or an asserted local design
  decision

Three independent verifier backends implement `Witness` against the same
carriers, so a claim proven under Kani *and* Creusot *and* Verus
independently is much stronger evidence than any one alone — and the
registry backing all of this is queryable at any time, not just
documentation (see [Try it](#try-it) below).

`amenable` is upstream of every framework that consumes it. Formal
verification does not depend on any downstream conversational or domain
framework; those frameworks depend on `amenable`.

See [`amenable.md`](amenable.md) for the trait-by-trait design and
[`docs/AMENABLE_PLAN.md`](docs/AMENABLE_PLAN.md) for the original
architectural rationale this project was built from. Planning documents
for individual features live in [`docs/`](docs/), indexed by
[`docs/PLANNING_INDEX.md`](docs/PLANNING_INDEX.md).

## Try it

`amenable::proof_chain` looks up a registered evidence chain and returns
an auditable report — the same thing the `amenable audit` CLI subcommand
prints. This is a real, tested example
([`crates/amenable/examples/audit_proof_chain.rs`](crates/amenable/examples/audit_proof_chain.rs)),
runnable as-is:

```rust
fn main() -> Result<(), amenable::ChainError> {
    let report = amenable::proof_chain("RustStdStandard<char>")?;
    println!("{report}");
    Ok(())
}
```

```console
$ cargo run --example audit_proof_chain
Proof chain for amenable_std::rust_std::RustStdStandard<char> (complete for: kani)

amenable_std::rust_std::RustStdStandard<char> (root)
  proof [kani]:
    harness: verify_char_unicode_scalar
    claim: /// `char` is constrained to Unicode scalar values (excludes the
            /// surrogate range `0xD800..=0xDFFF`) and round-trips through `u32`.
            #[kani::proof]
            fn verify_char_unicode_scalar() {
                let c: char = kani::any();
                let u = c as u32;

                assert!(
                    u <= 0xD7FF || (0xE000..=0x10FFFF).contains(&u),
                    "char is a valid Unicode scalar value"
                );

                let c2 = char::from_u32(u).expect("valid unicode scalar round-trips");
                assert!(c == c2, "char round-trips through u32");
            }
    rust.authority_kind: external_standard
    rust.authority: Rust Project Developers
    rust.source_crate: core
    rust.source_module: core::primitive
    source_url: https://doc.rust-lang.org/std/primitive.char.html
    type_name: char
    semantic_summary: The character carrier stores a Unicode scalar value.
```

That's the whole story in miniature: the claim ("`char` is a valid
Unicode scalar value and round-trips through `u32`") is backed by the
*actual* Kani proof source, not a description of one, and the report
says explicitly which verifiers it's complete for. Build with
`--features creusot,verus` and the same lookup returns all three
independent proofs for this claim, not just Kani's.

The `amenable` CLI wraps this and more:

- `amenable audit` — write the registered proof chain for one evidence
  name (what the example above does)
- `amenable assess` — record and report structured, reviewer-owned
  assessments of registered proof harnesses
- `amenable gallery` — run and inspect non-production Kani proof-gallery
  experiments
- `amenable dump-registry` — write the full evidence and proof registry
  as JSON
- `amenable verify` — run registered proof harnesses through a verifier
  backend

Run `amenable --help` (or `amenable <subcommand> --help`) for the full
option set.

## Workspace

| Crate | Role |
| --- | --- |
| [`amenable`](crates/amenable/README.md) | Top-level facade + the CLI above |
| [`amenable_core`](crates/amenable_core/README.md) | The constitutional trait family itself — `Verifier`, `Witness`, `Evidence`, `Standard`, `Sidecar`, `Establish`, `Exchange`, `StateMachine`, `Provenance` |
| [`amenable_derive`](crates/amenable_derive/README.md) | Proc macros the trait family needs — derives (`Standard`, `ProofToken`, `Sidecar`, `Witness`, ...), attributes (`#[exchange]`, `#[establish]`, `#[calculation]`, ...), and `harness!` |
| [`amenable_gaap`](crates/amenable_gaap/README.md) | GAAP ledger worked example — a real, backend-neutral evidence chain proven on all three verifiers, doubling as the reference walkthrough for building a new one |
| [`amenable_std`](crates/amenable_std/README.md) | `RustStdType` + the registry where all three verifiers' witnesses converge |
| [`amenable_kani`](crates/amenable_kani/README.md) | Kani backend — ~445 proof harnesses |
| [`amenable_creusot`](crates/amenable_creusot/README.md) | Creusot backend — ~175 harnesses |
| [`amenable_verus`](crates/amenable_verus/README.md) | Verus backend — ~485 verified proof functions |

Building a new evidence chain (a new worked example, or extending an
existing one)? [`amenable_derive`'s onboarding
walkthrough](crates/amenable_derive/README.md#onboarding-building-a-new-worked-example)
covers the order to reach for these macros in, using `amenable_gaap`'s
own construction as the worked reference.

## Verifiers

- **Kani** (`amenable_kani`) — bounded model checking over real Rust code
  via CBMC.
- **Creusot** (`amenable_creusot`) — real Pearlite `requires`/`ensures`
  contracts, discharged by SMT.
- **Verus** (`amenable_verus`) — native Rust contracts checked by Verus's
  own SMT-backed toolchain; the only one of the three that runs natively
  on Windows.

Each backend has real limits on what it can check directly (state-space
size, platform gating, missing spec/contract surface for a given std
call), and each documents its own strategy for working around them —
usually a small, Amenable-owned accommodation model standing in for the
real type, with the proof's evidence still hand-linked back to the real
type's registration. See each crate's own README (linked in the table
above) for the specifics, including how the `std::os::windows` cluster
is handled by all three despite none of Kani/Creusot/`creusot-rustc`
running on Windows.

## Status

The core constitutional trait family is implemented with zero runtime
dependencies, and all three verifier backends are wired in and actively
exercised. The per-backend proof counts in the table above aren't three
disjoint slices of `std` — most tracked types carry proofs from all
three backends at once, which is exactly what the aggregate figure below
measures directly.

Per the project's own coverage audit (`cordial coverage --crate-name
amenable_std`), **422 of 457 accountable stable `std`/`core` types
(92.3%) have complete evidence plus every applicable verifier's
witness** as of this writing — re-run that command for a current figure
rather than trusting this snapshot; the accountable-type universe grows
as the standard library does. Of the 35 open items, most are either
confirmed false-negatives in the audit tool's own type-alias resolution
(`core::num::NonZero*`, `LayoutErr` — no real gap) or the
`std::os::windows` cluster's Verus proofs, only checkable on the
`windows-latest` CI job mentioned above; a smaller remainder
(`core::range::*`, a few `os::unix` raw-type aliases) is untriaged
backlog.

Not yet built: structural proof-quality heuristics on `Witness` itself
(automatic detection of vacuous or corner-cut proofs — `amenable.md`
describes the target design), and the audit-inversion registry (a
reverse index from a cited `Standard` root to every dependent that
relies on it, for direct review). Reviewer-driven proof assessment
(`amenable assess`, above) is a separate, already-implemented path that
depends on neither — see [`docs/AMENABLE_PLAN.md`](docs/AMENABLE_PLAN.md)
for the full phased plan.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
