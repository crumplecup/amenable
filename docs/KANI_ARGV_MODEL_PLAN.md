# Kani Argv Accommodation Model

## Goal

Add a small Kani-only semantic model for process argument behavior so proofs
that currently fail under Kani's synthetic process model can move to an
explicit Amenable-owned accommodation boundary.

The immediate targets are `std::env::Args` and `std::env::ArgsOs`, whose direct
`std::env::args()` / `args_os()` setup currently allows a verifier-only empty
argv counterexample even though a real process includes its own program slot.

## Boundary

- `amenable_core` remains unchanged.
- `amenable_kani` owns the accommodation model.
- direct std execution paths that fail today remain documented in the proof
  gallery as false trails.
- production proofs may switch to the accommodation model only when the proof
  comment states the conformance assumption explicitly.

## Initial Scope

1. Add a documented Kani-only argv model with:
   - one guaranteed program slot
   - bounded extra argument slots
   - shared slot-count law for both UTF-8 and `OsString` views
2. Migrate the `Args` and `ArgsOs` proofs from direct std paths to the modeled
   path.
3. Keep the direct synthetic-process mismatch path in the gallery as the
   unsupported baseline for these proofs.
4. Add small integration tests for the model's deterministic laws.

## Non-Goals for This Slice

- no modeling of environment variables or PATH composition yet
- no attempt to prove non-UTF-8 `OsString` content behavior
- no claim that host process creation or launcher behavior itself is verified

## Acceptance Criteria

- `amenable_kani` exports a documented argv accommodation model.
- the `Args` and `ArgsOs` production proofs no longer depend on Kani's direct
  synthetic process state.
- the gallery still preserves the failing direct std path as a false trail.
- scoped checks/tests pass through the repo `justfile`.
