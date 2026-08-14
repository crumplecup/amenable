# Verus Derive-Witness Composition Plan

## Status

🔲 In progress — Phases 1–7 implemented and verified.

Phases 1–3 each left proof content unchanged (`just verify-verus`
stayed at `335 verified, 0 errors` throughout): `ClassifiedWitness<V>`
with export-time enforcement and the `#[allow(dead_code)]` fix
(commits `58baf89`/`aa14160`); owned-`String` conversion on
`VerusCheckedProof` (commit `eb21da0`); the `VerusCallShape`
call-shape registry, redesigned during implementation as a separate
additive registry rather than a `VerusCheckedProof` field (commit
`3e7b17c`; see Design B).

**Phase 4 is the core deliverable — real composition, no more free
booleans** (commit `dbb0092`): `just verify-verus` went to `334
verified, 0 errors` — down from 335, correctly: the three tautological
canary proofs are gone, replaced by two genuine ones that call real
Verus code and would break if that code's real behavior changed. Two
design gaps only the real `verus` tool surfaced during this phase
(Verus `ensures` clauses can't reference body-local `let`s, only
parameters and the declared return binding; spec predicates need an
explicit `#[cfg(verus_keep_ghost)]`-gated `use`) are documented in the
file's own module doc comment. Also fixed a real, independent bug
found while wiring this up: `write_verus_witness_modules` aborted its
entire batch at the first failing export — since `inventory`
registration is process-global, one broken registration anywhere would
silently starve every other, unrelated, working export in the same
process; every export now renders independently.

Phase 5 (`requires` propagation, commit `874904b`) needed no renderer
changes — category 4 was already generic in Phase 4's implementation —
just a real canary (`EscapeAscii`'s harness, the first registered one
with a genuine precondition) proving the existing propagation path
against real data instead of an empty list. `just verify-verus` went
to `335 verified, 0 errors` — up from 334, one new genuine proof.

**Phase 6 overturned its own premise** (commit `21edd94`): "mutating/
model-method leaves need receiver construction and `old`/`final`
translation" turned out to be wrong — every real registered harness,
including `RefCell`'s, is already a plain value-returning function;
its `&mut self`/`old`/`final` methods are purely internal to that one
harness's body, never independently composed. `VerusCallKind::Method`
was never needed. The real gap `RefCell`'s harness exposed instead: its
own `ensures` mixes raw tuple-field projections with a named-predicate
citation whose own argument is a projection-and-cast
(`result.5 as int`) — neither fits a structured `predicate(args)`-only
citation. Replaced `VerusPredicateCite`/`VerusCiteArg` with plain
`$placeholder` text templates (`VerusCallShape.requires`/`.ensures:
Vec<String>`), which handle raw expressions and bare calls uniformly.
Also found and fixed a second real gap: an import's real defining
module isn't always the harness's own (`observed_value_matches_input`
is defined in `primitive_shapes_carrier`, only privately `use`d by
`ref_cell_carrier` — importing it via the harness's module failed with
a real `E0603`). `VerusImport` now carries its own `module_path` per
entry. `just verify-verus` went to `336 verified, 0 errors` — up from
335, one new genuine proof.

**Phase 7 (enum `match`-per-variant composition, commit `b8ccaa8`)**:
`render_enum_module` adds a synthetic local selector enum and a
synthetic local result enum, with a real `match selector { ... }` in
both the body and `ensures` — see Design E. Reinstated
`RouteSegment::Variant` (left as a Phase-4-6 TODO) so sibling variants'
same-named params disambiguate correctly, and generalized
`PendingClause`'s `$result` substitution (`render_with`) to resolve to
a locally bound match-arm name instead of a top-level `result`/
`result.N` projection. Found a real bug via the real `verus` tool: an
artifact variant's own `.name` can carry a provenance rename (e.g.
`fallback`, lowercase) — fine as an audit label, not a valid PascalCase
Rust enum variant identifier; normalized before use in the synthetic
types. `just verify-verus` went to `337 verified, 0 errors` — up from
336, one new genuine proof.

Phase 8 (full rollout) not started.

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
a comment, not enough to call it. **Implemented as a separate, additive
registry, not a field on `VerusCheckedProof`** (a refinement made during
Phase 3, not the original plan): `VerusCheckedProof` has ~280
construction sites in `verus_witness.rs`, almost none of which are
opted into Verus export (`register_witness_exports!` is deliberately
opt-in — see its own doc comment; today only the 3 canary types are
exported at all). A required new field on `VerusCheckedProof` would
force touching every one of those 280 sites for no immediate benefit,
since the renderer only ever needs a real call shape for harnesses
actually reachable from an exported type. Instead:

```rust
pub struct VerusCallShape {
    pub module_path: String,       // e.g. "crate::rust_std::char_carrier"
    pub name: String,              // real fn name
    pub params: Vec<VerusParam>,   // symbolic inputs, in order
    pub requires: Vec<String>,     // $placeholder templates -- see below
    pub ensures: Vec<String>,      // $placeholder templates
    pub imports: Vec<VerusImport>, // real (module_path, name) pairs needing a `use`
    pub kind: VerusCallKind,       // always Function { returns } in practice so far
}

pub struct VerusParam {
    pub name: String,
    pub ty: String,
}

pub struct VerusImport {
    pub module_path: String,  // the predicate/spec-fn's OWN defining module --
    pub name: String,         // not necessarily the harness's own (see Phase 6)
}

pub enum VerusCallKind {
    Predicate,                        // a bare open spec fn, no call needed
    Function { returns: String },     // the only kind any real harness needs so far
}

/// Additive, opt-in, keyed by harness name — mirrors
/// amenable_core::WitnessExportRecord's own opt-in registration story.
pub struct VerusCallShapeRecord {
    pub harness: &'static str,
    pub call_shape: fn() -> VerusCallShape,
}
```

**This is the Phase 6 (final) shape, not the original design.** The
original `requires`/`ensures` were `Vec<VerusPredicateCite>` — a
structured `{ predicate: String, args: Vec<VerusCiteArg> }` (`VerusCiteArg`
= `Result` or `Param(name)`), assuming every clause is a bare
`predicate(args)` call. That held for `char_roundtrip`/`escape_ascii`,
but broke on `RefCell`'s real harness (see Phase 6 in Status above):
its `ensures` mixes raw tuple-field projections (`result.0`,
`!result.1`, ...) with a call whose own argument is a projection-and-
cast (`result.5 as int`), neither of which a bare-call-only structure
can express. `requires`/`ensures` became plain text templates instead
— the harness's own real clause text, verbatim, with `$result`/
`$paramname` placeholders — which the renderer substitutes without
ever needing to parse the clause's grammar.

`register_verus_call_shape!` registers one entry per harness name (not
per `Witness`-impl type — several types can share one harness, e.g.
`RustStdStandard<char>`/`ValidUnicodeScalar`/the canary's
`CheckedVerusExportLeaf` all reuse `verify_char_roundtrip`). The
renderer looks a leaf's harness up via `verus_call_shape(name)`; a
`None` result for a harness actually reached from an exported type is a
real, well-defined tool error (an `AmenableResult::Err` from the
renderer, not a silent fallback to the old free-boolean behavior) — a
qualitatively smaller and more honest gap than the `Opaque` case Design
A closes, since it only means "this already-classified leaf hasn't
been wired into the composition renderer yet," not "this leaf has no
proof at all."

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
5. **Leaves whose `ensures` mixes raw expressions with named-predicate
   citations** (e.g. `RefCell`'s harness: raw tuple-field projections
   like `result.0`/`!result.1` alongside a named-predicate citation
   whose own argument is itself a projection-and-cast,
   `observed_value_matches_input(result.5 as int, updated as int)`):
   not a distinct call shape — every real registered harness, including
   this one, is a plain value-returning function, so it falls out of
   (2). What's distinct is only the citation text: it doesn't fit a
   structured `predicate(args)`-only representation, so `requires`/
   `ensures` are plain `$placeholder` text templates
   (`$result`/`$paramname` substituted at render time) copied verbatim
   from the real Verus source, handling raw expressions and bare calls
   uniformly. (Phase 6 originally planned this category as
   "mutating/model-method leaves" needing receiver construction and
   `old`/`final` translation — investigation of the real harnesses
   showed no such method-shaped call is ever registered, so
   `VerusCallKind::Method` was dropped from the design.)
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

### E. Enum composition: selector param + `match`, unified result enum

