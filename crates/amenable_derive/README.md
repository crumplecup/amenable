# amenable_derive

> Proc macros for the `amenable` constitutional trait family.

## What this crate is

Mechanical, repetitive scaffolding the trait family needs across many
call sites — captured once here instead of hand-written per type. Every
macro below either generates an impl that's the same shape every time
(a derive), or wires a hand-written impl/method into the audit registry
without touching its real content (an attribute). None of them ever
author a proof's actual claim — that always stays hand-written, next to
the macro invocation that wires it in.

If you're building a new worked example (a new evidence chain from
scratch, the way `amenable_gaap`'s GAAP ledger was), skip to
[Onboarding](#onboarding-building-a-new-worked-example) below — it
walks the same macros in the order you'd actually reach for them.

## Macro reference

### Evidence & provenance

- **`#[derive(Provenance)]`** (attrs: `#[provenance(...)]`) — projects a
  struct's or enum's own fields into structured, chain-derived
  `Provenance` metadata. Container attributes: `crate = "path"` (default
  `amenable_core`), `tag = "name"` (the discriminant key an enum's
  metadata carries, default `"variant"`). Field/variant attributes:
  `rename = "name"`, `skip`.

  ```rust
  #[derive(Debug, Clone, PartialEq, Eq, Default, amenable_derive::Provenance)]
  #[provenance(crate = "amenable_core", tag = "authority_kind")]
  pub enum Authority {
      RustProject(#[provenance(rename = "authority")] WitnessLeaf, WitnessLeaf),
      #[provenance(rename = "local_design")]
      LocalDesign { #[provenance(rename = "owner")] owner: WitnessLeaf },
  }
  ```

- **`#[derive(Standard)]`** (attrs: `#[standard(...)]`) — generates
  *both* `Standard` and `Evidence` impls from one attribute, since a
  root's own basis and its provenance value are always the same thing.
  For a root claim that's **asserted and audited**, not proven — a
  typestate marker, a citation-backed law. Attributes: `basis = "Type"`
  (usually `"Self"`), `basis_ctor` (defaults to `Self::default()`),
  `provenance`/`provenance_type` (defaults to `Self`/`self.clone()`),
  `bound = "Self: SomeTrait"` (repeatable — needed when `basis` isn't
  literally `Self`, so the generated impl's own where-clause can state
  what it needs).

  ```rust
  /// Every transfer starts here, asserted by construction, not proven.
  #[derive(Debug, Clone, Copy, PartialEq, Eq, Default, amenable_derive::Standard)]
  #[standard(basis = "Self")]
  pub struct Pending;
  ```

- **`#[derive(Evidence)]`** (attrs: `#[evidence(basis = "..",
  basis_ctor = "..", bound = "..")]`) — the other half of that split:
  for a claim that gets a **real proof body**, not asserted. Generates
  the whole trivial-root `impl Evidence` (`Audit` fixed to `()` — a
  provable claim has no citation to audit) plus the same `EvidenceLink`
  auto-registration `#[derive(Standard)]` does for its own root,
  non-generic case. Same `basis`/`basis_ctor`/`bound` attributes as
  `#[derive(Standard)]`, minus the `provenance`/`provenance_type` half.
  This is the common case — reach for it first.

  ```rust
  #[derive(Debug, Clone, Copy, PartialEq, Eq, Default, amenable_derive::Evidence)]
  #[evidence(basis = "Self")]
  pub struct AmountPositive;
  ```

- **`#[amenable_derive::evidence]`** — the attribute-macro form, for a
  **hand-written** `impl Evidence for ...` block whose `basis()`/
  `audit()` bodies are real, non-trivial content (a composite claim
  built from another `Evidence` type, or an `Audit` richer than `()`).
  Computes `is_root()` from the block's own `Basis` declaration,
  purely syntactically (compares `Basis` against the literal `Self` —
  no `TypeId`, no `'static` bound needed), and nothing else — the rest
  of the impl stays exactly as authored. Doesn't register `EvidenceLink`
  the way the derive does; add that by hand if the type needs to be
  chain-lookup-discoverable.

  ```rust
  #[amenable_derive::evidence]
  impl Evidence for SomeCompositeClaim {
      type Basis = SomeOtherEvidenceType;
      type Audit = RichAuditReport;
      fn basis() -> Self::Basis { SomeOtherEvidenceType }
      fn audit(&self) -> Self::Audit { /* real content */ }
  }
  ```

- **`#[derive(Witness)]`** (attrs: `#[provenance(...)]`,
  `#[witness(verus(module = "..."))]`) — structural closure over
  already-witnessed members: folds a product type's child proofs into
  one composite proof, or a sum type's per-variant proofs into one
  composite sum proof, generating a new nominal `<Type>WitnessProof`
  type. Used when the thing being witnessed is built up from smaller
  already-witnessed pieces (`amenable_std`'s own provenance-chain
  fixtures are the reference usage — see
  `crates/amenable_derive/tests/support/mod.rs`), not for a single
  atomic claim (use `#[evidence]`/`kani_ensures!`-style hand-wiring for
  those instead).

### Proof tokens & lawful exchange

- **`#[derive(ProofToken)]`** (attrs: `#[proof_token(proposition =
  "...")]`) — generates `impl ProofToken for X { type Proposition = Y;
  }`. Every hand-written `ProofToken` impl in this workspace was this
  identical one-line shape (`amenable_kani::stoplight`'s tokens and
  `amenable_gaap::tokens`'s have both since converted to this derive;
  `amenable_creusot::stoplight` and the much larger `rust_std` corpus
  still hand-write it, for real reasons of their own); this collapses
  the duplication and registers a `ProofTokenMintRecord` (read by
  codegen tools like `amenable verus emit-gaap-tokens`) unconditionally,
  at no cost to callers that don't care.

  ```rust
  #[derive(Debug, Clone, amenable_derive::ProofToken)]
  #[proof_token(proposition = "Pending")]
  pub struct PendingToken(());
  ```

- **`#[amenable_derive::establish(credential = "..", proposition = "..",
  verifier = "..")]`** — an attribute, not a derive (the generated impl
  targets the *proposition*, a different type from the token struct it
  sits on, usually in a different crate — a derive can't do that).
  Generates the trivial-mint half of `Establish<C, V>`: ignore the
  credential, construct `Self(())`. `verifier` is optional:
  - **Given** (`verifier = "KaniVerifier"`): the original, concrete
    form — one impl for exactly that verifier.
  - **Omitted**: a single **backend-generic** blanket impl instead,
    `impl<V: Verifier> Establish<C, V> for Y where Y: Witness<V>`. Use
    this when the token's proposition lives in a neutral crate
    (`amenable_gaap`, not a per-backend crate) — see
    [`amenable_gaap::tokens`](../amenable_gaap/src/tokens.rs) for four
    real uses. No verifier-specific code is ever needed here again when
    a fourth backend arrives; it only needs its own `Witness<V>` impl.

  ```rust
  // Concrete form (token lives in a per-backend crate):
  #[derive(Debug, Clone, Copy, amenable_derive::ProofToken)]
  #[proof_token(proposition = "Red")]
  #[amenable_derive::establish(
      credential = "YellowToken",
      verifier = "KaniVerifier",
      proposition = "Red"
  )]
  pub struct RedToken(());

  // Verifier-less form (proposition lives in a neutral crate):
  #[derive(Debug, Clone, amenable_derive::ProofToken)]
  #[proof_token(proposition = "Validated")]
  #[amenable_derive::establish(credential = "PendingToken", proposition = "Validated")]
  pub struct ValidatedToken(());
  ```

- **`#[derive(Sidecar)]`** (attrs: `#[sidecar(...)]` container,
  `#[sidecar(primary)]`/`#[sidecar(token)]` field markers) — generates
  `impl Sidecar<V> for X` plus a constructor, for the "payload + the
  token proving its current state" carrier shape every `Exchange` edge
  needs. Container attributes: `verifier = "..."` (optional, same
  generic-vs-concrete split as `establish` above and for the identical
  reason), `proposition = "..."` (defaults to the primary field's own
  type — set this when the proposition is a separate phantom generic
  parameter instead, e.g. `Transfer<S, Token>`), `constructor =
  "pub"|"pub(crate)"|...` (visibility of the generated `new()`).

  ```rust
  #[derive(Debug, Clone, amenable_derive::Sidecar)]
  #[sidecar(proposition = "S", constructor = "pub(crate)")]
  pub struct Transfer<S, Token> {
      #[sidecar(primary)]
      payload: TransferPayload,
      #[sidecar(token)]
      token: Token,
      _state: std::marker::PhantomData<S>,
  }
  ```

- **`#[amenable_derive::exchange(cfg = .., verifier = "..", evidence =
  "..", proof_artifact = .., harness_fn = .., harness_const = ..,
  evidence_id = "..", creusot_ensures = "..")]`** — the full bundle for
  a *concrete*-verifier `Exchange` edge. Applied to `impl SelfType { fn
  method(&self, input: Input) -> Result<Output, Error> { .. } }` — one
  method, its body left exactly as authored — and generates: the
  `Witness<V>` impl for `evidence` (naming the harness), the
  `ProofRecord` registration backing it, the `Exchange<Input, Output,
  V>` impl delegating to `method`, an `ExchangeEdgeRecord` capturing
  `method`'s body verbatim (for another backend's codegen to read), and
  `method`'s own injected `#[cfg_attr(cfg, cfg::ensures(..))]` contract,
  calling through `evidence`'s own separately-registered `Ensures<V>`
  impl. Requires a non-generic impl block with exactly one method — see
  [`amenable_kani::stoplight`](../amenable_kani/src/stoplight.rs) for
  three real edges.

  ```rust
  #[amenable_derive::exchange(
      cfg = kani,
      verifier = "KaniVerifier",
      evidence = "Yellow",
      proof_artifact = CalculationProof,
      harness_fn = verify_green_transitions_only_to_yellow,
      harness_const = VERIFY_GREEN_TRANSITIONS_ONLY_TO_YELLOW_SRC,
  )]
  impl Stoplight {
      fn green_to_yellow(
          &self,
          input: Established<Green, GreenToken>,
      ) -> Result<Established<Yellow, YellowToken>, StoplightError> {
          let token = Yellow::establish(input.sidecar());
          Ok(Established::new(Yellow, token))
      }
  }
  ```

- **`#[amenable_derive::capture_exchange_body(evidence = "..",
  creusot_ensures = "..", method_generics = "..", kani_ensures = "..",
  kani_requires = "..")]`** — a narrower sibling of `#[exchange(..)]`
  for a method that's **generic over `V`** and has no single concrete
  verifier for `#[exchange(..)]`'s own bundle to name. Captures the
  method's body verbatim into an `ExchangeEdgeRecord`, the identical way
  `#[exchange(..)]` does — no `Witness<V>` impl, no `Exchange` impl,
  either way. `method_generics` (optional, default `""`) names extra
  generic parameters the real method declares beyond `Self`'s own (e.g.
  `"V"`) as a bare comma-separated list — needed only when the captured
  body itself calls another generic method with an explicit turbofish
  (`Self::helper::<V>(..)`), so a generated companion has something
  named `V` to resolve against.

  `kani_ensures = "true"` (optional, default `"false"`) generates the
  method's own Kani contract too: `|result: &Result<Output, Error>|
  <Evidence as Ensures<V>>::ensures(result.clone())` — real once every
  edge delegates to its target evidence type's own registered claim
  instead of restating it (see the decision table below), since that
  collapses every edge onto the identical mechanical shape, differing
  only in which `Output`/`Error`/`Evidence` names get substituted, all
  three of which this macro already has. Opt-in, not automatic: a
  future caller with a genuinely different Kani contract shape (or
  none) shouldn't be forced into this one. `kani_requires = ".."`
  (optional) splices a real, hand-authored precondition expression into
  `#[cfg_attr(kani, kani::requires(..))]` alongside it — not
  mechanical, since not every edge needs one and there's no way to
  derive *which* condition from the signature. When `kani_ensures =
  "true"`, the real method must **not** carry its own hand-written
  `#[cfg_attr(kani, kani::ensures(..))]` — the macro injects one.

  ```rust
  #[amenable_derive::capture_exchange_body(
      evidence = "Committed",
      creusot_ensures = "match result { Ok(committed) => committed_amount_holds(committed.payload.amount.0), Err(_) => false, }",
      kani_ensures = "true",
      kani_requires = "input.primary().amount().value() > 0"
  )]
  impl Ledger {
      pub fn commit<V: amenable_core::Verifier>(
          &self,
          input: Transfer<Validated, ValidatedToken>,
      ) -> Result<Transfer<Committed, CommittedToken>, TransferError>
      where
          Committed: amenable_core::Evidence
              + amenable_core::Witness<V>
              + amenable_core::Ensures<V, Input = Result<Transfer<Committed, CommittedToken>, TransferError>, Bound = bool>,
          /* .. */
      {
          let payload = input.primary().clone();
          let token = Committed::establish(input.sidecar());
          Ok(Transfer::new(payload, token))
      }
  }
  ```

