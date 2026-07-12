# AMENABLE_PLAN.md

## Goal

`amenable` is the foundational, dependency-light crate defining the trait
family for lawful proof-carrying software structure. It is upstream of every
framework that consumes it. Formal verification does not depend on
elicitation, or on any other conversational or domain framework — those
frameworks depend on formal verification. `amenable` exists to make that
dependency direction explicit and enforceable in code.

This started as an incubation module inside `elicitation` (see git history
for `crates/elicitation/src/amenable.rs`), explored as a parallel track
against `elicitation`'s existing `Prop`/`Established<P>`/`ProvableFrom<C>`
machinery. That incubation phase is over. The trait family proved out and has
been relocated to its own crate and repository. The elicitation-specific
bridge code that adapted the constitutional traits onto `Prop` and
`VerifiedStateMachine` was deleted rather than migrated — it inverted the
dependency the wrong way, and any future bridge from `elicitation` into
`amenable` belongs in `elicitation`, not here.

## Status

- `amenable` is a Cargo workspace, not a single crate. See "Workspace
  Architecture" below for the full layout, the orphan-rule constraint that
  drove it, and the planned `amenable_derive` proc-macro crate that will
  host derive support such as `#[derive(Provenance)]`.
- Core trait family (`Verifier`, `Witness`, `Witnessed`, `Evidence`,
  `Standard`, `Objective`, `AsStandard`, `AsObjective`, `ProofToken`,
  `Sidecar`, `Establish`, `Exchange`, `StateMachine`, `Amenable`,
  `Provenance`, `MetadataEntry`) lives in `amenable_core`, split into focused
  modules, zero runtime dependencies. The top-level `amenable` facade crate
  re-exports it (plus `amenable_std`) for user convenience.
- Current code still carries separate `Standard` and `Objective` root roles,
  but the next design pass is expected to collapse them into a single
  `Standard` obligation category whose source distinction lives in
  `Provenance`, not in sibling root traits.
- `RustStdType` (trait and its full std-lib registrations, necessarily
  together — see Workspace Architecture) lives in `amenable_std`.
- `amenable_kani`/`amenable_creusot`/`amenable_verus` are scaffolded stub
  crates with no content — the concrete `WitnessSource<V>` proof-emission
  machinery per backend is Phase 4 work, not started.
- Source-code capture (`Code`/`Fragment`) has moved out of this workspace
  entirely, into its own repository, `homecoming` — see "Relationship to
  `homecoming`" below. It is not a dependency of `amenable` yet.
- `elicitation` no longer contains any `amenable`-adjacent code. It has not
  yet been updated to depend on `amenable`.

## Workspace Architecture

`amenable` is a workspace of six crates, driven by two forces: a per-role
split so each concern gets its own dependency-scoped home, and a hard Rust
constraint that overrides the split wherever it applies.

**The orphan-rule constraint.** Rust forbids `impl ForeignTrait for
ForeignType` — at least one of the trait or the type must be local to the
crate doing the `impl`. Any trait meant to be implemented *directly* on a
standard-library type (`bool`, `i32`, `String`, ...) therefore cannot be
split across an interface crate and a downstream consumer crate the way
`elicitation`'s shadow-crate pattern normally works: the trait and its
std-lib registrations have to live in the same crate, because no other crate
can ever legally supply the `impl`. This is not a style preference — it is
enforced by the compiler and discovered the hard way (see the abandoned
`amenable_std`-as-downstream-consumer attempt in git history, which failed
to compile with `E0117` for exactly this reason). This constraint has a
second-order effect: it justifies a crate-per-trait architecture for any
trait carrying std-lib coverage, since each such crate ends up owning a
genuinely large proof/registration surface for the standard library on top
of its trait definition.

The layout:

- **`amenable_core`** — the constitutional trait family that is *not*
  implemented directly on foreign types (`Verifier`, `Witness`, `Evidence`,
  `Standard`, `Provenance`, `Certificate`, `Registry`, `Sidecar`,
  `Establish`, `Exchange`, `StateMachine`, `Amenable`). It owns the
  abstract trait interfaces only, not the default concrete certificate or
  registry implementations. Zero runtime dependencies.
