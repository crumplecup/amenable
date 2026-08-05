//! Creusot proof-function content for Rust standard-library carriers.
//!
//! This crate contains *only* what `cargo creusot -- -p amenable_creusot`
//! needs to translate: the harness functions themselves and the trusted
//! logic wrappers they depend on. Nothing here references
//! `RustStdStandard`, registers a `ProofRecord`, or implements
//! `CreusotWitness` — that machinery moved to
//! `amenable_std::creusot_witness`, which imports the `&'static str`
//! constants below rather than duplicating the contract text. See that
//! module's doc comment for why: creusot-rustc's translator sweeps every
//! local item in a `creusot-std`-dependent crate, `#[cfg(creusot)]`-gated
//! or not, and chokes on ordinary Rust infrastructure (a return-position
//! `impl Trait` panicked its intrinsics pass outright; the `static` item
//! `::inventory::submit!` expands to hits "unsupported definition kind").
//!
//! `char` and `String` carry a genuine constraint worth stating as a real
//! Creusot postcondition; every other std carrier `amenable_std` proves
//! about has no invariant beyond what the type system already guarantees,
//! so there's nothing to translate for them here at all.
//!
//! Both contracts here are machine-checked, not just syntactically valid:
//! `just verify-creusot` runs `cargo creusot prove -- -p amenable_creusot`
//! (translation + `why3find` SMT solving) and reports `Proved (7 files) ✔`
//! — every goal in this crate discharges, including these two.

#[cfg(creusot)]
use creusot_std::logic::Int;
#[cfg(creusot)]
use creusot_std::macros::{check, ensures, extern_spec, logic, requires, trusted};
#[cfg(creusot)]
use creusot_std::std::time::nanos_to_secs;
#[cfg(creusot)]
use std::borrow::Cow;
#[cfg(creusot)]
use std::boxed::Box;
#[cfg(creusot)]
use std::cmp::{Ordering, Reverse};
#[cfg(creusot)]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, TryReserveError, VecDeque};
#[cfg(creusot)]
use std::ffi::CString;
#[cfg(creusot)]
use std::mem::ManuallyDrop;
#[cfg(creusot)]
use std::num::{
    FpCategory, IntErrorKind, NonZero, ParseFloatError, ParseIntError, Saturating, TryFromIntError,
    Wrapping,
};
#[cfg(creusot)]
use std::time::Duration;

amenable_derive::harness! {
    creusot, VERIFY_CHAR_ROUNDTRIP_SRC, {
        /// `char` is constrained to Unicode scalar values (excludes the
        /// surrogate range `0xD800..=0xDFFF`) and round-trips through
        /// itself — the same claim the Kani harness checks by symbolic
        /// exploration, restated as a Creusot postcondition.
        ///
        /// NOTE: this deliberately goes further than the reference pattern
        /// in `elicitation`'s `verification::proof_helpers::creusot_char`,
        /// which states only `ensures(result == c)` — identity, no range
        /// check — and does the same for every other stdlib opaque type it
        /// covers this way (`String`, `PathBuf`, `Duration`, `SystemTime`).
        /// The range check uses `c@` (the `View`/`ShallowModel` operator,
        /// yielding Pearlite's arbitrary-precision `Int`), not `c as u32` —
        /// confirmed by a real translation error, not a guess: `error:
        /// unsupported cast from char to u32 (allowed: bool as integer,
        /// integer as integer, or pointer as pointer)`. `char`'s `View`
        /// impl in `creusot-std` maps to `Int` via a builtin
        /// (`creusot.prelude.Char.to_int`), which is exactly what `@` is
        /// for per the Creusot guide's own Pearlite reference.
        #[requires(true)]
        #[ensures(result == c)]
        #[ensures(c@ <= 0xD7FF || (c@ >= 0xE000 && c@ <= 0x10FFFF))]
        fn verify_char_roundtrip(c: char) -> char {
            c
        }
    }
}

// `String::len` is a program function, not callable from `#[ensures]`
// (Pearlite logic context) directly — confirmed by a real translation
// error, not a guess: `error: called program function 'std::string::String
// ::len' in logic context`. `elicitation`'s own `logic_fns.rs` solves this
// with exactly this shape: a `#[trusted] #[logic(opaque)]` wrapper whose
// body is the Pearlite `dead` placeholder (an axiom — the relationship to
// the real method is asserted, not proven) so the length claim can appear
// in a postcondition at all.
#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
fn string_len(_s: &String) -> usize {
    dead
}