### Harnesses & calculations

- **`harness!(cfg_name, CONST_NAME, { item })`** — defines a
  `#[cfg(cfg_name)]`-gated proof harness item and, alongside it, an
  always-available `&'static str` constant holding the harness's
  verbatim source (captured via `Span::source_text`, whitespace and
  all — the item must be written directly at the call site, not
  threaded through an intermediate `macro_rules!` layer, for the
  capture to work). A `kani` invocation also registers a
  `KaniProofRegistration`, so the CLI can discover the harness without
  source scanning. This is how an audit report shows a proof exactly as
  its author wrote it, not a hand-maintained description that could
  drift.

  ```rust
  amenable_derive::harness! {
      kani, VERIFY_GREEN_TRANSITIONS_ONLY_TO_YELLOW_SRC, {
          #[kani::proof_for_contract(Stoplight::green_to_yellow)]
          fn verify_green_transitions_only_to_yellow() {
              let stoplight = Stoplight;
              let input = Established::<Green, GreenToken>::root();
              let _ = stoplight.green_to_yellow(input);
          }
      }
  }
  ```

- **`#[calculation(token = TokenType)]`** — turns a plain function into
  a chain link in the evidence graph: it knows it has a method, knows
  it yields `TokenType`, and registers itself, for the case where the
  evidence is computed from arguments (`Sum::new(a.value + b.value)`)
  rather than reached through a typestate transition.

  ```rust
  #[calculation(token = AddToken)]
  pub fn add(a: Debit, b: Credit) -> Sum {
      Sum { value: a.value + b.value }
  }
  ```

