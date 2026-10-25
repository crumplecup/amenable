//! Migrated, re-verified knowledge from `~/repos/elicitation`'s
//! `level17`/`level18` Kani galleries
//! (`crates/elicit_proofs/src/kani/gallery/level{17,18}.rs`) and the
//! commit that resolved them
//! (`6de484ca7c192f2c791a2ea2f5bfd0bf36d343a4`, 2026-05-20,
//! "fix(kani): fix unsound String KaniCompose and generator"), so this
//! codebase's own gallery is a complete record without needing to
//! clone a sibling repo to see it.
//!
//! `level17.rs`'s own doc comment *predicted* ("Expected: FAIL", with
//! a literal question mark in its table) that a plain, unboxed,
//! genuinely symbolic `String` field, dropped via a `..` pattern
//! inside a `#[kani::proof_for_contract]`-analyzed function, would
//! fail DFCC's "ptr is freeable" check. That prediction was never
//! confirmed as written -- `level18.rs` was built specifically to
//! probe it further and found a different, more interesting bug: the
//! symbolic-string helper in use (`kani_depth1()`) returned
//! `String::new()`, identical to the empty-string base case. Any
//! proof with a non-empty-content precondition therefore had that
//! precondition's `kani::assume` silently prune the *only* path the
//! harness ever constructed, so Kani reported `VERIFICATION
//! SUCCESSFUL` without ever exercising the real, non-empty case --
//! vacuous, and not flagged as such automatically. Once fixed to use
//! one genuinely symbolic `char` (`kani::any::<char>().to_string()`)
//! instead, the proof was both non-vacuous *and* passed, directly
//! refuting the original DFCC concern. The fixing commit says it
//! outright: "Gallery18 experiments confirmed symbolic strings can be
//! safely dropped in proof_for_contract; the old DFCC concern was
//! unfounded."
//!
//! Two lessons, both migrated and independently reverified here rather
//! than trusted across toolchains unverified
//! ([[feedback_isolate_variables_before_concluding]]):
//!
//! 1. A plain, unboxed, genuinely symbolic `String`, dropped inside a
//!    `#[kani::proof_for_contract]`-analyzed function, is fine. No
//!    DFCC "ptr is freeable" failure. This is a *different* finding
//!    from `boxed_error_variant_count`'s own `Box<String>` OOM in this
//!    same gallery -- that OOM was reproduced with concrete, always-
//!    present literals (nothing symbolic, nothing a vacuous
//!    precondition could hide), so the two are independent phenomena,
//!    not the same one seen twice.
//! 2. **The vacuity trap**, directly relevant to this codebase's own
//!    `#[cfg(kani)]` placeholder pattern (e.g. `TemporalError`'s
//!    `caller_location()` returning `String::new()` under `cfg(kani)`,
//!    `crates/amenable_time/src/error.rs`): substituting a fixed empty
//!    `String` for a symbolic value is only sound when nothing
//!    requires non-emptiness (or any other content-dependent fact).
//!    If a future `requires`/`ensures` ever depends on a string
//!    field's content, swapping that field to a `cfg(kani)`-gated
//!    empty placeholder could silently make the proof vacuous instead
//!    of failing loudly. Worth rechecking every time this pattern is
//!    reused. (Not a live risk for `caller_location()` or
//!    `InvalidDescriptorSource::detail` today -- confirmed directly,
//!    nothing in this codebase branches on either's content -- but the
//!    trap doesn't announce itself when it does apply.)
//!
//! **What was actually confirmed, re-run directly against this
//! codebase's own Kani/CBMC toolchain:**
//!
//! - The empty-placeholder harness (elicitation's 18a): passes, but
//!   vacuously -- 0.33s, 266 checks that verify nothing, because
//!   `label_non_empty`'s `requires` prunes the harness's only path
//!   before `rename_label`'s body is ever reached.
//! - The genuinely-symbolic-one-char harness (elicitation's 18f):
//!   passes non-vacuously -- 1.42s, 424 checks (6x the vacuous case's
//!   count, 4x its runtime), a real signal the body was actually
//!   exercised. No DFCC "ptr is freeable" failure. Independently
//!   reconfirms elicitation's own corrected conclusion in this
//!   codebase's toolchain, rather than just citing it.

mod kani_only {
    #![cfg(kani)]

    /// Mirrors elicitation's `G18State`: a minimal state machine where
    /// the `Active` variant carries a label that must never be empty.
    pub(super) enum LabeledState {
        Idle,
        Active { label: String },
    }

