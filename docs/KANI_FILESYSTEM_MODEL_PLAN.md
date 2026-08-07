# Kani Filesystem Accommodation Model

## Goal

Move the `std::fs` proof queue off the real tempdir / metadata / file-locking
path and onto an Amenable-owned filesystem model that states the observable
laws we actually rely on.

The first migration step is narrow and deliberate:

- recursive directory creation preserves every missing ancestor directory
- directory entries report the created file's own name and full path

## Why

The proof gallery already records that even a tiny real-filesystem tempdir
scenario can time out under Kani. The production `std::fs` harnesses are still
written against that same boundary, so the next step is a shared accommodation
model rather than more direct std trimming.

## Steps

1. Add a verifier-facing filesystem model under `amenable_kani`.
2. Migrate the first `std::fs` proofs in queue order onto that model.
3. Add focused model tests.
4. Re-run scoped package checks and the migrated proofs through the `amenable`
   CLI.
5. Record proof assessments, then extend the same model to the remaining `fs`
   proofs in queue order.
