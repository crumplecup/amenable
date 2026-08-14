# Verus Derive-Witness Composition Plan

## Status

🔲 In progress — Phase 1 (`ClassifiedWitness<V>` + export-time
enforcement + the `#[allow(dead_code)]` fix) is implemented and
verified: `just check-all-verus` passes in full, `just verify-verus`
still reports `335 verified, 0 errors` (unaffected — Phase 1 is
enforcement, not proof content), and the negative case (an
intentionally-unclassified leaf) was confirmed twice against the real
crate to fail `cargo check` with the expected `E0277`, naming the exact
leaf, then restored. Phases 2–8 (owned-`String` conversion, real
call-shape metadata, and the renderer rewrite) not started.

## Problem

Commits `969b460`..`0a0abd5` added a derive-witness composition and Verus
export pipeline: `#[derive(Witness)]` now also generates a `WitnessArtifact`
tree (structural closure over already-witnessed fields/variants), and
`amenable::verus_export` renders that tree into literal Verus source under
`amenable_verus/src/derived_witness/`, wired into `just check-verus`/
`verify-verus`/etc. via a new `emit-verus-witnesses` step.

The rendered proofs are tautological. A real generated file:

```rust
pub open spec fn foo_holds(foo_member_checked_checked_holds: bool) -> bool {
    foo_member_checked_checked_holds && foo_member_marker_holds()
}

pub proof fn verify_foo(foo_member_checked_checked_holds: bool)
    requires foo_member_checked_checked_holds,
    ensures foo_holds(foo_member_checked_checked_holds),
{
    assert(foo_holds(foo_member_checked_checked_holds));
}
```

Every non-trivial leaf becomes a free `bool` parameter, assumed true via
`requires`, then trivially reasserted through a spec fn whose body is
exactly that parameter. This holds for any boolean value, real claim or
not — it is `fn f(p: bool) requires p ensures p { assert(p); }` with extra
names. Confirmed against the real `verus` tool: `just verify-verus` went
from `332 verified, 0 errors` to `335 verified, 0 errors` after these
commits landed — three tautologies, indistinguishable in the output from
the 332 genuine proofs. The mechanism doesn't distinguish `Opaque`
(unclassified) leaves from `Checked`/`Trusted` ones either — both get the
same free-boolean treatment. `filtered_metadata()` in `verus_export.rs`
deliberately excludes the leaf's real `claim` (verbatim spec-fn source)
from even the generated comment, so the one piece of data that could
connect the composite to real content is discarded before rendering.

Two smaller issues riding along:

- `crates/amenable_std/src/verus_derive_canary.rs` has
  `#[allow(dead_code, reason = "...")]` on `VerusExportCanaryEnum` —
  violates this project's unconditional "never use `#[allow]`" policy.
- `VerusCheckedProof { harness: &'static str, claim: &'static str, ... }`
  and the call-shape metadata this plan adds hold string data directly as
  struct fields typed `&'static str` — commits the struct to only ever
  being constructed from compile-time literal content.

## Goals

1. **Real composition.** A composite type's generated Verus proof must
   depend on its real leaf proofs actually holding — calling the real
   leaf harness function or citing the real leaf spec-fn predicate by
   name — not on an assumed free boolean. Breaking a leaf's real proof
   must break the composite's generated proof.
2. **Opaque leaves cannot compose in — enforced as a real compile error**,
   not a runtime `Result::Err` (only discoverable by running a separate
   CLI tool) and not a `const`-eval panic (`E0080`, the only failure mode
   `const` evaluation has — verified below). Field-level precision: the
   error must name the actual unclassified leaf, not just the top-level
   exported type.
