# Proof Assessment Rubric and Review Ledger

## Implementation Status

Implemented in `amenable assess`. The command records versioned JSON Lines
assessments with `version: "0.1.0"` and a CLI-produced Unix-seconds
`timestamp`, reports score means and distributions, and lists the registered
Kani proofs awaiting a first assessment. Legacy `schema_version: 1` records
remain readable for append-only back-compatibility. The initial assessment of
`amenable_kani::calculator::add_impl_computes_exact_sum` is recorded with a
`strengthen` recommendation.

## Goal

Provide a developer-facing way to assess the quality of registered executable
proofs. A verifier can establish that a harness satisfies the property it
states under its model. An assessment records a separate human or agent
judgment: whether that property, model, and assertion are persuasive evidence
for the intended contract.

## Architectural Boundary

Assessment is neither proof registration nor verification output.

- `KaniProof` inventory remains the catalog of executable harnesses.
- `kani-verification-results.csv` remains a latest-result table for Kani
  executions.
- `proof-assessments.jsonl` is an append-only record of reviewer judgments.

A passing verifier result never implies a favorable assessment. Conversely, a
critical assessment does not rewrite a verifier result. Keeping these facts
separate makes it possible to identify proofs that pass mechanically but need
stronger claims, models, assumptions, or assertions.

## Rubric

Every assessment assigns an integer from 0 through 4 to each axis:

- **Claim alignment:** the stated assertion establishes the intended semantic
  property.
- **Assumption adequacy:** preconditions are justified, representative, and
  not vacuous.
- **Model and implementation fidelity:** the harness exercises the intended
  production behavior rather than a weaker surrogate.
- **Assertion strength:** the oracle excludes the meaningful incorrect
  outcomes.
- **Adversarial coverage:** the proof considers boundaries, error cases, and
  relevant state or aliasing cases.
- **Clarity and maintainability:** a skeptical reader can understand, audit,
  and safely evolve the proof.

The scale has a shared interpretation: 0 is absent or misleading, 1 is
materially inadequate, 2 is partial, 3 is strong, and 4 is exceptional.

Each assessment also carries a recommendation (`accept`, `strengthen`,
`replace`, or `retire`) and an unrestricted `String` comment. The scores make
trends and queues computable; the comment preserves the reviewer's actual
argument and proposed remedy.

## Artifact Format

The default artifact is `artifacts/proof-assessments.jsonl`. It is deliberately
append-only because several reviewers may independently assess a proof and a
later assessment must not erase an earlier judgment. JSON Lines, rather than
CSV, safely represents arbitrary multiline comment text without weakening the
schema.

Each JSON object contains an assessment version, proof ID, reviewer, Unix
timestamp, six rubric scores, recommendation, and comment. The CLI owns that
metadata so reviewers should record assessments through `amenable assess
proof`, not by hand-editing the artifact. Initially, proof IDs must be present
in the compiled Kani inventory; users of the crate receive the same workflow
when their code registers a Kani harness.

## CLI

```text
amenable assess proof --proof <stable-proof-id> --reviewer <name> \
  --claim-alignment <0..4> --assumption-adequacy <0..4> \
  --model-fidelity <0..4> --assertion-strength <0..4> \
  --adversarial-coverage <0..4> --clarity <0..4> \
  --recommendation <accept|strengthen|replace|retire> --comment <text>

amenable assess report [--proof <stable-proof-id>]
amenable assess queue
```

`--comment-file <path>` is an alternative to `--comment` for long-form review
text. Both produce the same owned `String` in the persisted record.

`report` summarizes the number of assessments, score distribution and mean
per axis, plus recommendation counts. `queue` identifies registered proofs
that have no assessment yet. Numeric summaries support prioritization, but
the tool never converts an aggregate score into a certification claim.

## Acceptance Criteria

- Recording an assessment requires every rubric axis, a recommendation,
  reviewer, and comment.
- Comments can contain arbitrary long or multiline text.
- Assessment records append; no prior review is overwritten.
- A record targets a currently registered Kani proof ID.
- Reports and queues make missing and weakly assessed proofs visible.
- The feature does not alter Kani registration or verification-result
  semantics.

## First Substantive Review Loop

The calculator `add`, `debit`, and `credit` harnesses are smoke tests, so the
first review of the standard-library corpus begins with
`amenable_kani::rust_std::alloc_boxed::verify_box_derefs_and_writes_through`.
The pre-refinement assessment recognizes a focused lifecycle gap: the harness
checks final destruction of a boxed value, but not that replacing a non-`Copy`
pointee through `DerefMut` destroys the displaced value exactly once. The
first refinement adds that observable replacement case and reruns the exact
Kani harness using its native timeout. The assessment ledger preserves the
pre-refinement judgment as the reason for the change.