- **`amenable_std`** — `RustStdType`: interface and its complete std-lib
  registrations together, per the orphan-rule constraint above, plus the
  default concrete `Certificate`/`Registry` implementation and the local
  newtype wrappers used to promote supported std-lib carriers into the
  `Standard` role lawfully.
- **`amenable_kani`** / **`amenable_creusot`** / **`amenable_verus`** —
  concrete `WitnessSource<V>` proof-emission machinery per verifier backend.
  Depend on `amenable_core` directly, never on the facade.
- **`amenable`** — the top-level facade, re-exporting `amenable_core` and
  `amenable_std` (and, once they have content, the verifier-backend crates)
  for user convenience. This is the single sanctioned exception to "no
  re-exports between workspace crates" — see `CLAUDE.md`'s Workspace
  Organization section. Satellite crates depend on `amenable_core`, never on
  this facade, to avoid a circular dependency.

Source-code capture (`Code`/`Fragment`, the lateralizing composition traits)
was scaffolded here as `amenable_code`, then split out entirely into its own
repository, `homecoming` (`github.com/crumplecup/homecoming`) — see
"Relationship to `homecoming`" below. It is not part of this workspace.

Verified end to end: a throwaway smoke-test crate depending only on
`amenable` successfully called `amenable::KaniVerifier::metadata()` and read
a fact through the re-exported `Provenance` interface, confirming the facade
re-exports are load-bearing, not just declared.

## Architectural Thesis

The strongest interpretation of `amenable` is constitutional rather than
behavioral.

Its traits primarily define legal roles and admissibility criteria inside a
proof economy:

- which types are permitted to serve as trusted roots
- which types may count as derived evidence
- which exchanges are lawful
- which workflows are closed under those exchanges
- which seams must carry proof sidecars explicitly

At this stage, the trait bounds themselves matter more than a fully enumerated
method surface. Methods should emerge from real boundary pressure, not be
invented ceremonially.

## Relationship to `elicitation`

The prior plan treated `elicitation`'s `Prop`, `Established<P>`,
`ProvableFrom<C>`, and `VerifiedStateMachine` as fixed primitives that
`amenable` should map onto. That framing is retired.

`elicitation`'s proof machinery is reference material, not a template to
port. It grew up ad hoc, inside a framework whose primary job is
conversational elicitation, not proof discipline. Its shape reflects that
history — including gaps that let weak or vacuous proof implementations pass
silently. Rebuilding this surface in `amenable` is an upgrade to a cleaner,
unified interface, not a scavenge-and-relocate exercise. The trait family and
narrative in [amenable.md](amenable.md) are the design target; `elicitation`'s
existing code is consulted for what worked and what didn't, not copied.

The eventual shape:

- `amenable`: constitutional law for proof-bearing software structure, plus
  the concrete proof-emission and evidence machinery that today lives
  ad hoc inside `elicitation`
- `elicitation`: a consumer of `amenable`, providing conversational
  elicitation of strongly-typed values; its proof-carrying types are built on
  top of `amenable`'s traits rather than defining their own parallel scheme
- interface crates such as `elicit_temporal`: domain law anchored in external
  standards, expressed through `amenable`'s `Standard` role plus
  source-specific `Provenance`

As `amenable` absorbs proof infrastructure, the corresponding code in
`elicitation` (`Prop`, `kani_proof`/`creusot_proof`/`verus_proof` glue,
`Established<P>`, `ProvableFrom<C>`, `FormalMethod`, `VerifiedStateMachine`)
becomes redundant and should shrink or disappear, simplifying that workspace.
This is expected to be a substantial migration once it is underway — that is
a sign the extraction is real, not a reason to avoid it.

## Design Constraints

The crate should remain strict on structure and light on dependency burden.

- no unnecessary runtime dependencies
- no backend-specific verifier coupling in the core trait family
- no policy specific to one domain such as time, GIS, or UI
- no erosion of the explicit sidecar token pattern
- no reinvention of concrete time, geometry, or workflow payload types

The crate must earn its abstraction budget. A trait belongs in `amenable` only
if it tightens one of:

- auditability
- lawful proof exchange
- closure under composition
- separation between trusted roots and derived evidence

## Proposed Trait Family

The trait family, as designed in [amenable.md](amenable.md), is implemented:

