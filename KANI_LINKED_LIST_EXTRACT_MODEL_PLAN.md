# Kani LinkedList ExtractIf Accommodation Model

## Goal

Move `LinkedList::extract_if` proof review off the direct std path and onto an
Amenable-owned model that states the two behaviors we actually rely on:

- yielded elements are exactly the visited predicate matches, in order
- dropping the extractor early leaves the unvisited suffix intact

## Why

The direct std `LinkedList::extract_if` path is already documented in the proof
gallery as a timeout false trail, even after switching from eager collection to
incremental `next()` observation. The next step is therefore a small semantic
model, not more assertion trimming.

## Steps

1. Add a verifier-facing `LinkedList::extract_if` accommodation model under
   `amenable_kani`.
2. Migrate the production proof to the model and state the conditional boundary
   explicitly in the harness docs.
3. Add focused model tests.
4. Re-run scoped package checks and the single proof through the `amenable` CLI.
5. Record the proof assessment, or extend the gallery if the first model still
   fails.
