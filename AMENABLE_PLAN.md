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

- Core trait family (`Verifier`, `Witness`, `Witnessed`, `Evidence`,
  `Standard`, `Objective`, `AsStandard`, `AsObjective`, `ProofToken`,
  `Sidecar`, `Establish`, `Exchange`, `StateMachine`, `Amenable`,
  `Provenance`, `MetadataEntry`, `RustStdType`) is implemented, split into
  focused modules, and compiles with zero runtime dependencies.
- `elicitation` no longer contains any `amenable`-adjacent code. It has not
  yet been updated to depend on `amenable`.
- No proof-emission backend (Kani/Creusot/Verus token-stream generation) has
  been rebuilt here yet. `WitnessSource` implementations do not exist for any
  concrete type in this crate today.

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
  standards, expressed through `amenable`'s `Standard`/`Objective` roles

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
also a `Provenance` object — a `Standard` or `Objective` certification,
subject to the same discipline (who decided this, why, what is the
citation) as any other trusted assumption.

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
2. An explicit `Standard` or `Objective` certification of provenance — a
   metadata record naming the authority, source, and rationale for a human
   trust decision.

There is no third option where a claim is silently or implicitly trusted
because nobody got around to writing a proof or a certification. If a claim
cannot currently support a mathematical proof, the obligation does not
disappear — it converts into an obligation to produce a certification of
provenance instead. Relying on the Rust standard library's documented
guarantees, for example, is a legitimate trust decision, but it must be
registered as a `Standard` certification (naming the Rust project as
authorizing body, the relevant doc page as authoritative source, and so on)
rather than assumed for free by a blanket impl. This is the specific failure
mode the previous `elicitation`-side bridge fell into — a blanket
`AsStandard<Is<T>>: Standard` impl for every `T: RustStdType` — and it is
exactly what the certification requirement is designed to prevent: trust
without a registered record of the decision to trust.

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

### Why the metadata methods are the gold standard

The `Standard` and `Objective` disclosure methods (authorizing body,
authoritative source, source scope, normative summary, fidelity rationale /
design authority, architectural context, intended invariant, rationale) are
not incidental documentation. They are the mechanism by which a trust
decision about a Rust program invariant becomes an auditable, first-class
artifact instead of an implicit assumption baked silently into a blanket impl
or a comment. Registering this metadata inside the formal verification
framework itself — rather than in a README or a commit message — is what
makes it a certification rather than a note.

## Auditing as a First-Class Constitutional Concern

Auditing lives inside the very traits that define proof roles and exchange
roles, not as a bolted-on concern.

- `Standard` audits external authority, source, clause scope, summary, and
  fidelity rationale
- `Objective` audits design authority, authorship, architectural fit, intended
  invariant, and rationale
- `Evidence` audits dependency lineage back to the `Standard` and `Objective`
  roots that justify it
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

## Audit Inversion: From Rule to Dependents

Auditing a `Standard` or `Objective` should not be limited to reading the
citation forward, from a dependent claim back to the root it names. The more
useful review question often runs the other way: given a specific rule (the
MUTCD 10-second/5-second dwell-time minimum, say), show every piece of code
in the system that relies on it, for direct review — the code "inside the
unsafe block," not just a pointer to it.

Two distinct mechanisms are needed, not one:

1. **Literal code capture, not string pointers.** `Evidence::audit_surface()`
   today returns `&'static [&'static str]`, presumably identifiers or
   file/line references. `amenable.md` already asks for something stronger
   under `Evidence` — "produce the source code associated with computing the
   result." For a reviewer to judge whether the code actually upholds a
   cited rule without leaving the report, the audit surface needs to carry
   literal source text, not a name to go look up. This is capturable at
   compile time (a proc-macro reads and stringifies the annotated function
   body, the automated equivalent of `stringify!`).
2. **A reverse index from root to dependents.** Forward, a derived `Evidence`
   names the `Standard`/`Objective` roots it relies on. Backward — "given
   this rule, show every dependent" — requires each dependent to register
   itself against the root it cites in a way the root can enumerate without
   any static knowledge of who depends on it. That is a compile-time
   registry keyed by the root type, collecting `(dependent identity, audit
   code, citation)` records — the shape this workspace already uses
   elsewhere for exactly this kind of static-submission/runtime-enumeration
   problem (an `inventory`-crate pattern is the leading candidate).

Any record type built for this (working name `AuditRecord`) must own its
data — `String` for dependent identity and captured audit code, not
`&'static str` — since captured source and formatted identities are
runtime-computed, not literals. See the struct-ownership rule in the design
checklist below.

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
- [ ] Confirm `Standard`/`Objective` disclosure methods are sufficient to
  register a Rust-standard-library trust decision as a certification
  (the `RustStdType` surface already carried over is the first test case).
- [ ] Reject any blanket impl of `Standard`, `Objective`, or `Evidence` that
  would let a type earn certification without an explicit, per-type
  registration.
