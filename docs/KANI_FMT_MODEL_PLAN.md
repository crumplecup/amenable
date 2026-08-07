# Kani Fmt Accommodation Model

## Goal

Add a small Kani-only semantic model for formatting-builder behavior so proofs
that currently time out inside std formatting machinery can move to an
explicit Amenable-owned accommodation boundary.

The immediate targets are `core::fmt::Arguments`, `DebugList`, `DebugMap`,
`DebugSet`, `DebugStruct`, and `DebugTuple`, whose current direct rendering
proofs time out before Kani can establish their punctuation and ordering laws.

## Boundary

- `amenable_core` remains unchanged.
- `amenable_kani` owns the accommodation model.
- direct std execution paths that fail today remain documented in the proof
  gallery as false trails.
- production proofs may switch to the accommodation model only when the proof
  comment states the conformance assumption explicitly.

## Initial Scope

1. Add a documented Kani-only formatter model with:
   - symbolic leaf display/debug tokens
   - opaque label tokens for supplied type/field/key names
   - deterministic structural laws for `Arguments` display pass-through
   - deterministic structural punctuation/ordering laws for the one-field /
     two-entry `Debug*` builder shapes already used by the proof queue
2. Migrate the `Arguments`, `DebugList`, `DebugMap`, `DebugSet`,
   `DebugStruct`, and `DebugTuple` proofs from direct std paths to the modeled
   path.
3. Keep the direct formatting timeout path in the gallery as the unsupported
   baseline for this proof family.
4. Add small integration tests for the model's deterministic laws.

## Non-Goals for This Slice

- no attempt to model arbitrary formatting directives or locale behavior
- no claim that std's full integer/string rendering internals are verified
- no attempt to cover `Display` trait implementation correctness in general

## Acceptance Criteria

- `amenable_kani` exports a documented formatter accommodation model.
- the six replace-marked `fmt` production proofs no longer depend on Kani's
  direct formatting machinery.
- the gallery still preserves the direct formatting timeout path as a false
  trail.
- scoped checks/tests pass through the repo `justfile`.
