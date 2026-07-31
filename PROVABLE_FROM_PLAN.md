# Fixing `Establish` to Actually Gate Obligations

## Goal

Close a real soundness gap in `Establish`: nothing today stops
`establish()` from being called on a credential value that never
demonstrated anything. `amenable.md` states the actual law this trait
family is supposed to uphold:

> The trait interface is only valid way to manufacture a proof token.
> This restricts the audit surface to the narrow bottleneck of the trait
> method implementations.

`Establish::establish(credential: &C) -> Self::Token` does not honor
that. It accepts *any* value of type `C` and mints a token regardless,
because nothing distinguishes "a `KaniChannel` that actually ran
`send`/`recv` and demonstrated delivery" from "a `KaniChannel` fresh off
`unbounded()`, never touched."

`Establish` **is** this codebase's deliberate rename of `elicitation`'s
`ProvableFrom<C>` — not a cousin of it, not a "similar idea." The fix
below lands entirely inside `Establish` itself: no sibling trait, and no
mandated concrete carrier struct in the shape of `elicitation`'s
`Established<P>`. The obligation a caller must supply is expressed purely
as a trait bound, satisfiable by whatever concrete type each site finds
natural — never a fixed "`MyStrictCustomType` is the one true credential
shape" design.

## Problem, With Receipts