amenable_derive::harness! {
    creusot, VERIFY_STRING_ROUNDTRIP_SRC, {
        /// `String` round-trips through itself and preserves length.
        ///
        /// This is deliberately weaker than the Kani harness, which checks
        /// UTF-8 validity directly (`std::str::from_utf8`), but deliberately
        /// stronger than `elicitation`'s reference `creusot_string` (plain
        /// `ensures(result == s)`, no length claim). Stating "these bytes
        /// are valid UTF-8" as a first-class Pearlite predicate would need
        /// either a modeled builtin for UTF-8 well-formedness or a
        /// byte-level encoding lemma, so that part stays out of scope. The
        /// length claim goes through `string_len` (see above) since
        /// `.len()` itself can't appear in a postcondition directly.
        #[requires(true)]
        #[ensures(result == s)]
        #[ensures(string_len(&result) == string_len(&s))]
        fn verify_string_roundtrip(s: String) -> String {
            s
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_COW_DESTRUCTURE_RECOVERS_THE_WRAPPED_VALUE_SRC, {
        /// `Cow` stores either a borrowed or owned value, and
        /// destructuring the enum recovers that wrapped `i32`
        /// unchanged.
        ///
        /// `creusot-std` 0.11.0 ships no contracts for
        /// `alloc::borrow::Cow`, and calling uncontracted external
        /// methods such as `Deref::deref` or `Cow::into_owned` would
        /// poison the whole goal. So this uses only local construction
        /// and pattern matching on the enum itself.
        #[requires(true)]
        #[ensures(match value {
            Cow::Borrowed(borrowed) => result == *borrowed,
            Cow::Owned(owned) => result == owned,
        })]
        fn verify_cow_destructure_recovers_the_wrapped_value(value: Cow<'static, i32>) -> i32 {
            match value {
                Cow::Borrowed(borrowed) => *borrowed,
                Cow::Owned(owned) => owned,
            }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_BTREE_MAP_ITERATES_IN_KEY_ORDER_SRC, {
        /// `BTreeMap::iter` yields entries in ascending key order,
        /// regardless of insertion order, and observing iteration does
        /// not remove entries from the map.
        ///
        /// `#[trusted]`: `creusot-std` 0.11.0 ships no contracts or
        /// `ShallowModel` for `BTreeMap`, and the `elicitation`
        /// reference guide already calls that out as the reason the real
        /// container remains opaque to Creusot. That blocks a real
        /// iterator/refinement proof over the concrete std type today.
        /// So this states the same representative claim the Kani harness
        /// checks with Amenable's accommodation model, but marks the
        /// Creusot boundary honestly as trusted rather than pretending we
        /// discharged a proof Creusot cannot currently express.
        #[trusted]
        #[requires(k1 < k2)]
        #[ensures(match result {
            (Some((first_k, first_v)), Some((second_k, second_v)), Some(removed_first), Some(removed_second), empty) =>
                first_k == k1
                    && first_v == v1
                    && second_k == k2
                    && second_v == v2
                    && removed_first == v1
                    && removed_second == v2
                    && empty,
            _ => false,
        })]
        fn verify_btree_map_iterates_in_key_order(
            k1: i32,
            k2: i32,
            v1: i32,
            v2: i32,
        ) -> (
            Option<(i32, i32)>,
            Option<(i32, i32)>,
            Option<i32>,
            Option<i32>,
            bool,
        ) {
            let mut map = BTreeMap::new();
            map.insert(k2, v2);
            map.insert(k1, v1);

            let (first, second) = {
                let mut iter = map.iter();
                (
                    iter.next().map(|(k, v)| (*k, *v)),
                    iter.next().map(|(k, v)| (*k, *v)),
                )
            };

            let removed_first = map.remove(&k1);
            let removed_second = map.remove(&k2);

            (first, second, removed_first, removed_second, map.is_empty())
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_BTREE_SET_ITERATES_IN_SORTED_ORDER_SRC, {
        /// `BTreeSet::iter` yields elements in ascending order,
        /// regardless of insertion order, and observing iteration does
        /// not remove elements from the set.
        ///
        /// `#[trusted]`: `creusot-std` 0.11.0 ships no contracts or
        /// `ShallowModel` for `BTreeSet`, so the concrete std carrier is
        /// still opaque to Creusot today. This states the same
        /// representative claim the Kani harness checks with Amenable's
        /// ordered-set accommodation model, but keeps the Creusot
        /// boundary explicit instead of pretending the real std type was
        /// machine-proved.
        #[trusted]
        #[requires(a < b)]
        #[ensures(match result {
            (Some(first), Some(second), removed_first, removed_second, empty) =>
                first == a
                    && second == b
                    && removed_first
                    && removed_second
                    && empty,
            _ => false,
        })]
        fn verify_btree_set_iterates_in_sorted_order(
            a: i32,
            b: i32,
        ) -> (Option<i32>, Option<i32>, bool, bool, bool) {
            let mut set = BTreeSet::new();
            set.insert(b);
            set.insert(a);

            let (first, second) = {
                let mut iter = set.iter();
                (iter.next().copied(), iter.next().copied())
            };

            let removed_first = set.remove(&a);
            let removed_second = set.remove(&b);

            (first, second, removed_first, removed_second, set.is_empty())
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_BINARY_HEAP_POP_YIELDS_THE_MAXIMUM_FIRST_SRC, {
        /// `BinaryHeap::pop` returns the greatest remaining element
        /// first, and ownership transfers out of the heap without
        /// dropping the popped value.
        ///
        /// `#[trusted]`: `creusot-std` 0.11.0 ships no contracts for
        /// `BinaryHeap`, so Creusot cannot state or discharge this over
        /// the concrete std carrier directly today. This keeps the same
        /// representative claim as Amenable's Kani proof, including the
        /// explicit drop-count observation, but marks the boundary
        /// honestly as trusted.
        #[trusted]
        #[requires(true)]
        #[ensures(match result {
            (first, second, after_pop, after_drop_popped, after_drop_heap) =>
                first == Some(if a >= b { a } else { b })
                    && second == Some(if a >= b { b } else { a })
                    && after_pop == 0u32
                    && after_drop_popped == 1u32
                    && after_drop_heap == 2u32,
        })]
        fn verify_binary_heap_pop_yields_the_maximum_first(
            a: i32,
            b: i32,
        ) -> (Option<i32>, Option<i32>, u32, u32, u32) {
            let mut heap = BinaryHeap::new();
            heap.push(a);
            heap.push(b);
            let first = heap.pop();
            let second = heap.pop();

            struct OrderedDropWitness {
                id: i32,
                drop_count: std::rc::Rc<std::cell::Cell<u32>>,
            }
            impl PartialEq for OrderedDropWitness {
                fn eq(&self, other: &Self) -> bool {
                    self.id == other.id
                }
            }
            impl Eq for OrderedDropWitness {}
            impl PartialOrd for OrderedDropWitness {
                fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                    Some(self.cmp(other))
                }
            }
            impl Ord for OrderedDropWitness {
                fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                    self.id.cmp(&other.id)
                }
            }
            impl Drop for OrderedDropWitness {
                fn drop(&mut self) {
                    self.drop_count.set(self.drop_count.get() + 1);
                }
            }

            let drop_count = std::rc::Rc::new(std::cell::Cell::new(0));
            let (after_pop, after_drop_popped, after_drop_heap) = {
                let mut witness_heap = BinaryHeap::new();
                witness_heap.push(OrderedDropWitness { id: 1, drop_count: drop_count.clone() });
                witness_heap.push(OrderedDropWitness { id: 2, drop_count: drop_count.clone() });
                let popped = witness_heap.pop().unwrap();
                let after_pop = drop_count.get();
                drop(popped);
                let after_drop_popped = drop_count.get();
                drop(witness_heap);
                let after_drop_heap = drop_count.get();
                (after_pop, after_drop_popped, after_drop_heap)
            };

            (first, second, after_pop, after_drop_popped, after_drop_heap)
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_BINARY_HEAP_DRAIN_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_SRC, {
        /// `BinaryHeap::drain` yields every pushed element exactly once
        /// in arbitrary order, exhausts the iterator, and leaves the
        /// heap empty. A partially-consumed drain also transfers a
        /// yielded value to its caller without dropping it, and
        /// dropping the unfinished drain destroys every element that
        /// remains in the heap.
        ///
        /// `#[trusted]`: `creusot-std` 0.11.0 ships no contracts for
        /// `BinaryHeap` or its `Drain` carrier, so Creusot cannot
        /// currently express or discharge this over the concrete std
        /// carrier directly. This keeps the same representative
        /// observation as the Kani harness, including the explicit
        /// unfinished-drain drop-count behavior, while making the
        /// trusted boundary explicit.
        #[trusted]
        #[requires(true)]
        #[ensures(match result {
            (
                first,
                second,
                exhausted,
                empty,
                after_next,
                after_drop_yielded,
                after_drop_drain,
                empty_after_partial_drop,
            ) =>
                ((first == Some(a) && second == Some(b))
                    || (first == Some(b) && second == Some(a)))
                    && exhausted == None
                    && empty
                    && after_next == 0u32
                    && after_drop_yielded == 1u32
                    && after_drop_drain == 3u32
                    && empty_after_partial_drop,
        })]
        fn verify_binary_heap_drain_yields_every_pushed_element_once(
            a: i32,
            b: i32,
        ) -> (Option<i32>, Option<i32>, Option<i32>, bool, u32, u32, u32, bool) {
            let mut heap = BinaryHeap::new();
            heap.push(a);
            heap.push(b);
            let mut drain = heap.drain();
            let first = drain.next();
            let second = drain.next();
            let exhausted = drain.next();
            drop(drain);
            let empty = heap.is_empty();

            struct OrderedDropWitness {
                id: i32,
                drop_count: std::rc::Rc<std::cell::Cell<u32>>,
            }
            impl PartialEq for OrderedDropWitness {
                fn eq(&self, other: &Self) -> bool {
                    self.id == other.id
                }
            }
            impl Eq for OrderedDropWitness {}
            impl PartialOrd for OrderedDropWitness {
                fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                    Some(self.cmp(other))
                }
            }
            impl Ord for OrderedDropWitness {
                fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                    self.id.cmp(&other.id)
                }
            }
            impl Drop for OrderedDropWitness {
                fn drop(&mut self) {
                    self.drop_count.set(self.drop_count.get() + 1);
                }
            }

            let drop_count = std::rc::Rc::new(std::cell::Cell::new(0));
            let (after_next, after_drop_yielded, after_drop_drain, empty_after_partial_drop) = {
                let mut witness_heap = BinaryHeap::new();
                witness_heap.push(OrderedDropWitness { id: 1, drop_count: drop_count.clone() });
                witness_heap.push(OrderedDropWitness { id: 2, drop_count: drop_count.clone() });
                witness_heap.push(OrderedDropWitness { id: 3, drop_count: drop_count.clone() });
                let mut drain = witness_heap.drain();
                let yielded = drain.next().unwrap();
                let after_next = drop_count.get();
                drop(yielded);
                let after_drop_yielded = drop_count.get();
                drop(drain);
                let after_drop_drain = drop_count.get();
                let empty_after_partial_drop = witness_heap.is_empty();
                (
                    after_next,
                    after_drop_yielded,
                    after_drop_drain,
                    empty_after_partial_drop,
                )
            };

            (
                first,
                second,
                exhausted,
                empty,
                after_next,
                after_drop_yielded,
                after_drop_drain,
                empty_after_partial_drop,
            )
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_BINARY_HEAP_INTO_ITER_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_SRC, {
        /// `BinaryHeap::into_iter` yields every pushed element exactly
        /// once in arbitrary order. A partially-consumed iterator also
        /// transfers a yielded value to its caller without dropping it,
        /// and dropping the unfinished iterator destroys every element
        /// it still owns exactly once.
        ///
        /// `#[trusted]`: `creusot-std` 0.11.0 ships no contracts for
        /// `BinaryHeap` or its `IntoIter` carrier, so Creusot cannot
        /// currently express or discharge this over the concrete std
        /// carrier directly. This keeps the same representative
        /// observation as Amenable's Kani proof, including the explicit
        /// unfinished-iterator drop-count behavior, while making the
        /// trusted boundary explicit.
        #[trusted]
        #[requires(true)]
        #[ensures(match result {
            (first, second, after_next, after_drop_yielded, after_drop_iter) =>
                ((first == Some(a) && second == Some(b))
                    || (first == Some(b) && second == Some(a)))
                    && after_next == 0u32
                    && after_drop_yielded == 1u32
                    && after_drop_iter == 3u32,
        })]
        fn verify_binary_heap_into_iter_yields_every_pushed_element_once(
            a: i32,
            b: i32,
        ) -> (Option<i32>, Option<i32>, u32, u32, u32) {
            let mut heap = BinaryHeap::new();
            heap.push(a);
            heap.push(b);
            let mut collected: Vec<i32> = heap.into_iter().collect();
            collected.sort_unstable();
            let first = collected.first().copied();
            let second = collected.get(1).copied();

            struct OrderedDropWitness {
                id: i32,
                drop_count: std::rc::Rc<std::cell::Cell<u32>>,
            }
            impl PartialEq for OrderedDropWitness {
                fn eq(&self, other: &Self) -> bool {
                    self.id == other.id
                }
            }
            impl Eq for OrderedDropWitness {}
            impl PartialOrd for OrderedDropWitness {
                fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                    Some(self.cmp(other))
                }
            }
            impl Ord for OrderedDropWitness {
                fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                    self.id.cmp(&other.id)
                }
            }
            impl Drop for OrderedDropWitness {
                fn drop(&mut self) {
                    self.drop_count.set(self.drop_count.get() + 1);
                }
            }

            let drop_count = std::rc::Rc::new(std::cell::Cell::new(0));
            let (after_next, after_drop_yielded, after_drop_iter) = {
                let mut witness_heap = BinaryHeap::new();
                witness_heap.push(OrderedDropWitness {
                    id: 1,
                    drop_count: drop_count.clone(),
                });
                witness_heap.push(OrderedDropWitness {
                    id: 2,
                    drop_count: drop_count.clone(),
                });
                witness_heap.push(OrderedDropWitness {
                    id: 3,
                    drop_count: drop_count.clone(),
                });
                let mut iterator = witness_heap.into_iter();
                let first = iterator.next().unwrap();
                let after_next = drop_count.get();
                drop(first);
                let after_drop_yielded = drop_count.get();
                drop(iterator);
                let after_drop_iter = drop_count.get();
                (after_next, after_drop_yielded, after_drop_iter)
            };

            (first, second, after_next, after_drop_yielded, after_drop_iter)
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_LINKED_LIST_IS_FIFO_THROUGH_BACK_AND_FRONT_SRC, {
        /// `LinkedList::push_back` followed by `pop_front` behaves as a
        /// FIFO queue, and ownership transfers out of the list without
        /// dropping the popped value.
        ///
        /// `#[trusted]`: `creusot-std` 0.11.0 ships no contracts for
        /// `LinkedList`, so Creusot cannot express or discharge this
        /// over the concrete std carrier today. This keeps the same
        /// representative observation as the Kani proof, including the
        /// explicit drop-count behavior, while making the trusted
        /// boundary explicit.
        #[trusted]
        #[requires(true)]
        #[ensures(match result {
            (first, second, third, empty, after_pop, after_drop_popped, after_drop_list) =>
                first == Some(a)
                    && second == Some(b)
                    && third == None
                    && empty
                    && after_pop == 0u32
                    && after_drop_popped == 1u32
                    && after_drop_list == 2u32,
        })]
        fn verify_linked_list_is_fifo_through_back_and_front(
            a: i32,
            b: i32,
        ) -> (Option<i32>, Option<i32>, Option<i32>, bool, u32, u32, u32) {
            let mut list = LinkedList::new();
            list.push_back(a);
            list.push_back(b);
            let first = list.pop_front();
            let second = list.pop_front();
            let third = list.pop_front();
            let empty = list.is_empty();

            struct DropWitness {
                drop_count: std::rc::Rc<std::cell::Cell<u32>>,
            }
            impl Drop for DropWitness {
                fn drop(&mut self) {
                    self.drop_count.set(self.drop_count.get() + 1);
                }
            }

            let drop_count = std::rc::Rc::new(std::cell::Cell::new(0));
            let (after_pop, after_drop_popped, after_drop_list) = {
                let mut witness_list = LinkedList::new();
                witness_list.push_back(DropWitness { drop_count: drop_count.clone() });
                witness_list.push_back(DropWitness { drop_count: drop_count.clone() });
                let popped = witness_list.pop_front().unwrap();
                let after_pop = drop_count.get();
                drop(popped);
                let after_drop_popped = drop_count.get();
                drop(witness_list);
                let after_drop_list = drop_count.get();
                (after_pop, after_drop_popped, after_drop_list)
            };

            (
                first,
                second,
                third,
                empty,
                after_pop,
                after_drop_popped,
                after_drop_list,
            )
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_LINKED_LIST_INTO_ITER_YIELDS_OWNED_VALUES_IN_ORDER_SRC, {
        /// `LinkedList::into_iter` consumes the list and yields its
        /// owned values in front-to-back order. A partially-consumed
        /// iterator transfers its yielded value to the caller without
        /// dropping it, and dropping the unfinished iterator destroys
        /// every remaining owned value exactly once.
        ///
        /// `#[trusted]`: `creusot-std` 0.11.0 ships no contracts for
        /// `LinkedList` or its `IntoIter` carrier, so Creusot cannot
        /// currently express or discharge this over the concrete std
        /// carrier directly. This keeps the same representative
        /// observation as Amenable's Kani proof, including the explicit
        /// unfinished-iterator drop-count behavior, while making the
        /// trusted boundary explicit.
        #[trusted]
        #[requires(true)]
        #[ensures(match result {
            (first, second, third, after_next, after_drop_yielded, after_drop_iter) =>
                first == Some(a)
                    && second == Some(b)
                    && third == None
                    && after_next == 0u32
                    && after_drop_yielded == 1u32
                    && after_drop_iter == 3u32,
        })]
        fn verify_linked_list_into_iter_yields_owned_values_in_order(
            a: i32,
            b: i32,
        ) -> (Option<i32>, Option<i32>, Option<i32>, u32, u32, u32) {
            let mut list = LinkedList::new();
            list.push_back(a);
            list.push_back(b);
            let mut it = list.into_iter();
            let first = it.next();
            let second = it.next();
            let third = it.next();

            struct DropWitness {
                drop_count: std::rc::Rc<std::cell::Cell<u32>>,
            }
            impl Drop for DropWitness {
                fn drop(&mut self) {
                    self.drop_count.set(self.drop_count.get() + 1);
                }
            }

            let drop_count = std::rc::Rc::new(std::cell::Cell::new(0));
            let (after_next, after_drop_yielded, after_drop_iter) = {
                let mut witness_list = LinkedList::new();
                witness_list.push_back(DropWitness {
                    drop_count: drop_count.clone(),
                });
                witness_list.push_back(DropWitness {
                    drop_count: drop_count.clone(),
                });
                witness_list.push_back(DropWitness {
                    drop_count: drop_count.clone(),
                });
                let mut iterator = witness_list.into_iter();
                let first = iterator.next().unwrap();
                let after_next = drop_count.get();
                drop(first);
                let after_drop_yielded = drop_count.get();
                drop(iterator);
                let after_drop_iter = drop_count.get();
                (after_next, after_drop_yielded, after_drop_iter)
            };

            (
                first,
                second,
                third,
                after_next,
                after_drop_yielded,
                after_drop_iter,
            )
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_TRY_RESERVE_REJECTS_AN_IMPOSSIBLE_CAPACITY_SRC, {
        /// `Vec::try_reserve` reports failure via `TryReserveError`
        /// for an impossible reservation request, without disturbing
        /// values already stored in the vector.
        ///
        /// `#[trusted]`: `creusot-std` 0.11.0 ships no contracts for
        /// `Vec::try_reserve` or for the `TryReserveError` carrier it
        /// returns, so Creusot cannot currently express or discharge
        /// the concrete allocation-failure path over the std type
        /// itself. This keeps the same representative observation as
        /// the Kani harness and makes the trusted boundary explicit.
        #[trusted]
        #[requires(true)]
        #[ensures(match result {
            (Some(_error), observed_first, observed_second, observed_len) =>
                observed_first == first
                    && observed_second == second
                    && observed_len == 2usize,
            _ => false,
        })]
        fn verify_try_reserve_rejects_an_impossible_capacity(
            first: i32,
            second: i32,
        ) -> (Option<TryReserveError>, i32, i32, usize) {
            let mut values = vec![first, second];
            let error = values.try_reserve(usize::MAX).err();
            let observed_first = values[0];
            let observed_second = values[1];
            let observed_len = values.len();
            (error, observed_first, observed_second, observed_len)
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_BOX_NEW_PRESERVES_THE_WRAPPED_VALUE_SRC, {
        /// `Box::new` stores the supplied `i32`, and `Box::as_ref`
        /// exposes that same wrapped value through a shared borrow.
        ///
        /// This leans directly on `creusot-std`'s own upstream
        /// contracts for `Box::new` (`*result == val`) and
        /// `Box::as_ref` (`**self == *result`) instead of postulating
        /// any local model.
        #[requires(true)]
        #[ensures(result == value)]
        fn verify_box_new_preserves_the_wrapped_value(value: i32) -> i32 {
            let boxed = Box::new(value);
            *boxed.as_ref()
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_VEC_DEQUE_PUSHES_AND_POPS_FROM_BOTH_ENDS_SRC, {
        /// `VecDeque` is genuinely double-ended: pushing one element to
        /// the back and another to the front, then popping from each
        /// end, returns the value pushed to that end. Ownership also
        /// transfers out of the deque without dropping the popped
        /// value.
        ///
        /// `#[trusted]`: `creusot-std` 0.11.0 ships no contracts for
        /// `VecDeque`, so Creusot cannot express or discharge this
        /// over the concrete std carrier directly today. This keeps the
        /// same representative observation as the Kani proof,
        /// including the explicit drop-count behavior, while making the
        /// trusted boundary explicit.
        #[trusted]
        #[requires(true)]
        #[ensures(match result {
            (front, back, exhausted_front, exhausted_back, empty, after_pop, after_drop_popped, after_drop_deque) =>
                front == Some(b)
                    && back == Some(a)
                    && exhausted_front == None
                    && exhausted_back == None
                    && empty
                    && after_pop == 0u32
                    && after_drop_popped == 1u32
                    && after_drop_deque == 2u32,
        })]
        fn verify_vec_deque_pushes_and_pops_from_both_ends(
            a: i32,
            b: i32,
        ) -> (Option<i32>, Option<i32>, Option<i32>, Option<i32>, bool, u32, u32, u32) {
            let mut dq = VecDeque::new();
            dq.push_back(a);
            dq.push_front(b);
            let front = dq.pop_front();
            let back = dq.pop_back();
            let exhausted_front = dq.pop_front();
            let exhausted_back = dq.pop_back();
            let empty = dq.is_empty();

            struct DropWitness {
                drop_count: std::rc::Rc<std::cell::Cell<u32>>,
            }
            impl Drop for DropWitness {
                fn drop(&mut self) {
                    self.drop_count.set(self.drop_count.get() + 1);
                }
            }

            let drop_count = std::rc::Rc::new(std::cell::Cell::new(0));
            let (after_pop, after_drop_popped, after_drop_deque) = {
                let mut witness_dq = VecDeque::new();
                witness_dq.push_back(DropWitness { drop_count: drop_count.clone() });
                witness_dq.push_back(DropWitness { drop_count: drop_count.clone() });
                let popped = witness_dq.pop_front().unwrap();
                let after_pop = drop_count.get();
                drop(popped);
                let after_drop_popped = drop_count.get();
                drop(witness_dq);
                let after_drop_deque = drop_count.get();
                (after_pop, after_drop_popped, after_drop_deque)
            };

            (
                front,
                back,
                exhausted_front,
                exhausted_back,
                empty,
                after_pop,
                after_drop_popped,
                after_drop_deque,
            )
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_VEC_DEQUE_INTO_ITER_YIELDS_OWNED_VALUES_IN_ORDER_SRC, {
        /// `VecDeque::into_iter` consumes the deque and yields its
        /// owned values in front-to-back order. A partially-consumed
        /// iterator transfers its yielded value to the caller without
        /// dropping it, and dropping the unfinished iterator destroys
        /// every remaining owned value exactly once.
        ///
        /// `#[trusted]`: `creusot-std` 0.11.0 ships no contracts for
        /// `VecDeque` or its `IntoIter` carrier, so Creusot cannot
        /// currently express or discharge this over the concrete std
        /// carrier directly. This keeps the same representative
        /// observation as Amenable's Kani proof, including the explicit
        /// unfinished-iterator drop-count behavior, while making the
        /// trusted boundary explicit.
        #[trusted]
        #[requires(true)]
        #[ensures(match result {
            (first, second, third, after_next, after_drop_yielded, after_drop_iter) =>
                first == Some(a)
                    && second == Some(b)
                    && third == None
                    && after_next == 0u32
                    && after_drop_yielded == 1u32
                    && after_drop_iter == 3u32,
        })]
        fn verify_vec_deque_into_iter_yields_owned_values_in_order(
            a: i32,
            b: i32,
        ) -> (Option<i32>, Option<i32>, Option<i32>, u32, u32, u32) {
            let mut dq = VecDeque::new();
            dq.push_back(a);
            dq.push_back(b);
            let mut it = dq.into_iter();
            let first = it.next();
            let second = it.next();
            let third = it.next();

            struct DropWitness {
                drop_count: std::rc::Rc<std::cell::Cell<u32>>,
            }
            impl Drop for DropWitness {
                fn drop(&mut self) {
                    self.drop_count.set(self.drop_count.get() + 1);
                }
            }

            let drop_count = std::rc::Rc::new(std::cell::Cell::new(0));
            let (after_next, after_drop_yielded, after_drop_iter) = {
                let mut witness_deque = VecDeque::new();
                witness_deque.push_back(DropWitness {
                    drop_count: drop_count.clone(),
                });
                witness_deque.push_back(DropWitness {
                    drop_count: drop_count.clone(),
                });
                witness_deque.push_back(DropWitness {
                    drop_count: drop_count.clone(),
                });
                let mut iterator = witness_deque.into_iter();
                let first = iterator.next().unwrap();
                let after_next = drop_count.get();
                drop(first);
                let after_drop_yielded = drop_count.get();
                drop(iterator);
                let after_drop_iter = drop_count.get();
                (after_next, after_drop_yielded, after_drop_iter)
            };

            (
                first,
                second,
                third,
                after_next,
                after_drop_yielded,
                after_drop_iter,
            )
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_VEC_DEQUE_DRAIN_REMOVES_AND_YIELDS_IN_ORDER_SRC, {
        /// `VecDeque::drain(..)` yields every element in front-to-back
        /// order, leaves the deque empty, and transfers yielded
        /// ownership to the caller without dropping it. Dropping an
        /// unfinished whole-deque drain destroys every remaining owned
        /// value exactly once and still leaves the deque empty.
        ///
        /// `#[trusted]`: `creusot-std` 0.11.0 ships no contracts for
        /// `VecDeque` or its `Drain` carrier, so Creusot cannot
        /// currently express or discharge this over the concrete std
        /// carrier directly. This keeps the same representative
        /// observation as Amenable's Kani proof, including the explicit
        /// unfinished-drain drop-count behavior, while making the
        /// trusted boundary explicit.
        #[trusted]
        #[requires(true)]
        #[ensures(match result {
            (first, second, third, empty, after_next, after_drop_yielded, after_drop_drain, empty_after_partial_drop) =>
                first == Some(a)
                    && second == Some(b)
                    && third == None
                    && empty
                    && after_next == 0u32
                    && after_drop_yielded == 1u32
                    && after_drop_drain == 3u32
                    && empty_after_partial_drop,
        })]
        fn verify_vec_deque_drain_removes_and_yields_in_order(
            a: i32,
            b: i32,
        ) -> (Option<i32>, Option<i32>, Option<i32>, bool, u32, u32, u32, bool) {
            let mut dq = VecDeque::new();
            dq.push_back(a);
            dq.push_back(b);
            let mut drain = dq.drain(..);
            let first = drain.next();
            let second = drain.next();
            let third = drain.next();
            drop(drain);
            let empty = dq.is_empty();

            struct DropWitness {
                drop_count: std::rc::Rc<std::cell::Cell<u32>>,
            }
            impl Drop for DropWitness {
                fn drop(&mut self) {
                    self.drop_count.set(self.drop_count.get() + 1);
                }
            }

            let drop_count = std::rc::Rc::new(std::cell::Cell::new(0));
            let (after_next, after_drop_yielded, after_drop_drain, empty_after_partial_drop) = {
                let mut witness_deque = VecDeque::new();
                witness_deque.push_back(DropWitness {
                    drop_count: drop_count.clone(),
                });
                witness_deque.push_back(DropWitness {
                    drop_count: drop_count.clone(),
                });
                witness_deque.push_back(DropWitness {
                    drop_count: drop_count.clone(),
                });
                let mut drain = witness_deque.drain(..);
                let first = drain.next().unwrap();
                let after_next = drop_count.get();
                drop(first);
                let after_drop_yielded = drop_count.get();
                drop(drain);
                let after_drop_drain = drop_count.get();
                let empty_after_partial_drop = witness_deque.is_empty();
                (
                    after_next,
                    after_drop_yielded,
                    after_drop_drain,
                    empty_after_partial_drop,
                )
            };

            (
                first,
                second,
                third,
                empty,
                after_next,
                after_drop_yielded,
                after_drop_drain,
                empty_after_partial_drop,
            )
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_VEC_DEQUE_ITER_YIELDS_REFERENCES_IN_ORDER_SRC, {
        /// `VecDeque::iter` yields shared references in front-to-back
        /// order and leaves the deque unchanged.
        ///
        /// `#[trusted]`: `creusot-std` 0.11.0 ships no contracts for
        /// `VecDeque` or its `Iter` carrier, so Creusot cannot
        /// currently express or discharge this over the concrete std
        /// carrier directly. This keeps the same representative
        /// observation as Amenable's Kani proof while making the
        /// trusted boundary explicit.
        #[trusted]
        #[requires(true)]
        #[ensures(match result {
            (first_seen, second_seen, exhausted, popped_first, popped_second, empty) =>
                first_seen == Some(a)
                    && second_seen == Some(b)
                    && exhausted == None
                    && popped_first == Some(a)
                    && popped_second == Some(b)
                    && empty,
        })]
        fn verify_vec_deque_iter_yields_references_in_order(
            a: i32,
            b: i32,
        ) -> (Option<i32>, Option<i32>, Option<i32>, Option<i32>, Option<i32>, bool) {
            let mut dq = VecDeque::new();
            dq.push_back(a);
            dq.push_back(b);
            let (first_seen, second_seen, exhausted) = {
                let mut it = dq.iter();
                (it.next().copied(), it.next().copied(), it.next().copied())
            };
            let popped_first = dq.pop_front();
            let popped_second = dq.pop_front();
            let empty = dq.is_empty();

            (
                first_seen,
                second_seen,
                exhausted,
                popped_first,
                popped_second,
                empty,
            )
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_VEC_DEQUE_ITER_MUT_WRITES_THROUGH_SRC, {
        /// `VecDeque::iter_mut` yields mutable references in
        /// front-to-back order, and writes through those references are
        /// reflected at the corresponding deque positions.
        ///
        /// `#[trusted]`: `creusot-std` 0.11.0 ships no contracts for
        /// `VecDeque` or its `IterMut` carrier, so Creusot cannot
        /// currently express or discharge this over the concrete std
        /// carrier directly. This keeps the same representative
        /// observation as Amenable's Kani proof while making the
        /// trusted boundary explicit.
        #[trusted]
        #[requires(true)]
        #[ensures(match result {
            (first_after, second_after, empty) =>
                first_after == Some(updated_first)
                    && second_after == Some(updated_second)
                    && empty,
        })]
        fn verify_vec_deque_iter_mut_writes_through(
            first: i32,
            second: i32,
            updated_first: i32,
            updated_second: i32,
        ) -> (Option<i32>, Option<i32>, bool) {
            let mut dq = VecDeque::new();
            dq.push_back(first);
            dq.push_back(second);
            {
                let mut iterator = dq.iter_mut();
                *iterator.next().unwrap() = updated_first;
                *iterator.next().unwrap() = updated_second;
            }
            let first_after = dq.pop_front();
            let second_after = dq.pop_front();
            let empty = dq.is_empty();
            (first_after, second_after, empty)
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_CSTRING_EXCLUDES_THE_TERMINATOR_AND_REJECTS_INTERIOR_NUL_SRC, {
        /// `CString::new` appends its own terminating nul, exposes the
        /// payload bytes without that terminator through `as_bytes`,
        /// and rejects any input that already contains an interior nul
        /// byte.
        ///
        /// `#[trusted]`: `creusot-std` 0.11.0 ships no contracts for
        /// `CString` construction or observation, so Creusot cannot
        /// express this directly over the concrete std carrier today.
        /// This keeps the same representative observation as the Kani
        /// harness while making the trusted boundary explicit.
        #[trusted]
        #[requires(byte@ != 0)]
        #[ensures(match result {
            (payload_len, observed_byte, payload_with_nul_len, terminator, interior_nul_rejected) =>
                payload_len == 1usize
                    && observed_byte == Some(byte)
                    && payload_with_nul_len == 2usize
                    && terminator == Some(0u8)
                    && interior_nul_rejected,
        })]
        fn verify_cstring_excludes_the_terminator_and_rejects_interior_nul(
            byte: u8,
        ) -> (usize, Option<u8>, usize, Option<u8>, bool) {
            let cstring = CString::new(vec![byte]).unwrap();
            let payload = cstring.as_bytes();
            let payload_with_nul = cstring.as_bytes_with_nul();
            let interior_nul_rejected = CString::new(vec![byte, 0, byte]).is_err();
            (
                payload.len(),
                payload.first().copied(),
                payload_with_nul.len(),
                payload_with_nul.get(1).copied(),
                interior_nul_rejected,
            )
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_FROM_VEC_WITH_NUL_REQUIRES_THE_NUL_ONLY_AT_THE_END_SRC, {
        /// `CString::from_vec_with_nul` accepts a nul-terminated byte
        /// vector only when the sole nul byte is the final one.
        ///
        /// `#[trusted]`: `creusot-std` 0.11.0 ships no contracts for
        /// `CString::from_vec_with_nul` or its error carrier, so
        /// Creusot cannot discharge this over the concrete std type
        /// directly today. This keeps the same representative
        /// observation as the Kani harness while making the trusted
        /// boundary explicit.
        #[trusted]
        #[requires(byte@ != 0)]
        #[ensures(match result {
            (accepted, missing_nul_rejected, interior_nul_rejected) =>
                accepted && missing_nul_rejected && interior_nul_rejected,
        })]
        fn verify_from_vec_with_nul_requires_the_nul_only_at_the_end(
            byte: u8,
        ) -> (bool, bool, bool) {
            (
                CString::from_vec_with_nul(vec![byte, 0]).is_ok(),
                CString::from_vec_with_nul(vec![byte, byte]).is_err(),
                CString::from_vec_with_nul(vec![byte, 0, byte]).is_err(),
            )
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_INTO_STRING_ERROR_RECOVERS_THE_ORIGINAL_CSTRING_SRC, {
        /// `CString::into_string` fails on non-UTF-8 payload bytes, and
        /// `IntoStringError::into_cstring` recovers exactly the
        /// original owned `CString`.
        ///
        /// `#[trusted]`: `creusot-std` 0.11.0 ships no contracts for
        /// `CString::into_string` or `IntoStringError`, so Creusot
        /// cannot discharge this over the concrete std types directly
        /// today. This keeps the same representative observation as the
        /// Kani harness while making the trusted boundary explicit.
        #[trusted]
        #[requires(true)]
        #[ensures(match result {
            (payload_len, first, second, terminator) =>
                payload_len == 3usize
                    && first == Some(0xFFu8)
                    && second == Some(120u8)
                    && terminator == Some(0u8),
        })]
        fn verify_into_string_error_recovers_the_original_cstring() -> (usize, Option<u8>, Option<u8>, Option<u8>) {
            let invalid = CString::new(vec![0xFFu8, b'x']).unwrap();
            let recovered = invalid.into_string().unwrap_err().into_cstring().into_bytes_with_nul();
            (
                recovered.len(),
                recovered.first().copied(),
                recovered.get(1).copied(),
                recovered.get(2).copied(),
            )
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_NUL_ERROR_REPORTS_THE_INTERIOR_NULS_POSITION_SRC, {
        /// `NulError::nul_position` reports the index of the first
        /// interior nul byte that caused `CString::new` to reject the
        /// input.
        ///
        /// `#[trusted]`: `creusot-std` 0.11.0 ships no contracts for
        /// `CString::new` or `NulError`, so Creusot cannot discharge
        /// this over the concrete std types directly today. This keeps
        /// the same representative observation as the Kani harness
        /// while making the trusted boundary explicit.
        #[trusted]
        #[requires(byte@ != 0)]
        #[ensures(match result {
            (single_nul_index, first_of_two_index) =>
                single_nul_index == 1usize && first_of_two_index == 1usize,
        })]
        fn verify_nul_error_reports_the_interior_nuls_position(byte: u8) -> (usize, usize) {
            let single_nul_index = CString::new(vec![byte, 0, byte]).unwrap_err().nul_position();
            let first_of_two_index =
                CString::new(vec![byte, 0, 0, byte]).unwrap_err().nul_position();
            (single_nul_index, first_of_two_index)
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_DURATION_NEW_NORMALIZES_NANOS_AND_CARRIES_INTO_SECS_SRC, {
        /// `Duration::new` does not require `nanos < 1_000_000_000` — it
        /// normalizes: any whole-second carry in `nanos` is added to
        /// `secs`, and `subsec_nanos()` reports the remainder. Same claim
        /// as the Kani harness (`amenable_kani::rust_std::time`), restated
        /// as a Creusot postcondition — right down to the `secs.checked_add
        /// (carry).is_some()` precondition Kani assumes, here expressed as
        /// `secs@ + (nanos@ / 1_000_000_000) <= u64::MAX@` (Pearlite's `@`
        /// operator lifts to arbitrary-precision `Int`, so this is exactly
        /// "the real u64 addition wouldn't overflow", not an approximation).
        ///
        /// `creusot-std` ships its own trusted `extern_spec!` for
        /// `Duration::new`/`as_secs`/`subsec_nanos` (`creusot_std::std::
        /// time`) — but `#[check(ghost)]` extern-spec methods are still
        /// *program* functions, not `#[logic]` ones, so `result.as_secs()`
        /// can't be called directly inside `#[ensures]` any more than
        /// `String::len()` could — confirmed by a real translation error,
        /// not a guess: `error: called program function 'std::time::
        /// Duration::as_secs' in logic context`. Unlike `String::len`,
        /// no local `#[trusted]` wrapper is needed to route around it:
        /// `creusot_std::std::time` already exports `nanos_to_secs`/
        /// `secs_to_nanos` as plain `#[logic(open)]` functions (the exact
        /// terms `as_secs`/`subsec_nanos`'s own postconditions are stated
        /// in), so the claim below is expressed directly in terms of
        /// `result@` (the `View` operator, Duration's total nanosecond
        /// count as Pearlite's arbitrary-precision `Int`) and those
        /// existing logic functions instead.
        ///
        /// This also means this harness proves less than the Kani one:
        /// Kani exercises the real `std::time::Duration` implementation,
        /// symbolically; this proves only that `creusot-std`'s OWN trusted
        /// axiom for `Duration::new`'s total nanosecond count decomposes
        /// the way `as_secs`/`subsec_nanos`'s OWN trusted axioms claim it
        /// should — internal consistency between two independently-trusted
        /// specifications, not agreement with the real implementation.
        #[requires(secs@ + (nanos@ / 1_000_000_000) <= u64::MAX@)]
        #[ensures(nanos_to_secs(result@) == secs@ + (nanos@ / 1_000_000_000))]
        #[ensures(result@ % 1_000_000_000 == nanos@ % 1_000_000_000)]
        fn verify_duration_new_normalizes_nanos_and_carries_into_secs(
            secs: u64,
            nanos: u32,
        ) -> Duration {
            Duration::new(secs, nanos)
        }
    }
}