## Second Substantive Review Loop

The next proof in verification-ledger order is
`amenable_kani::rust_std::alloc_collections::verify_binary_heap_drain_yields_every_pushed_element_once`.
Its baseline establishes unordered multiset preservation and an empty heap
after exhaustive consumption. The review identifies an early-termination
gap: a caller may consume one value and drop the unfinished `Drain`. The
refinement makes ownership transfer and cleanup observable with an
`Ord`-compatible drop witness: the yielded value must remain owned by the
caller, while dropping the unfinished drain destroys every remaining element
and leaves the heap empty.

## Third Substantive Review Loop

The next proof,
`amenable_kani::rust_std::alloc_collections::verify_binary_heap_into_iter_yields_every_pushed_element_once`,
has the same accurately limited unordered-multiset claim but owns its source
heap rather than borrowing it. Its refinement therefore makes early iterator
drop observable: a yielded non-`Copy` element remains owned by the caller,
and dropping the unfinished `IntoIter` destroys each unyielded element.

## Fourth Substantive Review Loop

For `verify_binary_heap_iter_yields_every_pushed_element_once`, the iterator
only borrows its heap, so early-drop destruction is not its relevant lifecycle
question. The refinement instead verifies non-mutation: after complete
iteration, the heap retains both elements and `pop` still follows its
priority-order contract.

## Fifth Substantive Review Loop

`verify_binary_heap_peek_mut_exposes_the_maximum` already proves an unmodified
guard exposes and preserves the maximum. Its distinctive missing behavior is
the `PeekMut` drop-time repair path. The refinement lowers the guarded maximum
to the other heap value, releases the guard, and verifies that the heap is
re-established with both resulting values available.

## Sixth Substantive Review Loop

`verify_btree_map_iterates_in_key_order` checks ordered, key-value iteration
after reverse insertion, but initially leaves the collected borrowed entries
as its final observation. The refinement explicitly ends that borrow and
removes both entries, checking their associated values and the empty final
map. This joins the iteration claim to its non-mutating behavior; the
separate non-`Copy` removal lifecycle check remains focused on ownership.

## Seventh Substantive Review Loop

`verify_btree_set_iterates_in_sorted_order` has the analogous borrowed-output
gap. Its refinement releases the collected references, removes both ordered
elements, and asserts that the set is empty. Since this uses the same B-tree
implementation family, its individual Kani result is also recorded with a
native bounded diagnostic run rather than inferred from the map harness.

## Eighth Substantive Review Loop

`verify_linked_list_extract_if_partitions_by_the_predicate` initially proves
only exhaustive extraction over one ordered fixture. Its refinement adds the
distinct early-termination contract: after yielding the first matching value,
dropping `ExtractIf` must retain all unvisited elements in their original
order. This is an observable behavioral boundary that exhaustive collection
cannot exercise.

## Ninth Substantive Review Loop

`verify_linked_list_into_iter_yields_owned_values_in_order` establishes
front-to-back order under exhaustive consumption. The refinement makes partial
consumption observable with non-`Copy` values: the caller owns the yielded
value, and dropping the unfinished iterator destroys only its remaining
values.

## Tenth Substantive Review Loop

`verify_linked_list_is_fifo_through_back_and_front` already checks ordered
transfer and non-`Copy` ownership. The refinement closes the observable queue
state by asserting empty behavior after the two expected pops.

## Eleventh Substantive Review Loop

`verify_linked_list_iter_mut_writes_through` originally demonstrated a
single write but not iterator progression or the association between each
mutable reference and its list position. The refinement writes distinct
arbitrary replacements through two successive iterator results, checks
exhaustion, and then observes both values in list order.

## Twelfth Substantive Review Loop

`verify_linked_list_iter_yields_references_in_order` already checks ordered
borrowed results and exhaustion. The refinement ends that borrow and removes
the original values in order, proving that shared iteration did not consume or
alter the list.

## Thirteenth Substantive Review Loop

`verify_try_reserve_rejects_an_impossible_capacity` originally asserted that
a small reservation succeeds. That is not a Rust guarantee because an
allocator may report an allocation error for any request. The refinement
removes that unsound premise and proves the actual contract boundary: an
impossible capacity is rejected while an existing vector's values remain
available and ordered.

## Fourteenth Substantive Review Loop

`verify_vec_deque_drain_removes_and_yields_in_order` initially covers only an
exhausted drain. The refinement uses non-`Copy` values to prove a yielded value
belongs to its caller and that dropping an unfinished whole-deque drain
destroys every remaining value while leaving the deque empty.

