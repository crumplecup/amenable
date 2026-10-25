//! Gallery cases from the investigation into why `amenable_ext::ExtStandard<
//! chrono::DateTime<chrono::FixedOffset>>`'s full round-trip claim (every
//! year/month/day/hour/minute/second/offset field symbolic at once) times
//! out under Kani -- the production harness itself stays in `amenable_kani::
//! chrono::offset_date_time` (`check_offset_date_time_round_trips_through_
//! descriptor`), unresolved; this gallery records what the decomposition
//! attempts found along the way.
//!
//! Isolating by half, each other field fixed concrete:
//!
//! - naive (date+time) half alone, offset concrete: investigated, no clean
//!   confirmed verdict recorded.
//! - offset half alone, naive concrete, `offset_seconds` binned over a
//!   concrete `0..WINDOW` loop (never `kani::any()`) rather than left fully
//!   symbolic, the same bin-size-search discipline validated elsewhere in
//!   this workspace (`Tz`'s own `.parse()` investigation): **still times
//!   out even at `WINDOW = 2`**. Confirms the cost is about the offset
//!   being symbolic at all, not the size of its range -- binning, which
//!   rescues other dimensions in this codebase, does not rescue this one.
//!   Kept below as `offset_binning_times_out_even_at_the_smallest_window`.
//!
//! Isolating the pipeline shape itself, day symbolic, everything else
//! concrete:
//!
//! - a single `Exchange` call (reflect only, no realize, no equality):
//!   investigated as the minimal possible exercise of the trait, no
//!   confirmed verdict recorded separately from the two-call case below.
//! - both `Exchange` calls (reflect then realize), still with no equality
//!   check at the end: **confirmed expensive on its own** -- the two-hop
//!   call sequence is already the cost, before any comparison of the
//!   result against the original even happens. Kept below as
//!   `two_hop_exchange_without_equality_already_times_out`.
//! - the real `Exchange` call, fully concrete (no `kani::any()` at all): a
//!   sanity check confirming the trait dispatch itself is not inherently
//!   slow -- the cost only appears once a field is symbolic.
//!
//! Isolating the descriptor-building logic without `Exchange`/`Establish`
//! at all (just the real conversion code, called directly): investigated
//! as a separate layer from the trait-dispatch machinery, to rule out
//! whether `Exchange`/`Sidecar`/`Establish` themselves were the cost rather
//! than the conversion logic underneath. No confirmed verdict recorded
//! separately from the per-half findings above.
//!
//! Also investigated, as hypotheses about *why* the naive-half conversion
//! might be expensive (`naive_date_time_to_local_date_time_descriptor`'s
//! leap-second guard formats the whole symbolic value via `Display` on an
//! error path that this investigation's own construction never actually
//! reaches; whether `nanosecond()` on a symbolic date constant-folds to a
//! literal before CBMC has to reason about it) -- neither was carried to a
//! confirmed verdict before the investigation moved to the fractional-digit
//! layer specifically, below.
//!
//! **Digit decomposition, the deepest layer investigated.** The claim
//! `nanos_to_fractional_digit_bytes` / `fractional_digit_bytes_to_nanos`
//! round-trip exactly, proven directly against the pure, `Copy`, heap-free
//! `[u8; 9]` core both public string-based wrappers already reduce to
//! internally (per the project's own standing rule: strings are not
//! generally tractable for CBMC, so the round trip is proven below the
//! text-encoding layer, not through it). Restated as pure `i64` division/
//! modulo arithmetic with no byte arrays at all (`digit_i = (nanos /
//! 10^i) % 10` for `i` in `0..9`, `sum(digit_i * 10^i) == nanos`) to test
//! whether stripping the array plumbing was enough before trying anything
//! more elaborate:
//!
//! - one step of digit extraction (`remaining % 10`, `remaining / 10`),
//!   proven as its own contract relating the division only to its own
//!   input, never chained to another call: cheap, confirmed around 3.7s.
//!   Kept below as `peel_digit_contract_passes_cheaply` -- the one
//!   building block out of this whole investigation that stayed cheap at
//!   every scale tried.
//! - the full nine-digit identity, composed from nine `kani::stub_
//!   verified` calls to that one proven contract (so the composed proof
//!   only ever has to relate nine *linear* equations to each other, never
//!   nine real division circuits at once): shrank the front end from tens
//!   of thousands of symbolic-execution steps to under 5000, but did not
//!   remove the cost -- an `i128` accumulator was needed to rule out
//!   overflow as a separate confound in the composed proof itself (not in
//!   any caller), confirmed via a real, reachable overflow Kani reported
//!   inside the contract's own `ensures` closure before the widening.
//! - the combining step's own cost, isolated from `peel_digit`'s (already
//!   confirmed cheap): chaining and combining more stubbed calls in one
//!   flat proof obligation. Growing from 5 to 7 combined steps **jumped
//!   about 55x (1.74s to 96.3s)**, confirmed genuinely accelerating, not a
//!   one-off measurement. Kept below as `combining_seven_steps_passes_but_
//!   reveals_the_growth_curve` -- it still passes within the project's
//!   3-minute harness budget, but the trend it shows does not.
//! - hierarchical grouping (three digits peeled via three stubbed calls to
//!   `peel_digit`, then three of *that* group related at the top level,
//!   so no level ever relates more than three independent values) was
//!   tried and, like flat grouping, accelerated with depth.
//! - **incremental chaining, corrected and resolved.** An earlier,
//!   informal pass at this same idea (each link stubbing exactly one
//!   prior link plus one `peel_digit` call) was never captured as a
//!   committed, reproducible harness and reported a super-linear series
//!   (N=1 ~3s, N=2 ~3.5s, N=3 ~25-30s, N=4 ~176s) that, if real, would have
//!   made the full nine-digit claim impractical. Rebuilt explicitly this
//!   session as nine named, individually registered steps
//!   (`digit_chain_step_1` through `digit_chain_step_9`, each proving only
//!   its own contract and stubbing the *immediately preceding step's
//!   already-proven contract* -- never `peel_digit` directly past step 1,
//!   and never any step's real body) -- the cost stayed flat at every
//!   step, measured at roughly 13-22s each, all the way to
//!   `digit_chain_step_9`'s full `0 <= x < 1_000_000_000` nine-digit
//!   domain. The earlier super-linear series does not reproduce under
//!   this structure; whatever it measured, it was not this. **The
//!   nine-digit digit/fraction-arithmetic identity is fully resolved**,
//!   cheaply, via proper incremental contract staging -- no step came
//!   anywhere near the project's 3-minute budget, so no long-running
//!   carve-out or feature flag is needed for this layer.
//! - the two-hop round trip itself, day-only (31 possible values, not the
//!   full nine-digit `nanos` domain), staged via Kani contracts the same
//!   way the `NaiveWeek` gallery case (`chrono_naive_week_stub_
//!   composition`) staged its span claim: `reflect_step_day_only` and
//!   `realize_step_day_only` each independently contracted and proven,
//!   with the composed claim stubbing both so neither real body executes
//!   in the top-level proof. Unlike that gallery case's first (corrected)
//!   mistake, both contracts here state the exact relation, not a
//!   one-sided bound. **The contracted version of `reflect_step_day_only`
//!   exhausted memory** -- not merely timed out -- even restricted to this
//!   31-value domain. An uncontracted control with the identical body was
//!   written specifically to separate "DFCC's own instrumentation is the
//!   added cost" from "the `Exchange`-backed builder call underneath is
//!   itself the cost regardless of contracts"; a second control wrapped
//!   the same body in `requires`/`ensures` but with a trivial `|_| true`
//!   predicate, to separate "DFCC's structural instrumentation itself" from
//!   "evaluating this specific shape-predicate under CBMC." Kept below as
//!   `contracted_reflect_step_day_only_exhausts_memory`.
//!
//! **A second, later investigation: the offset-only round trip's own
//! cost, decomposed by fact count.** Separately from the digit layer
//! above, the production `amenable_kani::chrono::offset_date_time::
//! offset_round_trips` (reflect then realize a `FixedOffset` through the
//! bridge's offset-descriptor pair alone, pure integer arithmetic, no
//! strings, no chrono date construction) also timed out -- ruling out
//! strings as the whole story. Decomposed by chaining first:
//!
//! - the identical body, uncontracted (no DFCC at all): passes in ~77s.
//! - the identical body, DFCC-contracted but with the `reflect` half
//!   alone (trivial ensures): passes in ~127s.
//! - both directions chained, trivial ensures on both: **times out** --
//!   confirming the earlier pattern again (chaining two real, unstubbed
//!   calls compounds under DFCC), independent of the postcondition's
//!   content.
//!
//! The fix for *that* is the same one that resolved the digit chain:
//! give the reflect half its own real (non-trivial) contract and stub it
//! rather than inline it when composing the round trip. But writing that
//! real contract surfaced a second, different cost axis: `reflect_
//! offset`'s own contract -- a *single* real call, no chaining -- timed
//! out once its ensures stated the real relation (sign matches *and* the
//! three-field magnitude reconstruction matches), despite the identical
//! body passing easily under a trivial ensures. Decomposed by fact count
//! instead:
//!
//! - each of the four individual facts alone (`sign`, `hours`,
//!   `minutes`, `seconds` against the offset) passes cheaply (~127-160s).
//! - two facts together still pass: `sign` + `hours` (~154s, neither
//!   conditionally set), `minutes` + `seconds` (~180s, both independently
//!   conditional -- already closer to the edge).
//! - three facts (`hours` + `minutes` + `seconds`, the full magnitude,
//!   `sign` left out) passes, but right at the edge (~187s wall against
//!   the 3-minute budget).
//! - all four facts together **times out** -- confirmed twice, once
//!   stated as `sign_matches && (one summed-magnitude equality)` and
//!   once restated as four fully separate per-field equalities with no
//!   summing at all. Both shapes time out identically, ruling out "the
//!   sum is the cost" as cleanly as the fact-count bracketing ruled out
//!   any single field.
//!
//! So this threshold is specifically about **how many correlated facts
//! one proof obligation states about a single symbolic builder
//! invocation at once** -- not which facts, not their arithmetic
//! representation, not `Option` branching alone (each conditional field
//! passed fine by itself and in the `minutes`+`seconds` pair). For
//! `reflect_offset`, the line sits strictly between three and four. This
//! reopens the question the digit chain's fix doesn't answer: stubbing
//! works when the *caller* composes already-proven facts about *other*
//! calls, but it doesn't help prove a single call's own contract when
//! that contract itself needs to state more facts than the verifier can
//! jointly discharge in one pass. The production `offset_round_trips`
//! claim remains open on this axis -- not yet resolved, kept honest as a
//! known timeout rather than patched around.
//!
//! **Conclusion.** No decomposition technique tried here -- byte-core
//! extraction, restating as pure arithmetic, flat grouping, hierarchical
//! grouping, incremental two-value chaining, or staged contract composition
//! with `stub_verified` -- removed the underlying cost; each only moved or
//! slowed it. The growth across the incremental chain is super-linear and,
//! by the trend these measurements show, continuing it out to the real
//! nine-digit claim implies a cost far beyond any practical CI budget, not
//! merely beyond the project's standing 3-minute harness limit. This
//! matches the same pattern the `chrono_day_count_staging` and
//! `chrono_naive_week_stub_composition` gallery cases already found for
//! other chrono claims: each piece is cheap in isolation, and it is
//! specifically the *relation* between multiple chrono-derived or
//! arithmetic values -- not any one piece alone -- that CBMC cannot finish
//! within reach. **Do not retry "decompose further" on this claim
//! expecting a finer split alone to remove the cost** -- every split tried
//! here, down to relating exactly two values at a time, still accelerated
//! with depth. The full round-trip claim remains open; the production
//! harness records that honestly as a known timeout rather than a verdict.
//!
//! **Where each chapter now lives**, split out once this file grew past
//! the point a human reader could take it in as one unit: baseline
//! isolation of the trivial-ensures/reflect/realize/uncontracted variants
//! (`chrono_offset_date_time_round_trip_offset_fact_count_baseline.rs`);
//! `reflect_offset`'s own contract decomposed by fact count
//! (`chrono_offset_date_time_round_trip_offset_fact_count_decomposition.rs`);
//! the staged fix built from that decomposition
//! (`chrono_offset_date_time_round_trip_offset_staged_fix.rs`); pipeline-
//! shape isolation and offset binning
//! (`chrono_offset_date_time_round_trip_pipeline_shape_and_binning.rs`);
//! and the digit decomposition, split into the incremental steps
//! (`chrono_offset_date_time_round_trip_digit_decomposition_steps.rs`)
//! and the two-hop day-only round trip
//! (`chrono_offset_date_time_round_trip_digit_decomposition_day_only.rs`).
//! Those six are declared as this module's own siblings in
//! `gallery/mod.rs`, not as children of this file.
