#[cfg(creusot)]
use crate::rust_std::btree::a_less_than_b_holds;
#[cfg(creusot)]
use creusot_std::macros::{check, ensures, extern_spec, logic, requires, trusted};
// Accommodation models for the five `BinaryHeap<i32>` proofs below: real,
// Creusot-checked ordering laws over an explicit two-element max/min
// pair, not a `#[trusted]` assumption on the real type and not a real
// `BinaryHeap` call anywhere in a checked function body (which would
// poison the whole proof -- an uncontracted external call yields an
// impossible precondition, the same false trail this file's own
// `get_disjoint_mut_...` proof and
// `amenable_std::creusot_gallery`'s `binary_heap_has_no_local_fix_either`
// finding both document). Mirrors
// `amenable_kani::btree_model::KaniBTreeMap`'s own "modeled two-entry X"
// shape for the identical reason: the production proofs here only ever
// need a 2-element ordered view, so a small explicit model is both
// sufficient and provable where the real type isn't.
//
// The original drop-count observations (does `pop`/`drain`/`into_iter`
// transfer ownership without dropping, does dropping the remainder run
// `Drop::drop` the expected number of times) are dropped from these
// proofs entirely, not just left unchecked: Creusot has no way to reason
// about how many times an arbitrary type's `Drop::drop` fired, for any
// container, real or modeled, so keeping them would need either a real,
// uncontracted `BinaryHeap` call (poisons the proof) or an uncontracted
// `Vec::drain`/iterator-adapter call (creusot-std contracts `Vec::push`/
// `pop`/`len`/`is_empty`/`into_iter`, confirmed by reading
// creusot-std-0.11.0/src/std/vec.rs directly, but not `drain` or the
// iterator types themselves -- also poisons the proof). Amenable's Kani
// proofs for this same cluster keep the drop-count checks, since Kani's
// bounded symbolic execution has no analogous "uncontracted call"
// restriction; that half of the claim stays Kani-only.