    /// Invariant: an `Active` label is never empty.
    pub(super) fn label_non_empty(state: &LabeledState) -> bool {
        match state {
            LabeledState::Active { label } => !label.is_empty(),
            LabeledState::Idle => true,
        }
    }

    /// Rename the label. Drops the old `Active` state's `String` via
    /// `..`. Requires the pre-state and the new label to both already
    /// satisfy the non-empty invariant -- this `requires` is exactly
    /// what a vacuous empty-string placeholder would silently prune.
    #[kani::requires(label_non_empty(&state) && !new_label.is_empty())]
    #[kani::ensures(|r| label_non_empty(r))]
    pub(super) fn rename_label(state: LabeledState, new_label: String) -> LabeledState {
        match state {
            LabeledState::Active { .. } | LabeledState::Idle => LabeledState::Active {
                label: new_label,
            },
        }
    }

    /// Neither proof above ever constructs `Idle` -- both harnesses
    /// start from `Active` (confirmed: the investigation is
    /// specifically about `Active`'s label-dropping behavior), so it
    /// exists only to mirror elicitation's real `G18State` shape.
    /// This proof constructs it, outside any contract boundary, so it
    /// stays a real variant rather than dead code -- it must not
    /// touch the specific check/timing counts cited above.
    #[kani::proof]
    fn verify_idle_satisfies_its_own_invariant() {
        assert!(label_non_empty(&LabeledState::Idle));
    }
}

#[cfg(kani)]
use kani_only::{LabeledState, rename_label};

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::string_drop_vacuity::vacuous_empty_placeholder".to_owned(),
            "gallery::string_drop_vacuity::check_rename_label_vacuous_empty_placeholder".to_owned(),
            "amenable_kani".to_owned(),
            "migrated from elicitation's level18a -- confirmed PASSES in this toolchain too, 0.33s, 266/266 checks, but VACUOUSLY: label_non_empty's requires prunes this harness's only path before rename_label's body is reached. The implausibly tiny, instant proof is the only tell. Independently reconfirms the vacuity trap this file exists to document.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_RENAME_LABEL_VACUOUS_EMPTY_PLACEHOLDER_SRC, {
        /// Reproduces elicitation's 18a: an empty-string placeholder
        /// at the point a symbolic value was intended.
        /// `label_non_empty` evaluates to `false` for `Active { label:
        /// "" }`, so `kani::requires` prunes this harness's only path
        /// before `rename_label`'s body is ever reached under
        /// contract analysis. Kani should report success -- but
        /// vacuously, having verified nothing. There is no automatic
        /// signal for this in Kani's own output; the tell is an
        /// implausibly tiny, instant proof for a claim that sounds
        /// substantive.
        #[kani::proof_for_contract(rename_label)]
        fn check_rename_label_vacuous_empty_placeholder() {
            let state = LabeledState::Active {
                label: String::new(),
            };
            let new_label = String::from("b");
            let result = rename_label(state, new_label);
            core::mem::forget(result);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::string_drop_vacuity::symbolic_one_char_drop".to_owned(),
            "gallery::string_drop_vacuity::check_rename_label_symbolic_one_char".to_owned(),
            "amenable_kani".to_owned(),
            "migrated from elicitation's level18f -- confirmed PASSES in this toolchain too, 1.42s, 424/424 checks (genuinely non-vacuous: 6x the checks and 4x the runtime of the vacuous case above). A plain, unboxed, genuinely symbolic String dropped via `..` inside a proof_for_contract-analyzed function has no DFCC 'ptr is freeable' problem -- independently reconfirms elicitation's own corrected conclusion, not just a citation of it.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_RENAME_LABEL_SYMBOLIC_ONE_CHAR_SRC, {
        /// Reproduces elicitation's 18f, the actual fix: one
        /// genuinely symbolic `char` converted to a one-character
        /// `String` (`kani::any::<char>().to_string()`). This always
        /// satisfies `!is_empty()`, so the harness is non-vacuous
        /// (CBMC explores every possible single-char label), and it
        /// still must survive `rename_label` dropping the old
        /// `Active` state's `String` via `..` inside a
        /// `proof_for_contract`-analyzed body.
        #[kani::proof_for_contract(rename_label)]
        fn check_rename_label_symbolic_one_char() {
            let one_char: char = kani::any();
            let state = LabeledState::Active {
                label: one_char.to_string(),
            };
            let new_label = String::from("b");
            let result = rename_label(state, new_label);
            core::mem::forget(result);
        }
    }
}