Struct/tuple-struct composition synthesizes a flat parameter list and a
plain tuple return type from the union of its checked leaves' own real
harness parameters/returns (never the real struct's own fields) — a
single function, single signature, single body expression. An enum's
variants can each carry a *different* set of checked leaves, so there
is no single tuple type that fits every variant; some case-split is
unavoidable. Three shapes were considered:

- **Take the real enum value, spec-fn only.** Truest to "the
  constructed value" as an actual runtime value — match on a real
  `MyEnum` parameter, cite each variant's own predicates. Rejected:
  Verus spec fns can't call exec fns, so this only works when every
  leaf is already a bare predicate (category 1); it can't express
  today's checked leaves, which are value-returning function calls
  (category 2) — most of what's registered so far.
- **One independent function per variant.** No selector, no `match`
  keyword: emit N sibling functions, each exactly like today's
  struct-composition renderer scoped to one variant's own members.
  Simplest to implement, and structurally sound (each proof only ever
  concerns one variant, so nothing is claimed across variants) — but
  doesn't literally satisfy the "match on the constructed value" shape
  the Problem/Goals sections call for, and splits one artifact export
  into N unrelated top-level functions with no single point that states
  "these are mutually exclusive."
- **Selector param + `match`, unified result enum (chosen).** One
  function. A synthetic local selector enum (one unit variant per real
  artifact variant name) is added as a parameter alongside the union of
  every variant's own params (`NameAllocator`, already collision-safe
  per route, now keyed by a reinstated `RouteSegment::Variant(name)` —
  the exact gap the renderer's own leftover comment flagged). A
  synthetic local result enum (one variant per real artifact variant,
  payload = that variant's own checked-call tuple, no payload if it has
  none) is the return type. The body is a real `match selector { ... }`
  building the matching arm's own result variant; `ensures` is a real
  `match selector { ... }` over `match result { ... }` per arm, each arm
  citing exactly that variant's own composed claim and `false`/`true`
  for the mismatched-arm fallback. This is the literal match-per-variant
  structure the plan calls for, generalizes the existing per-leaf
  `PendingClause`/`NameAllocator` machinery rather than replacing it
  (the `$result` placeholder now resolves to a locally bound name like
  `r`/`r0`/`r1` inside a variant's own `match result` arm, instead of
  `result`/`result.N` at the function's top level — same substitution
  mechanism, different reference string), and keeps every variant's own
  requirement (never claim a property of a variant that wasn't
  constructed) enforced by the type system itself: only the `match
  result` arm matching the current `selector` arm is ever reachable.

Scope: only the export's own root shape may be `Enum` — an `Enum`-shaped
*member* nested inside a struct or another enum variant still hits the
existing "not supported yet" error. No real nested-enum type is
registered anywhere in this codebase yet, so building the (materially
harder — nested selector/result types, nested match) general case would
be speculative; add it if and when a real one shows up.

## Implementation phases

Each phase ends with a real `verus` run (`just verify-verus`, not just
`cargo check`) and `just check-all-verus`, committed separately.

1. ✅ **`ClassifiedWitness<V>` + propagation + export-time enforcement.**
   Smallest, self-contained, immediately closes "can compose something
   unproven into an export." Also fixed the `#[allow(dead_code)]` while
   touching `verus_derive_canary.rs` in this phase (commits `58baf89`,
   `aa14160`).
2. ✅ **Owned-`String` conversion** on `VerusCheckedProof`. Mechanical,
   unblocked phase 3 starting clean (commit `eb21da0`).
3. ✅ **`VerusCallShape`/`VerusParam` registry**, redesigned during
   implementation as a separate additive registry rather than a
   `VerusCheckedProof` field (see Design B above for why). Registered
   the canary's harness, `verify_char_roundtrip` (commit `3e7b17c`).
   Remaining harnesses get registered as later phases' canaries need
   them, or in phase 8's rollout.
4. ✅ **Renderer: categories 1–3** (direct predicates, value-returning
   functions, multi-clause). Get the existing canaries producing real,
   content-bearing composite proofs — no free booleans anywhere
   (commit `dbb0092`). Implemented as category 2 (value-returning
   functions) directly, since `verify_char_roundtrip` already covers
   category 1's shape by being called and cited rather than assumed;
   category 3 (multi-clause) fell out for free, no special-casing
   needed. Also fixed `write_verus_witness_modules`'s all-or-nothing
   failure mode (see Status above) — not originally scoped to this
   phase, but discovered to be necessary while wiring up the first two
   real exports side by side with a temporarily-unsupported one.
5. ✅ **Renderer: category 4** (`requires` propagation). Registered
   `EscapeAscii`'s real harness (a genuine precondition,
   `escape_ascii_input_is_printable_ascii`) and a canary exercising it
   — the propagation logic was already generic from Phase 4, so no
   renderer changes were needed, only the real data to prove it against
   (commit `874904b`).
6. ✅ **Renderer: category 5** (`ensures` mixing raw expressions with
   named-predicate citations). Added a canary wrapping `RefCell`'s real
   harness. Originally scoped as "mutating/model-method leaves"
   (`VerusCallKind::Method` with receiver construction and `old`/
   `final` translation), but investigation of the real harness showed
   every registered call, including this one, is already a plain
   value-returning function — no method-shaped call is ever registered,
   so category 2's call handling already covered it. The real gap was
   citation *text*: `result.0`/`!result.1`/`result.5 as int` don't fit
   a structured `predicate(args)`-only representation. Replaced
   `VerusPredicateCite`/`VerusCiteArg` with plain `$placeholder` text
   templates. Also fixed a second real gap found via `verus` itself: an
   import's defining module isn't always the harness's own module
   (`observed_value_matches_input` lives in `primitive_shapes_carrier`,
   only privately `use`d by `ref_cell_carrier`); `VerusImport` now
   carries its own `module_path` per entry (commit `21edd94`).
7. ✅ **Renderer: category 6** (enum `match`-per-variant generation, see
   Design E). Enabled a concrete `VerusExportCanaryEnum` instantiation
   in `emit_verus_witnesses!` (its `Balanced`/`Adjustment` variants
   already mix a checked and a trusted leaf vs. a trusted-only leaf —
   genuinely different real leaf types) and added a second, two-variant
   fixture directly in `amenable/tests/verus_export_test.rs`
   (`LocalWorkingEnumEvidence`) asserting the generated `match` content
   verbatim. The real `match` arms discharge for real: `just
   verify-verus` went to `337 verified, 0 errors` (commit `b8ccaa8`).
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