## Fifteenth Substantive Review Loop

`verify_vec_deque_into_iter_yields_owned_values_in_order` begins with complete
ordered consumption. Its refinement adds the partial-consumption ownership
case: one yielded non-`Copy` value remains caller-owned, while dropping the
iterator cleans up the remaining owned values.

## Sixteenth Substantive Review Loop

`verify_vec_deque_iter_yields_references_in_order` proves ordered borrowed
results and exhaustion. The refinement releases the iterator and removes both
original values, explicitly demonstrating that the shared traversal is
non-mutating.

## Seventeenth Substantive Review Loop

`verify_vec_deque_iter_mut_writes_through` originally checks one mutable
write. The refinement writes distinct arbitrary replacements through two
successive mutable iterator results, checks iterator exhaustion, and observes
the updated deque in front-to-back order.

## Eighteenth Substantive Review Loop

`verify_vec_deque_pushes_and_pops_from_both_ends` checks the intended
end-specific values and non-`Copy` ownership. The refinement adds the terminal
empty-deque observations after both removals, ruling out residual or duplicate
elements in the exercised state.

## Nineteenth Substantive Review Loop

`verify_cstring_excludes_the_terminator_and_rejects_interior_nul` establishes
the two stated rules with an arbitrary non-nul byte, but only observes the
terminator indirectly through `as_bytes`. The refinement will check the
concrete nul-terminated representation as well, including that its appended
terminator is exactly one byte after the payload. This makes the representation
boundary claimed by the proof directly observable without changing its
well-formed input assumption.

## Twentieth Substantive Review Loop

`verify_from_vec_with_nul_requires_the_nul_only_at_the_end` checks an accepted
trailing nul and a missing-nul rejection, but its stated contract also excludes
an interior nul. The refinement will add that distinct malformed layout. It is
the direct negative counterpart to the accepted representation and avoids
relying on the separate constructor proof to establish this API's behavior.

## Twenty-First Substantive Review Loop

`verify_into_string_error_recovers_the_original_cstring` correctly makes an
invalid single byte fail and compares the recovered payload. The refinement
will make the recovery assertion cover an additional byte after the invalid
leading byte. Both arbitrary and fixed suffix forms exceeded Kani's native
bounded runs (15 seconds and then 30 seconds) without a counterexample. The
stronger source form is retained as an explicit timeout result; a future
replacement needs a verifier-aware specification boundary or library model,
not manual unwind controls or an outer process timeout.

## Twenty-Second Substantive Review Loop

`verify_nul_error_reports_the_interior_nuls_position` establishes an interior
nul at one index. The refinement will include two adjacent nul bytes and
assert that the reported position is the first one. This distinguishes an
error that locates the rejecting byte from an implementation that merely
reports an arbitrary nul in the input.

## Twenty-Third Substantive Review Loop

`verify_rc_weak_upgrade_fails_once_the_strong_count_hits_zero` checks upgrade
availability before the last strong drop and failure after it, plus value-drop
lifetime. The refinement will make the successful upgrade's payload and its
temporary strong-reference lifecycle observable, so the positive branch proves
more than an arbitrary `Some` result.

## Twenty-Fourth Substantive Review Loop

`verify_from_utf16_rejects_a_lone_surrogate` covers a valid basic code unit and
a lone high surrogate. The refinement will add the symmetrical lone low
surrogate rejection, because either unpaired half is invalid UTF-16 and the
name's general claim should not depend on which half is used.

## Twenty-Fifth Substantive Review Loop

`verify_from_utf8_error_recovers_the_original_bytes` has a strong direct
recovery assertion for an arbitrary prefix followed by a guaranteed-invalid
byte. Kani nevertheless times out in this library conversion path under its
native 15-second diagnostic bound. It is recorded as a `replace` candidate for
a future verifier-aware UTF-8 specification boundary; the existing claim is
not weakened and no unwind control is added.

## Twenty-Sixth Substantive Review Loop

`verify_string_drain_removes_and_yields_the_content` directly observes both
the drained output and final empty source for an arbitrary ASCII character,
but Kani times out on the standard-library drain path under its native
15-second diagnostic bound. The source remains an appropriately focused
contract example; future verification requires a model or specification
boundary rather than timeout masking.

## Twenty-Seventh Substantive Review Loop

`verify_arc_weak_upgrade_fails_once_the_strong_count_hits_zero` is the
thread-safe analogue of the refined `Rc::Weak` proof. Its positive upgrade
branch will likewise be strengthened to observe the original value and the
temporary strong-count transition, rather than checking only `Some`.

