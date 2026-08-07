# amenable_creusot

> Creusot verifier backend for the `amenable` constitutional trait family.

## What this crate is

`CreusotVerifier` and the `CreusotWitness` trait are defined here — there
is only one verifier Creusot works with, Creusot, so the marker belongs
with the crate that means it. But unlike Kani and Verus, the impls
bridging `CreusotWitness`/`Witness<CreusotVerifier>` to concrete std
carriers (`RustStdStandard<T>`) live in `amenable_std` instead of here.

That split exists because `creusot-rustc`'s whole-crate translation pass
sweeps every local item in a `creusot-std`-dependent crate — including
ones no `#[cfg(creusot)]` gate protects, since Rust items don't need to
*run* to be enumerated — and chokes on ordinary Rust infrastructure that's
completely unremarkable to plain `rustc`: a return-position `impl Trait`
on a local `impl` panicked its intrinsics-gathering pass outright (a real
ICE, confirmed empirically, not a hypothetical), and the `static` item
`inventory::submit!` expands to hits "unsupported definition kind" the
same way. So this crate stays pure Pearlite proof-function content — the
thing `cargo creusot -- -p amenable_creusot` actually needs to translate
— and everything about *finding* those proofs (the witness bridge, the
registry) lives in `amenable_std` instead, the crate that already owns
the types being proved about.

That split is legal under Rust's orphan rule via a different
justification than usual: it's `RustStdStandard<T>` (the `Self` type,
local to `amenable_std`) satisfying the "one local type" requirement
there, rather than the verifier marker (local here).

`rust_std.rs` holds the actual harness functions; `witness.rs` holds the
trait/marker definitions `amenable_std` implements against.

## Coverage

**93 harness registrations** in `rust_std.rs`. Most carry a real,
machine-checked Pearlite `requires`/`ensures` contract, discharged by SMT
via `cargo creusot prove` — confirmed by actually running the prover
locally, not just the translator. Where `creusot-std` has no contract
surface for the underlying std call (or the call touches OS-backed state
Creusot can't reason about at all), the harness states the same claim
under an explicit `#[trusted]` boundary instead of quietly claiming
coverage it doesn't have.

`std::os::windows::*` types can't even be named here at all —
`creusot-rustc` has no Windows target, and its whole-crate translator
can't tolerate the `inventory::submit!`-based witness wiring this crate's
proof functions need to stay free of regardless (see above). That
cluster's four harnesses instead prove a law over a synthetic
`isize`/`u64`/`u32` model — real and fully Creusot-proved, not a
`#[trusted]` stub — with the evidence hand-linked to the real types'
registrations by string in `amenable_std::creusot_witness`, mirroring
`amenable_kani::os_windows_model`'s identical bypass for the identical
reason (Kani/CBMC don't run on Windows either).

## See also

- [Root README](../../README.md) for the project-wide overview and
  verifier summary.
- [`amenable_std`](../amenable_std/README.md) for the registry these
  witnesses bridge into, and overall coverage across all three verifiers.
- [`amenable_kani`](../amenable_kani/README.md),
  [`amenable_verus`](../amenable_verus/README.md) for the other two
  backends.