- `Verifier`
- `Witness`
- `Standard`
- `Objective`
- `Evidence`
- `Sidecar`
- `Establish`
- `Exchange`
- `StateMachine`
- `Amenable`

Refinement continues against the same three questions used during
incubation:

1. Does it mark a distinct constitutional role?
2. Does it clarify a real proof boundary?
3. Does it map cleanly onto the concrete proof machinery being rebuilt here?

Collapse, renaming, or removal is only justified when it preserves the
original architectural law more faithfully, not when it merely makes the
surface more convenient or superficially simpler.

## Core Design Principle: Explicit Sidecar Exchange

`amenable` states plainly that explicit proof sidecars are the canonical
exchange grammar. Every lawful step consumes a precondition token and mints a
postcondition token:

```rust
fn perform_step(
    &self,
    input: Payload,
    _pre: PreconditionToken,
) -> Result<(Output, PostconditionToken), Error>;
```

That pattern is the architectural center. It is not accidental boilerplate;
it is the governing discipline of the framework, and every downstream
consumer inherits it by depending on `amenable`.

## Core Design Principle: States Are Roots, Transitions Are Relations

A state-machine invariant splits into two categorically different kinds of
claims, and the trait family keeps that split visible rather than flattening
it into one undifferentiated notion of "proof."

**Transitions have mathematical shape.** "Yellow never transitions directly
to Green" is a relation between two states, decidable from the code that
implements the transition. A model checker can exhaustively verify it. This
is `Witness` territory: a genuine, machine-checked proof artifact.

**States do not.** Ask what the proof looks like for "the light is currently
Green" — not the transition into Green, just the bare fact of being in that
state — and there is no computation to point to. The first assertion any
running instance of the program makes ("I'm green") is not derived from
anything prior; it is asserted. This is not a gap that more proof effort
closes. It is categorically the same shape as the MUTCD dwell-time example
from the certification section below: an externally-sourced or by-design
fact the program relies on but cannot derive. A state claim is therefore
also a `Provenance` object — a `Standard` certification, subject to the same
discipline (who decided this, why, what is the citation) as any other
trusted assumption.

This mirrors the classic TLA+ specification shape, `Init ∧ □[Next]_vars`:
`Init` is an assumed starting condition, asserted rather than proven; `Next`
is the transition relation, the part a model checker actually has purchase
on. `amenable` did not invent this split — it names it, and holds
`Init`-shaped claims to the same certification discipline as any other
unprovable trust decision, rather than letting them slip through unexamined
because "it's just the state, not a real proof."

This generalizes past power-on. Any time a state is asserted rather than
arrived at through a proven transition — recovering from a checkpoint,
reading a sensor, receiving state from an external system — the same rule
applies: it is a root, and roots require `Provenance`, not a fabricated
derivation.

**Design implication for `StateMachine`/`Amenable` (not yet resolved):**
today `StateMachine` only exposes `type State` and `type Invariant`, with no
notion of a certified initial/root-state claim distinct from the
`Witness`-backed transitions `Amenable` proves over them. Whether to add an
explicit associated `Provenance`-bound root claim to `StateMachine`, versus
leaving the certification as an implementation-side convention, is open —
see Open Questions.

## Proof Quality and Certification of Provenance

This is the central discipline `amenable` exists to enforce, and the reason
the trait family is worth having at all.

### The problem: cut corners are silent by default

Proof obligations satisfied by an empty or near-empty implementation — an
emitted token stream that type-checks but asserts nothing, a `Witness` that
returns `TokenStream::new()` — are indistinguishable from real proofs at the
type level. This has happened in practice, including from automated agents
generating proof code under time or context pressure. A trait signature alone
does not stop it. Only a discipline that makes triviality checkable, and that
refuses to accept unexamined trust, closes the gap.

### Two lawful ways to discharge a claim, never a third

Every `Evidence`-bearing claim must be backed by exactly one of:

1. A genuine machine-checked proof, emitted through `Witness` and consumed by
   a `Verifier` backend.
2. An explicit `Standard` certification of provenance — a metadata-carrying
   root obligation naming the authority, source, and rationale for a human
   trust decision.

