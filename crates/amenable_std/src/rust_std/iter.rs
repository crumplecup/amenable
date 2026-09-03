//! `RustStdType` registrations for `core::iter`.
//!
//! `Intersperse`, `IntersperseWith`, `MapWindows`, `ArrayChunks`, and
//! `ByRefSized` are deliberately not covered here — all sit behind unstable
//! feature gates (`iter_intersperse`, `iter_map_windows`,
//! `iter_array_chunks`, `std_internals`), not nameable from a stable
//! toolchain.
//!
//! `Flatten`, `Peekable`, and `FlatMap` get hand-written impls instead of
//! the generic macros: unlike most adapters (whose bounds live only on
//! their `Iterator` trait impl, not the struct itself), these three carry
//! a real bound on the struct definition itself (e.g. `Peekable<I:
//! Iterator>`, since it stores `Option<I::Item>`), so an unconstrained
//! `impl<T> RustStdType for Peekable<T>` doesn't type-check.

use std::iter::{
    Chain, Cloned, Copied, Cycle, Empty, Enumerate, Filter, FilterMap, FlatMap, Flatten, FromFn,
    Fuse, Inspect, Map, MapWhile, Once, OnceWith, Peekable, Repeat, RepeatN, RepeatWith, Rev, Scan,
    Skip, SkipWhile, StepBy, Successors, Take, TakeWhile, Zip,
};

use crate::rust_std::macros::{
    impl_rust_std_type_generic1, impl_rust_std_type_generic2, impl_rust_std_type_generic3,
    register_rust_std_standard_evidence,
};

macro_rules! iter_adapter1 {
    ($ty:ident, $summary:expr) => {
        impl_rust_std_type_generic1!(
            $ty,
            "core",
            "core::iter",
            concat!(
                "https://doc.rust-lang.org/core/iter/struct.",
                stringify!($ty),
                ".html"
            ),
            $summary
        );
    };
}

macro_rules! iter_adapter2 {
    ($ty:ident, $summary:expr) => {
        impl_rust_std_type_generic2!(
            $ty,
            "core",
            "core::iter",
            concat!(
                "https://doc.rust-lang.org/core/iter/struct.",
                stringify!($ty),
                ".html"
            ),
            $summary
        );
    };
}

iter_adapter2!(
    Map,
    "The Map carrier lazily applies a closure to each item of an underlying iterator."
);
iter_adapter2!(
    Filter,
    "The Filter carrier lazily yields only the items of an underlying iterator that satisfy a predicate."
);
iter_adapter2!(
    FilterMap,
    "The FilterMap carrier lazily applies a closure that both filters and maps each item of an underlying iterator."
);

impl<I, U: IntoIterator, F> crate::RustStdType for FlatMap<I, U, F> {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn rust_language_provenance() -> crate::RustLanguageProvenance {
        crate::RustLanguageProvenance::for_source("core", "core::iter")
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn rust_doc_url() -> &'static str {
        "https://doc.rust-lang.org/core/iter/struct.FlatMap.html"
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
    fn rust_semantics_summary() -> &'static str {
        "The FlatMap carrier lazily maps each item to an iterator and flattens the results into one sequence."
    }
}

impl<I: Iterator> crate::RustStdType for Flatten<I>
where
    I::Item: IntoIterator,
{
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn rust_language_provenance() -> crate::RustLanguageProvenance {
        crate::RustLanguageProvenance::for_source("core", "core::iter")
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn rust_doc_url() -> &'static str {
        "https://doc.rust-lang.org/core/iter/struct.Flatten.html"
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
    fn rust_semantics_summary() -> &'static str {
        "The Flatten carrier lazily flattens an iterator of iterators into one sequence."
    }
}

iter_adapter2!(
    Chain,
    "The Chain carrier lazily sequences two iterators end to end."
);
iter_adapter1!(
    Cloned,
    "The Cloned carrier lazily clones each item of an underlying iterator of references."
);
iter_adapter1!(
    Copied,
    "The Copied carrier lazily copies each item of an underlying iterator of references."
);
iter_adapter1!(
    Cycle,
    "The Cycle carrier lazily repeats an underlying iterator's sequence forever."
);
iter_adapter1!(
    Enumerate,
    "The Enumerate carrier lazily pairs each item of an underlying iterator with its index."
);
iter_adapter1!(
    Fuse,
    "The Fuse carrier guarantees an underlying iterator keeps returning None once it has done so once."
);
iter_adapter2!(
    Inspect,
    "The Inspect carrier lazily calls a closure on each item of an underlying iterator without changing it."
);
impl<I: Iterator> crate::RustStdType for Peekable<I> {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn rust_language_provenance() -> crate::RustLanguageProvenance {
        crate::RustLanguageProvenance::for_source("core", "core::iter")
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn rust_doc_url() -> &'static str {
        "https://doc.rust-lang.org/core/iter/struct.Peekable.html"
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
    fn rust_semantics_summary() -> &'static str {
        "The Peekable carrier lets an underlying iterator's next item be inspected without consuming it."
    }
}