### Kani composition

- **`#[derive(KaniCompose)]`** — generates bounded, depth-limited Kani
  constructors (`kani_depth0`/`kani_depth1`/`kani_depth2`/`kani_any`)
  by delegating every field to its own `KaniCompose` impl. For building
  `kani::any()`-shaped values of a composite struct/enum without
  unbounded recursion through nested `Option<Box<..>>`/`Vec<..>` fields.

  ```rust
  #[derive(amenable_derive::KaniCompose, Debug, Clone, PartialEq, Eq)]
  struct Node {
      name: String,
      maybe_child: Option<Box<Leaf>>,
      flags: Vec<bool>,
  }
  ```

### Verus bridging (`feature = "verus"`)

Six more macros exist only under the `verus` feature, all consumed by
`amenable_std::verus_witness` and `amenable_core::verus_carrier` to
bridge Verus's own `verus! { .. }` proofs (which can't use `inventory`/
`harness!` — Verus never resolves `Cargo.toml`, so no proc-macro crate
is reachable from a real `verus --crate-type=lib` run) into the same
audit registry every other backend's witnesses populate:

- **`verus_ensures_fragments!("harness_name")`** /
  **`verus_requires_fragments!("harness_name")`** — expand to a
  `&'static [&'static str]` array literal of a real Verus harness's
  actual `ensures`/`requires` clauses, extracted from its source at
  compile time. A renamed or removed harness is a compile error here,
  not a silently-stale description.