There is no third option where a claim is silently or implicitly trusted
because nobody got around to writing a proof or a certification. If a claim
cannot currently support a mathematical proof, the obligation does not
disappear — it converts into an obligation to produce a certification of
provenance instead. Relying on the Rust standard library's documented
guarantees, for example, is a legitimate trust decision, but it must be
registered as a `Standard` certification whose `Provenance` names the Rust
project as authorizing body, the relevant doc page as authoritative source,
and so on, rather than assumed for free by a blanket impl. This is the
specific failure mode the previous `elicitation`-side bridge fell into — a
blanket `AsStandard<Is<T>>: Standard` impl for every `T: RustStdType` — and
it is exactly what the certification requirement is designed to prevent:
trust without a registered record of the decision to trust.

For standard-library-backed trust decisions, the lawful carrier is a local
wrapper type in `amenable_std`, not a direct `impl Standard for i32` in that
crate. Rust's orphan rules forbid implementing the foreign `Standard` trait
from `amenable_core` directly for a foreign type like `i32` in
`amenable_std`. The design therefore uses explicit newtype carriers such as
`RustStdStandard<T>` to say "this program accepts Rust's documented
semantics for `T` as a standard," while keeping the trust decision
auditable and explicit in code.

### Direction of construction: provenance first, evidence later

The causal direction is:

`Provenance` -> `Standard` -> `Evidence` -> `Witness`

not the reverse.

`Provenance` is a metadata-carrying type. It records where a standard came
from and why that standard is trusted: the authority, source document,
clause or scope, and rationale for the code-level encoding. A `Standard`
selects and upholds that provenance-backed obligation. `Evidence` is then
produced from the standard plus the action taken to uphold it. A `Witness`
is the verifier-facing proof artifact for evidence that has mathematical
shape.

This matters because it rules out designs where `Standard` is derived from
`Evidence`, or where a root trust assumption quietly inherits certification
just by existing. Certification begins with an explicit provenance record and
only then becomes a standard the program claims to uphold.

### Proof-quality heuristics and diagnostics on `Witness`

`Witness` gains diagnostic methods for catching corner-cut proofs before they
are trusted as evidence, in addition to its existing `proof()`,
`lineage_summary()`, and `audit_surface()` methods:

- a triviality check: is the emitted proof artifact structurally empty or a
  no-op (for example, an empty token stream, or a body with no assertions
  referencing the claimed invariant)?
- a reference check: does the emitted proof artifact actually name or operate
  on the type/invariant it claims to establish, rather than being boilerplate
  copied from an unrelated proof?
- a reporting surface that downstream tooling (CI checks, review agents) can
  query directly, rather than needing to parse token streams themselves

These are heuristics, not a soundness guarantee — a determined implementer
can still write a bad proof that passes every heuristic. The goal is to raise
the cost of cutting corners and to give reviewers (human or agent) a fast,
structural signal, not to replace review.

### Structured provenance, projected predictably

The auditable object is not a loose map built by hand, and not a cluster of
special-case getter methods bolted directly onto `Standard`. The primary
object is a user-defined Rust struct or enum describing the provenance schema
itself. `Provenance` is the common projection trait over that structured
object.

The intended model:

- users define a concrete provenance type as a normal struct or enum
- the fields of that type define the authoritative provenance schema
- `#[derive(Provenance)]` in `amenable_derive` projects that schema into
  deterministic metadata entries
- a `Standard` implementation selects a concrete provenance instance and says
  "this is the obligation being upheld"

`Standard` should therefore produce provenance by value, not by borrowed
reference. That shape works for ordinary stored provenance records, but it
also works for zero-sized wrapper carriers such as `RustStdStandard<T>`,
whose provenance is computed from the wrapped type's registration rather
than stored in a field.

That keeps provenance flexible and strongly typed without sacrificing the
predictable key/value audit surface tools and reviewers need.

Enum support is first-class in this design, not optional icing. Closed
vocabularies such as authorizing body, source kind, or trust mode often want
enum structure, and different sources legitimately need different payload
fields. Derived enum provenance should therefore emit a deterministic tag plus
the variant payload fields, rather than collapsing everything into ad hoc
strings.

## Auditing as a First-Class Constitutional Concern

Auditing lives inside the very traits that define proof roles and exchange
roles, not as a bolted-on concern.

- `Provenance` audits authority, source, scope, and rationale
- `Standard` audits which provenance-backed obligation the program accepts and
  must uphold
