//! Isolates `GAAP_LEDGER_PLAN.md`'s Step 1 CBMC timeout:
//! `verify_validate_accepts_a_lawful_transfer` (`ledger.rs`, this
//! crate's own now-retired copy at the time -- `Ledger`'s real methods
//! moved to `amenable_gaap` in Step 7) times out under `just
//! verify-kani-contract` with two free symbolic `i64`s
//! (`amount`, `balance`) and two *concrete* account names
//! ("Alice"/"Bob").
//!
//! **Root cause, fully and precisely isolated (13 real experiments):**
//! a `String` comparison (`payload.from() != payload.to()`,
//! `AccountId`'s own `PartialEq`) evaluated *inside a `#[kani::ensures]`
//! closure* is expensive for CBMC — even though the identical
//! comparison is cheap everywhere else it was tested: as a bare,
//! standalone check
//! ([`account_id_inequality_over_concrete_strings_passes`], 0.5s), and
//! as an ordinary `if` inside `validate`'s own real function body (part
//! of every passing reproduction below). It is not the `Result`
//! pattern, not `String`/heap allocation in general, not the generic
//! `Sidecar`/`Establish`/`Transfer<S, Token>` machinery, not
//! `#[track_caller]`, not the `Exchange` trait's dispatch layer, not
//! multiple error variants, not `#[kani::ensures]`'s mere presence
//! (several intermediate hypotheses here, since disproven — see below),
//! and not the closure's other pieces (`.primary()`'s real `Sidecar`
//! trait dispatch, the `amount > 0` check — both proven cheap by
//! [`ensures_closure_checking_only_amount_passes`]). It is precisely:
//! *this one comparison, evaluated inside this one kind of closure*.
//!
//! The decisive pair, both against a local twin carrying `validate`'s
//! *exact* real body (branching, heap allocation, `&self`/`self.
//! balance`, everything): [`ensures_closure_checking_only_amount_
//! passes`] — `#[kani::ensures]` closure matches `Ok`/`Err` and checks
//! `.primary().amount().value() > 0`, no string comparison anywhere in
//! the closure — **verifies successfully in 1.2s**.
//! [`ensures_closure_with_accounts_distinct_string_comparison`] — the
//! identical closure and body, with *only* `payload.from() !=
//! payload.to()` added to the `Ok` arm — **times out**. That is the
//! entire, isolated difference.
//!
//! [`trivial_ensures_closure_on_the_real_body_fails_fast`] was the
//! pivot that reopened this investigation after `calling_ledger_
//! validate_directly_times_out` seemed to close it on "`#[kani::
//! ensures]`'s mere presence": the identical real branching/allocating
//! body, contracted with the *trivial* closure `result.is_ok()`
//! (matching every `Stoplight` claim's own shape) — **verifies in
//! 1.27s and correctly reports a real, expected `Failed` check** (the
//! trivial claim is genuinely false on the `Err` path) — not a
//! timeout. That result is what redirected the investigation from "the
//! attribute itself" to "the closure's own content."
//!
//! Earlier false trail, kept for the record (gallery discipline: a
//! failed hypothesis is still a real result — every function below
//! whose name ends `_passes` was *predicted* to time out when written,
//! and renamed to match what actually happened): the first eight
//! experiments seemed to isolate the cause to "a symbolic condition
//! selecting an `Ok` arm that heap-allocates."
//! [`bare_result_transfer_payload_passes`],
//! [`symbolic_branch_constructing_real_transfer_validated_passes`],
//! [`returning_real_result_type_from_a_function_passes`],
//! [`symbolic_pending_transfer_extraction_passes`],
//! [`full_combination_inline_without_calling_validate`], and
//! [`three_distinct_error_variants_from_one_function_passes`] each
//! reproduced the *exact* real types, logic, and symbolic branching
//! with no `#[kani::ensures]` anywhere in their call graph — every one
//! of them fast, cleanly falsifying that hypothesis.
//!
//! Real, characterized, precise — not a guess dressed up as one.
//!
//! **What comparisons work inside a `#[kani::ensures]` closure, checked
//! directly rather than assumed:** every real alternative to raw
//! `String` content comparison verifies fast, against `validate`'s
//! exact real body:
//!
//! - [`ensures_closure_with_enum_account_comparison_passes`] — a
//!   fieldless 2-variant enum (`AccountName`), derived `PartialEq`
//!   (discriminant comparison). **0.12s.** Doesn't scale as a general
//!   `AccountId` replacement (a real ledger has many accounts, not a
//!   fixed closed set of two).
//! - [`ensures_closure_with_numeric_id_comparison_passes`] — a bare
//!   `u64` newtype, fully symbolic (`kani::any()`, not a small closed
//!   set). **0.12s.** Scales to an arbitrarily large identity space.
//! - [`ensures_closure_with_uuid_shaped_comparison_passes`] — a `[u8;
//!   16]`-backed newtype modeling `uuid::Uuid`'s real internal
//!   representation (the `uuid` crate isn't a workspace dependency, so
//!   this models the shape directly rather than pulling it in). **0.42s.**
//! - [`ensures_closure_with_id_plus_name_hybrid_passes`] — the
//!   practically useful shape: `{ id: u64, name: AccountId }`, hand-
//!   written `PartialEq` comparing only `.id`. The `String` field is
//!   still constructed (identical allocation cost to today's
//!   `AccountId(String)`) — it's just never touched by the comparison.
//!   **0.83s.** Confirms a real fix doesn't have to sacrifice human-
//!   readable account names, only change what equality *checks*.
//!
//! **Correction, from a real counter-result — read this over the
//! "fixed size, not heap-allocated" framing above:**
//! [`ensures_closure_with_fixed_capacity_string_comparison_times_out`]
//! tested a *fixed-capacity* string (`{ bytes: [u8; 24], len: u8 }`,
//! stack-allocated, `PartialEq` comparing only `bytes[..len]`) —
//! **still times out**, just like `String`. "No heap allocation" was
//! never the real dividing line; it happened to be true of every
//! passing case above by coincidence, because `u64`/`UuidShapedId`/the
//! enum all also share the trait that actually matters: the
//! *comparison itself* has a length fixed at compile time, always the
//! same number of bytes regardless of content. `len` in the fixed-
//! capacity string is itself symbolic, so the comparison is still
//! variable-length even though the *storage* is bounded and stack-
//! allocated — the identical shape this project's own catalogued
//! "symbolic-length memcmp" timeout class already names, just now
//! confirmed to apply *inside a `#[kani::ensures]` closure specifically*,
//! not only to direct symbolic-length iteration.
//!
//! This points to a fix that needs no change to `#[amenable_derive::
//! exchange]`, `kani_ensures!`, or any other shared macro at all — the
//! generated `#[kani::ensures]` wiring was never the problem. The fix
//! lives entirely at the data-model layer: give `amenable_gaap::
//! AccountId` a cheap-to-compare identity whose comparison has a length
//! fixed at compile time (a numeric id, a UUID-shaped byte array — not
//! a capped-but-still-variable-length string, even a stack-allocated
//! one) that `PartialEq` actually uses, alongside whatever human-
//! readable name it already carries. See `GAAP_LEDGER_PLAN.md`'s Step 1
//! for the write-up.
//!
//! **Addendum, landed:** at the time of this investigation `AccountId`
//! was one struct carrying both `id`/`name`, with hand-written
//! `PartialEq`/`Eq`/`Hash`/`PartialOrd`/`Ord` comparing `id` alone —
//! exactly the shape this file's recommendation describes. It's since
//! split into two real types: `AccountId` (bare `Uuid` identity, every
//! comparison trait a plain derive) and `Account` (`id: AccountId,
//! name: String`, the record this file's own harnesses below still
//! construct). The split wasn't driven by a further CBMC finding —
//! this investigation's own fix already worked — but by a second,
//! unrelated tool conflict once this workspace's tracing rollout
//! reached the hand-written `PartialOrd::partial_cmp`: `#[instrument]`
//! wrapping its body in span-entry code made `clippy::
//! non_canonical_partial_ord_impl` misfire on the canonical `Some(self.
//! cmp(other))` delegation it could no longer pattern-match. Deriving
//! instead of hand-writing sidesteps that too, since a derived impl
//! has no source-visible function body for either tool to see.
//!
//! Split into the investigation's own narrative beats, each file's
//! cases still in their original order: [`controls_and_early_hypotheses`],
//! [`heap_allocation_false_trail`], [`real_body_reproduction`],
//! [`ensures_closure_isolation`], [`comparison_alternatives`], and
//! [`uuid_and_fixed_capacity_confirmation`].

mod comparison_alternatives;
mod controls_and_early_hypotheses;
mod ensures_closure_isolation;
mod heap_allocation_false_trail;
mod real_body_reproduction;
mod uuid_and_fixed_capacity_confirmation;