## Twenty-Eighth Substantive Review Loop

`verify_splice_replaces_a_range_and_yields_what_it_removed` checks the
fully-consumed splice result, but `Splice` also has an observable drop path:
replacement must complete if its removed iterator is dropped early. The
refinement will consume one removed element, drop the unfinished splice, and
check the fully replaced source sequence.

## Twenty-Ninth Substantive Review Loop

`verify_vec_drain_removes_and_yields_in_order` checks exhaustive ordered
draining, but leaves the partial-consumption cleanup contract unobserved. The
refinement will use non-`Copy` drop witnesses to show that a yielded value is
caller-owned, dropping an unfinished drain disposes every remaining element,
and the vector is empty afterward.

## Thirtieth Substantive Review Loop

`verify_vec_extract_if_partitions_by_the_predicate` proves exhaustive
partitioning for a concrete ordered input. The refinement will add the
early-termination boundary: after yielding the first match, dropping
`ExtractIf` must preserve all unvisited elements in order rather than
continuing extraction implicitly.

## Thirty-First Substantive Review Loop

`verify_vec_into_iter_yields_owned_values_in_order` establishes exhaustive
ordering but not ownership behavior when iteration stops early. The refinement
will use non-`Copy` drop witnesses: the yielded element remains caller-owned,
and dropping the unfinished iterator destroys precisely the remaining values.

## Thirty-Second Substantive Review Loop

`verify_vec_push_pop_round_trips` already covers indexing, last-in-first-out
transfer, final emptiness, and non-`Copy` destruction. The refinement will add
the explicit empty-pop observation, ensuring the exercised terminal state does
not merely have a zero length but also exposes the expected `None` API result.

## Thirty-Third Substantive Review Loop

`verify_try_from_slice_rejects_a_length_mismatch` proves exact two-element
conversion and rejects a longer source slice. The refinement will add the
symmetrical shorter slice, making both sides of the exact-length precondition
observable rather than treating length mismatch as only an oversize case.

## Thirty-Fourth Substantive Review Loop

`verify_backtrace_force_capture_always_actually_captures` reaches the real
platform backtrace implementation, which calls `_Unwind_Backtrace`; Kani
reports that foreign C function as unsupported. This is a verifier-model gap,
not an assertion counterexample, so the harness is a replacement candidate for
a platform/backtrace model. The following `BacktraceStatus` harness asserts
the same fact without additional behavior and should be retired as duplicative
once the canonical replacement exists.

## Thirty-Fifth Substantive Review Loop

`verify_ref_cell_dynamic_borrow_rules` proves that a shared borrow excludes a
mutable one and that mutation becomes available after release. The refinement
will add the converse live-mutable state: both shared and mutable re-borrows
must be rejected while a mutable guard exists, completing the observable
runtime borrow-exclusivity matrix.

## Thirty-Sixth Substantive Review Loop

`verify_args_os_reports_at_least_the_program_path` fails because Kani models
the process argument sequence as empty, producing a counterexample to the
real-process guarantee. This is a host-environment modelling gap. It and the
string `args` analogue require an argv model or an explicit harness input, not
a weaker assertion about the verifier's synthetic process state.

## Thirty-Seventh Substantive Review Loop

`verify_var_error_reports_an_unset_variable` asserts a process-global
precondition that the harness neither establishes nor can safely isolate:
another process configuration may set its chosen name. Kani also times out
while traversing the environment implementation under the native 15-second
diagnostic bound. The sound refinement is to retire this as an executable Kani
harness and retain `VarError` as provenance-only, matching the existing
non-mutating treatment of `Vars` and `VarsOs`.

## Thirty-Eighth Substantive Review Loop

`verify_cstr_excludes_the_terminating_nul_from_to_bytes` correctly proves the
payload projection for a symbolic non-NUL byte. The corresponding retained
representation is also part of the constructor contract, so the refinement
will assert `to_bytes_with_nul()` returns the complete original two-byte
input. This distinguishes removing the terminator from corrupting it.

## Thirty-Ninth Substantive Review Loop

`verify_from_bytes_until_nul_requires_a_nul_byte_somewhere` establishes the
success/failure partition, but its successful branch only observes `Ok`.
The refinement will inspect the produced `CStr` and prove that parsing ends at
the first NUL, preserving the expected payload and its terminating NUL rather
than silently accepting an unspecified prefix.

## Fortieth Substantive Review Loop