// `NonZero::get` is a plain program function too — same restriction as
// `String::len`, no `#[check(ghost)]` contract to trip over this time
// since creusot-std has no extern_spec for `NonZero<T>` at all. Trusted
// wrapper, same shape as `string_len`.
#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
fn nonzero_i16_get(_nz: &NonZero<i16>) -> i16 {
    dead
}

amenable_derive::harness! {
    creusot, VERIFY_NONZERO_I16_ROUNDTRIPS_SRC, {
        /// `NonZero::new` succeeds iff the input is nonzero, and `.get()`
        /// round-trips the wrapped value unchanged — the same claim
        /// `amenable_kani::rust_std::num::verify_nonzero_i16` checks by
        /// symbolic execution, restated as a Creusot postcondition.
        ///
        /// `#[trusted]`, unlike every other harness in this file: `new`
        /// is uncontracted (creusot-std covers plain integers and
        /// Duration, not `NonZero<T>` at all), and giving it one myself
        /// isn't practical — `extern_spec!` requires matching the real
        /// generic signature exactly (confirmed: `extern spec generics
        /// don't match` when targeting the concrete `NonZero<i16>`
        /// alone), and the real bound is `T: ZeroablePrimitive`, an
        /// `unsafe`, sealed, doc-comment-flagged-"currently permanently
        /// unstable" trait — not something nameable from outside `std`
        /// on stable Rust. So this states the same claim Kani checks by
        /// symbolic execution, honestly marked as asserted rather than
        /// mechanically discharged, the same way `elicitation`'s own
        /// reference pattern uses `#[trusted]` for claims judged "too
        /// hard to prove" rather than silently weakening them.
        ///
        /// One width, not all twelve `amenable_kani` proves separately
        /// (`i8` through `u128`/`usize`): the coverage checklist resolves
        /// every `NonZero{I,U}*` type alias back to the same evidence,
        /// `RustStdStandard<NonZero<i16>>`, so one representative case is
        /// what actually closes the gap there.
        #[trusted]
        #[requires(true)]
        #[ensures(match result {
            Some(_) => value != 0i16,
            None => value == 0i16,
        })]
        #[ensures(match result {
            Some(nz) => nonzero_i16_get(&nz) == value,
            None => true,
        })]
        fn verify_nonzero_i16_roundtrips(value: i16) -> Option<NonZero<i16>> {
            NonZero::new(value)
        }
    }
}

