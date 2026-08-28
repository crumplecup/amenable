//! `RustStdType` registrations for `alloc::collections`.
//!
//! All default-allocator-only, same tradeoff as `Box` (see
//! `rust_std::alloc_boxed`'s doc comment). Not covered here, all for
//! unstable-feature reasons: `binary_heap::DrainSorted`/`IntoIterSorted`
//! (`binary_heap_drain_sorted`, `binary_heap_into_iter_sorted`),
//! `linked_list::Cursor`/`CursorMut` (`linked_list_cursors`),
//! `vec_deque::Splice` (`deque_extend_front`), and `TryReserveErrorKind`
//! (`try_reserve_kind`).

use std::collections::binary_heap::{
    Drain as BinaryHeapDrain, IntoIter as BinaryHeapIntoIter, Iter as BinaryHeapIter,
    PeekMut as BinaryHeapPeekMut,
};
use std::collections::linked_list::{
    ExtractIf as LinkedListExtractIf, IntoIter as LinkedListIntoIter, Iter as LinkedListIter,
    IterMut as LinkedListIterMut,
};
use std::collections::vec_deque::{
    Drain as VecDequeDrain, IntoIter as VecDequeIntoIter, Iter as VecDequeIter,
    IterMut as VecDequeIterMut,
};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, TryReserveError, VecDeque};

use crate::rust_std::macros::{
    impl_rust_std_type, impl_rust_std_type_generic1, impl_rust_std_type_generic2,
    impl_rust_std_type_lifetime1, register_rust_std_standard_evidence,
};

impl_rust_std_type_generic2!(
    BTreeMap,
    "alloc",
    "alloc::collections::btree::map",
    "https://doc.rust-lang.org/alloc/collections/btree_map/struct.BTreeMap.html",
    "The BTreeMap carrier is an ordered map backed by a B-tree, keyed by Ord."
);

impl_rust_std_type_generic1!(
    BTreeSet,
    "alloc",
    "alloc::collections::btree::set",
    "https://doc.rust-lang.org/alloc/collections/btree_set/struct.BTreeSet.html",
    "The BTreeSet carrier is an ordered set backed by a B-tree, keyed by Ord."
);

// BinaryHeap<T: Ord> — the struct itself requires Ord (it stores elements
// in heap order), unlike BTreeMap/BTreeSet whose Ord bound lives only on
// their trait impls.
impl<T: Ord> crate::RustStdType for BinaryHeap<T> {
    fn rust_language_provenance() -> crate::RustLanguageProvenance {
        crate::RustLanguageProvenance::for_source("alloc", "alloc::collections::binary_heap")
    }

    fn rust_doc_url() -> &'static str {
        "https://doc.rust-lang.org/alloc/collections/binary_heap/struct.BinaryHeap.html"
    }

    fn rust_semantics_summary() -> &'static str {
        "The BinaryHeap carrier is a priority queue backed by a binary heap, ordered by Ord."
    }
}

impl_rust_std_type_generic1!(
    LinkedList,
    "alloc",
    "alloc::collections::linked_list",
    "https://doc.rust-lang.org/alloc/collections/linked_list/struct.LinkedList.html",
    "The LinkedList carrier is a doubly-linked list."
);

impl_rust_std_type_generic1!(
    VecDeque,
    "alloc",
    "alloc::collections::vec_deque",
    "https://doc.rust-lang.org/alloc/collections/vec_deque/struct.VecDeque.html",
    "The VecDeque carrier is a growable ring-buffer double-ended queue."
);

impl_rust_std_type!(
    TryReserveError,
    "alloc",
    "alloc::collections",
    "https://doc.rust-lang.org/alloc/collections/struct.TryReserveError.html",
    "The TryReserveError carrier reports that a fallible capacity reservation could not be satisfied."
);

impl<T: Ord> crate::RustStdType for BinaryHeapDrain<'_, T> {
    fn rust_language_provenance() -> crate::RustLanguageProvenance {
        crate::RustLanguageProvenance::for_source("alloc", "alloc::collections::binary_heap")
    }

    fn rust_doc_url() -> &'static str {
        "https://doc.rust-lang.org/alloc/collections/binary_heap/struct.Drain.html"
    }

    fn rust_semantics_summary() -> &'static str {
        "The Drain carrier removes and yields every element of a BinaryHeap in arbitrary order."
    }
}

impl<T: Ord> crate::RustStdType for BinaryHeapIntoIter<T> {
    fn rust_language_provenance() -> crate::RustLanguageProvenance {
        crate::RustLanguageProvenance::for_source("alloc", "alloc::collections::binary_heap")
    }

    fn rust_doc_url() -> &'static str {
        "https://doc.rust-lang.org/alloc/collections/binary_heap/struct.IntoIter.html"
    }

    fn rust_semantics_summary() -> &'static str {
        "The IntoIter carrier consumes a BinaryHeap, yielding its elements in arbitrary order."
    }
}

impl<T: Ord> crate::RustStdType for BinaryHeapIter<'_, T> {
    fn rust_language_provenance() -> crate::RustLanguageProvenance {
        crate::RustLanguageProvenance::for_source("alloc", "alloc::collections::binary_heap")
    }

    fn rust_doc_url() -> &'static str {
        "https://doc.rust-lang.org/alloc/collections/binary_heap/struct.Iter.html"
    }

    fn rust_semantics_summary() -> &'static str {
        "The Iter carrier lazily yields shared references to a BinaryHeap's elements in arbitrary order."
    }
}

