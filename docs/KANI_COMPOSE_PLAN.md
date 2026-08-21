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

## Scope Correction: `#[cfg(kani)]`-Only

**Status:** 🔲 Planning.

This doc said it from the start (line 11-12 above): `KaniCompose` is a
"modeling aid for building verifier-friendly inputs inside harnesses" --
a Kani-only discipline, not a general-purpose test helper. That framing
never made it into the trait's actual `cfg` boundary. `amenable_kani::
compose::symbolic_any` (the one primitive every `KaniCompose` impl routes
symbolic construction through) has carried a `#[cfg(not(kani))]` branch
that panics since day one:

```rust
pub(crate) fn symbolic_any<T>() -> T where T: KaniArbitrary {
    #[cfg(kani)]   { kani::any() }
    #[cfg(not(kani))] { panic!("KaniCompose symbolic construction is only available under cfg(kani)") }
}
```

`kani_assume` carries the same shape (`assert!` instead of `panic!`, same
idea). The *only* reason either needs to compile outside `cfg(kani)` at
all is that four ordinary `#[test]`s (not proofs) reach into
`KaniCompose::kani_depth0()`/`kani_depth1()`/`kani_depth2()` under plain
`cargo test`, for two different reasons -- confirmed by reading each call
site, not assumed:

- **Actually testing `KaniCompose` itself** (its own contract: "depth0 is
  Disabled", "depth2 is the third fixed representative", etc.):
  - `backtrace_model_test.rs::composed_statuses_cover_the_modeled_status_space`
  - `fmt_model_test.rs::composed_format_atoms_keep_display_and_debug_views`
- **Using a depth constructor as a lazy way to get "some instance"**, then
  asserting something about the *type*, not about `KaniCompose`:
  - `env_model_test.rs::argv_always_counts_the_program_slot` (`KaniArgv::
    kani_depth0()`, only to call `.args_count()`/`.extra_count()`)
  - `pipe_model_test.rs::fresh_pipe_shares_one_resource_across_distinct_endpoints`
    and `write_close_read_round_trips_buffered_bytes` (`KaniPipe::
    kani_depth0()`, only to exercise reader/writer resource sharing)

The second bucket is what's actually wagging the dog: `KaniArgv::
kani_depth0()`'s entire body is `Self::new(String::new(), 0)` --
`KaniArgv` already has a real inherent `new` these tests could call
directly (`argv_keeps_the_program_slot_and_extra_count` already does).
`KaniPipe::kani_depth0()`'s body (`KaniFd::live(0, 0)` /
`KaniFd::live(1, 0)`, fully fixed, no `symbolic_any` call at all) has no
inherent-constructor equivalent yet -- `KaniPipe::fresh()` exists but is
itself genuinely Kani-only (calls `symbolic_any`/`kani_assume` for real
non-deterministic resource ids), so it can't substitute here.

Also load-bearing, found while tracing the trait's actual reach, not
guessed: `amenable_derive::expand_kani_compose` (the `#[derive(
KaniCompose)]` codegen) emits the identical dual-mode shape for derived
enums' `kani_any()` (`kani_compose.rs` around the enum-variant-selection
arm) -- the same correction applies to its generated `impl` blocks, not
just the hand-written ones in `amenable_kani`. And `amenable/src/lib.rs`
re-exports `KaniCompose` unconditionally (`pub use amenable_kani::{...,
KaniCompose, ...}`, no `#[cfg(kani)]`) -- gating the trait without
touching that re-export would break every ordinary `cargo build -p
amenable`.

### Steps

1. **Bucket-B tests stop touching `KaniCompose`.** Rewrite
   `argv_always_counts_the_program_slot` to call `KaniArgv::new(..)`
   directly. Give `KaniPipe` a plain inherent constructor for its fixed
   depth-0 shape (name TBD, e.g. `minimal()`, mirroring the existing
   `fresh()` naming) and point `kani_depth0()` at it *and* point the two
   pipe tests at it instead of `kani_depth0()`.
2. **Bucket-A tests move with the trait.** They're genuinely testing
   `KaniCompose`'s own contract, so once the trait is proof-only they
   belong as real `#[kani::proof]` harnesses (via `harness!`, this
   crate's own convention), not `#[test]`s -- a bare `#[cfg(kani)]` on a
   `#[test]` fn would silently stop it from ever running at all (`cargo
   test` doesn't set `cfg(kani)`; `cargo kani` doesn't run `#[test]`s).
3. **Gate the trait.** `KaniCompose` (definition + all ~30 `impl
   KaniCompose for X` blocks, `impl_kani_compose_symbolic!`, `symbolic_any`,
   `kani_assume`) becomes `#[cfg(kani)]`-only; drop the `#[cfg(not(kani))]`
   panic/assert branches entirely -- real Kani plumbing, no fallback.
4. **Fix the derive.** `expand_kani_compose`'s generated `impl KaniCompose
   for X` wraps in `#[cfg(kani)]`; drop its own dual-mode panic branch the
   same way.
5. **Fix the re-export.** Split `KaniCompose` out of `amenable/src/lib.rs`'s
   unconditional `pub use amenable_kani::{...}` into its own `#[cfg(kani)]
   pub use amenable_kani::KaniCompose;`.
6. **Let the compiler find what step 3's grep couldn't.** Full workspace
   `cargo check --all-features` (ordinary, no kani) after steps 1-5;
   fix every site it flags. This is the real exhaustiveness check --
   `KaniCompose` methods are called from ~250 sites in `amenable_kani::
   src`, essentially all inside `harness!`-wrapped proof bodies, but that
   was confirmed by pattern, not by hand-checking all 250.
7. **Verify.** `cargo test --workspace` (ordinary build must still pass,
   with zero `KaniCompose` reachable outside `cfg(kani)`), then real
   `cargo kani` runs (serialized, `--harness-timeout`, no outer timeout)
   covering both the pre-existing harnesses and the two harnesses Bucket-A
   became in step 2.

### Acceptance Criteria

- `symbolic_any`/`kani_assume` have no `#[cfg(not(kani))]` branch left --
  cordial's panics scan finds zero findings in `amenable_kani::compose`,
  not an exempted one.
- `cargo build -p amenable` (ordinary, no kani feature/cfg) still
  succeeds.
- `cargo test --workspace` still passes, including the rewritten
  Bucket-B tests.
- The two Bucket-A claims still have real coverage, now as `#[kani::
  proof]` harnesses instead of `#[test]`s.