// `Ordering::reverse` is uncontracted (creusot-std has no coverage for
// `core::cmp::Ordering` at all) — and unlike `String::len`/`Duration::
// as_secs`, matching the `(o, result)` pair structurally in `#[ensures]`
// *without* calling `.reverse()` there doesn't route around it: the
// harness body still calls `.reverse()` to produce `result`, and calling
// any uncontracted external function yields an impossible precondition
// for the WHOLE goal, not just for logic-context call sites — confirmed
// by a real prove failure (`Goal ...vc_verify_ordering_reverse_swaps_
// less_and_greater: ✘`), not a guess. Unlike `NonZero::new`, though,
// `Ordering::reverse` has no generics and no sealed trait bound
// (`pub const fn reverse(self) -> Ordering`), so a local `extern_spec!`
// for it is actually practical — the same trusted-axiom pattern
// `creusot-std` itself uses for `Duration::new`, just written here
// instead of shipped upstream.
#[cfg(creusot)]
extern_spec! {
    impl Ordering {
        #[check(ghost)]
        #[ensures(match (self, result) {
            (Ordering::Less, Ordering::Greater) => true,
            (Ordering::Equal, Ordering::Equal) => true,
            (Ordering::Greater, Ordering::Less) => true,
            _ => false,
        })]
        fn reverse(self) -> Ordering;
    }
}

