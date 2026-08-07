# Kani Pipe Accommodation Model

## Goal

Add a small Kani-only semantic model for anonymous pipe behavior so proofs that
currently bottom out in unsupported `pipe2` calls can move to an explicit
Amenable-owned accommodation boundary.

The immediate targets are `std::io::PipeReader` and `std::io::PipeWriter`,
whose direct `std::io::pipe()` setup currently exceeds Kani's supported
surface.

## Boundary

- `amenable_core` remains unchanged.
- `amenable_kani` owns the accommodation model.
- direct std/libc execution paths that fail today remain documented in the
  proof gallery as false trails.
- production proofs may switch to the accommodation model only when the proof
  comment states the conformance assumption explicitly.

## Initial Scope

1. Add a documented Kani-only anonymous-pipe model with:
   - paired reader / writer handles
   - shared resource identity
   - bounded buffered byte delivery
   - explicit writer-close behavior for `read_to_end`
2. Migrate the `PipeReader` and `PipeWriter` proofs from a direct std path to
   the modeled path.
3. Keep the direct `pipe2` path in the gallery as the unsupported baseline.
4. Add small integration tests for the model's deterministic laws.

## Non-Goals for This Slice

- no broad filesystem or process-environment migration yet
- no claim that libc pipe creation itself is verified
- no attempt to model asynchronous scheduling, readiness, or kernel buffering
  beyond the delivery contract needed by the current proofs

## Acceptance Criteria

- `amenable_kani` exports a documented anonymous-pipe accommodation model.
- the `PipeReader` and `PipeWriter` production proofs no longer depend on
  direct `pipe2` execution under Kani.
- the gallery still preserves the unsupported direct `pipe2` path.
- scoped checks/tests pass through the repo `justfile`.