3. **No new `&'static str` struct fields.** Methods and function-pointer
   registry fields returning `&'static str` are unaffected (e.g.
   `ContractRecord.fragment: fn() -> &'static str` stays as-is — nothing
   is stored there, it's produced on demand). Fields that hold string
   data as part of a struct's own representation get owned `String`.
4. **No `#[allow]`.** Fix `verus_derive_canary.rs`'s dead-code root cause
   for real.

No narrowing of scope: composition must eventually handle every clause
shape a real leaf harness can have, not just the simplest one.

## Design

### A. Opaque enforcement: `ClassifiedWitness<V>` marker trait

Move `Witness<V>::support()` from a runtime method to something the type
system can already see the type has classified — an ordinary marker
trait, propagated through composition exactly the way
`amenable_derive::witness::add_witness_bounds` already propagates the
existing `Witness<__Verifier>` bound onto every field type.

```rust
trait ClassifiedWitness<V: Verifier>: Witness<V> {}
```

- Every real classification implements it: `bridge_verus_witness!`
  (used for every `VerusWitness`-backed type in `amenable_std`) emits
  `impl ClassifiedWitness<VerusVerifier> for $ty {}` alongside the
  `Witness<VerusVerifier>` impl it already generates. Any hand-written
  `impl Witness<V> for X` that genuinely classifies (`Checked`/
  `Trusted`/`Mixed`) does the same.
- `#[derive(Witness)]` propagates it structurally: the generated
  `impl<__Verifier, ...> ClassifiedWitness<__Verifier> for Composite<...>`
  gets one `where FieldType: ClassifiedWitness<__Verifier>` predicate per
  field, mirroring `add_witness_bounds`. A zero-field/unit type gets an
  empty where-clause and the impl applies unconditionally — correct,
  since an empty composition is genuinely `Trivial`, not `Opaque`, with no
  special-casing needed.
- `register_witness_exports!`/`emit_verus_witnesses!` requires
  `$ty: ClassifiedWitness<$verifier>` as an ordinary bound.

Verified with a standalone `rustc` reproduction (not hypothetical):

```rust
trait Witness {}

#[diagnostic::on_unimplemented(
    message = "`{Self}` has no registered classification for this verifier (Opaque)",
    note = "implement `ClassifiedWitness` for it (e.g. via `bridge_verus_witness!`) before it can be composed into an exported type"
)]
trait ClassifiedWitness: Witness {}

struct CheckedLeaf;
impl Witness for CheckedLeaf {}
impl ClassifiedWitness for CheckedLeaf {}

struct StillUnclassifiedLeaf;
impl Witness for StillUnclassifiedLeaf {} // deliberately no ClassifiedWitness impl

struct Composite<A, B> { a: A, b: B }
impl<A: Witness, B: Witness> Witness for Composite<A, B> {}
impl<A: ClassifiedWitness, B: ClassifiedWitness> ClassifiedWitness for Composite<A, B> {}

type MyComposite = Composite<CheckedLeaf, StillUnclassifiedLeaf>;

fn assert_exportable<T: ClassifiedWitness>() {}
fn main() { assert_exportable::<MyComposite>(); }
```

produces:

```text
error[E0277]: `StillUnclassifiedLeaf` has no registered classification for this verifier (Opaque)
  |
  = note: implement `ClassifiedWitness` for it (e.g. via `bridge_verus_witness!`) before it can be composed into an exported type
note: required for `Composite<CheckedLeaf, StillUnclassifiedLeaf>` to implement `ClassifiedWitness`
note: required by a bound in `assert_exportable`
```

A genuine `E0277` trait-resolution failure — `cargo check` fails, exit 1,
no binary produced, no `panic!`/`assert!`/`const`-eval involved anywhere,
and it names the *actual offending leaf type*, not just the top-level
composite. This works for arbitrarily deep and/or generic composition
because it's ordinary bounded generics, which Rust has never restricted
(unlike the const-generic-dispatch alternatives explored and rejected
below).

Two mechanisms considered and rejected during design, kept here so they
aren't re-derived:

- **`AmenableResult::Err` from `write_verus_witness_modules`** (the
  original proposal): only surfaces when someone manually runs
  `cargo run -p amenable -- emit-verus-witnesses`; silent otherwise.
  Rejected — doesn't fail `cargo check`, doesn't fail `cargo test`, easy
  to never encounter.
- **`const _: () = assert!(cond, msg);`** in the `register_witness_exports!`
  macro: does fail `cargo check` (verified with a real `rustc` repro —
  `error[E0080]: evaluation panicked: <message>`), and `concat!`/
  `stringify!` can build a real per-type message. But `const` evaluation
  has exactly one failure channel — a panic-shaped `E0080` — verified by
  also trying `Result`: `const CHECK: Result<(), &str> = Err("...")`
  **compiles successfully** (exit 0, just an unrelated "never used"
  warning) unless something later `.unwrap()`s/`panic!`s on it, which
  collapses back to the same `E0080`. `Result` adds indirection with no
  new capability in a `const` context. The bigger problem: an attempt to
  make the equivalent check work *generically* (inside
  `#[derive(Witness)]`'s own generated code, over still-unresolved field
  type parameters) hits a real stable-Rust wall —
  `error: generic parameters may not be used in const operations`
  (`generic_const_exprs`, still unstable) — confirmed with a repro before
  being abandoned in favor of the marker-trait design, which has no such
  restriction.

### B. Real composition: call-shape metadata + renderer rewrite

`VerusCheckedProof` currently carries a harness *name* and the whole
carrier file's source (for display/audit) — enough to describe a leaf in
a comment, not enough to call it. Extend it (owned `String`/`Vec`, not
`&'static str` — see Design C) with a machine-usable call shape:

```rust
pub struct VerusCallShape {
    pub module_path: String,           // e.g. "crate::rust_std::chars_carrier"
    pub name: String,                  // real fn or spec-predicate name
    pub params: Vec<VerusParam>,       // symbolic inputs, in order
    pub requires: Vec<String>,         // named preconditions the leaf itself needs
    pub kind: VerusCallKind,           // see below
}

pub struct VerusParam {
    pub name: String,
    pub ty: String,
}

pub enum VerusCallKind {
    /// A bare `open spec fn` predicate, directly citable by name —
    /// no call needed, the composite's own spec fn just conjoins it.
    Predicate,
    /// A value-returning function: call it, bind the result, cite its
    /// real `ensures` predicate with the bound result in scope.
    Function { returns: String },
    /// A `&mut self`/`old`/`final` method on a real model type: needs a
    /// constructed receiver, a real method call, and old/final rebinding
    /// at the composite's own before/after boundary.
    Method { receiver_ty: String, mutates: bool },
}
```

Renderer categories in `amenable::verus_export`, in the order the
implementation phases below tackle them (see Implementation):

1. **Direct predicate leaves** (the common case after this session's
   contract-bound-naming work — e.g. `observed_value_matches_input`,
   `observed_pair_matches_input`): the composite's own spec fn cites the
   real predicate by name with real typed parameters. No function call,
   no assumed boolean.
2. **Value-returning function leaves** (e.g.
   `verify_char_roundtrip(value: char) -> (result: char)`): the
   generated composite proof function calls it, binds `result`, and
   cites the real `ensures` predicate with the bound value.
3. **Multiple `ensures` clauses on one leaf**: not a special case — the
   leaf's artifact carries all of them, the composite conjoins all of
   them. Falls out of (1)/(2) for free.
4. **`requires` propagation**: a leaf's precondition (itself expected to
   be a named predicate) becomes part of the composite's own `requires`,
   applied to the same parameter the composite exposes. Standard
   modular Hoare-logic composition — preconditions accumulate upward,
   postconditions accumulate downward.