amenable_derive::harness! {
    creusot, VERIFY_ORDERING_REVERSE_SWAPS_LESS_AND_GREATER_SRC, {
        /// `Ordering` has exactly three inhabitants, and `.reverse()`
        /// swaps `Less`/`Greater` while fixing `Equal` — the same claim
        /// `amenable_kani::rust_std::cmp::verify_ordering_reverse_involution`
        /// checks (there, stated as an involution:
        /// `o.reverse().reverse() == o`, over an explicit enumeration of
        /// all three variants, since Kani has no `Arbitrary` impl for
        /// `Ordering`). Rests on the local `extern_spec!` above, which
        /// states the same swap as a trusted axiom on `reverse` itself —
        /// this harness just confirms the axiom is available and usable
        /// where a real proof needs it, the same relationship every
        /// `Duration` clause here has to `creusot-std`'s own axioms.
        ///
        /// Matching the `(o, result)` pair, not calling `.reverse()`
        /// again inside `#[ensures]`, already implies the involution
        /// Kani checks explicitly (applying the same swap twice is the
        /// identity), so no separate reverse-twice clause is needed.
        #[requires(true)]
        #[ensures(match (o, result) {
            (Ordering::Less, Ordering::Greater) => true,
            (Ordering::Equal, Ordering::Equal) => true,
            (Ordering::Greater, Ordering::Less) => true,
            _ => false,
        })]
        fn verify_ordering_reverse_swaps_less_and_greater(o: Ordering) -> Ordering {
            o.reverse()
        }
    }
}

