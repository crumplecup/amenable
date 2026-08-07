# amenable_verus

> Verus proof content for Rust standard-library carriers.

## What this crate is

This crate contains *only* what `verus --crate-type=lib
crates/amenable_verus/src/lib.rs` needs to check: real `verus! { ... }`
spec functions, nothing else. Unlike `amenable_creusot` (which still
depends on `amenable_core`/`amenable_derive`, since `cargo creusot`
resolves ordinary Cargo dependencies), Verus is invoked as a bare
compiler over a single file tree — it never sees `Cargo.toml`, so it
cannot resolve `amenable_core`, `inventory`, or any proc-macro crate at
all. Confirmed empirically: pointing `verus` at an earlier, pre-split
version of this crate (which depended on all three) failed immediately
with unresolved-crate errors, not proof errors.

So this crate has exactly two dependencies — `verus_builtin_macros` and
`vstd` — mirroring `elicitation_verus`'s own real, working structure. The
witness/registry machinery that bridges these proofs to
`amenable_core::Witness`/`amenable_std::RustStdStandard<T>` lives
entirely in `amenable_std::verus_witness` instead, which captures each
proof's verbatim source via `include_str!` (a plain Rust feature, no
proc-macro needed) rather than `amenable_derive::harness!` (which, being
a proc-macro from a crate Verus can't resolve, wouldn't compile under
Verus's toolchain either).

## Coverage

**332 verified proof functions**, confirmed by running `verus
--crate-type=lib crates/amenable_verus/src/lib.rs` directly: `332
verified, 0 errors`.

Most state a real `assume_specification` axiom directly against the
actual std type or method. Where `vstd` has no spec support for a type
at all, a hand-verified accommodation model states the same law over a
Verus-native stand-in instead.

`rust_std::os_windows_carrier` is the one cluster that's genuinely
different from every other file here: its `#[cfg(windows)]`-gated axioms
name the real `std::os::windows::*` types directly — not a model —
because Verus is the only one of this project's three verifier backends
that runs *natively on Windows*. Kani/CBMC and `creusot-rustc` don't, so
they fall back to synthetic Linux-compilable models for that same cluster
instead (see `amenable_kani`/`amenable_creusot`'s own READMEs). Nothing
in `os_windows_carrier.rs` has ever been checked on this crate's Linux
development host; it's checked only by the `verus-windows` GitHub Actions
workflow (`.github/workflows/verus-windows.yml`,
`workflow_dispatch`-triggered, `windows-latest`) — see that carrier's own
module doc comment for the full reasoning.

## See also

- [Root README](../../README.md) for the project-wide overview and
  verifier summary.
- [`amenable_std`](../amenable_std/README.md) for the registry these
  witnesses bridge into, and overall coverage across all three verifiers.
- [`amenable_kani`](../amenable_kani/README.md),
  [`amenable_creusot`](../amenable_creusot/README.md) for the other two
  backends.