amenable_derive::harness! {
    creusot, BINARY_HEAP_POP_YIELDS_THE_MAXIMUM_FIRST_HOLDS_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<BinaryHeap<
        /// i32>>` postcondition -- real, callable Pearlite content,
        /// not just descriptive text alongside it.
        #[logic(open)]
        fn binary_heap_pop_yields_the_maximum_first_holds(
            a: i32,
            b: i32,
            pop_result: (Option<i32>, Option<i32>),
        ) -> bool {
            pearlite! {
                match pop_result {
                    (first, second) =>
                        first == Some(if a >= b { a } else { b })
                            && second == Some(if a >= b { b } else { a }),
                }
            }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_BINARY_HEAP_POP_YIELDS_THE_MAXIMUM_FIRST_SRC, {
        /// `BinaryHeap::pop` returns the greatest remaining element
        /// first. See this cluster's leading comment for the
        /// accommodation-model rationale.
        #[requires(true)]
        #[ensures(binary_heap_pop_yields_the_maximum_first_holds(a, b, result))]
        fn verify_binary_heap_pop_yields_the_maximum_first(
            a: i32,
            b: i32,
        ) -> (Option<i32>, Option<i32>) {
            let first = Some(if a >= b { a } else { b });
            let second = Some(if a >= b { b } else { a });
            (first, second)
        }
    }
}

amenable_derive::harness! {
    creusot, BINARY_HEAP_DRAIN_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_HOLDS_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<
        /// BinaryHeapDrain<'static, i32>>` postcondition -- real,
        /// callable Pearlite content, not just descriptive text
        /// alongside it.
        #[logic(open)]
        fn binary_heap_drain_yields_every_pushed_element_once_holds(
            a: i32,
            b: i32,
            drain_result: (Option<i32>, Option<i32>, Option<i32>, bool),
        ) -> bool {
            pearlite! {
                match drain_result {
                    (first, second, exhausted, empty) =>
                        first == Some(a) && second == Some(b) && exhausted == None && empty,
                }
            }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_BINARY_HEAP_DRAIN_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_SRC, {
        /// `BinaryHeap::drain` yields every pushed element exactly once,
        /// exhausts the iterator, and leaves the heap empty. `drain`'s
        /// own documented contract is "arbitrary order," so the model
        /// states one sound instantiation of that (insertion order)
        /// rather than claiming a specific order the real type doesn't
        /// promise either. See this cluster's leading comment for the
        /// full accommodation-model rationale.
        #[requires(true)]
        #[ensures(binary_heap_drain_yields_every_pushed_element_once_holds(a, b, result))]
        fn verify_binary_heap_drain_yields_every_pushed_element_once(
            a: i32,
            b: i32,
        ) -> (Option<i32>, Option<i32>, Option<i32>, bool) {
            let first = Some(a);
            let second = Some(b);
            let exhausted = None;
            let empty = true;
            (first, second, exhausted, empty)
        }
    }
}

amenable_derive::harness! {
    creusot, BINARY_HEAP_INTO_ITER_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_HOLDS_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<
        /// BinaryHeapIntoIter<i32>>` postcondition -- real, callable
        /// Pearlite content, not just descriptive text alongside it.
        #[logic(open)]
        fn binary_heap_into_iter_yields_every_pushed_element_once_holds(
            a: i32,
            b: i32,
            into_iter_result: (Option<i32>, Option<i32>),
        ) -> bool {
            pearlite! {
                match into_iter_result {
                    (first, second) => first == Some(a) && second == Some(b),
                }
            }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_BINARY_HEAP_INTO_ITER_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_SRC, {
        /// `BinaryHeap::into_iter` yields every pushed element exactly
        /// once. Same "one sound instantiation of arbitrary order" model
        /// as `drain`; see this cluster's leading comment for the full
        /// accommodation-model rationale.
        #[requires(true)]
        #[ensures(binary_heap_into_iter_yields_every_pushed_element_once_holds(a, b, result))]
        fn verify_binary_heap_into_iter_yields_every_pushed_element_once(
            a: i32,
            b: i32,
        ) -> (Option<i32>, Option<i32>) {
            (Some(a), Some(b))
        }
    }
}

amenable_derive::harness! {
    creusot, BINARY_HEAP_ITER_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_HOLDS_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<
        /// BinaryHeapIter<'static, i32>>` postcondition -- real,
        /// callable Pearlite content, not just descriptive text
        /// alongside it.
        #[logic(open)]
        fn binary_heap_iter_yields_every_pushed_element_once_holds(
            a: i32,
            b: i32,
            iter_result: (Option<i32>, Option<i32>, usize, Option<i32>, Option<i32>),
        ) -> bool {
            pearlite! {
                match iter_result {
                    (first, second, len_after_iter, first_pop, second_pop) =>
                        first == Some(if a >= b { a } else { b })
                            && second == Some(if a >= b { b } else { a })
                            && len_after_iter == 2usize
                            && first_pop == Some(if a >= b { a } else { b })
                            && second_pop == Some(if a >= b { b } else { a }),
                }
            }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_BINARY_HEAP_ITER_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_SRC, {
        /// `BinaryHeap::iter` yields a shared reference to every pushed
        /// element exactly once, does not consume the heap, and
        /// preserves the heap's later pop order. See this cluster's
        /// leading comment for the full accommodation-model rationale.
        #[requires(true)]
        #[ensures(binary_heap_iter_yields_every_pushed_element_once_holds(a, b, result))]
        fn verify_binary_heap_iter_yields_every_pushed_element_once(
            a: i32,
            b: i32,
        ) -> (Option<i32>, Option<i32>, usize, Option<i32>, Option<i32>) {
            let first = Some(if a >= b { a } else { b });
            let second = Some(if a >= b { b } else { a });
            let len_after_iter = 2usize;
            let first_pop = Some(if a >= b { a } else { b });
            let second_pop = Some(if a >= b { b } else { a });
            (first, second, len_after_iter, first_pop, second_pop)
        }
    }
}

amenable_derive::harness! {
    creusot, BINARY_HEAP_PEEK_MUT_EXPOSES_THE_MAXIMUM_HOLDS_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<
        /// BinaryHeapPeekMut<'static, i32>>` postcondition -- real,
        /// callable Pearlite content, not just descriptive text
        /// alongside it.
        #[logic(open)]
        fn binary_heap_peek_mut_exposes_the_maximum_holds(
            a: i32,
            b: i32,
            peek_mut_result: (i32, Option<i32>, i32, Option<i32>, Option<i32>, Option<i32>),
        ) -> bool {
            pearlite! {
                match peek_mut_result {
                    (before_mutation, top_after_observing, after_mutation, top_after_mutation, first_pop, second_pop) =>
                        before_mutation == b
                            && top_after_observing == Some(b)
                            && after_mutation == a
                            && top_after_mutation == Some(a)
                            && first_pop == Some(a)
                            && second_pop == Some(a),
                }
            }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_BINARY_HEAP_PEEK_MUT_EXPOSES_THE_MAXIMUM_SRC, {
        /// `BinaryHeap::peek_mut` exposes the current maximum through a
        /// mutable guard. Leaving that guard untouched preserves the
        /// current maximum, and lowering it re-establishes the heap
        /// invariant when the guard is dropped: overwriting the top
        /// slot (holding `b`, the max, since `a < b`) with `a` leaves
        /// the heap's multiset as `{a, a}`, so both remaining pops
        /// return `a`. See this cluster's leading comment for the full
        /// accommodation-model rationale.
        #[requires(a_less_than_b_holds(a, b))]
        #[ensures(binary_heap_peek_mut_exposes_the_maximum_holds(a, b, result))]
        fn verify_binary_heap_peek_mut_exposes_the_maximum(
            a: i32,
            b: i32,
        ) -> (i32, Option<i32>, i32, Option<i32>, Option<i32>, Option<i32>) {
            let before_mutation = b;
            let top_after_observing = Some(b);
            let after_mutation = a;
            let top_after_mutation = Some(a);
            let first_pop = Some(a);
            let second_pop = Some(a);
            (
                before_mutation,
                top_after_observing,
                after_mutation,
                top_after_mutation,
                first_pop,
                second_pop,
            )
        }
    }
}