5. **Mutating/model-method leaves** (`RefCell`/`Weak`-style `old`/`final`
   methods on a real model struct): construct or accept the receiver,
   call the real method, rebind `old`/`final` at the composite's own
   before/after boundary.
6. **Enum composition**: a real value only occupies one variant at a
   time, so the current flat `&&` across every variant's hypothetical
   claim is wrong independent of the tautology issue — no value can
   simultaneously be every variant. The generated proof for an enum
   composite must `match` on the constructed value and prove only the
   constructed variant's own composed claim in that arm.

### C. Owned-`String` fields

- `VerusCheckedProof.harness`/`.claim`: `&'static str` → `String`.
  `MetadataEntry::new` already takes `impl Into<String>`, so no call
  site breaks.
- `VerusCallShape`/`VerusParam` (new, Design B): all owned `String`.
- **Unaffected:** `ContractRecord.fragment: fn() -> &'static str`,
  `WitnessExportRecord.evidence/destination_module/verifier`, and any
  other registry field whose type is a function pointer. The field
  itself is a fn pointer (trivially `'static`/`Copy` regardless of what
  it returns) — nothing is stored, so there's nothing to own. These stay
  as-is; a method returning `&'static str` is not the footgun, a struct
  field holding one is.

### D. Remove `#[allow(dead_code)]`

