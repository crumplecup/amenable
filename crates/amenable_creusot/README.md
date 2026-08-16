# amenable_creusot

> Creusot verifier backend for the `amenable` constitutional trait family.

## What this crate is

`CreusotVerifier` and the `CreusotWitness` trait are defined here — there
is only one verifier Creusot works with, Creusot, so the marker belongs
with the crate that means it. But unlike Kani and Verus, the impls
bridging `CreusotWitness`/`Witness<CreusotVerifier>` to concrete std
carriers (`RustStdStandard<T>`) live in `amenable_std` instead of here.

That split exists because `creusot-rustc`'s whole-crate translation pass
chokes on ordinary Rust infrastructure that's completely unremarkable to
plain `rustc`: a return-position `impl Trait` on a local `impl` panicked
its intrinsics-gathering pass outright (a real ICE, confirmed
empirically, not a hypothetical), `Box<dyn Iterator<..>>` as a concrete
associated-type value is rejected outright ("forbidden dyn type... dyn
support is currently minimal"), and `inventory::collect!`/`inventory::
submit!`'s generated `static` items hit "unsupported definition kind" /
"unsupported constant value" the same way. All three are avoidable with
precise `#[cfg(not(creusot))]` gating *in place* — confirmed in an
isolated probe crate, see `amenable_std::creusot_gallery`'s own
`cfg_not_creusot_gating_avoids_the_inventory_and_dyn_iterator_errors`
case — so the historical fix of relocating *all* witness-bridge/registry
code to `amenable_std` wasn't the only option, just the one applied at
the time for the ~90-carrier `rust_std.rs` surface. `stoplight.rs`
registers its own, much smaller set of `ProofRecord`s directly, gated
this way, rather than following that same relocation. This crate still
stays pure Pearlite proof-function content plus its own gated registry
entries — the thing `cargo creusot -- -p amenable_creusot` actually
needs to translate — while `rust_std.rs`'s much larger witness-bridge
surface (the witness bridge, the registry, for ~90 std carriers) still
lives in `amenable_std` instead, unchanged for now.

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
