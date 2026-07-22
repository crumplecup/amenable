# KaniCompose for Amenable

## Goal

Introduce a verifier-specific bounded-construction trait for Kani so Amenable
can model strings, vectors, options, tuples, and user-defined carriers without
falling back to unconstrained `kani::any()` on recursive or heap-backed shapes.

`KaniCompose` is not a new constitutional role. The existing
`ProofToken`/`Sidecar`/`Establish`/`Exchange` family already governs lawful
proof exchange. `KaniCompose` belongs in the Kani backend as a modeling aid for
building verifier-friendly inputs inside harnesses.

## Boundary

- `amenable_core` remains unchanged: no new constitutional trait is added.
- `amenable_kani` owns the `KaniCompose` trait and built-in implementations.
- `amenable_derive` owns `#[derive(KaniCompose)]`.
- The top-level `amenable` facade may re-export the trait and derive for user
  convenience.

## Initial Scope

1. Add `amenable_kani::KaniCompose` with depth-based constructors and a bounded
   `kani_any()`.
2. Provide implementations for:
   - primitive scalars
   - `String`
   - `Vec<T>`
   - `Option<T>`
   - `Box<T>`
   - tuples
3. Add `#[derive(KaniCompose)]` for structs and enums, with pragmatic handling
   for common standard carriers.
4. Add a small Kani-only conformance surface that validates syntax and the
   depth semantics mechanically.

## Non-Goals for This Slice

- no new proof-token or exchange traits
- no result-ledger changes
- no broad migration of all production harnesses in the same patch
- no attempt to solve every std-lib accommodation immediately

## Follow-On Work

- move timeout-prone std-lib harnesses from raw symbolic construction to
  `KaniCompose`
- add field-level escape hatches such as skip/bounded policies where the first
  derive pass proves too blunt
- capture successful and failed modeling patterns in the proof gallery as they
  emerge

## Acceptance Criteria

- `amenable_kani` exports a documented `KaniCompose` trait.
- `amenable_derive` can derive `KaniCompose` for representative structs and
  enums.
- the workspace passes scoped compilation/tests for touched crates.
- at least one Kani-native conformance proof is executable with the local Kani
  toolchain.
