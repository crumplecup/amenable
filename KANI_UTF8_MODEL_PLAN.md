# Kani UTF-8 Accommodation Model

## Goal

Lift the bounded UTF-8 modeling work from `elicitation` into `amenable_kani`
so proofs that currently time out inside the direct owned
`String::from_utf8` / `FromUtf8Error` std path can move to an explicit
Amenable-owned accommodation boundary.

The immediate target is
`alloc_string::verify_from_utf8_error_recovers_the_original_bytes`, which
still times out under Kani's native `--harness-timeout 3m` even after the
proof was reduced to a fixed two-byte invalid UTF-8 vector. The nearby direct
`std::str::Utf8Error` proof remains tractable, so the blocker is specifically
the owned conversion / recovery path rather than UTF-8 reasoning in general.

## Boundary

- `amenable_core` remains unchanged.
- `amenable_kani` owns the accommodation model.
- the direct std `String::from_utf8` timeout remains documented in the proof
  gallery as a false trail.
- production proofs may switch to the accommodation model only when the proof
  comment states the conformance assumption explicitly.
- `elicitation` is the source pattern; `amenable` is the distilled and
  tightened verifier-facing surface.

## Initial Scope

1. Add a documented Kani-only bounded UTF-8 model with:
   - bounded byte storage
   - explicit valid / invalid UTF-8 classification
   - exact owned-byte recovery semantics for the invalid case
2. Introduce the smallest modeled surface needed to express:
   - a successful owned UTF-8 conversion for valid bounded bytes, and
   - a recovery object for invalid bounded bytes whose recovered bytes are
     exactly the original owned bytes
3. Migrate the `FromUtf8Error` production proof from the direct std path to
   the modeled path.
4. Preserve the reduced direct-std timeout representative in the gallery.
5. Add small tests that pin the model's byte-preservation laws.

## Non-Goals for This Slice

- no attempt to re-verify all of Rust's UTF-8 decoder internals inside
  `amenable`
- no attempt to model arbitrary heap allocation behavior or full `String`
  APIs
- no migration of already-tractable direct std UTF-8 proofs unless they later
  show the same verifier-fit problem
- no outer process timeout wrappers; the native Kani per-harness timeout
  remains the execution policy

## Acceptance Criteria

- `amenable_kani` exports a documented UTF-8 accommodation model.
- the `FromUtf8Error` production proof no longer depends on direct
  `String::from_utf8` execution.
- the gallery preserves the fixed-invalid-vector direct std timeout path as a
  false trail.
- scoped checks/tests pass through the repo `justfile`.