iter_adapter1!(
    Rev,
    "The Rev carrier lazily reverses the direction of a double-ended iterator."
);
impl_rust_std_type_generic3!(
    Scan,
    "core",
    "core::iter",
    "https://doc.rust-lang.org/core/iter/struct.Scan.html",
    "The Scan carrier lazily maps each item of an underlying iterator while threading mutable state through the closure."
);
iter_adapter1!(
    Skip,
    "The Skip carrier lazily discards a fixed number of items from the start of an underlying iterator."
);
iter_adapter2!(
    SkipWhile,
    "The SkipWhile carrier lazily discards items from the start of an underlying iterator while a predicate holds."
);
iter_adapter1!(
    StepBy,
    "The StepBy carrier lazily yields every nth item of an underlying iterator."
);
iter_adapter1!(
    Take,
    "The Take carrier lazily yields at most a fixed number of items from an underlying iterator."
);
iter_adapter2!(
    TakeWhile,
    "The TakeWhile carrier lazily yields items from an underlying iterator while a predicate holds."
);
iter_adapter2!(
    MapWhile,
    "The MapWhile carrier lazily maps items from an underlying iterator while the closure keeps returning Some."
);
iter_adapter2!(
    Zip,
    "The Zip carrier lazily pairs up items from two iterators, stopping when either is exhausted."
);

iter_adapter1!(
    Once,
    "The Once carrier yields exactly one value then stops."
);
iter_adapter1!(
    OnceWith,
    "The OnceWith carrier lazily calls a closure exactly once to produce its single value."
);
iter_adapter1!(Repeat, "The Repeat carrier yields the same value forever.");
iter_adapter1!(
    RepeatWith,
    "The RepeatWith carrier lazily calls a closure to produce a value forever."
);
iter_adapter1!(
    RepeatN,
    "The RepeatN carrier yields the same value a fixed number of times."
);
iter_adapter1!(Empty, "The Empty carrier yields no values at all.");
iter_adapter2!(
    Successors,
    "The Successors carrier lazily generates a sequence where each item is computed from the previous one."
);
iter_adapter1!(
    FromFn,
    "The FromFn carrier turns a closure returning Option into an iterator."
);

// `Range<i32>` (or `Iter<'static, i32>`/`IntoIter<Range<i32>>` where an
// adapter wraps an iterator of iterators or a borrowing source) is the one
// representative source iterator this module's proof batch covers; closure
// parameters use a bare `fn` pointer type rather than a closure, since
// closures have no nameable type to register evidence against (see
// `LazyCell<i32, fn() -> i32>` in `rust_std::cell` for the same pattern).
// `Range`/`IntoIter`/`Iter` are named here without importing them: a
// `$ty:ty` macro fragment is purely syntactic (it doesn't need to resolve
// to a real item, confirmed empirically), and this macro only ever
// stringifies its argument — never uses it in a real type position — so
// there is nothing to import.
//
// `Filter` and `FilterMap` are the two exceptions to the `Range<i32>` source
// convention: their `next()` is implemented in std via `Iterator::find`,
// which routes through `Range<i32>::try_fold`. That generic try_fold loop's
// termination bound depends on a runtime comparison between two symbolic
// `i32` endpoints, so Kani's (sound) unwinder cannot conclude the loop is
// bounded no matter how tightly a `kani::assume` narrows the range — it
// still unwinds past several hundred iterations and times out. A
// `std::array::IntoIter<i32, 1>` source has the same `try_fold`-routed
// `find`, but its loop bound is a compile-time array length, so the
// unwinder concludes immediately. `FlatMap`'s outer source has the same
// hazard, so it uses the same array source while keeping its inner
// `Range<i32>` (the actual subject of the claim) unchanged. See
// `crates/amenable_kani/src/gallery/replace_recommendations.rs` for the
// direct `Range<i32>`-sourced false trail this replaces.
register_rust_std_standard_evidence!(
    Map<Range<i32>, fn(i32) -> i32>,
    Filter<std::array::IntoIter<i32, 1>, fn(&i32) -> bool>,
    FilterMap<std::array::IntoIter<i32, 1>, fn(i32) -> Option<i32>>,
    FlatMap<std::array::IntoIter<i32, 1>, Range<i32>, fn(i32) -> Range<i32>>,
    Flatten<IntoIter<Range<i32>>>,
    std::iter::Chain<Range<i32>, Range<i32>>,
    Zip<Range<i32>, Range<i32>>,
    Enumerate<Range<i32>>,
    Rev<Range<i32>>,
    Cloned<Iter<'static, i32>>,
    Copied<Iter<'static, i32>>,
    Cycle<Range<i32>>,
    Fuse<Range<i32>>,
    Inspect<Range<i32>, fn(&i32)>,
    Peekable<Range<i32>>,
    Scan<Range<i32>, i32, fn(&mut i32, i32) -> Option<i32>>,
    Skip<Range<i32>>,
    SkipWhile<Range<i32>, fn(&i32) -> bool>,
    StepBy<Range<i32>>,
    std::iter::Take<Range<i32>>,
    TakeWhile<Range<i32>, fn(&i32) -> bool>,
    MapWhile<Range<i32>, fn(i32) -> Option<i32>>,
    std::iter::Once<i32>,
    OnceWith<fn() -> i32>,
    std::iter::Repeat<i32>,
    RepeatWith<fn() -> i32>,
    RepeatN<i32>,
    std::iter::Empty<i32>,
    Successors<i32, fn(&i32) -> Option<i32>>,
    std::iter::FromFn<fn() -> Option<i32>>,
);