impl<T: Ord> crate::RustStdType for BinaryHeapPeekMut<'_, T> {
    fn rust_language_provenance() -> crate::RustLanguageProvenance {
        crate::RustLanguageProvenance::for_source("alloc", "alloc::collections::binary_heap")
    }

    fn rust_doc_url() -> &'static str {
        "https://doc.rust-lang.org/alloc/collections/binary_heap/struct.PeekMut.html"
    }

    fn rust_semantics_summary() -> &'static str {
        "The PeekMut carrier is a mutable borrow of a BinaryHeap's greatest element, which re-heapifies the heap when dropped if the value was modified."
    }
}

impl_rust_std_type_lifetime1!(
    LinkedListIter,
    "alloc",
    "alloc::collections::linked_list",
    "https://doc.rust-lang.org/alloc/collections/linked_list/struct.Iter.html",
    "The Iter carrier lazily yields shared references to each element of a LinkedList."
);

impl_rust_std_type_lifetime1!(
    LinkedListIterMut,
    "alloc",
    "alloc::collections::linked_list",
    "https://doc.rust-lang.org/alloc/collections/linked_list/struct.IterMut.html",
    "The IterMut carrier lazily yields mutable references to each element of a LinkedList."
);

impl_rust_std_type_generic1!(
    LinkedListIntoIter,
    "alloc",
    "alloc::collections::linked_list",
    "https://doc.rust-lang.org/alloc/collections/linked_list/struct.IntoIter.html",
    "The IntoIter carrier consumes a LinkedList, yielding its elements in order."
);

impl<T, F: FnMut(&mut T) -> bool> crate::RustStdType for LinkedListExtractIf<'_, T, F> {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn rust_language_provenance() -> crate::RustLanguageProvenance {
        crate::RustLanguageProvenance::for_source("alloc", "alloc::collections::linked_list")
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn rust_doc_url() -> &'static str {
        "https://doc.rust-lang.org/alloc/collections/linked_list/struct.ExtractIf.html"
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
    fn rust_semantics_summary() -> &'static str {
        "The ExtractIf carrier lazily removes and yields LinkedList elements matching a predicate."
    }
}

impl_rust_std_type_lifetime1!(
    VecDequeDrain,
    "alloc",
    "alloc::collections::vec_deque",
    "https://doc.rust-lang.org/alloc/collections/vec_deque/struct.Drain.html",
    "The Drain carrier removes and yields a range of a VecDeque's elements in order."
);

impl_rust_std_type_lifetime1!(
    VecDequeIter,
    "alloc",
    "alloc::collections::vec_deque",
    "https://doc.rust-lang.org/alloc/collections/vec_deque/struct.Iter.html",
    "The Iter carrier lazily yields shared references to each element of a VecDeque."
);

impl_rust_std_type_lifetime1!(
    VecDequeIterMut,
    "alloc",
    "alloc::collections::vec_deque",
    "https://doc.rust-lang.org/alloc/collections/vec_deque/struct.IterMut.html",
    "The IterMut carrier lazily yields mutable references to each element of a VecDeque."
);

impl_rust_std_type_generic1!(
    VecDequeIntoIter,
    "alloc",
    "alloc::collections::vec_deque",
    "https://doc.rust-lang.org/alloc/collections/vec_deque/struct.IntoIter.html",
    "The IntoIter carrier consumes a VecDeque, yielding its elements in order."
);

// `i32` (and `'static` for the borrowing carriers) is the representative
// element type/lifetime this module's proof batch covers.
// `fn(&mut i32) -> bool` stands in for `LinkedListExtractIf`'s predicate
// closure, which has no nameable type of its own.
//
// The Drain/IntoIter/Iter/IterMut/PeekMut/ExtractIf carriers are written
// fully-qualified here, not via the locally-aliased `BinaryHeapDrain`-style
// names imported above: those bare names (`Drain`, `IntoIter`, `Iter`, ...)
// recur across many other containers with the exact same spelling, so only
// the qualified path disambiguates which container's carrier a given
// registration means for tooling reading the registry (e.g. `elicit_doc`'s
// coverage report).
register_rust_std_standard_evidence!(
    BTreeMap<i32, i32>,
    BTreeSet<i32>,
    BinaryHeap<i32>,
    LinkedList<i32>,
    VecDeque<i32>,
    TryReserveError,
    std::collections::binary_heap::Drain<'static, i32>,
    std::collections::binary_heap::IntoIter<i32>,
    std::collections::binary_heap::Iter<'static, i32>,
    std::collections::binary_heap::PeekMut<'static, i32>,
    std::collections::linked_list::Iter<'static, i32>,
    std::collections::linked_list::IterMut<'static, i32>,
    std::collections::linked_list::IntoIter<i32>,
    std::collections::linked_list::ExtractIf<'static, i32, fn(&mut i32) -> bool>,
    std::collections::vec_deque::Drain<'static, i32>,
    std::collections::vec_deque::Iter<'static, i32>,
    std::collections::vec_deque::IterMut<'static, i32>,
    std::collections::vec_deque::IntoIter<i32>,
);
