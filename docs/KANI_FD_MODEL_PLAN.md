# Kani FD Accommodation Model

## Goal

Add a small Kani-only semantic model for Unix file-descriptor behavior so
proofs that currently bottom out in unsupported libc calls can move to an
explicit Amenable-owned accommodation boundary.

The immediate target is `std::os::unix::io::OwnedFd`, whose direct `File` /
`BorrowedFd` setup currently reaches `fcntl` and therefore exceeds Kani's
supported surface.

## Boundary

- `amenable_core` remains unchanged.
- `amenable_kani` owns the accommodation model.
- direct std/libc execution paths that fail today remain documented in the
  proof gallery as false trails.
- production proofs may switch to the accommodation model only when the proof
  comment states the conformance assumption explicitly.

## Initial Scope

1. Add a documented Kani-only Unix fd model with:
   - live/dead status
   - raw fd observation
   - resource identity
   - borrow / duplicate / file-ownership transfer laws
2. Migrate the `OwnedFd` proof from a direct std path to the modeled path.
3. Keep the direct `fcntl` path in the gallery as the unsupported baseline.
4. Add small integration tests for the model's deterministic laws.
5. Add a repo `justfile` so scoped checks and tests have canonical entrypoints.

## Non-Goals for This Slice

- no broad migration of process, filesystem, or network proofs yet
- no claim that libc itself is verified
- no removal of the existing gallery diagnostics

## Acceptance Criteria

- `amenable_kani` exports a documented Unix fd accommodation model.
- the `OwnedFd` production proof no longer depends on direct `fcntl`
  execution under Kani.
- the gallery still preserves the unsupported direct `fcntl` path.
- scoped checks/tests pass through the repo `justfile`.
