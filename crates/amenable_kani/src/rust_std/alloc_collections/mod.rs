//! `KaniWitness` impls for `alloc::collections`.
//!
//! `BinaryHeap`'s `Drain`/`IntoIter`/`Iter` proofs deliberately claim only
//! that every pushed element comes out exactly once (checked via sorting
//! both sides) — not that they come out in priority order. Confirmed
//! empirically first: `BinaryHeap::into_iter()` over `[3, 1, 2]` (pushed in
//! that order) yields `[3, 1, 2]`, the heap's internal array order, not
//! `[1, 2, 3]`. Only `.pop()` (and `.peek()`/`.peek_mut()`) guarantee
//! priority order — that distinction is exactly what `BinaryHeap`'s own
//! proof states, in contrast to these three.
//!
//! Split by collection: [`btree`] (`BTreeMap`/`BTreeSet`, plus their
//! shared ordering/emptiness markers), [`binary_heap`], [`binary_heap_iterators`],
//! [`linked_list_vec_deque_basic`] (`LinkedList`/`VecDeque`/`TryReserveError`),
//! [`shared_markers`] (the reference/peek markers several iterator
//! proofs across this crate share), [`linked_list_iterators`], and
//! [`vec_deque_iterators`].

mod binary_heap;
mod binary_heap_iterators;
mod btree;
mod linked_list_iterators;
mod linked_list_vec_deque_basic;
mod shared_markers;
mod vec_deque_iterators;

pub use binary_heap_iterators::PeekRevealsTheStoredReference;
pub use btree::{EmptiedContainerReportsEmpty, FirstValueIsLessThanTheSecond};
pub use shared_markers::IteratorYieldsAReferenceToTheStoredValue;
