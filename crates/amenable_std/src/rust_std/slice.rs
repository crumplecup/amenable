//! `RustStdType` registrations for `core::slice`.
//!
//! `ArrayWindows<'a, T, const N: usize>` is deliberately not covered here —
//! unstable (`array_windows`), and const-generic besides (none of this
//! module's macros model a const parameter).

use std::slice::{
    ChunkBy, ChunkByMut, Chunks, ChunksExact, ChunksExactMut, ChunksMut, EscapeAscii,
    GetDisjointMutError, Iter, IterMut, RChunks, RChunksExact, RChunksExactMut, RChunksMut, RSplit,
    RSplitMut, RSplitN, RSplitNMut, Split, SplitInclusive, SplitInclusiveMut, SplitMut, SplitN,
    SplitNMut, Windows,
};

use crate::rust_std::macros::{
    impl_rust_std_type, impl_rust_std_type_lifetime0, impl_rust_std_type_lifetime1,
    impl_rust_std_type_lifetime2_pred1, impl_rust_std_type_lifetime2_pred2,
    register_rust_std_standard_evidence,
};

macro_rules! slice_iter1 {
    ($ty:ident, $summary:expr) => {
        impl_rust_std_type_lifetime1!(
            $ty,
            "core",
            "core::slice",
            concat!(
                "https://doc.rust-lang.org/core/slice/struct.",
                stringify!($ty),
                ".html"
            ),
            $summary
        );
    };
}

/// For the `Split`/`RSplit`/`*N` family: `P: FnMut(&T) -> bool` on the
/// struct itself.
macro_rules! slice_split_family {
    ($ty:ident, $summary:expr) => {
        impl_rust_std_type_lifetime2_pred1!(
            $ty,
            "core",
            "core::slice",
            concat!(
                "https://doc.rust-lang.org/core/slice/struct.",
                stringify!($ty),
                ".html"
            ),
            $summary
        );
    };
}

/// For `ChunkBy`/`ChunkByMut`: `P: FnMut(&T, &T) -> bool` on the struct
/// itself (a binary adjacency predicate, unlike the `Split` family's unary
/// one).
macro_rules! slice_chunk_by_family {
    ($ty:ident, $summary:expr) => {
        impl_rust_std_type_lifetime2_pred2!(
            $ty,
            "core",
            "core::slice",
            concat!(
                "https://doc.rust-lang.org/core/slice/struct.",
                stringify!($ty),
                ".html"
            ),
            $summary
        );
    };
}

slice_iter1!(
    Chunks,
    "The Chunks carrier lazily yields non-overlapping, non-final-short chunks of a slice."
);
slice_iter1!(
    ChunksExact,
    "The ChunksExact carrier lazily yields non-overlapping, exactly-sized chunks of a slice, discarding any remainder."
);
slice_iter1!(
    ChunksExactMut,
    "The ChunksExactMut carrier lazily yields non-overlapping, exactly-sized mutable chunks of a slice, discarding any remainder."
);
slice_iter1!(
    ChunksMut,
    "The ChunksMut carrier lazily yields non-overlapping, non-final-short mutable chunks of a slice."
);
slice_iter1!(
    Iter,
    "The Iter carrier lazily yields shared references to each element of a slice."
);
slice_iter1!(
    IterMut,
    "The IterMut carrier lazily yields mutable references to each element of a slice."
);
slice_iter1!(
    RChunks,
    "The RChunks carrier lazily yields non-overlapping chunks of a slice from the back."
);
slice_iter1!(
    RChunksExact,
    "The RChunksExact carrier lazily yields non-overlapping, exactly-sized chunks of a slice from the back, discarding any remainder."
);
slice_iter1!(
    RChunksExactMut,
    "The RChunksExactMut carrier lazily yields non-overlapping, exactly-sized mutable chunks of a slice from the back, discarding any remainder."
);
slice_iter1!(
    RChunksMut,
    "The RChunksMut carrier lazily yields non-overlapping mutable chunks of a slice from the back."
);
slice_iter1!(
    Windows,
    "The Windows carrier lazily yields overlapping windows of a fixed size over a slice."
);

