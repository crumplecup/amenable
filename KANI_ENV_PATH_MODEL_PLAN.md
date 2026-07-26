# Kani Env Path Accommodation Model

## Goal

Add a small Kani-only semantic model for PATH-style helper behavior so proofs
that currently time out in `std::env::join_paths()` and `split_paths()` can
move to an explicit Amenable-owned accommodation boundary.

The immediate targets are `std::env::JoinPathsError` and
`std::env::SplitPaths<'static>`, whose direct std helper paths currently remain
tractable at compile time but burn solver time in the platform helper logic.

## Boundary

- `amenable_core` remains unchanged.
- `amenable_kani` owns the accommodation model.
- direct std execution paths that time out today remain documented in the proof
  gallery as false trails.
- production proofs may switch to the accommodation model only when the proof
  comment states the conformance assumption explicitly.

## Initial Scope

1. Add a documented Kani-only PATH-style model with:
   - a platform separator
   - a bounded joinable-path subset
   - an explicit unjoinable-path error boundary
   - a simple join/split round-trip law for separator-free modeled paths
2. Migrate the `JoinPathsError` and `SplitPaths<'static>` proofs from direct
   std helpers to the modeled path.
3. Keep reduced direct std timeout representatives in the proof gallery.
4. Add small integration tests for the deterministic model laws.

## Non-Goals for This Slice

- no attempt to model every platform quoting rule in full fidelity
- no modeling of real filesystem semantics or `PathBuf` normalization
- no claim that arbitrary host paths are covered beyond the stated modeled
  subset

## Acceptance Criteria

- `amenable_kani` exports a documented env-path accommodation model.
- the `JoinPathsError` and `SplitPaths<'static>` production proofs no longer
  depend on Kani's direct std helper implementations.
- the gallery preserves reduced timeout representatives for the direct std
  path.
- scoped checks/tests pass through the repo `justfile`.
