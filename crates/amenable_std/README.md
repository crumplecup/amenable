# amenable_std

> `RustStdType`: interface and concrete registrations for Rust
> standard-library types.

## What this crate is

Traits meant to be implemented directly on foreign standard-library types
must live in the crate that defines them — Rust's orphan rules leave no
other option, since neither the trait nor `bool`/`i32`/`String`/etc. is
local anywhere else. So rather than an interface crate plus a downstream
consumer, this crate defines `RustStdType` and its full std-lib coverage
together, alongside the default concrete certificate and registry types,
serving as the canonical gold-standard registrations other crates can
depend on instead of re-registering the same std types themselves.

This is also where all three verifier backends' witnesses converge: each
of `amenable_kani`, `amenable_creusot`, and `amenable_verus` is
architecturally constrained to stay pure proof content (Kani/CBMC's
symbolic-execution model, `creusot-rustc`'s whole-crate translation pass,
and Verus's bare-compiler invocation each rule out ordinary bridging
machinery living alongside the proofs themselves — see each crate's own
README for the specifics). The bridge from each of those proofs to a
concrete `RustStdStandard<T>` — the `Witness` impl, the registered
`ProofRecord` — lives here instead, in `rust_std.rs`,
`verus_witness.rs` (behind the `verus` feature), and `creusot_witness.rs`
(behind the `creusot` feature).

## Coverage

Per the project's own audit tool (`elicit_doc`), **421 of 440 accountable
stable `std`/`core` types (95.7%) have complete evidence plus every
applicable verifier's witness**. The remaining 19 are either:

- confirmed false-negatives in the audit tool's own type-alias resolution
  (`core::num::NonZero*`, `core::alloc::LayoutErr`) — real coverage
  already exists under the type's canonical name, confirmed by direct
  compile-test or by reading the crate's own documented rationale, not
  assumed; or
- the `std::os::windows` cluster (`EncodeWide`, `BorrowedHandle`,
  `BorrowedSocket`, `HandleOrInvalid`, `OwnedHandle`, `OwnedSocket`),
  whose Verus proofs are real (`rust_std/os_windows.rs`'s
  `#[cfg(windows)]` registrations, `amenable_verus::rust_std::
  os_windows_carrier`'s `assume_specification` axioms) but only
  checkable on the `verus-windows` GitHub Actions workflow
  (`workflow_dispatch`-triggered, `windows-latest`), never on this
  crate's Linux development host. Kani and Creusot already cover this
  same cluster — see their own READMEs for how, given that neither of
  those two verifiers can run on Windows at all.

## See also

- [Root README](../../README.md) for the project-wide overview and
  verifier summary.
- [`amenable_kani`](../amenable_kani/README.md),
  [`amenable_creusot`](../amenable_creusot/README.md),
  [`amenable_verus`](../amenable_verus/README.md) for each backend's own
  coverage and architecture.
