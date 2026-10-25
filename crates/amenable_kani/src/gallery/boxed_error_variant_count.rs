//! Gallery cases from the investigation into why `amenable_ext::
//! narrow_to_u8` (a trivially cheap, bounded `u32 -> u8` narrowing
//! helper, confirmed cheap alone and confirmed cheap called three times
//! in a row under a plain `#[kani::proof]`) blows CBMC's memory the
//! moment the exact same three calls are wrapped in a function that is
//! *itself* under `#[kani::proof_for_contract]` analysis -- even with a
//! trivial `|_| true` ensures on both sides. Confirmed directly from a
//! real `cargo kani` run, capped at 8 GiB of virtual memory to fail fast
//! instead of taking the whole machine down (no swap configured; an
//! uncapped run before this one did exactly that):
//!
//! ```text
//! converting SSA
//! Out of memory
//!
//! CBMC failed with status 6
//! VERIFICATION:- FAILED
//! ```
//!
//! The `aborting path on assume(false)` lines immediately before the
//! crash are almost all `alloc::raw_vec::RawVecInner::deallocate` --
//! heap-deallocation paths tied to `TemporalError`'s own
//! `Box<TemporalErrorKind>` field. The real function's own error path
//! only ever constructs exactly one of `TemporalErrorKind`'s nine
//! variants (`InvalidDescriptor`, via `invalid(...)`) -- never the
//! other eight. Hypothesis: a `Box`-wrapped sum type with many variants
//! is expensive for CBMC/DFCC to model across a contract boundary
//! regardless of which variant a given call site can actually produce,
//! and restricting to the variant(s) actually reachable should shrink
//! that cost. These two standalone toy types -- no connection to the
//! real error machinery, so this isolates exactly one variable -- test
//! that directly: a nine-variant `Box`ed enum mirroring
//! `TemporalErrorKind`'s rough shape, against a one-variant version with
//! the same payload, under the identical call pattern that crashed.
//! Error handling built this way (an umbrella `Kind` enum boxed behind
//! one error struct, CLAUDE.md pattern 2/3) is central to this whole
//! codebase, so whatever this finds generalizes well past this one
//! chrono bridge function.
//!
//! **What was actually confirmed, in order:**
//!
//! - Nine variants, boxed (mirroring `TemporalErrorKind` exactly):
//!   out-of-memory crash, reproducing the real failure exactly.
//! - One variant, boxed: **still** out-of-memory, even though the
//!   program shrank to 17,027 steps and 246 remaining VCCs -- a tiny
//!   problem by verification standards. This disproves the variant-count
//!   hypothesis outright: restricting to the variant(s) actually
//!   producible does not fix it.
//! - Nine variants, *not* boxed under `cfg(kani)` (same payload,
//!   `detail: String` kept): **still** out-of-memory. This disproves
//!   "`Box` alone is the cause": `Payload`'s own `String` field is
//!   backed by a heap-allocating `Vec<u8>` internally -- the exact same
//!   `RawVecInner::deallocate` machinery `Box` uses. Removing the outer
//!   `Box` without removing the inner `String` changes nothing.
//! - Nine variants, not boxed, payload holds a plain `u32` instead of a
//!   `String` -- **no heap allocation anywhere in the error at all**:
//!   passes cleanly in 9.6s (32/32 checks, `VERIFICATION:- SUCCESSFUL`).
//! - A fifth case tests a specific inversion a reviewer proposed: instead
//!   of boxing the whole `*Kind` enum (as above), keep the enum's own
//!   discriminant tiny and unboxed -- most variants carry no heap
//!   allocation -- and box only the one heavy variant's own payload
//!   (`enum SmallDiscriminantKind { Tiny(u32), Heavy(Box<Payload>) }`,
//!   never itself wrapped in an outer `Box`). The reasoning behind the
//!   proposal: CBMC's cost should come from modeling the max-sized
//!   variant inline within a boxed union, so moving the `Box` one level
//!   inward -- so it wraps only the field that needs it -- should let
//!   the union itself (and the solver's reasoning about the enum type)
//!   stay small. Confirmed **still out-of-memory**, same
//!   `RawVecInner::deallocate`-driven failure during SSA-to-SAT
//!   conversion, at 18,213 steps / 253 remaining VCCs. This directly
//!   rules out "the whole enum is boxed" as the driver too: moving the
//!   `Box` from around the enum to around just the heavy field, while
//!   *keeping the exact same reachable heap allocation* on the error
//!   path, changes nothing.
//!
//! **First-pass conclusion, later revised below.** The driver is heap
//! allocation itself -- `Box`, `String`, `Vec`, anything backed by
//! `RawVec` -- reachable anywhere in a function's error path, when
//! that function is called (even unstubbed, even unboxed, even with a
//! single trivial variant) from inside another function under
//! `#[kani::proof_for_contract]` analysis. It is not about how many
//! variants the error enum has, not about string *content* (no proof
//! here ever inspected a string's bytes), not specifically about
//! `Box`, and not about *where* the `Box` sits relative to the enum
//! (whole-enum-boxed and only-the-heavy-field-boxed both OOM
//! identically, case five above).
//!
//! **This turned out to be too broad.** A later escalation ladder
//! (further down this file), built one primitive at a time
//! specifically to pull apart what this first pass conflated, found
//! that `Box<u32>`, `Vec<u32>`, a bare `String`, and even `String`
//! living directly inside an enum variant (with or without an extra
//! outer struct layer, boxed or not) **all pass cleanly** on their
//! own. The actual trigger, isolated precisely, is narrower: `Box`
//! wrapping a type that is *itself* already heap-allocating
//! (`Box<String>`, confirmed sufficient entirely on its own, no enum
//! or wrapper struct required) -- a doubled heap indirection, not
//! heap allocation per se. Two further rungs sharpen this precisely:
//! a bare `&'static str` (a fat pointer into static data, no
//! allocation, no `Drop` impl) and `Box<&'static str>` (a `Box`
//! wrapping that same pointer) **both pass cleanly** -- 9.5s and
//! 29.8s respectively, the fastest and cheapest rungs on the whole
//! ladder. So it is specifically "`Box` wraps a type that *owns* a
//! heap allocation" (has its own `RawVec`/`Drop`-driven
//! deallocation), not "`Box` wraps anything that happens to contain a
//! pointer" -- a reference, boxed or not, is never the problem.
//!
//! **That rule was still wrong, in the other direction.** It predicts
//! `Box<Vec<u32>>` should OOM the same way `Box<String>` does -- `Vec`
//! is `RawVec`-backed exactly like `String`. It doesn't: confirmed
//! **passes**, 76.5s, 159/159 checks. Nor does `Box<Vec<u8>>`,
//! matching `String`'s own internal byte type exactly (confirmed
//! passes, 67.0s, 152/152 checks) -- ruling out element type as the
//! explanation too. `Box<Vec<T>>` passes for every `T` tried; only
//! `Box<String>` OOMs. The true boundary, as far as this ladder
//! reaches, is narrower still: it is specific to `String`, not to
//! "any `RawVec`-owning type behind a `Box`" -- something about
//! `String`'s own representation or drop glue (UTF-8 invariant,
//! `Display`/`Debug` machinery, or some CBMC-model-specific detail of
//! `std::string::String` vs. `std::vec::Vec`) that `Vec` does not
//! share, not yet pulled apart further. This still fully explains the
//! real crash:
//! `TemporalError`'s `Box<TemporalErrorKind>`, with
//! `InvalidDescriptorSource`'s `String` fields nested inside the
//! boxed enum, is exactly this shape. It's that DFCC's own
//! contract-checking instrumentation, combined with CBMC's heap-object
//! modeling, blows up the SSA-to-SAT encoding when a boxed
//! allocation's own contents require further heap-object modeling --
//! not simply because heap allocation exists somewhere in the
//! picture. The real fix for `amenable_ext::narrow_to_u8` (and any
//! other function reachable this way) remains to avoid *all* heap
//! allocation on its error path specifically under `cfg(kani)`
//! (confirmed sufficient by `no_box_no_heap_fallible` above), while
//! leaving the real, non-Kani behavior (string details, `miette`
//! rendering) completely unchanged -- since nothing in this codebase
//! ever pattern-matches on `TemporalErrorKind`'s variants for its own
//! control flow (confirmed directly: every `TemporalErrorKind::`
//! reference anywhere in `amenable_ext`/`amenable_kani` is a
//! constructor call, never a `match`), the error's internal
//! representation is free to differ under Kani as long as the public
//! shape stays identical.
//!
//! **Open thread, not yet resolved.** `no_box_fallible` above (nine
//! variants, each carrying a two-field `Payload { detail: String,
//! line: u32 }`, with *no* `Box` anywhere under `cfg(kani)`) still
//! OOMs -- which looks like a second, independent route to the same
//! failure, since the later ladder's unboxed `WrappedMixedError`/
//! `MixedKind`/bare-`String` cases all pass. The two variables
//! conflated between them -- variant *count* (9 vs. 2) and payload
//! shape (two-field `Payload` struct vs. a bare `String`) -- have not
//! been pulled apart from each other. Whichever it turns out to be,
//! it doesn't change the recommended fix (remove all heap allocation
//! under `cfg(kani)`, already confirmed sufficient), so this is left
//! as a genuinely open question rather than chased further here.
//!
//! **Prior art in `~/repos/elicitation`, with a correction.**
//! `~/repos/elicitation` has already chased two related but distinct
//! failure classes -- but the first one below required digging past a
//! superseded hypothesis to find the actual, confirmed conclusion:
//!
//! - `crates/elicit_proofs/src/kani/gallery/level17.rs` isolates a
//!   *plain, unboxed* `String` field dropped inside a
//!   `#[kani::proof_for_contract]`-analyzed function. Concrete content
//!   passes (17a/17b, matching this file's rung 3). 17c's doc comment
//!   *predicts* ("Expected: FAIL", literally with a question mark in
//!   the table) that symbolic content would fail DFCC's "ptr is
//!   freeable" check -- but that prediction was never the final word.
//!   `level18.rs`, built specifically to probe this further, found the
//!   real bug was something else entirely: their `kani_depth1()`
//!   helper returned `String::new()`, identical to depth-0, which made
//!   any proof with a non-empty precondition **vacuous** (the
//!   `kani::assume` silently pruned the only path, so the proof
//!   reported PASS without ever exercising the real case). Once fixed
//!   to return one genuinely symbolic char (`kani::any::<char>()
//!   .to_string()`), the proof was both non-vacuous *and* passed --
//!   directly refuting the DFCC concern. Confirmed by the fixing
//!   commit itself (`6de484ca`, 2026-05-20, "fix(kani): fix unsound
//!   String KaniCompose and generator"): "Gallery18 experiments
//!   confirmed symbolic strings can be safely dropped in
//!   proof_for_contract; the old DFCC concern was unfounded." So
//!   elicitation's own conclusion is the *opposite* of what 17c
//!   predicted: a plain, unboxed, genuinely symbolic `String`, dropped
//!   under `proof_for_contract`, is fine. This does not overturn
//!   anything in this file -- every `Box<String>` rung here used
//!   concrete, always-present literals (never a vacuous precondition
//!   to hide behind), so this file's OOM is a separate, independently
//!   reproduced phenomenon from elicitation's retracted hypothesis,
//!   not a confirmation of it. The real, transferable lesson is the
//!   **vacuity trap**: substituting `String::new()` for a symbolic
//!   value under `cfg(kani)` is only sound when nothing requires
//!   non-emptiness (or any other content-dependent precondition) --
//!   worth rechecking before applying this file's own "empty the
//!   String under cfg(kani)" recommendation to any real field that
//!   does have such a precondition (`InvalidDescriptorSource`'s
//!   `detail` currently has none, confirmed directly: nothing in this
//!   codebase ever branches on its content).
//! - `crates/elicitation_kani/src/diag.rs` and
//!   `crates/elicit_proofs/src/kani/diag.rs` chase a *different*
//!   mechanism: large struct variants inflating an enum's overall
//!   union footprint, fixed by boxing the **struct variant itself**
//!   (`Box<ConnectionProfile>`) -- the opposite of what helped in this
//!   file's `small_discriminant` rung, where boxing the heavy variant
//!   changed nothing. Not the same failure class as this file's;
//!   worth not conflating the two.
//! - `elicitation::verification::types::Utf8Bytes<const MAX_LEN:
//!   usize>` (`crates/elicitation/src/verification/types/utf8.rs`) is
//!   the production answer to "how do you represent a bounded string
//!   to Kani cheaply": a fixed-size **stack array** (`[u8; MAX_LEN]` +
//!   `len`), never `String`/`Vec`/`RawVec` internally, with real
//!   byte-by-byte UTF-8 validation swapped under `cfg(kani)` for a
//!   single `kani::any::<bool>()` -- sidestepping the exact
//!   `run_utf8_validation` loop blowup this file's rung 12 hit, by
//!   never calling it at all. A stronger candidate for
//!   `InvalidDescriptorSource`'s eventual production fix than simply
//!   emptying the `String` under `cfg(kani)`, if genuine symbolic
//!   string reasoning is ever wanted there.
//!
//! **Retired branch: mirror/proxy types for `narrow_to_u8` and
//! `fixed_offset_to_utc_offset_descriptor` specifically.** A later
//! session built `gallery::narrow_to_u8_mirror` and `gallery::
//! fixed_offset_descriptor_mirror` -- `MirrorTemporalError`, a
//! `KaniUtf8Buffer`-backed stand-in for `TemporalError`, substituted
//! at every real call site -- to test whether the *unrestricted*
//! domain (`requires(true)`, the `Err` arm genuinely reachable, not
//! assumed away) could be made cheap by swapping the error
//! representation alone. It confirmed two things worth keeping: the
//! proxy approach does work end to end (mirror contract, the real
//! three-call pattern, and `stub_verified` composition all passed),
//! and `stub_verified`'s own postcondition-checking overhead can cost
//! *more* than it saves once the inlined body is already cheap
//! (112.6s stubbed vs. 58.4s unstubbed on the same claim) -- a
//! reusable data point about when not to reach for stubbing by
//! default.
//!
//! Both files were removed once two things became clear. First, the
//! real wall was already resolved more simply, directly on the real
//! functions, via domain restriction: `narrow_to_u8`'s own `requires
//! (value <= 255)` (every real call site's actual domain, confirmed
//! by inspection) makes its `Err` arm -- and therefore the
//! `#[track_caller]`-driven `ComponentOutOfRangeSource` construction
//! inside it -- provably unreachable, sidestepping the heap-allocation
//! question entirely rather than needing a cheap substitute for it.
//! See `chrono_offset_date_time_round_trip_pipeline_shape_and_binning.
//! rs`, which proves the real, un-mirrored `narrow_to_u8` directly,
//! repeated three times, contracted, with no proxy involved. Second,
//! the mirror's own premise was inaccurate: its doc comment described
//! the real `narrow_to_u8` as `(value: u32, context: &'static str)`;
//! the real signature has always been `(value: u32, component:
//! TemporalComponent)` -- a small `Copy` enum, never a string. The
//! real `String` this whole investigation chased lives one level
//! deeper, in `ComponentOutOfRangeSource::file` (the `#[track_caller]`
//! call-site path) -- and that field is already substituted for a
//! cheap `String::new()` under `cfg(kani)` in production
//! (`crates/amenable_time/src/error/validation.rs`), the same pattern
//! applied across every error-source type in `amenable_time::error`.
//! The mirror ladder's general finding (proxy types work, stubbing
//! isn't always a win) stays true and is recorded here; the two files
//! themselves added no coverage the production code and the direct
//! proof above didn't already provide, so they were retired rather
//! than kept as a redundant parallel path.
//!
//! **Where each chapter now lives**, split out once this file grew
//! past the point a human reader could take it in as one unit:
//! first-pass hypothesis test, variant-count rungs
//! (`boxed_error_variant_count_first_pass_variant_count.rs`) and
//! box-placement inversion
//! (`boxed_error_variant_count_first_pass_box_placement.rs`);
//! escalation ladder rungs 1-5
//! (`boxed_error_variant_count_escalation_primitives.rs`), rung 6 plus
//! its two bounded-`String` follow-ups, rungs 12-13
//! (`boxed_error_variant_count_escalation_string_boxing.rs`), rungs
//! 7 and 9-11 (`boxed_error_variant_count_escalation_references_and_vectors.rs`),
//! and the production-fix candidates, rung 14 plus the three proxy
//! variants (`boxed_error_variant_count_production_fix_candidates.rs`).
//! Those six are declared as this module's own siblings in
//! `gallery/mod.rs`, not as children of this file.