slice_chunk_by_family!(
    ChunkBy,
    "The ChunkBy carrier lazily groups consecutive slice elements sharing a predicate-defined relation."
);
slice_chunk_by_family!(
    ChunkByMut,
    "The ChunkByMut carrier lazily groups consecutive mutable slice elements sharing a predicate-defined relation."
);
slice_split_family!(
    RSplit,
    "The RSplit carrier lazily splits a slice on a predicate, yielding subslices from the back."
);
slice_split_family!(
    RSplitMut,
    "The RSplitMut carrier lazily splits a mutable slice on a predicate, yielding subslices from the back."
);
slice_split_family!(
    RSplitN,
    "The RSplitN carrier lazily splits a slice on a predicate into at most n subslices, from the back."
);
slice_split_family!(
    RSplitNMut,
    "The RSplitNMut carrier lazily splits a mutable slice on a predicate into at most n subslices, from the back."
);
slice_split_family!(
    Split,
    "The Split carrier lazily splits a slice on a predicate, yielding subslices."
);
slice_split_family!(
    SplitInclusive,
    "The SplitInclusive carrier lazily splits a slice on a predicate, keeping the matched element at the end of each subslice."
);
slice_split_family!(
    SplitInclusiveMut,
    "The SplitInclusiveMut carrier lazily splits a mutable slice on a predicate, keeping the matched element at the end of each subslice."
);
slice_split_family!(
    SplitMut,
    "The SplitMut carrier lazily splits a mutable slice on a predicate, yielding subslices."
);
slice_split_family!(
    SplitN,
    "The SplitN carrier lazily splits a slice on a predicate into at most n subslices."
);
slice_split_family!(
    SplitNMut,
    "The SplitNMut carrier lazily splits a mutable slice on a predicate into at most n subslices."
);

impl_rust_std_type_lifetime0!(
    EscapeAscii,
    "core",
    "core::slice",
    "https://doc.rust-lang.org/core/slice/struct.EscapeAscii.html",
    "The EscapeAscii carrier lazily yields the printable-ASCII-escaped bytes of a byte slice."
);

impl_rust_std_type!(
    GetDisjointMutError,
    "core",
    "core::slice",
    "https://doc.rust-lang.org/core/slice/enum.GetDisjointMutError.html",
    "The GetDisjointMutError carrier reports why a request for multiple disjoint mutable slice elements failed."
);

// `'static` and `i32` are the representative lifetime/element type this
// module's proof batch covers; `fn(&i32) -> bool` / `fn(&i32, &i32) -> bool`
// stand in for the split/chunk-by families' predicate closures, which have
// no nameable type of their own (see `register_rust_std_standard_evidence!`'s
// own doc comment).
register_rust_std_standard_evidence!(
    Chunks<'static, i32>,
    ChunksExact<'static, i32>,
    ChunksExactMut<'static, i32>,
    ChunksMut<'static, i32>,
    Iter<'static, i32>,
    IterMut<'static, i32>,
    RChunks<'static, i32>,
    RChunksExact<'static, i32>,
    RChunksExactMut<'static, i32>,
    RChunksMut<'static, i32>,
    Windows<'static, i32>,
    ChunkBy<'static, i32, fn(&i32, &i32) -> bool>,
    ChunkByMut<'static, i32, fn(&i32, &i32) -> bool>,
    RSplit<'static, i32, fn(&i32) -> bool>,
    RSplitMut<'static, i32, fn(&i32) -> bool>,
    RSplitN<'static, i32, fn(&i32) -> bool>,
    RSplitNMut<'static, i32, fn(&i32) -> bool>,
    Split<'static, i32, fn(&i32) -> bool>,
    SplitInclusive<'static, i32, fn(&i32) -> bool>,
    SplitInclusiveMut<'static, i32, fn(&i32) -> bool>,
    SplitMut<'static, i32, fn(&i32) -> bool>,
    SplitN<'static, i32, fn(&i32) -> bool>,
    SplitNMut<'static, i32, fn(&i32) -> bool>,
    EscapeAscii<'static>,
    GetDisjointMutError,
);