// Unlike `NonZero<T>`, `Wrapping<T>`'s arithmetic impls aren't one
// generic `impl<T: Sealed> Add for Wrapping<T>` — std generates a
// separate, concrete `impl Add for Wrapping<i32>` (and one per other
// width) via a `macro_rules!` (`library/core/src/num/wrapping.rs`,
// confirmed by reading the real source, not assumed), so an
// `extern_spec!` targeting this one concrete instantiation matches the
// real signature exactly, the same way `Ordering::reverse`'s did.
// `.0` is a public tuple-field projection, not a method call, so it's
// fine inside `#[ensures]` without a trusted wrapper; the plain
// (non-`@`) `+` between the two `i32` fields relies on Pearlite's native
// machine-integer semantics matching real wraparound, the same
// convention `creusot-std`'s own `spec_op_common!` macro uses for
// `i32::wrapping_add`'s postcondition.
#[cfg(creusot)]
extern_spec! {
    impl std::ops::Add for Wrapping<i32> {
        #[check(ghost)]
        #[ensures(result.0 == self.0 + rhs.0)]
        fn add(self, rhs: Wrapping<i32>) -> Wrapping<i32>;
    }
}

amenable_derive::harness! {
    creusot, VERIFY_WRAPPING_ADD_MATCHES_THE_INNER_WRAPPING_ADD_SRC, {
        /// `Wrapping<T>`'s `+` operator wraps on overflow exactly like the
        /// inner type's `wrapping_add` — the same claim
        /// `amenable_kani::rust_std::num::verify_wrapping_add_matches_the_inner_wrapping_add`
        /// checks by symbolic execution (there, comparing against
        /// `a.wrapping_add(b)` directly). Rests on the local `extern_spec!`
        /// above, the same relationship every other non-`char`/`String`
        /// harness in this file has to a trusted axiom on the real method
        /// it exercises.
        #[requires(true)]
        #[ensures(result.0 == a.0 + b.0)]
        fn verify_wrapping_i32_add_wraps(a: Wrapping<i32>, b: Wrapping<i32>) -> Wrapping<i32> {
            a + b
        }
    }
}

// Same per-concrete-type macro shape as `Wrapping<T>` (confirmed by
// reading the real source, `library/core/src/num/saturating.rs`: `impl
// const Add for Saturating<$t>` generated once per width, not one
// generic sealed-trait impl), so a local `extern_spec!` is practical
// here too — but the semantics are clamping, not wraparound, so the
// postcondition restates `creusot-std`'s own three-way `@`-lifted
// contract for the plain `i32::saturating_add` method (`spec_op_common!`
// in `creusot_std::std::num`) in terms of the wrapper's `.0` fields,
// rather than reusing Wrapping's plain-`+` idiom.
#[cfg(creusot)]
extern_spec! {
    impl std::ops::Add for Saturating<i32> {
        #[check(ghost)]
        #[ensures(
            (self.0@ + rhs.0@) >= i32::MIN@ && (self.0@ + rhs.0@) <= i32::MAX@
            ==> result.0@ == (self.0@ + rhs.0@)
        )]
        #[ensures((self.0@ + rhs.0@) < i32::MIN@ ==> result.0@ == i32::MIN@)]
        #[ensures((self.0@ + rhs.0@) > i32::MAX@ ==> result.0@ == i32::MAX@)]
        fn add(self, rhs: Saturating<i32>) -> Saturating<i32>;
    }
}

amenable_derive::harness! {
    creusot, VERIFY_SATURATING_ADD_MATCHES_THE_INNER_SATURATING_ADD_SRC, {
        /// `Saturating<T>`'s `+` operator saturates at the numeric bounds
        /// exactly like the inner type's `saturating_add` — the same
        /// claim `amenable_kani::rust_std::num::verify_saturating_add_matches_the_inner_saturating_add`
        /// checks by symbolic execution. Rests on the local `extern_spec!`
        /// above, which restates `creusot-std`'s own trusted axiom for
        /// `i32::saturating_add` in terms of `Saturating<i32>`'s wrapper
        /// field.
        #[requires(true)]
        #[ensures(
            (a.0@ + b.0@) >= i32::MIN@ && (a.0@ + b.0@) <= i32::MAX@
            ==> result.0@ == (a.0@ + b.0@)
        )]
        #[ensures((a.0@ + b.0@) < i32::MIN@ ==> result.0@ == i32::MIN@)]
        #[ensures((a.0@ + b.0@) > i32::MAX@ ==> result.0@ == i32::MAX@)]
        fn verify_saturating_i32_add_clamps(a: Saturating<i32>, b: Saturating<i32>) -> Saturating<i32> {
            a + b
        }
    }
}

// Trusted logic wrapper for `ParseIntError::kind()` — same shape as
// `nonzero_i16_get`/`string_len`: an ordinary getter, modeled as an axiom
// tying a real method's result to a logic-context-callable value. Used
// both by the `FromStr` extern_spec below (to state what error kind a
// given input produces) and by the harness itself (to check the result).
#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
fn parse_int_error_kind(_e: &ParseIntError) -> IntErrorKind {
    dead
}

// Real, computable (`#[logic(open)]`, not opaque) — whether a char is an
// ASCII digit. `c@` is char's own View (Unicode scalar value as `Int`,
// same operator the char contract above uses); 48/57 are `'0'`/`'9'`.
#[cfg(creusot)]
#[logic(open)]
fn is_ascii_digit(c: char) -> bool {
    pearlite! { c@ >= 48 && c@ <= 57 }
}

// `str::parse::<i32>()` (`FromStr::from_str`) is uncontracted everywhere
// — not `creusot-std` (checked: no `FromStr` coverage for integers at
// all), not `elicitation` (checked: no prior art). Four clauses below,
// each a real, general (not just-for-these-inputs) but *sufficient*
// (not exhaustive) condition — true for every string matching the
// pattern, not merely the four concrete ones the harness exercises:
//
// - Empty: exact, matches real behavior for every empty string.
// - InvalidDigit: exact — any character outside an optional leading
//   sign that isn't an ASCII digit forces this outcome, for any string.
// - Pos/NegOverflow: deliberately *not* exact (no digit-value
//   accumulation, which would need a recursive logic function over the
//   digit sequence to state precisely) — instead: an all-digit string
//   with a nonzero leading digit and more than 10 digits (i32::MAX is
//   10 digits) is unconditionally too large for `i32` regardless of the
//   exact value, since the leading nonzero digit alone already puts the
//   magnitude at or above 10^10. True for any string of that shape, not
//   only the 20-nines literal the harness happens to use.
#[cfg(creusot)]
extern_spec! {
    impl core::str::FromStr for i32 {
        #[check(ghost)]
        #[ensures(s@.len() == 0 ==> match result {
            Err(ref e) => parse_int_error_kind(e) == IntErrorKind::Empty,
            Ok(_) => false,
        })]
        #[ensures(
            (exists<i: Int> 0 <= i && i < s@.len()
                && !(i == 0 && (s@[i] == '+' || s@[i] == '-'))
                && !is_ascii_digit(s@[i]))
            ==> match result {
                Err(ref e) => parse_int_error_kind(e) == IntErrorKind::InvalidDigit,
                Ok(_) => false,
            }
        )]
        #[ensures(
            s@.len() > 10
                && is_ascii_digit(s@[0]) && s@[0] != '0'
                && forall<i: Int> 0 <= i && i < s@.len() ==> is_ascii_digit(s@[i])
            ==> match result {
                Err(ref e) => parse_int_error_kind(e) == IntErrorKind::PosOverflow,
                Ok(_) => false,
            }
        )]
        #[ensures(
            s@.len() > 11
                && s@[0] == '-'
                && is_ascii_digit(s@[1]) && s@[1] != '0'
                && forall<i: Int> 1 <= i && i < s@.len() ==> is_ascii_digit(s@[i])
            ==> match result {
                Err(ref e) => parse_int_error_kind(e) == IntErrorKind::NegOverflow,
                Ok(_) => false,
            }
        )]
        fn from_str(s: &str) -> Result<i32, ParseIntError>;
    }
}

// `NonZero<i32>::from_str` is a *different* `FromStr` impl from `i32`'s own
// (`impl FromStr for NonZero<$Int>`, generated once per concrete width by
// the same `nonzero_integer!` macro that generates `Wrapping`/`Saturating`'s
// per-width arithmetic impls — confirmed by reading the real source, not
// assumed), so it needs its own extern_spec rather than following from the
// one above. The real impl parses via `from_str_radix`/`from_ascii_radix`
// (accepts a valid `i32` first, then checks nonzero), so "the input is
// exactly the one-character string `\"0\"`" is both real and exact for the
// `Zero` outcome — not a narrowed sufficient condition the way the
// Pos/NegOverflow clauses above are, since it's the only single-digit
// all-zero string there is.
#[cfg(creusot)]
extern_spec! {
    impl core::str::FromStr for NonZero<i32> {
        #[check(ghost)]
        #[ensures(s@.len() == 1 && s@[0] == '0' ==> match result {
            Err(ref e) => parse_int_error_kind(e) == IntErrorKind::Zero,
            Ok(_) => false,
        })]
        fn from_str(s: &str) -> Result<NonZero<i32>, ParseIntError>;
    }
}