- **`verus_ensures_witness!(Type, "evidence_name", "harness_fn")`** /
  **`verus_requires_witness!(..)`** — generates a real `impl
  Ensures<VerusVerifier> for Type` (`Bound = &'static [&'static str]`,
  one clause per element) plus one `ContractRecord` registration per
  clause.
- **`verus_ensures_predicate!(Type, "evidence_name",
  "spec_fn_name")`** / **`verus_requires_predicate!(..)`** — the same
  shape, for a claim that's a real, named `pub open spec fn` shared
  across several harnesses/carrier files, rather than one harness's own
  clause list.

  ```rust
  amenable_derive::verus_ensures_witness!(
      RustStdStandard<char>,
      "amenable_std::rust_std::RustStdStandard<char>",
      "verify_char_roundtrip"
  );
  ```

## Choosing between similar macros

A few of the macros above look interchangeable at a glance. They aren't:

| If... | use... |
| --- | --- |
| The claim is asserted and cited (a typestate root, a documented law), not proven | `#[derive(Standard)]` |
| The claim has a real proof body, and `Evidence`'s trivial-root shape fits (`Basis = Self`, no real `Audit`) | `#[derive(Evidence)]` — reach for this first |
| The claim has a real proof body but a non-trivial `Basis`/`Audit` (composite, not a root) | `#[amenable_derive::evidence]` on a hand-written impl |
| Your carrier/token/edge lives in a **per-backend** crate and names one concrete verifier | the concrete form: `establish`/`Sidecar`'s `verifier = ".."`, `#[exchange(..)]` |
| Your carrier/token/edge lives in a **neutral** crate (e.g. `amenable_gaap`) usable by every backend | the verifier-less form: `establish`/`Sidecar` with `verifier` omitted, `#[capture_exchange_body(..)]` instead of `#[exchange(..)]` |
| The method is non-generic and you want the *whole* bundle generated (contract + `Witness<V>` + `Exchange` impl) | `#[exchange(..)]` |
| The method is generic over `V`, and its contract calls through its target evidence type's own `Ensures<V>` impl | `#[capture_exchange_body(evidence = "..", kani_ensures = "true")]` — generates the Kani contract too |
| The method is generic over `V` and needs a genuinely different Kani contract shape (or none) | `#[capture_exchange_body(..)]` with `kani_ensures` omitted — registers the body only, contract stays hand-written |

