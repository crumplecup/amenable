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
`amenable` makes that distinction explicit and load-bearing.

Every claim in an `amenable` program is backed by exactly one of two
lawful things — never a silent third option:

- **a machine-checked proof** — emitted through `Witness`, checked by a
  `Verifier` backend. Three independent backends (Kani, Creusot, Verus)
  check the same claims against the same carriers, so a claim proven
  under all three is much stronger evidence than any one alone.
- **an explicit `Standard` certification of provenance** — a structured,
  auditable record naming the authority, source, and rationale for a
  trust decision that *cannot* be mathematically derived: a third-party
  standard, an RFC, a company policy, or an asserted local design
  decision with no external authority to cite.

When a claim can't be discharged by a proof, the framework does not fall
back to "trivially trusted." It requires a `Standard` certification
instead, and no blanket implementation is allowed to grant one for free.
The trait interface is the only way to manufacture a proof token, which
keeps the audit surface to the narrow bottleneck of the trait method
implementations.

## What you get

### 1. An auditable surface of assumptions

Every non-trivial program rests on invariants it can't prove from within
itself — the same shape as an `unsafe` block needing a `// SAFETY:`
comment. Today those assumptions live in prose: a doc comment cites "ASC
230" or "RFC 3339 §5.6", a reviewer takes it on trust, and nothing ties
the citation to the code that depends on it or lets you enumerate what's
been assumed.

`amenable` turns each one into a **type**. A contract type that `impl`s
`Standard` names one assumption; its `Provenance` record carries the
*why* — the standard number, the section, a link, the rationale — as
structured metadata you can query, not text buried in a `///`:

```rust
use amenable_core::{MetadataEntry, Standard};

/// The source and destination accounts must differ.
#[derive(Debug, Clone, Copy, Default, amenable_derive::Standard)]
#[standard(basis = "Self")]
pub struct AccountsDistinct;

// The citation lives with the type, as data, not in a doc comment.
impl amenable_core::Provenance for AccountsDistinct {
    type MetadataIter = std::vec::IntoIter<MetadataEntry>;
    fn metadata(&self) -> Self::MetadataIter {
        vec![
            MetadataEntry::new("standard", "ASC 230"),
            MetadataEntry::new("title", "Statement of Cash Flows"),
            MetadataEntry::new("rationale", "gross vs. net presentation needs distinct counterparties"),
        ]
        .into_iter()
    }
}

// Retrievable, not buried — and the same record flows into `Standard::report()`
// and into a tracked `Certificate` via `Standard::certification(..)`:
assert_eq!(AccountsDistinct.get("standard").unwrap().value(), "ASC 230");
```

The `Provenance` schema is yours to define. When a citation shape
repeats across many contract types — an ISO number, an ASC section, an
internal policy ID, all with the same fields — make it its own struct,
`#[derive(amenable_derive::Provenance)]` it once, and point
`#[standard(provenance_type = "..")]` at it; the derive projects that
struct's fields into the queryable record for you.

For a code reviewer, the set of `Standard` types **is** the assumption
surface: ideally a closure over the problem space, with every relevant
assumption represented by a type you can list, look up, render, and
certify — instead of a doc comment you have to go find.

### 2. Multi-backend verification, derived

You don't hand-write proof scaffolding three times. You decorate ordinary
structs and enums with `amenable` derives and attributes, and the family
wires the rest:

| Macro | For |
| --- | --- |
| `#[derive(Provenance)]` | project a struct's fields into structured, queryable metadata |
| `#[derive(Evidence)]` / `#[derive(Standard)]` | a claim with a real proof / an asserted-and-cited root claim |
| `#[derive(ProofToken)]` / `#[establish(..)]` / `#[derive(Sidecar)]` | proof tokens, the lawful exchanges that mint them, and the payload+token carriers that flow through an exchange |
| `#[exchange(..)]` | a proof-carrying state transition — its real body captured once, verbatim, then checked by every backend |
| `#[derive(StateMachine)]` | a closed set of states and the lawful transitions over them; states are asserted `Provenance` roots, transitions are proven relations |

The macros capture the structure once; each backend consumes it — Kani
directly, Creusot and Verus through generated companion files — so the
same annotated types are checked three ways without re-authoring the
proofs per backend.

### Worked examples

Two complete evidence chains ship as reference implementations — proof of
concept for the derives, and a walkthrough of how to apply them to your
own domain:

- **`Stoplight`** (`amenable_core`/`amenable_kani`, with backend
  companions) — the minimal case: a three-state machine
  (`Green → Yellow → Red → Green`) where every transition is a proven
  `Exchange` and every state is an asserted `Standard` carrying its own
  `Provenance` record. The reference for the `Exchange`/`StateMachine`
  derives.
- **`amenable_gaap`** ([README](crates/amenable_gaap/README.md)) — a real
  double-entry ledger: `Pending → Validated → Committed` with a
  `reject`/`rollback` branch. Its invariants — `AmountPositive`,
  `SufficientFunds`, `AccountsDistinct`, `BalancedEntries`, each resting
  on an accounting standard (ASC sections, foundational double-entry
  rules) — are each a named contract type rather than an inline
  assertion, proven on all three backends with zero per-backend
  duplicate logic. The reference walkthrough for building your own.

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
            /// ...
            #[kani::proof]
            fn verify_char_unicode_scalar() {
                let c: char = kani::any();
                let u = c as u32;

                assert!(
                    <ValidUnicodeScalar as Ensures<crate::KaniVerifier>>::ensures(u),
                    "char is a valid Unicode scalar value"
                );

                let c2 = char::from_u32(u).expect("valid unicode scalar round-trips");
                assert!(
                    <RustStdStandard<char> as Ensures<crate::KaniVerifier>>::ensures((c, c2)),
                    "char round-trips through u32"
                );
            }
    rust.authority_kind: external_standard
    rust.authority: Rust Project Developers
    rust.source_crate: core
    rust.source_module: core::primitive
    source_url: https://doc.rust-lang.org/std/primitive.char.html
    type_name: char
    semantic_summary: The character carrier stores a Unicode scalar value.
```

That's the whole story in miniature. The claim ("`char` is a valid
Unicode scalar value and round-trips through `u32`") is backed by the
*actual* Kani proof source, not a description of one — and the proof
body doesn't restate the bound inline: it calls through
`ValidUnicodeScalar` and `RustStdStandard<char>`, the named contract
types that own those two claims (value 1 above, applied to `std` itself).
The provenance metadata that grounds the root (`rust.authority`,
`source_url`, …) is right there in the report, and the report says
explicitly which verifiers it's complete for. Build with
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
| [`amenable_derive`](crates/amenable_derive/README.md) | Proc macros the trait family needs — derives (`Standard`, `Provenance`, `Evidence`, `ProofToken`, `Sidecar`, `StateMachine`, `Witness`, `KaniCompose`), attributes (`#[exchange]`, `#[establish]`, `#[calculation]`, …), and `harness!` |
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

See [`amenable.md`](amenable.md) for the trait-by-trait design and
[`docs/AMENABLE_PLAN.md`](docs/AMENABLE_PLAN.md) for the architectural
rationale this project was built from. Planning documents for individual
features live in [`docs/`](docs/), indexed by
[`docs/PLANNING_INDEX.md`](docs/PLANNING_INDEX.md).

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
three backends at once, which is what the aggregate figure below
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