amenable_derive::harness! {
    creusot, VERIFY_INT_ERROR_KIND_CLASSIFIES_PARSE_FAILURES_SRC, {
        /// Each representative integer-parse failure mode produces the
        /// matching `IntErrorKind` variant — the same claim
        /// `amenable_kani::rust_std::num::verify_int_error_kind_classifies_parse_failures`
        /// checks, restated as a real, `why3find`-discharged Creusot
        /// postcondition against the local `FromStr` extern_specs above,
        /// not `#[trusted]`. All five of Kani's cases, not four: `Zero`
        /// (parsing `"0"` as `NonZero<i32>`) is now covered too, via the
        /// second extern_spec above.
        ///
        /// Calls `<i32 as FromStr>::from_str`/`<NonZero<i32> as
        /// FromStr>::from_str` directly, not `s.parse::<T>()`:
        /// `str::parse<F>` is a distinct generic wrapper method
        /// (`FromStr::from_str(self)`, called through, not inlined), so
        /// extern-speccing `from_str` doesn't cover calls made through
        /// `parse` — confirmed by a real warning (`calling external
        /// function 'parse' with no contract will yield an impossible
        /// precondition`) before switching call sites.
        #[requires(true)]
        #[ensures(match result {
            (Err(ref e1), Err(ref e2), Err(ref e3), Err(ref e4), Err(ref e5)) => {
                parse_int_error_kind(e1) == IntErrorKind::Empty
                    && parse_int_error_kind(e2) == IntErrorKind::InvalidDigit
                    && parse_int_error_kind(e3) == IntErrorKind::PosOverflow
                    && parse_int_error_kind(e4) == IntErrorKind::NegOverflow
                    && parse_int_error_kind(e5) == IntErrorKind::Zero
            }
            _ => false,
        })]
        fn verify_int_error_kind_classifies_parse_failures() -> (
            Result<i32, ParseIntError>,
            Result<i32, ParseIntError>,
            Result<i32, ParseIntError>,
            Result<i32, ParseIntError>,
            Result<NonZero<i32>, ParseIntError>,
        ) {
            (
                <i32 as std::str::FromStr>::from_str(""),
                <i32 as std::str::FromStr>::from_str("not a number"),
                <i32 as std::str::FromStr>::from_str("99999999999999999999"),
                <i32 as std::str::FromStr>::from_str("-99999999999999999999"),
                <NonZero<i32> as std::str::FromStr>::from_str("0"),
            )
        }
    }
}

// `impl TryFrom<i32> for u8` is generated once per concrete
// (source, target) pair by `impl_try_from_both_bounded!`
// (`library/core/src/convert/num.rs`, confirmed by reading the real
// source, not assumed) — same per-concrete-instantiation shape as
// `Wrapping`/`Saturating`'s arithmetic impls and `Ordering::reverse`, so a
// local `extern_spec!` targeting this one pair matches the real signature
// exactly. Unlike `IntErrorKind`'s parsing contract, this one is exact, not
// merely sufficient: the real body is `if u < 0 { Err(NegOverflow) } else
// if u > 255 { Err(PosOverflow) } else { Ok(u as u8) }`, so "fits in
// 0..=255" is precisely the success condition, not an approximation of it.
// No `creusot-std` coverage and no `elicitation` prior art for
// `TryFromIntError`/`TryFrom` (checked both first).
#[cfg(creusot)]
extern_spec! {
    impl TryFrom<i32> for u8 {
        #[check(ghost)]
        #[ensures(match result {
            Ok(v) => value@ >= 0 && value@ <= 255 && v@ == value@,
            Err(_) => value@ < 0 || value@ > 255,
        })]
        fn try_from(value: i32) -> Result<u8, TryFromIntError>;
    }
}

amenable_derive::harness! {
    creusot, VERIFY_TRY_FROM_INT_ERROR_OCCURS_EXACTLY_WHEN_OUT_OF_RANGE_SRC, {
        /// `u8::try_from(i32)` fails with `TryFromIntError` exactly when the
        /// source value doesn't fit in `u8`, and succeeds with the same
        /// value otherwise — the same claim
        /// `amenable_kani::rust_std::num::verify_try_from_int_error_occurs_exactly_when_out_of_range`
        /// checks by symbolic execution over `kani::any()`, restated as a
        /// real Creusot postcondition against the local `extern_spec`
        /// above (not `#[trusted]`): both directions of the iff are a
        /// single `match` clause there, so this harness just confirms the
        /// axiom is usable at a concrete call site, the same relationship
        /// every non-`char`/`String` harness in this file has to a trusted
        /// axiom on the real method it exercises.
        #[requires(true)]
        #[ensures(match result {
            Ok(v) => value@ >= 0 && value@ <= 255 && v@ == value@,
            Err(_) => value@ < 0 || value@ > 255,
        })]
        fn verify_try_from_int_error_occurs_exactly_when_out_of_range(
            value: i32,
        ) -> Result<u8, TryFromIntError> {
            u8::try_from(value)
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_PARSE_INT_ERROR_REPORTS_THE_KIND_OF_THE_FAILURE_SRC, {
        /// `ParseIntError::kind()` reports the specific reason the parse
        /// failed, not just that it failed — the same claim
        /// `amenable_kani::rust_std::num::verify_parse_int_error_reports_the_kind_of_the_failure`
        /// checks by symbolic execution. Already implied by the
        /// `InvalidDigit` clause of the `FromStr for i32` `extern_spec!`
        /// above (which every other `IntErrorKind` harness in this file
        /// also rests on): this harness just states that same fact as
        /// `ParseIntError`'s own claim, at the one concrete input Kani
        /// exercises.
        #[requires(true)]
        #[ensures(match &result {
            Err(e) => parse_int_error_kind(e) == IntErrorKind::InvalidDigit,
            Ok(_) => false,
        })]
        fn verify_parse_int_error_reports_the_kind_of_the_failure() -> Result<i32, ParseIntError>
        {
            <i32 as std::str::FromStr>::from_str("not a number")
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_FP_CATEGORY_MATCHES_THE_VALUE_IT_CLASSIFIES_SRC, {
        /// Each representative floating-point value classifies into the
        /// `FpCategory` variant matching its own `is_*` predicates — the
        /// same claim
        /// `amenable_kani::rust_std::num::verify_fp_category_matches_the_value_it_classifies`
        /// checks by symbolic execution.
        ///
        /// `#[trusted]`, unlike every real proof in this file: `f64` has
        /// no `View` impl in `creusot-std` at all (`self@` is
        /// unavailable), and a bare float literal inside
        /// `#[ensures]`/`#[requires]` panics `creusot-rustc` outright (a
        /// real internal compiler error, not a diagnosed one) — both
        /// confirmed, not guessed; see the `f64_has_no_view_impl_at_all`
        /// and `float_literals_in_pearlite_ice_the_compiler` gallery
        /// findings. The postcondition below never needs a float
        /// literal or `@` itself (it only compares the resulting
        /// `FpCategory` values, an ordinary enum), so it parses and would
        /// translate — but there is no way to give `f64::classify` a real
        /// `extern_spec!` connecting an arbitrary input float to its
        /// category under these constraints, so the harness body's own
        /// float literals (needed to construct the five representative
        /// inputs) are what force `#[trusted]` here, the same honest
        /// fallback `NonZero::new` uses for its own genuine, confirmed
        /// blocker.
        #[trusted]
        #[requires(true)]
        #[ensures(match result {
            (FpCategory::Nan, FpCategory::Infinite, FpCategory::Zero, FpCategory::Normal, FpCategory::Subnormal) => true,
            _ => false,
        })]
        fn verify_fp_category_matches_the_value_it_classifies() -> (
            FpCategory,
            FpCategory,
            FpCategory,
            FpCategory,
            FpCategory,
        ) {
            let subnormal = f64::MIN_POSITIVE / 2.0;
            (
                f64::NAN.classify(),
                f64::INFINITY.classify(),
                0.0f64.classify(),
                f64::MIN_POSITIVE.classify(),
                subnormal.classify(),
            )
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_PARSE_FLOAT_ERROR_OCCURS_ONLY_FOR_UNPARSEABLE_INPUT_SRC, {
        /// A non-numeric string fails to parse as `f64` with
        /// `ParseFloatError`, while a valid numeric string succeeds — the
        /// same claim
        /// `amenable_kani::rust_std::num::verify_parse_float_error_occurs_only_for_unparseable_input`
        /// checks by symbolic execution.
        ///
        /// `#[trusted]`, unlike `ParseIntError`'s analogous harness: this
        /// claim never needs to characterize a float VALUE (only
        /// `Result::is_ok`/`is_err`), so it looked tractable by the same
        /// char/int-literal-only technique `IntErrorKind`'s Pos/NegOverflow
        /// clauses use — and a real `extern_spec!` for `FromStr for f64`
        /// using exactly that technique DOES translate cleanly (`cargo
        /// creusot -- -p amenable_creusot` succeeds, including a
        /// well-formedness check on the extern_spec itself). But
        /// `why3find prove`'s automatic strategy fails to discharge the
        /// harness's own goal against it: the goal splits into two
        /// sub-cases and one is left unattempted (`null` in the emitted
        /// `proof.json`, not a reported counterexample) — reproduced with
        /// the Err clause alone, the Ok clause alone, and both together,
        /// all three isolate to the same unresolved split. The identical
        /// technique (`s@.len()`/`s@[i]` char comparisons via
        /// `is_ascii_digit`) proves fine for `i32`'s `FromStr` in this
        /// same file, so the difference is specific to `f64` appearing in
        /// the `Result` — not fully root-caused (no diagnostic points at
        /// a specific cause the way the `f64` View/literal ICEs do for
        /// `FpCategory`), but confirmed reproducible across several
        /// independent attempts, not a "looks hard" guess. See
        /// `amenable_std::creusot_gallery`'s
        /// `parse_float_error_extern_spec_translates_but_wont_discharge`
        /// finding for the full repro.
        #[trusted]
        #[requires(true)]
        #[ensures(match result {
            (Err(_), Ok(_)) => true,
            _ => false,
        })]
        fn verify_parse_float_error_occurs_only_for_unparseable_input()
        -> (Result<f64, ParseFloatError>, Result<f64, ParseFloatError>) {
            (
                <f64 as std::str::FromStr>::from_str("not a float"),
                <f64 as std::str::FromStr>::from_str("3.14"),
            )
        }
    }
}