- [ ] Decide whether `StateMachine` gains an explicit associated
  `Provenance`-bound root/init claim (see "States Are Roots, Transitions Are
  Relations" above), distinct from the `Witness`-backed transitions
  `Amenable` proves over.
- [x] Redesign `Provenance` as a capability interface rather than a fixed
  collection type: `fn iter(&self) -> impl Iterator<Item = MetadataEntry>` is
  the only required method, with `get`/`contains_key`/`len`/`is_empty` as
  defaults derived from it. Superseded the earlier `BTreeSet<MetadataEntry>`
  plan — locking in any concrete collection, `BTreeSet` included, would
  still describe structure rather than capability.
- [x] Change `MetadataEntry`'s fields from `&'static str` to owned `String`,
  with a `new(impl Into<String>, impl Into<String>)` constructor.
- [x] Keep the verifier metadata marker structs (`KaniVerifierMetadata`,
  `CreusotVerifierMetadata`, `VerusVerifierMetadata`) zero-sized despite the
  move to owned `MetadataEntry`, by yielding owned entries lazily from a
  `const` slice inside `iter()` rather than storing a field — the "promise"
  of provenance stays zero-sized; the data materializes only on request.
- [ ] Design the `AuditRecord` registry for rule-to-dependents audit
  inversion (see "Audit Inversion: From Rule to Dependents" above): literal
  source capture plus a compile-time registry keyed by `Standard`/
  `Objective` root type.

### Phase 3: Proof-emission upgrade

Rebuild the concrete proof-emission surface — the successor to
`elicitation`'s `Prop::kani_proof`/`creusot_proof`/`verus_proof` — as a
`WitnessSource` implementation story native to `amenable`, informed by but
not copied from the existing `elicitation` code.

- [ ] Decide the shape of the leaf proof-emission trait(s) implementers
  write against (successor to `Prop`).
- [ ] Decide how token-stream generation is represented without pulling in
  `proc-macro2`/`quote` as a hard dependency of the constitutional core —
  or decide that a thin, optional feature-gated dependency is warranted here.
- [ ] Rebuild the `Established<P>`-equivalent proof-sidecar token as a
  concrete `ProofToken`/`Sidecar` implementation.
- [ ] Rebuild the `ProvableFrom<C>`-equivalent exchange relation as a
  concrete `Establish<C>` implementation.
- [ ] Rebuild the `VerifiedStateMachine`-equivalent closed-world story as a
  concrete `Amenable` implementation.

### Phase 4: `elicitation` becomes a consumer

- [ ] Add `amenable` as a dependency of `elicitation`.
- [ ] Migrate `elicitation`'s proof-carrying types onto `amenable`'s traits.
- [ ] Delete the superseded `elicitation`-local proof machinery
  (`Prop`, `Established<P>`, `ProvableFrom<C>`, `FormalMethod`,
  `VerifiedStateMachine`) once nothing depends on it, simplifying that
  workspace.
- [ ] Confirm the elicitation test suite and verification coverage checks
  still pass against the new foundation.

### Phase 5: Downstream proving grounds

- [ ] Re-evaluate whether `elicit_temporal` is the best first external
  proving ground for `Standard`/`Objective` root-role registrations anchored
  in an external spec (ISO 8601 / RFC 3339 / IXDTF).
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
  a `Standard`/`Objective` certification — never a blanket impl that grants
  trust for free.
- [ ] `Witness` proof-quality heuristics catch at least the vacuous-proof
  failure mode that motivated adding them.
- [ ] `Amenable` explains a rebuilt `VerifiedStateMachine`-equivalent as one
  concrete realization of the constitutional pattern rather than as a
  separate or rival design.
- [ ] Auditing is built into the constitutional roles themselves rather than
  bolted on later as an afterthought.
- [ ] Audit methods remain role-specific and jurisdiction-specific instead of
  collapsing into a shapeless metadata surface.
- [ ] `elicitation`'s proof machinery shrinks as `amenable` absorbs it, rather
  than the two surfaces persisting in parallel indefinitely.
- [x] Struct fields holding runtime-populated data own their data (`String`,
  `Vec<T>`); `&'static str`/`&'static [T]` is reserved for no-`&self` trait
  methods describing a fixed, compile-time-known fact about a type.
- [ ] Every state a `StateMachine` claims to occupy is backed by `Provenance`
  (a `Standard`/`Objective` root), the same as any other claim with no
  mathematical shape to prove — states are never silently exempted from the
  certification requirement just because they "aren't really proofs."

## Open Questions

- [ ] What is the return shape for `Witness` proof-quality heuristics —
  boolean, reasoned enum, or structured report?
- [ ] Can triviality detection work generically over an opaque proof artifact
  type, or does it need backend-specific knowledge (a Kani token stream and a
  Verus token stream fail differently)?
- [ ] Does token-stream generation belong in the dependency-light core, or in
  an optional feature-gated module, given `proc-macro2`/`quote` are real
  runtime dependencies?
- [ ] What is the smallest first-pass method surface that meaningfully raises
  proof quality for `Standard` and `Objective` without prematurely freezing
  the design?
- [ ] Should `Standard` and `Objective` be subtraits of `Evidence`, or should
  they remain distinct constitutional roles that merely participate in the
  broader proof economy?
- [ ] Does `Sidecar` need to be a first-class trait, or is payload plus a
  proof token sufficient as the canonical shape?
- [ ] Should `Amenable` name a general closed proof system, or specifically
  the closed-world state-machine story?
- [ ] Should `StateMachine` gain an explicit associated root/init claim bound
  to `Standard`/`Objective`, distinct from the `Witness`-backed transition
  relation `Amenable` proves over — or is that better left as an
  implementation-side convention than a trait-level requirement?
- [ ] What is the return shape for an `AuditRecord` (dependent identity,
  captured audit code, citation), and does the root-to-dependents registry
  belong in the dependency-light core or an optional feature-gated module —
  the same dependency question as token-stream generation?

## Success Condition

This plan succeeds when `amenable` is the load-bearing proof foundation that
`elicitation` and other frameworks depend on — not the other way around —
and when:

- every trust decision in the system is either a real proof or a registered
  certification of provenance, with no silent third option
- proof-quality heuristics catch the specific corner-cutting failure modes
  that motivated building them
- `elicitation`'s ad hoc proof machinery has measurably shrunk because
  `amenable` absorbed its responsibilities
- the trait family remains small, dependency-light, and teachable
