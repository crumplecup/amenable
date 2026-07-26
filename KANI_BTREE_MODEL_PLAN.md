# Kani BTree Accommodation Model

## Goal

Add a small Kani-only semantic model for focused `BTreeMap` / `BTreeSet`
ordering laws so proofs that currently time out inside real std B-tree
traversal can move to an explicit Amenable-owned accommodation boundary.

The immediate targets are
`alloc_collections::verify_btree_map_iterates_in_key_order` and
`alloc_collections::verify_btree_set_iterates_in_sorted_order`, whose current
direct std paths time out even though their claims are small and clear.

## Boundary

- `amenable_core` remains unchanged.
- `amenable_kani` owns the accommodation model.
- direct std execution paths that fail today remain documented in the proof
  gallery as false trails.
- production proofs may switch to the accommodation model only when the proof
  comment states the conformance assumption explicitly.

## Initial Scope

1. Add a documented Kani-only BTree model with:
   - deterministic two-entry ordered-map semantics
   - deterministic two-entry ordered-set semantics
   - post-observation removal semantics sufficient to show that observing the
     ordered entries does not consume them
2. Migrate the `BTreeMap<i32, i32>` and `BTreeSet<i32>` proofs from the direct
   std path to the modeled path.
3. Add one reduced BTree gallery case that preserves the direct symbolic std
   iteration timeout as the unsupported baseline for this proof family.
4. Add small integration tests for the model's deterministic laws.

## Non-Goals for This Slice

- no attempt to model arbitrary B-tree node structure or balancing logic
- no claim that std's full allocation, drop, or iterator internals are
  directly verified
- no attempt to cover more than the two-entry ordered-iteration shape already
  used by the production queue

## Acceptance Criteria

- `amenable_kani` exports a documented BTree accommodation model.
- the `BTreeMap` and `BTreeSet` production proofs no longer depend on direct
  std B-tree traversal.
- the gallery preserves the direct std symbolic-iteration timeout path as a
  false trail.
- scoped checks/tests pass through the repo `justfile`.
