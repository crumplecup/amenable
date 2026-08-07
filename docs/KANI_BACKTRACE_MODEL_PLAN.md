# Kani Backtrace Accommodation Model

## Goal

Add a small Kani-only semantic model for forced backtrace capture so proofs
that currently fail at the `_Unwind_Backtrace` foreign boundary can move to an
explicit Amenable-owned accommodation boundary.

The immediate targets are `std::backtrace::Backtrace` and
`std::backtrace::BacktraceStatus`, whose current direct
`Backtrace::force_capture()` proofs fail before Kani can establish the
documented Rust-facing status law.

## Boundary

- `amenable_core` remains unchanged.
- `amenable_kani` owns the accommodation model.
- direct std execution paths that fail today remain documented in the proof
  gallery as false trails.
- production proofs may switch to the accommodation model only when the proof
  comment states the conformance assumption explicitly.

## Initial Scope

1. Add a documented Kani-only backtrace model with:
   - explicit modeled backtrace status variants
   - a deterministic `force_capture()` law returning `Captured`
   - a `status()` observer over that modeled result
2. Migrate the `Backtrace` and `BacktraceStatus` proofs from direct std paths
   to the modeled path.
3. Keep the direct foreign-boundary path in the gallery as the unsupported
   baseline for these proofs.
4. Add small integration tests for the model's deterministic laws.

## Non-Goals for This Slice

- no modeling of actual frame collection or symbolization
- no claim that platform unwinding internals are themselves verified
- no attempt to prove environment-sensitive `Backtrace::capture()` behavior

## Acceptance Criteria

- `amenable_kani` exports a documented backtrace accommodation model.
- the `Backtrace` and `BacktraceStatus` production proofs no longer depend on
  Kani's direct unwinding boundary.
- the gallery still preserves the failing direct std path as a false trail.
- scoped checks/tests pass through the repo `justfile`.
