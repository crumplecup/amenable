# The Amenable Trait Family

A declarative interface amenable to formal verification.

## Trait Verifier

The elicitation framework supports three different verifiers: Kani, Creusot and Verus. Verifier contains abstractions over the backend interface, making it easier for the calling side to backend agnostic.

## Trait Witness

Witness takes a generic parameter V: Verifier These methods provide proofs for the verifier to consume, along the lines of here is the Kani proof in code, here is the same proof as a tokenstream. This type also includes methods for auditing proofs.

Witness also exposes proof-quality heuristics and diagnostics: structural checks that catch vacuous or corner-cut proofs — for example, an emitted proof artifact that is empty or a no-op, or one that never references the invariant it claims to establish — before they are trusted as evidence. These are heuristics, not a soundness guarantee, but they give reviewers (human or agent) a fast, structural signal instead of requiring a full manual read of every proof body.

## Trait Standard

Standard is a type of Witness that references a normative provenance record — whether that's a third-party standard (an ISO, a citation) or a locally-asserted design decision with no external authority to cite (`FlowersArePretty`). Both were originally separate traits (`Standard` for the former, `Objective` for the latter); they were collapsed into one `Standard` trait early on, since the two never differed in shape, only in what their `Provenance` record named as the authority. The distinction lives entirely in `Provenance`'s own content now, not in a sibling root trait. Standard provides methods for querying the reference source, and auditing fidelity to it. The proofs for these types include authoritative references and informative descriptions, rather than mathematical operations. Like unsafe code blocks, implementations of the methods in Standard create an audit friendly surface for upholding this class of program invariants.

## Trait Evidence

Methods for types that depend on a Standard, including ways to audit the proof chain leading back to the originating Standard.
Eg. PolygonValid

* audit - For a given method, produce the source code associated with computing the result.

Additional convenience methods for analytics and heuristics.

Standard trivially implements Evidence by referring to its own identity.

## Trait Sidecar

The Sidecar defines a primary type and an accompanying "proof" type. Imagine methods like:

* primary - the payload, arguments for a method call or the return data from a call
* sidecar - evidence token associated with the primary data

The primary data must implement Witness. The sidecar type must implement Witness and Evidence.

## Trait Establish

The Established trait represents the relationship between evidence tokens and proofs deduced from the accumulation of evidence. Tokens in this context are types that implement Evidence, and methods on this trait permit exchanging one type of evidence for another. For example:

* establish(T: Evidence) -> U: Evidence

## Trait Exchange

The Exchange trait defines the semantics of producing one Sidecar type, and receiving another Sidecar type in return. Exchanges are the process by which the trait interface composes evidence from leaf constructors into higher order abstractions that aggregate bundles of evidence. For example:

* exchange(T: Sidecar) -> U: Sidecar

Since each Sidecar contains Evidence, users must implement Establish to define legal exchanges of proof tokens, or they will not be able to produce the required proof to form a valid Sidecar type.

## Trait StateMachine

Methods expose a bounded set of states and transitions comprising a finite state machine.

Transitions and states are not the same kind of claim. A transition is a relation between two states and has mathematical shape — a model checker can prove it. A state, considered on its own, has no such shape: nothing derives "the light is currently Green," it is simply asserted. States are therefore roots, not proofs, and every state a StateMachine claims to be in is a Provenance object (a Standard certification), the same as any other trusted assumption the framework cannot derive from first principles. See the Narrative section below.

## Amenable

A closed set of Exchanges, where the program is a State Machine and all state transitions implement Exchange.

Methods enumerate the set of Exchanges, all the transitions that are allowing inside the program, and provide accessors to the formal proofs associated with a given state or transition. The proof accessor for a transition is Witness-backed; the accessor for a state is Provenance-backed.

## Homecoming (external)

Source-code capture — a type gives back the code that produced a value instead of the value itself — is not defined here. It lives in its own repository, [homecoming](https://github.com/crumplecup/homecoming), as the `Homecoming` trait. It is not part of the `amenable` trait family; `amenable` is expected to depend on it once its design stabilizes, with `Witness` the leading candidate for a trait that also implements `Homecoming`. See that repository's `homecoming.md` and `HOMECOMING_PLAN.md` for the design, and "Relationship to `homecoming`" in `docs/AMENABLE_PLAN.md` for why it matters here specifically: the code a `Witness` proof ran over is only a meaningful receipt if it can be tied, exactly, to the code that actually ships.

## Narrative

Standards are a way to plug trusted assumptions into formal verifiers, in a way that makes them auditable — whether the assumption cites a third-party authority or asserts a local design decision with no external authority behind it (originally two separate traits, `Standard` and `Objective`; collapsed into one, since `Provenance` alone already carries the distinction — see "Trait Standard" above). Carefully crafted human decision, based on policy or legal standards, can produce a contained audit trail of source code. Largely speaking, this comes down to embedding good metadata describing the condition, the reference standard or program invariant that it represents, links to the third party standards, or internal steps taken to uphold the goal. The metadata embedded into the formal proofs is the true value of trusted assumptions in formal models, because they create a paper trail detailing how the programmers intended to uphold their promises. This is a good thing because it sweeps up a broader class of program invariants than what we can prove with a few math operations.

Proofs generally refer to evidence that desired program invariants are preserved throughout all states of the program. Acquiring proofs can be computationally intensive (polygon with metadata), or ineffable and requiring human audit (flowers are pretty). The trait interface is only valid way to manufacture a proof token. This restricts the audit surface to the narrow bottleneck of the trait method implementations.

When a claim cannot be discharged by a mathematical proof, the framework does not fall back to declaring it trivially trusted. It requires a Standard certification instead — a metadata record naming the authority, source, and rationale for the trust decision. This is the gold standard for reporting on decisions about program invariants in Rust: every claim is backed by either a machine-checked proof or an explicit, auditable certification of provenance. There is no third, silent option, and no blanket implementation is allowed to grant that certification for free.

This is not only true of external standards like MUTCD dwell-time minimums. It is true of state itself. Consider a stoplight state machine: the transition rule "Yellow never jumps directly to Green" is a real mathematical claim about a relation between two states, and Witness can prove it. But ask what the proof looks like for the bare fact "the light is currently Green" — not the transition into Green, the state of being Green — and there is nothing to derive it from. The first assertion a running program makes is asserted, not computed. This is the same shape as an `unsafe` block needing a `// SAFETY:` comment: a root fact the program relies on but cannot prove from within itself. So a state claim is also a Provenance object, exactly like any other Standard. This generalizes past power-on: recovering from a checkpoint, reading a sensor, receiving state from an external system are all cases of a state being asserted rather than arrived at through a proven transition, and all of them are roots requiring Provenance rather than a fabricated derivation. Transitions have mathematical shape; states do not, and never will, no matter how much proof effort is applied — the distinction is categorical, not a temporary gap.

Once `amenable` depends on homecoming, Witness and Homecoming must never be allowed to independently drift apart on the same claim — a structurally real proof over the wrong code is more dangerous than an obviously empty one, because it does not look suspicious. The fix is not to weld the two traits together architecturally; it is to make their agreement a checkable claim, the same discipline the rest of the family runs on. Emit a value's code, reconstruct the value from it, and confirm it matches the original: that is the baseline soundness obligation for Homecoming itself, and it belongs to that repository, not this one. Reconstruct a whole program from its emitted fragments, regenerate its proofs, and compare them to the original program's proofs: that is the receipt made concrete, proving Witness and Homecoming agree without requiring them to be the same mechanism.