`VerusExportCanaryEnum`'s `Balanced`/`Adjustment` variants are never
constructed at runtime — only their *shape* is inspected via derive-macro
codegen. Fix during implementation via one of CLAUDE.md's documented
paths (real construction in a test, `pub(crate)` narrowing, or
restructuring) — not another `#[allow]`.

## Implementation phases

Each phase ends with a real `verus` run (`just verify-verus`, not just
`cargo check`) and `just check-all-verus`, committed separately.

1. ✅ **`ClassifiedWitness<V>` + propagation + export-time enforcement.**
   Smallest, self-contained, immediately closes "can compose something
   unproven into an export." Also fixed the `#[allow(dead_code)]` while
   touching `verus_derive_canary.rs` in this phase (commits `58baf89`,
   `aa14160`).
2. **Owned-`String` conversion** on `VerusCheckedProof`. Mechanical,
   unblocks phase 3 starting clean.
3. **`VerusCallShape`/`VerusParam` plumbing** into every `impl
   VerusWitness`/`bridge_verus_witness!` site, starting with the canary
   leaves.
4. **Renderer: categories 1–3** (direct predicates, value-returning
   functions, multi-clause). Get the existing canaries producing real,
   content-bearing composite proofs — no free booleans anywhere.
5. **Renderer: category 4** (`requires` propagation). Add a canary
   exercising a leaf with a real precondition.
6. **Renderer: category 5** (mutating/model-method leaves). Add a canary
   wrapping a `RefCell`-style model leaf.
7. **Renderer: category 6** (enum `match`-per-variant generation).
   Rewrite the enum path; extend `VerusExportCanaryEnum` (or add a new
   canary) so at least two variants use genuinely different real leaf
   types, and verify the generated `match` arms each discharge for real.
8. **Rollout.** Regenerate `derived_witness/` from real (non-canary)
   crate registrations, `just verify-verus` across the whole tree,
   confirm zero free-boolean tautologies remain in generated output, and
   confirm attempting to export a deliberately-opaque type fails
   `cargo check` with the expected `E0277`.

## Verification plan

- Real `verus --crate-type=lib crates/amenable_verus/src/lib.rs` at every
  phase — track the "N verified" count explicitly the way this session's
  contract-bound-naming work did, as an audit trail against silent
  regressions.
- `just check-all-verus` (fmt/clippy/test across `amenable_verus`,
  `amenable_std --features verus`, `amenable --features verus`) after
  every phase.
- A real `rustc` reproduction (scratch file, not committed) before
  relying on any non-obvious language mechanism, matching the practice
  used to settle Design A above — don't assert a technique works,
  demonstrate it.

## Affected crates

`amenable_core` (`witness.rs`: `ClassifiedWitness`, `VerusCallShape`/
`VerusParam` types), `amenable_derive` (`witness.rs`: propagate
`ClassifiedWitness`, thread call-shape data instead of just names),
`amenable_std` (`verus_witness.rs`: every `VerusWitness` impl site gets
richer, owned-`String` metadata; `verus_derive_canary.rs`: remove
`#[allow]`, add canaries per phase 5–7), `amenable` (`verus_export.rs`:
the renderer rewrite is the bulk of this work).