This was found, not theorized: `elicit_doc`'s antipattern scanner flags
`Establish::establish`'s credential parameter as `unused_underscore_arg`
across every accommodation-model site added this session (`KaniChannel`'s
9 consumers, `KaniSplitObservation`/`KaniSplitNObservation`'s 10,
`fs_model`'s 9, the `KaniUtf8Buffer` chain). That flag was dismissed
mid-session as a tool false positive and, on rereading, that dismissal
doesn't hold up. Even the `KaniUtf8Buffer` chain, held up at the time as
the *clean* two-hop worked example, has the same hole:
`KaniUtf8Buffer::<2>::establish(&KaniAssumedUtf8Validity::asserted_valid())`
mints a `KaniUtf8BufferToken`, and that token is never passed to the
downstream `RustStdStandard::<String>::establish()` call — the first
hop's proof is minted and immediately dropped.

It runs deeper than the accommodation models. `Stoplight`'s
`exchange` — the worked example this session treated as the *reference*
for "hold onto the real token" — has the identical bug, just harder to
spot because its state markers carry no data to fake:

```rust
impl Exchange<Established<Green, GreenToken>, Established<Yellow, YellowToken>> for Stoplight {
    fn exchange(&self, input: Established<Green, GreenToken>) -> Result<Established<Yellow, YellowToken>, Self::Error> {
        let token = Yellow::establish(input.primary());   // bare &Green — the real GreenToken sits unused in `input.sidecar()`
        Ok(Established::new(Yellow, token))
    }
}
```

`input.sidecar()` — the actual, already-minted `GreenToken` — is right
there and is never called. This confirms the bug is in `Establish`'s
shape itself, not a modeling mistake specific to any one accommodation
model.

## Reference (Not a Template): How `elicitation` Enforces This

`AMENABLE_PLAN.md` already retired treating `elicitation`'s `Prop` /
`Established<P>` / `ProvableFrom<C>` as primitives to port wholesale —
that framing stands. Its enforcement mechanism was useful to study anyway
(`crates/elicitation/src/contracts.rs`): a `pub(crate)`-or-narrower
zero-sized credential struct, generated per obligation by a
`proof_credential!` macro, constructible only by the factory function
that performed the real check. Merely possessing a value of the
credential type is the proof.

That mechanism is *not* what this plan adopts, and the design review
sharpened exactly why: it locks every obligation to one bespoke,
hand-named struct, which is `elicitation`'s own `Established<P>`-the-struct
problem, not a fix for it. `amenable` already owns the correct primitive
for "a value that can only exist because a real proof happened" —
`ProofToken` — and it's a trait, not a struct. The fix is to require
`Establish`'s credential to satisfy it, not to invent a parallel
credential-struct mechanism next to it.

## The Fix

One change to `Establish`, in `amenable_core::exchange.rs`: bound the
credential on `ProofToken`, and take it by value.

```rust
pub trait Establish<C, V: Verifier>: Evidence + Witness<V> + Sized
where
    C: ProofToken,
{
    type Token: ProofToken<Proposition = Self>;
    fn establish(credential: C) -> Self::Token;
}
```

This is the whole mechanism. No sibling trait, no mandated struct family:

- **Only a real token satisfies the bound.** Every `ProofToken` this
  codebase writes has a private inner field (`RustStdSenderToken(())`,
  `GreenToken`, `KaniUtf8BufferToken`, …), so the only way to hold one is
  to have obtained it from an earlier `establish()` call. A bare domain
  value (`KaniChannel<i32>`, `Green`) no longer type-checks as a
  credential at all — the compiler rejects it before a linter would ever
  need to notice an unused argument.
- **The output is automatically a legal input elsewhere**, for free.
  `Self::Token: ProofToken<Proposition = Self>` was already required;
  now that's exactly the shape `C` demands too. "Use the output as the
  next input" is what the bound enforces, not a convention.
- **No new type family per site.** Whatever `ProofToken` a site already
  needs to mint for its own claim *is* the obligation type for whatever
  comes next. Nothing named `MyStrictCustomType` gets invented.

### What each existing family needs

- **`AddEvidence`/`calculator.rs`:** `AddEvidence` itself becomes its own
  reflexive `ProofToken` (`type Proposition = AddEvidence`) — not `Sum`
  as first assumed: `Sum::new` is a public, unguarded constructor (any
  `i64` passes), so a bare `Sum` never demonstrated `add_impl` actually
  ran. `AddEvidence` is only reachable from outside the module through
  `add::<V>()`, which already requires `Debit: Witness<V>` and
  `Credit: Witness<V>` — holding one *is* the proof. The
  `#[calculation]` macro's generated `where` bound moved from
  `Establish<#output_ty, ...>` to `Establish<#evidence_name, ...>` to
  match.
- **`Stoplight`:** the actual bug fix. `Establish<Green, KaniVerifier> for
  Yellow` retargets to `Establish<GreenToken, KaniVerifier> for Yellow`
  (and the same for `Yellow`→`Red`, `Red`→`Green`), and the three
  `exchange` bodies swap `input.primary()` for `input.sidecar()`. The
  tokens (`GreenToken`/`YellowToken`/`RedToken`) already exist; they were
  just being ignored in favor of the worthless bare state marker.
- **Accommodation models (worked example: `KaniChannel` →
  `RustStdStandard<Sender<i32>>`):** the real new work, because there is
  no existing token for "this run demonstrated the law" yet.
  `KaniChannel<i32>` is a root `Standard` — asserting a root is
  legitimately unforgeable *by design* (that's `Standard`/`Provenance`'s
  job, not `Establish`'s), but "this specific channel instance delivered"
  is a derived claim, and derived claims need a real token:

  ```rust
  pub struct KaniChannelDeliveryToken(());
  impl ProofToken for KaniChannelDeliveryToken {
      type Proposition = KaniChannel<i32>;
  }

  impl KaniChannel<i32> {
      /// The only way to obtain a `KaniChannelDeliveryToken` — it has no
      /// public constructor of its own.
      pub fn demonstrate_delivery(
          mut self,
          value: i32,
      ) -> Result<KaniChannelDeliveryToken, KaniSendError<i32>> {
          self.send(value)?;
          assert_eq!(self.recv(), Ok(value), "the sent value is receivable");
          Ok(KaniChannelDeliveryToken(()))
      }
  }

  impl Establish<KaniChannelDeliveryToken, KaniVerifier> for RustStdStandard<Sender<i32>> {
      type Token = RustStdSenderToken;
      fn establish(credential: KaniChannelDeliveryToken) -> Self::Token {
          RustStdSenderToken(())
      }
  }
  ```

  Harness call site becomes
  `let delivery = channel.demonstrate_delivery(value).unwrap(); let _token = RustStdStandard::<Sender<i32>>::establish(delivery);`
  — order is now enforced by the compiler, not by comment.

## Resolution

The full retrofit landed in one pass, not staged behind the `KaniChannel`
worked example: the true blast radius was ~65 `Establish` sites, not the
~28 originally estimated, and Rust's whole-workspace compilation means
the `C: ProofToken` bound change breaks every existing impl simultaneously
— there is no way to land the trait change and defer the rest to a
follow-up. Given that, every site got the real fix (a purpose-built
`demonstrate_*` method + witness `ProofToken`, or a reflexive impl where
the type was already gated by construction) in this pass, covering
`sync_mpsc.rs` (10, the original worked example plus its siblings),
`slice.rs` (12), `fs.rs` (10), `io.rs`/`alloc_string.rs` (7),
`process.rs` (10), `sync_lock.rs` (6), `thread.rs` (2), `path.rs`/
`panic.rs`/`std_panic.rs` (5), `std_hash.rs`/`std_time.rs` (2), and the
chained UTF-8 buffer family (`utf8_model.rs`/`primitives.rs`/
`std_ffi.rs`, 3).

Naming settled on `demonstrate_<claim>` for the consuming method and
`Kani<Claim>WitnessToken` for the resulting witness, applied uniformly.
Where two `Establish` impls demonstrated the exact same fact from
identically-shaped harness bodies (e.g. `Sender`/`SyncSender` both
reducing to "send then recv returns the same value", or `SplitN`/
`SplitNMut` sharing byte-for-byte identical assertions), the witness type
was reused rather than duplicated; everywhere else each site kept its own
dedicated witness, matching the codebase's existing per-site convention.
No macro was added — the per-site pattern (one `ProofToken` struct, one
consuming method) turned out lighter than a `proof_credential!`-style
generator would have needed to produce.

## Boundary

- Unlike the prior `amenable_kani`-local model plans (`KANI_UTF8_MODEL_PLAN.md`,
  `KANI_FILESYSTEM_MODEL_PLAN.md`, `KANI_COMPOSE_PLAN.md`), this one
  **does** touch `amenable_core` — the gap is in the constitutional trait
  itself, not a backend-specific modeling concern.
- `amenable_kani`'s `calculator.rs` and `stoplight.rs` are in scope for
  the same reason `amenable_core` is: they're where the bound change's
  fallout (and, for `Stoplight`, an actual bug fix) lands.
- `elicitation`'s `contracts.rs` remains reference material only, per
  `AMENABLE_PLAN.md`'s existing stance. No new trait is added to
  `amenable_core` alongside `Establish`; no struct family resembling
  `Established<P>` is introduced.
- `amenable_creusot`/`amenable_verus`'s own `Witness` backends are
  unaffected; this is specific to `Establish`.

## Non-Goals

- No macro in `amenable_derive` — the per-site pattern turned out light
  enough that generating it wasn't worth the indirection (see
  Resolution).
- No change to `Sidecar`/`Exchange`'s own trait definitions — `Stoplight`
  needed a bug fix in its `Establish`/`Exchange` *impls*, not a change to
  the `Sidecar`/`Exchange` trait shapes themselves.

## Acceptance Criteria

- `Establish<C, V>` requires `C: ProofToken` and takes the credential by
  value, workspace-wide. ✅
- `AddEvidence` (with `AddEvidence: ProofToken` added, reflexively) and
  `Stoplight` (with its three `Establish` impls retargeted to the
  `*Token` types and `exchange` bodies using `.sidecar()`) compile
  against it with no behavioral regression — confirmed by the existing
  `calculation_test.rs` and `stoplight_test.rs` suites still passing
  unchanged. ✅
- Every accommodation-model `Establish` site across `amenable_kani` (the
  full list in Resolution, ~65 sites total) has a real gated credential:
  either a purpose-built `demonstrate_*` witness token, or a reflexive
  `ProofToken` impl on a type already gated by construction
  (`KaniUtf8Buffer<2>`). No site is satisfied by a shim that type-checks
  without closing the actual gap. ✅
- `elicit_doc quality antipatterns` no longer flags any retrofitted
  `establish` parameter as `unused_underscore_arg`. Not yet re-run this
  session — pending.
- `cargo check --workspace`, `cargo clippy --workspace --all-targets`,
  and `cargo test --workspace` are all clean. ✅
- Native Kani verification confirms the retrofit didn't change harness
  semantics for a representative sample spanning every distinct family
  of change: `calculator::add_impl_computes_exact_sum` (reflexive
  evidence-by-construction), all three `stoplight::verify_*_transitions_*`
  harnesses (the real bug fix), four `sync_mpsc` sites including the
  original `Sender`/`Sender<i32>` worked example, and the full
  `utf8_model` → `primitives`/`std_ffi` two-hop chain. ✅ The remaining
  ~55 sites follow the identical mechanical pattern and are covered by
  `cargo test --workspace` rather than individually re-run under Kani.