- `Evidence` audits dependency lineage and concrete action taken to uphold the
  relevant `Standard`
- `Witness` audits proof-quality heuristics alongside the raw proof artifact
- `Exchange` audits the lawful transformation from one proof state to another,
  including the preconditions relied on and the postconditions established
- `Amenable` audits the closed set of lawful transitions and the overall
  invariant-preservation story of the system

Taken together, those trait-scoped audit surfaces explain:

- what the system believes
- why it believes it
- which authority justifies the claim
- which code path is responsible for upholding it
- how one lawful proof state became another
- whether the proof backing the claim shows signs of being cut short

Audit methods remain role-specific and jurisdiction-specific. `amenable`
should not collapse auditing into one vague metadata trait; each
constitutional role exposes the audit surface proper to that role.

## Relationship to `homecoming`

Source-code capture (originally sketched here as `Code`, in the scaffolded
`amenable_code` crate) has moved to its own repository,
[`homecoming`](https://github.com/crumplecup/homecoming). Full design
detail — the `Homecoming` trait, `Fragment` as a `syn`-typed,
`petgraph`-backed graph, the lateralizing composition traits, and the
round-trip proof obligation — lives in that repository's
`HOMECOMING_PLAN.md`, not here. `homecoming` does not depend on `amenable`
and never will; the dependency runs the other way once `homecoming`'s
design stabilizes (Phase 3 below).

Two points from that design are load-bearing for `amenable` specifically
and worth restating here rather than only in the other repo:

- **Why this matters for `Witness`, not just for agent tooling.** A
  solver's verdict is only meaningful relative to the exact statement it
  checked. The code a `Witness` proof ran over — the receipt establishing
  what, precisely, was verified — has to be *exact* to be of any value at
  all. An approximate or reconstructed rendering of "what the code probably
  looked like" breaks the chain of custody between what was verified and
  what ships, silently, in a way that looks like assurance while providing
  none.
- **`Witness::proof()` and `Homecoming::code()` must never independently
  drift apart**, once `amenable` depends on `homecoming` — a structurally
  real proof over the *wrong* code is more dangerous than an obviously
  vacuous one, because it does not look suspicious. The fix is not to
  hard-couple the two traits architecturally; it is to make their agreement
  a checkable claim, the same discipline the rest of the family runs on.
  Reconstruct a whole program from its emitted fragments, regenerate its
  proofs, and compare them to the original program's proofs: if
  round-tripping through `Homecoming` always yields the same proof, the two
  traits are provably in agreement despite being independently implemented.

Once `Homecoming` exists as a dependency, `Evidence::audit_surface()` (and
the `AuditRecord` registry below) will not need a separate
proc-macro-stringify capture mechanism of their own — they can call
`.code()` on whatever produced the claim and get the real, live fragment
back, rather than a hand-maintained string that can silently drift from the
code it describes.

## Audit Inversion: From Rule to Dependents

Auditing a `Standard` should not be limited to reading the citation forward,
from a dependent claim back to the root it names. The more useful review
question often runs the other way: given a specific rule (the MUTCD
10-second/5-second dwell-time minimum, say), show every piece of code in the
system that relies on it, for direct review — the code "inside the unsafe
block," not just a pointer to it.

Two distinct mechanisms are needed, not one:

1. **Literal code capture, not string pointers.** `Evidence::audit_surface()`
   today returns `&'static [&'static str]`, presumably identifiers or
   file/line references. `amenable.md` already asks for something stronger
   under `Evidence` — "produce the source code associated with computing the
   result." For a reviewer to judge whether the code actually upholds a
   cited rule without leaving the report, the audit surface needs to carry
   literal source text, not a name to go look up. `homecoming`'s
   `Homecoming` trait (see "Relationship to `homecoming`" above) will supply
   this directly once `amenable` depends on it — no separate
   proc-macro-stringify mechanism of our own needed; the audit surface can
   call `.code()` on whatever produced the claim.
2. **A reverse index from root to dependents.** Forward, a derived `Evidence`
   names the `Standard` root it relies on. Backward — "given this rule, show
   every dependent" — requires each dependent to register itself against the
   root it cites in a way the root can enumerate without any static
   knowledge of who depends on it. That is a compile-time registry keyed by
   the root type, collecting `(dependent identity, audit code, citation)`
   records — the shape this workspace already uses elsewhere for exactly
   this kind of static-submission/runtime-enumeration problem (an
   `inventory`-crate pattern is the leading candidate).

Any record type built for this (working name `AuditRecord`) must own its
data — `String` for dependent identity and captured audit code, not
`&'static str` — since captured source and formatted identities are
runtime-computed, not literals. See the struct-ownership rule in the design
checklist below. Once `homecoming` is a dependency, `AuditRecord`'s "audit
code" field is most naturally a captured `homecoming::Fragment` rather than
a bare `String`.

## Phased Implementation Plan

### Phase 1: Core-side incubation module — done, relocated

- [x] Add an exploratory `amenable` module inside `crates/elicitation`.
- [x] Write module-level docs describing the constitutional role.
- [x] Add the initial trait family as marker or near-marker traits.
- [x] State the sidecar exchange pattern explicitly in docs.
- [x] Relocate the core trait family to its own crate and repository.
- [x] Delete the `elicitation`-side bridge code rather than migrate it.

### Phase 2: Proof-quality heuristics and certification discipline

- [ ] Design the triviality and reference-check methods on `Witness`.
- [ ] Decide the return shape for heuristic results (bool, enum with reasons,
  structured report) — favor something a reviewing agent can act on directly.
- [ ] Write at least one concrete `Witness` implementation exercising the
  heuristics against both a real and a deliberately vacuous proof body, to
  confirm the heuristic actually distinguishes them.
- [x] Collapse `Standard` and `Objective` into a single root-obligation
  category, with the distinction between external standards and local design
  intent carried entirely by `Provenance`.
- [ ] Redesign `Evidence` around rich associated output types (`type Lineage`,
  `type Audit`) rather than a fixed `&'static str`/`&'static [&'static str]`
  reporting shape.
- [ ] Redesign `Provenance` from the current iter-only capability into a
  richer trait over concrete user-defined structs and enums, with predictable
  metadata projection plus convenience methods built from it.
- [ ] Add `amenable_derive` as a proc-macro crate hosting
  `#[derive(Provenance)]`, starting with struct support.
- [ ] Extend `#[derive(Provenance)]` to enums, including tagged variants and
  payload fields, with explicit collision rules and deterministic lowering.
- [x] Confirm the derive-based provenance model is sufficient to register a
  Rust-standard-library trust decision as a certification (the `RustStdType`
  surface in `amenable_std` is the first proving ground).
- [x] Decide the lawful std-lib `Standard` carrier shape: use explicit local
  wrapper/newtype carriers in `amenable_std` (for example,
  `RustStdStandard<T>`) rather than trying to implement the foreign
  `Standard` trait directly for foreign std-lib types in that crate.
- [ ] Reject any blanket impl of `Standard`, `Evidence`, or `Provenance` that
  would let a type earn certification without an explicit, per-type
  registration.
- [ ] Decide whether `StateMachine` gains an explicit associated
  `Provenance`-bound root/init claim (see "States Are Roots, Transitions Are
  Relations" above), distinct from the `Witness`-backed transitions
  `Amenable` proves over.
- [x] Redesign `Provenance` away from a fixed collection type and toward a
  capability interface. The current `fn iter(&self) -> impl Iterator<Item =
  MetadataEntry>` surface is the transitional form; the next step is to make
  that interface derive-friendly over concrete user-defined schemas rather
  than freezing `iter()` itself as the end-state API.
- [x] Change `MetadataEntry`'s fields from `&'static str` to owned `String`,
  with a `new(impl Into<String>, impl Into<String>)` constructor.
- [x] Keep the verifier metadata marker structs (`KaniVerifierMetadata`,
  `CreusotVerifierMetadata`, `VerusVerifierMetadata`) zero-sized despite the
  move to owned `MetadataEntry`, by yielding owned entries lazily from a
  `const` slice inside `iter()` rather than storing a field — the "promise"
  of provenance stays zero-sized; the data materializes only on request.
- [ ] Design the `AuditRecord` registry for rule-to-dependents audit
  inversion (see "Audit Inversion: From Rule to Dependents" above): literal
  source capture plus a compile-time registry keyed by `Standard` root type.

### Phase 3: Depend on `homecoming`

Not started; blocked on `homecoming`'s own design stabilizing (`Fragment`,
`Homecoming`, and the lateralizing composition traits — see that
repository's `HOMECOMING_PLAN.md`). Tracked here only from `amenable`'s side
of the dependency.

- [ ] Add `homecoming` as a dependency once its `Homecoming` trait and
  `Fragment` type exist.
- [ ] Decide which `amenable` traits implement or require `Homecoming`
  (`Witness` is the leading candidate).
- [ ] Decide whether `Evidence::audit_surface()` should be redefined in
  terms of `Homecoming::code()` now that the capability exists elsewhere,
  superseding the current fixed audit-reporting shape.
- [ ] Exercise the strong-form round-trip check (reconstruct a program,
  regenerate its proofs, compare to the original program's proofs) against
  at least one `Witness`-bearing example, once both `WitnessSource` (Phase 4
  below) and `Homecoming` exist.

### Phase 4: Proof-emission upgrade

Rebuild the concrete proof-emission surface — the successor to
`elicitation`'s `Prop::kani_proof`/`creusot_proof`/`verus_proof` — as a
`WitnessSource` implementation story in `amenable_kani`/`amenable_creusot`/
`amenable_verus`, informed by but not copied from the existing `elicitation`
code.

- [ ] Decide the shape of the leaf proof-emission trait(s) implementers
  write against (successor to `Prop`).
- [ ] Rebuild the `Established<P>`-equivalent proof-sidecar token as a
  concrete `ProofToken`/`Sidecar` implementation.
- [ ] Rebuild the `ProvableFrom<C>`-equivalent exchange relation as a
  concrete `Establish<C>` implementation.
- [ ] Rebuild the `VerifiedStateMachine`-equivalent closed-world story as a
  concrete `Amenable` implementation.
- [ ] Confirm `Witness::proof()` for a concrete example can be validated
  against `Homecoming::code()` for the same example via the strong-form
  round-trip check from Phase 3.

### Phase 5: `elicitation` becomes a consumer

- [ ] Add `amenable` as a dependency of `elicitation`.
- [ ] Migrate `elicitation`'s proof-carrying types onto `amenable`'s traits.
- [ ] Delete the superseded `elicitation`-local proof machinery
  (`Prop`, `Established<P>`, `ProvableFrom<C>`, `FormalMethod`,
  `VerifiedStateMachine`) once nothing depends on it, simplifying that
  workspace.
- [ ] Confirm the elicitation test suite and verification coverage checks
  still pass against the new foundation.

### Phase 6: Downstream proving grounds

- [ ] Re-evaluate whether `elicit_temporal` is the best first external
  proving ground for `Standard` registrations anchored in an external spec
  (ISO 8601 / RFC 3339 / IXDTF).
- [ ] Adopt only the smallest useful subset of the family in a downstream
  crate at first.

## Design Checklist

- [x] Every trait has a distinct constitutional role.
- [x] No trait exists solely to rename an existing concept without adding
  clarity.
- [x] The crate remains dependency-light (zero runtime dependencies today).
- [x] The sidecar token pattern is explicit and central.
- [ ] Domain-neutral law stays in `amenable`; domain-specific law stays in
  interface crates.
- [ ] Backend-verifier coupling stays out of the constitutional core unless it
  is proven necessary.
- [ ] Every `Evidence`-bearing claim is backed by either a `Witness` proof or
  a `Standard` certification — never a blanket impl that grants
  trust for free.
- [ ] `Witness` proof-quality heuristics catch at least the vacuous-proof
  failure mode that motivated adding them.
- [ ] `Amenable` explains a rebuilt `VerifiedStateMachine`-equivalent as one
  concrete realization of the constitutional pattern rather than as a
  separate or rival design.
- [ ] Auditing is built into the constitutional roles themselves rather than
  bolted on later as an afterthought.
- [ ] Audit methods remain role-specific and jurisdiction-specific while the
  underlying provenance projection stays deterministic and predictable.
- [ ] `elicitation`'s proof machinery shrinks as `amenable` absorbs it, rather
  than the two surfaces persisting in parallel indefinitely.
- [x] Struct fields holding runtime-populated data own their data (`String`,
  `Vec<T>`); `&'static str`/`&'static [T]` is reserved for no-`&self` trait
  methods describing a fixed, compile-time-known fact about a type.
- [ ] Every state a `StateMachine` claims to occupy is backed by `Provenance`
  (a `Standard` root), the same as any other claim with no
  mathematical shape to prove — states are never silently exempted from the
  certification requirement just because they "aren't really proofs."
- [x] Traits meant to be implemented directly on foreign standard-library
  types live in the same crate as their std-lib registrations, never split
  across an interface crate and a downstream consumer — the orphan rule
  leaves no other option, discovered the hard way via `E0117`.
- [ ] Derived provenance stays flexible at the schema level (user-defined
  structs and enums) while remaining predictable at the reporting level
  (deterministic tag/field lowering and stable key/value projection).
- [ ] `Witness` and `homecoming::Homecoming` are never allowed to
  independently drift apart on the same claim, once `homecoming` is a
  dependency — verified via the strong-form round-trip check (reconstruct,
  re-verify, compare proofs), not by a structural coupling between the two
  traits.

## Open Questions

- [ ] What is the return shape for `Witness` proof-quality heuristics —
  boolean, reasoned enum, or structured report?
- [ ] Can triviality detection work generically over an opaque proof artifact
  type, or does it need backend-specific knowledge (a Kani token stream and a
  Verus token stream fail differently)?
- [x] Does token-stream generation belong in the dependency-light core, or in
  an optional feature-gated module, given `proc-macro2`/`quote` are real
  runtime dependencies? — Resolved: neither. It lives in an entirely
  separate repository, `homecoming`, not depended on by `amenable_core` and
  not part of this workspace.
- [ ] What is the smallest first-pass method surface for `Provenance` and
  `Standard` that remains rich enough for audit use without prematurely
  freezing the design?
- [x] Should `Standard` expose provenance by reference, by value, or through
  an associated constructor pattern, once standards carry concrete provenance
  instances instead of only type-level facts? — Resolved: by value. This is
  the only shape that works cleanly for both stored provenance records and
  zero-sized wrapper carriers such as `RustStdStandard<T>`.
- [ ] What is the exact derive contract for enum provenance: required tag key,
  tuple-variant policy, flattening, and collision handling?
- [ ] Does `Sidecar` need to be a first-class trait, or is payload plus a
  proof token sufficient as the canonical shape?
- [ ] Should `Amenable` name a general closed proof system, or specifically
  the closed-world state-machine story?
- [ ] Should `StateMachine` gain an explicit associated root/init claim bound
  to `Standard`, distinct from the `Witness`-backed transition
  relation `Amenable` proves over — or is that better left as an
  implementation-side convention than a trait-level requirement?
- [ ] What is the return shape for an `AuditRecord` (dependent identity,
  captured audit code, citation) now that "audit code" is naturally a
  `homecoming::Fragment`? Depends on `homecoming`'s design settling first;
  see that repository's `HOMECOMING_PLAN.md` for the composition-trait
  questions (arity, node-shape count, round-trip return shape) that gate
  this — they are no longer tracked here since they are no longer this
  repository's design surface to resolve.

## Success Condition

This plan succeeds when `amenable` is the load-bearing proof foundation that
`elicitation` and other frameworks depend on — not the other way around —
and when:

- provenance is defined as a structured user-facing Rust type, not an ad hoc
  string bag
- `#[derive(Provenance)]` in `amenable_derive` makes that structure
  predictably auditable across structs and enums
- `Standard` is the single root-obligation role, with external-vs-local
  source distinctions carried by provenance rather than by sibling root
  traits
- `Evidence` and `Witness` sit downstream of that root-obligation story
  rather than being used to retroactively justify it

- every trust decision in the system is either a real proof or a registered
  certification of provenance, with no silent third option
- proof-quality heuristics catch the specific corner-cutting failure modes
  that motivated building them
- `homecoming::Homecoming` produces an exact receipt of what a `Witness`
  proof ran over, not an approximation — verified by round-trip checks, not
  trusted by convention
- `elicitation`'s ad hoc proof machinery has measurably shrunk because
  `amenable` absorbed its responsibilities
- the trait family remains small, dependency-light, and teachable