// `Reverse<T>: Ord` is ONE generic impl (`impl<T: Ord> Ord for
// Reverse<T>`, confirmed by reading the real source,
// `library/core/src/cmp.rs`), not a per-concrete-type macro like
// `Wrapping`/`Saturating` — closer in shape to `NonZero::new`'s generic
// impl than to those. A concrete `impl Ord for Reverse<i32>` extern_spec
// hits the identical "extern spec generics don't match" error
// `NonZero::new` did (confirmed, not assumed): `cmp` is defined once,
// generically. Unlike `ZeroablePrimitive`, though, `Ord` is an ordinary,
// nameable, stable trait, so the generic form is actually writable — with
// one addition: comparing `T` values via `>`/`==`/`<` inside `#[ensures]`
// needs `T: creusot_std::logic::OrdLogic` (a real, non-guessed
// requirement — the compiler's own error names it exactly:
// `the trait bound T: creusot_std::logic::OrdLogic is not satisfied`),
// creusot-std's logic-context comparison trait, distinct from the
// program-level `Ord` the real impl itself requires. So this proof is
// real and general over every `T: Ord + OrdLogic`, not narrowed to
// `i32` the way `Wrapping`/`Saturating`'s per-width proofs are.
#[cfg(creusot)]
extern_spec! {
    impl<T: Ord + creusot_std::logic::OrdLogic> Ord for Reverse<T> {
        #[check(ghost)]
        #[ensures(match result {
            Ordering::Less => other.0 > self.0,
            Ordering::Equal => other.0 == self.0,
            Ordering::Greater => other.0 < self.0,
        })]
        fn cmp(&self, other: &Reverse<T>) -> Ordering;
    }
}

amenable_derive::harness! {
    creusot, VERIFY_REVERSE_INVERTS_COMPARISON_SRC, {
        /// `Reverse<T>` inverts `T`'s comparison direction, and its `.0`
        /// field round-trips the wrapped value unchanged — the same claim
        /// `amenable_kani::rust_std::cmp::verify_reverse_inverts_comparison`
        /// checks by symbolic execution. Rests on the local `extern_spec!`
        /// above, the same relationship every non-`char`/`String` harness
        /// in this file has to a trusted axiom on the real method it
        /// exercises.
        #[requires(true)]
        #[ensures(match result.0 {
            Ordering::Less => b > a,
            Ordering::Equal => b == a,
            Ordering::Greater => b < a,
        })]
        #[ensures(result.1 == a)]
        fn verify_reverse_inverts_comparison(a: i32, b: i32) -> (Ordering, i32) {
            (Reverse(a).cmp(&Reverse(b)), Reverse(a).0)
        }
    }
}

// Unlike every non-`char`/`String` type above, `Option<T>` needs no local
// `extern_spec!` at all: `creusot_std::std::option` already ships real
// `#[check(ghost)]` contracts for `is_some`/`is_none`/`unwrap`/`unwrap_or`
// (`Option<T>: PartialEq` lets `!= None`/`== None`/`== Some(x)` appear
// directly in `#[ensures]` as native Pearlite equality, not a method
// call, so the "program function in logic context" restriction every
// other harness in this file routes around doesn't even apply here — the
// harness body calls the real methods in ordinary ghost/program context,
// and the postcondition states the same facts via plain equality on the
// results instead of re-calling them).
amenable_derive::harness! {
    creusot, VERIFY_OPTION_SOME_AND_NONE_ARE_DISJOINT_SRC, {
        /// `Some` round-trips its value through `unwrap`, and `None`
        /// falls back to `unwrap_or`'s default — the same claim
        /// `amenable_kani::rust_std::option_result::verify_option_some_and_none_are_disjoint`
        /// checks by symbolic execution, restated as a real Creusot
        /// postcondition against `creusot-std`'s own shipped `Option<T>`
        /// contracts (not a local `extern_spec!`, and not `#[trusted]`).
        #[requires(true)]
        #[ensures(result.0 != None)]
        #[ensures(result.1 == value)]
        #[ensures(result.2 == None)]
        #[ensures(result.3 == 0i32)]
        fn verify_option_some_and_none_are_disjoint(value: i32) -> (Option<i32>, i32, Option<i32>, i32) {
            let some: Option<i32> = Some(value);
            let none: Option<i32> = None;
            (some, some.unwrap(), none, none.unwrap_or(0))
        }
    }
}

// Same shape as `Option<i32>` above: `creusot_std::std::result` already
// ships real `#[check(ghost)]` contracts for
// `is_ok`/`is_err`/`unwrap`/`unwrap_err`, and `Result<T, E>: PartialEq`
// (via `T: PartialEq, E: PartialEq`) lets `== Ok(x)`/`== Err(x)` appear
// directly in `#[ensures]` as native Pearlite equality — no local
// `extern_spec!` needed.
amenable_derive::harness! {
    creusot, VERIFY_RESULT_OK_AND_ERR_ARE_DISJOINT_SRC, {
        /// `Ok` round-trips its value through `unwrap`, and `Err`
        /// round-trips its value through `unwrap_err` — the same claim
        /// `amenable_kani::rust_std::option_result::verify_result_ok_and_err_are_disjoint`
        /// checks by symbolic execution, restated as a real Creusot
        /// postcondition against `creusot-std`'s own shipped `Result<T, E>`
        /// contracts (not a local `extern_spec!`, and not `#[trusted]`).
        #[requires(true)]
        #[ensures(result.0 == value)]
        #[ensures(result.1 == err_value)]
        fn verify_result_ok_and_err_are_disjoint(value: i32, err_value: i32) -> (i32, i32) {
            let ok: Result<i32, i32> = Ok(value);
            let err: Result<i32, i32> = Err(err_value);
            (ok.unwrap(), err.unwrap_err())
        }
    }
}

// `ManuallyDrop<T>` is uncontracted everywhere — no `creusot-std`
// coverage (checked), no `elicitation` prior art (checked). Its private
// field (`value: MaybeDangling<T>`, not a public `.0`) and `into_inner`'s
// real `unsafe` raw-pointer-cast body rule out the field-projection idiom
// `Wrapping`/`Saturating`/`Reverse` use, but that doesn't block a real
// extern_spec here: every extern_spec in this file is already a trusted
// axiom on the real method, never independently checked against the
// method's own body (unsafe or not) — the same relationship `Ordering::
// reverse`'s extern_spec has to its real `match` body. `new`/`deref`/
// `into_inner` are each defined once, generically, over unconstrained `T`
// (`impl<T: ?Sized> const Deref for ManuallyDrop<T>`, `impl<T> ManuallyDrop<T>`)
// — no sealed/unstable bound the way `NonZero::new`'s `ZeroablePrimitive`
// is, so the generic extern_spec form is directly writable.
//
// `*self` can't appear in `deref`'s OWN postcondition, though — `self:
// &ManuallyDrop<T>`, so `*self` yields `ManuallyDrop<T>` (one layer of
// reference removed), not the wrapped `T`; using `deref`'s own Target
// inside a contract ABOUT `deref` is circular, confirmed by a real type
// error, not a guess (`expected type parameter T, found struct
// ManuallyDrop<T>`). Fixed the same way `String::len`/`NonZero::get`
// route around a private/unreachable value: a `#[trusted]
// #[logic(opaque)]` accessor axiomatizing "the value a `ManuallyDrop<T>`
// wraps," generic over `T` this time (every earlier trusted wrapper in
// this file was monomorphic).
#[cfg(creusot)]
#[trusted]
#[logic(opaque)]
fn manually_drop_value<T>(_m: &ManuallyDrop<T>) -> T {
    dead
}

#[cfg(creusot)]
extern_spec! {
    impl<T> ManuallyDrop<T> {
        #[check(ghost)]
        #[ensures(manually_drop_value(&result) == value)]
        fn new(value: T) -> ManuallyDrop<T>;

        #[check(ghost)]
        #[ensures(result == manually_drop_value(&slot))]
        fn into_inner(slot: ManuallyDrop<T>) -> T;
    }

    impl<T> std::ops::Deref for ManuallyDrop<T> {
        #[check(ghost)]
        #[ensures(*result == manually_drop_value(self))]
        fn deref(&self) -> &T;
    }
}

amenable_derive::harness! {
    creusot, VERIFY_MANUALLY_DROP_DEREFS_AND_INTO_INNER_ROUND_TRIP_SRC, {
        /// `ManuallyDrop` is transparent to its wrapped value through
        /// both `Deref` and `into_inner` — the same claim
        /// `amenable_kani::rust_std::mem::verify_manually_drop_derefs_and_into_inner_round_trip`
        /// checks by symbolic execution. Rests on the local `extern_spec!`
        /// above, the same relationship every non-`char`/`String` harness
        /// in this file has to a trusted axiom on the real method it
        /// exercises.
        #[requires(true)]
        #[ensures(result.0 == value)]
        #[ensures(result.1 == value)]
        fn verify_manually_drop_derefs_and_into_inner_round_trip(value: i32) -> (i32, i32) {
            let wrapped = ManuallyDrop::new(value);
            let deref_value = *wrapped;
            (deref_value, ManuallyDrop::into_inner(wrapped))
        }
    }
}