The exact-prefix refinement for
`verify_from_bytes_until_nul_requires_a_nul_byte_somewhere` reaches Kani's
native 15-second diagnostic timeout, whereas the direct success/failure
partition verifies. Retain that passing executable proof and record prefix
extraction as a future C-string model refinement; do not introduce unwind
controls or weaken the native verifier timeout boundary.

## Forty-First Substantive Review Loop

`verify_pending_never_resolves` checks one poll of an inert future, while its
claim says that pending is stable across polling. The refinement will poll the
same pinned future twice through the same safe context and require `Pending`
both times, making the repeated-poll invariant explicit without introducing
an executor or synthetic unwinding.

## Forty-Second Substantive Review Loop

`verify_poll_fn_dispatches_through_to_its_closure` currently accepts any
`Ready` value from a helper function, so it does not show that the supplied
callable ran or that its result was preserved. The refinement will use a local
closure with a `Cell<bool>` invocation marker and a symbolic captured value,
then assert both the marker and exact `Poll::Ready(value)` result.

## Forty-Third Substantive Review Loop

`verify_buf_writer_flushes_to_the_underlying_writer` proves the final flush
result but does not distinguish buffering from direct writes. The refinement
will assert that the wrapped `Vec<u8>` remains empty after a small write and
before `flush`, then retain the exact post-flush ownership assertion.

## Forty-Fourth Substantive Review Loop

`verify_split_segments_on_the_given_byte_and_drops_it` currently materializes
the entire `BufRead::split` iterator into a nested `Vec`, which is a likely
source of the native 15-second timeout rather than a meaningful part of the
claim. The refinement will observe the iterator incrementally with successive
`next()` calls, asserting the three expected segments and final exhaustion so
the separator-dropping contract remains explicit while reducing solver state.

## Forty-Fifth Substantive Review Loop

`verify_cloned_clones_each_referenced_item` proves the cloned value but leaves
open whether the adapter might yield extra elements from its one-element
source. The refinement will retain the symbolic single-item fixture and add an
explicit exhaustion check after the first `Some(value)`, closing the duplicate
yield gap without expanding the modeled behavior.

## Forty-Sixth Substantive Review Loop

`verify_copied_copies_each_referenced_item` has the same remaining weakness as
`Cloned`: it proves the first owned item but not exhaustion of the one-element
source. The refinement will mirror the `Cloned` improvement by asserting a
final `None` after the initial `Some(value)`, ruling out duplicate yields
without changing the proof shape.

## Forty-Seventh Substantive Review Loop

`verify_empty_yields_nothing` currently shows only one empty step. Since the
claim is stability rather than a one-shot observation, the refinement will
assert `None` across repeated `next()` calls on the same `Empty` iterator,
making persistent emptiness explicit without increasing conceptual scope.

## Forty-Eighth Substantive Review Loop

`verify_enumerate_pairs_each_item_with_its_index` correctly checks the first
two index-item pairs for a two-element source, but it does not explicitly show
that enumeration stops once the source does. The refinement will add a final
`None` observation after the two expected pairs, closing the only obvious
remaining gap in the representative trace.

## Forty-Ninth Substantive Review Loop

`verify_filter_map_applies_and_filters_in_one_step` times out under the native
15-second harness bound despite using a one-element source. Its current helper
closure couples the filter/map behavior to multiplication and overflow
preconditions that are incidental to the iterator contract. The refinement
will replace that helper with a simpler `Option`-shaped branch that preserves
the same single-step claim while reducing arithmetic burden on the solver.

## Fiftieth Substantive Review Loop

`verify_filter_yields_only_items_matching_the_predicate` also times out on a
one-element source. Its current predicate uses parity, which is not central to
the iterator claim and may add unnecessary arithmetic branching. The
refinement will switch to a simpler predicate shape while preserving the same
single-step contract, so any remaining timeout can be attributed to the
adapter path rather than the example logic.

## Fifty-First Substantive Review Loop

`verify_flat_map_flattens_each_generated_iterator` still times out under
Kani's native 3 minute harness timeout. Unlike the `filter` proofs, the
current harness still materializes both the flattened adapter output and the
direct inner iterator into `Vec`s. The refinement will compare the two traces
 incrementally with repeated `next()` calls over the bounded maximum length,
preserving the same claim while removing the eager collection artifact.

## Fifty-Second Substantive Review Loop

`verify_flatten_concatenates_the_inner_iterators` also times out under Kani's
native 3 minute harness timeout while still materializing both the flattened
output and the expected concatenation into `Vec`s. The refinement will compare
the flattened iterator against a direct chained iterator incrementally over the
bounded maximum length, eliminating eager collection without weakening the
ordered-concatenation claim.