## Onboarding: building a new worked example

This is the order the GAAP ledger (`amenable_gaap`) was actually built
in — see [`docs/GAAP_LEDGER_PLAN.md`](../../docs/GAAP_LEDGER_PLAN.md)
for the full narrative, including the real dead ends. If you're adding
a new evidence chain from scratch, this is the template:

1. **Define your evidence types** in a neutral crate (not a per-backend
   one). Typestate roots (states asserted by construction) get
   `#[derive(Standard)] #[standard(basis = "Self")]`. Claims that will
   get a real proof body get `#[derive(Evidence)] #[evidence(basis =
   "Self")]` — no hand-written impl needed for the common trivial-root
   case; fall back to a hand-written `impl Evidence` plus
   `#[amenable_derive::evidence]` only for a genuinely non-trivial
   `Basis`/`Audit`.
2. **Define your proof tokens**, one per evidence type reachable via a
   transition: `#[derive(ProofToken)] #[proof_token(proposition =
   "...")]`. Give each a real `Establish` impl via
   `#[amenable_derive::establish(credential = "..", proposition =
   "..")]` (omit `verifier` — you're in a neutral crate, so this should
   be the generic blanket form).
3. **Define your carrier type** (payload + current-state token) with
   `#[derive(Sidecar)] #[sidecar(proposition = "S", constructor =
   "pub(crate)")]` — `constructor = "pub(crate)"` if lawful
   construction should require going through a real `Establish` call,
   not a bare struct literal.
4. **Write your real transition methods** as plain, generic-over-`V`
   inherent methods on your carrier's owning type (`fn validate<V:
   Verifier>(&self, input: ..) -> Result<.., Error> where ..`) — the
   real logic, hand-written, once. Attach `#[amenable_derive::
   capture_exchange_body(evidence = "...")]` on a single-method `impl`
   block wrapping each one, so every backend's own codegen can read the
   body verbatim later.
5. **Per backend**, attach the real contract. If it calls through the
   transition's own target evidence type (the common case — see the
   decision table above), add `kani_ensures = "true"` to the same
   `capture_exchange_body(..)` attribute rather than hand-writing it;
   add `kani_requires = ".."` alongside it for a genuine precondition.
   Otherwise write `#[cfg_attr(kani, kani::ensures(..))]` directly on
   the method by hand. Either way, build the accommodation-model
   mirror plus generated companion a whole-crate translator needs for
   Creusot/Verus (see `amenable_creusot::ledger`/`amenable_verus::gallery::
   ledger_exchange`, and their own codegen tools in `crates/amenable/
   src/{creusot_export,verus_exchange_export}.rs`). **A Kani-contracted
   function's body must never delegate to a separate wrapper
   function** — attach the contract directly to the real method (see
   `GAAP_LEDGER_PLAN.md`'s Step 7 for the confirmed Kani 0.67.0 bug this
   avoids).
6. **Wrap every hand-written harness** in `harness!(cfg_name,
   CONST_NAME, { .. })` so its source is captured verbatim and
   discoverable via the CLI.
7. If a transition's claim is genuinely trivial (always succeeds, no
   real invariant), don't hide that — a bare `result.is_ok()` (Kani/
   Verus) or `#[ensures(true)]` (Creusot) documents the triviality
   rather than dressing it up as more than it is. `amenable_kani::
   stoplight`'s three edges are the reference example of this.

For the concrete, currently-complete result of following this order —
six real methods, three independent backends, zero per-backend
duplicate logic — read `amenable_gaap`'s own
[README](../amenable_gaap/README.md) as a guided tour of the finished
shape, then its source (`ledger/` — split into several files during a
later modularity pass, but still one module — then `tokens.rs`,
`transfer.rs`, `contracts.rs`) in that order.

## See also

- [Root README](../../README.md) for the project-wide overview.
- [`amenable_core`](../amenable_core/README.md) for the trait family
  these macros generate impls against.
- [`amenable_gaap`](../amenable_gaap/README.md) for the worked example
  that exercises the evidence/token/carrier/exchange macros above end
  to end, on all three backends.
