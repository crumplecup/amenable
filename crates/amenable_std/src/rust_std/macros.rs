//! Bulk-registration macros for `RustStdType`, so coverage work over the
//! full std-lib surface doesn't need hand-written boilerplate per type.
//!
//! These only implement `RustStdType` itself (a static-metadata trait with
//! no bound on any type parameter) — they intentionally do not touch
//! `RustStdStandard<T>` evidence registration or verifier proofs, which are
//! a separate, later pass (see `register_rust_std_standard_evidence!`).

/// Implement `RustStdType` for a concrete (non-generic) type.
macro_rules! impl_rust_std_type {
    ($ty:ty, $source_crate:literal, $source_module:literal, $url:expr, $summary:expr) => {
        impl $crate::RustStdType for $ty {
            fn rust_language_provenance() -> $crate::RustLanguageProvenance {
                $crate::RustLanguageProvenance::for_source($source_crate, $source_module)
            }

            fn rust_doc_url() -> &'static str {
                $url
            }

            fn rust_semantics_summary() -> &'static str {
                $summary
            }
        }
    };
}

/// Implement `RustStdType` for a type generic over one type parameter
/// (e.g. `Reverse<T>`, `Box<T>`). `RustStdType`'s methods are static
/// metadata with no dependency on `T` at all, so no bound on `T` is needed
/// — deliberately so, since a `'static` bound here would reject every
/// borrowing type (`slice::Iter<'a, T>` and friends).
macro_rules! impl_rust_std_type_generic1 {
    ($ty:ident, $source_crate:literal, $source_module:literal, $url:expr, $summary:expr) => {
        impl<T> $crate::RustStdType for $ty<T> {
            fn rust_language_provenance() -> $crate::RustLanguageProvenance {
                $crate::RustLanguageProvenance::for_source($source_crate, $source_module)
            }

            fn rust_doc_url() -> &'static str {
                $url
            }

            fn rust_semantics_summary() -> &'static str {
                $summary
            }
        }
    };
}

/// As [`impl_rust_std_type_generic1`], for a type generic over two
/// unconstrained type parameters (e.g. `ControlFlow<B, C>`).
macro_rules! impl_rust_std_type_generic2 {
    ($ty:ident, $source_crate:literal, $source_module:literal, $url:expr, $summary:expr) => {
        impl<A, B> $crate::RustStdType for $ty<A, B> {
            fn rust_language_provenance() -> $crate::RustLanguageProvenance {
                $crate::RustLanguageProvenance::for_source($source_crate, $source_module)
            }

            fn rust_doc_url() -> &'static str {
                $url
            }

            fn rust_semantics_summary() -> &'static str {
                $summary
            }
        }
    };
}

/// As [`impl_rust_std_type_generic1`], for a type generic over a lifetime
/// and one type parameter (e.g. `slice::Iter<'a, T>`). No bound on `'a` or
/// `T` — `RustStdType` carries no `'static` requirement, so this covers
/// the borrowing type in general, not just a `'static` instantiation of it.
macro_rules! impl_rust_std_type_lifetime1 {
    ($ty:ident, $source_crate:literal, $source_module:literal, $url:expr, $summary:expr) => {
        impl<'a, T> $crate::RustStdType for $ty<'a, T> {
            fn rust_language_provenance() -> $crate::RustLanguageProvenance {
                $crate::RustLanguageProvenance::for_source($source_crate, $source_module)
            }

            fn rust_doc_url() -> &'static str {
                $url
            }

            fn rust_semantics_summary() -> &'static str {
                $summary
            }
        }
    };
}

/// As [`impl_rust_std_type_lifetime1`], for a type whose type parameter is
/// *not* unconstrained on the struct itself (e.g. `str::Split<'a, P: Pattern>`,
/// where `Pattern` is a sealed, unstable trait). An unconstrained `impl<'a,
/// T> RustStdType for $ty<'a, T>` fails to compile for these — the struct
/// definition's own bound is checked at every usage site of the type,
/// including this impl, so `T` must be resolved to a concrete type that
/// already satisfies it (confirmed empirically: `Pattern` itself can't be
/// named in a bound from stable, but a concrete instantiation like
/// `Split<'a, char>` compiles fine, since `char: Pattern` is already
/// established inside std). `$inner` picks that concrete instantiation.
macro_rules! impl_rust_std_type_lifetime1_concrete {
    ($ty:ident, $inner:ty, $source_crate:literal, $source_module:literal, $url:expr, $summary:expr) => {
        impl<'a> $crate::RustStdType for $ty<'a, $inner> {
            fn rust_language_provenance() -> $crate::RustLanguageProvenance {
                $crate::RustLanguageProvenance::for_source($source_crate, $source_module)
            }

            fn rust_doc_url() -> &'static str {
                $url
            }

            fn rust_semantics_summary() -> &'static str {
                $summary
            }
        }
    };
}

/// As [`impl_rust_std_type_generic1`], for a type generic over three
/// unconstrained type parameters (e.g. `iter::FlatMap<I, U, F>`).
macro_rules! impl_rust_std_type_generic3 {
    ($ty:ident, $source_crate:literal, $source_module:literal, $url:expr, $summary:expr) => {
        impl<A, B, C> $crate::RustStdType for $ty<A, B, C> {
            fn rust_language_provenance() -> $crate::RustLanguageProvenance {
                $crate::RustLanguageProvenance::for_source($source_crate, $source_module)
            }

            fn rust_doc_url() -> &'static str {
                $url
            }

            fn rust_semantics_summary() -> &'static str {
                $summary
            }
        }
    };
}

/// Implement `RustStdType` for a type generic over a single lifetime and no
/// type parameter (e.g. `panic::Location<'a>`).
macro_rules! impl_rust_std_type_lifetime0 {
    ($ty:ident, $source_crate:literal, $source_module:literal, $url:expr, $summary:expr) => {
        impl<'a> $crate::RustStdType for $ty<'a> {
            fn rust_language_provenance() -> $crate::RustLanguageProvenance {
                $crate::RustLanguageProvenance::for_source($source_crate, $source_module)
            }

            fn rust_doc_url() -> &'static str {
                $url
            }

            fn rust_semantics_summary() -> &'static str {
                $summary
            }
        }
    };
}

/// As [`impl_rust_std_type_lifetime1`], for a type whose predicate
/// parameter is bounded `P: FnMut(&T) -> bool` on the struct itself (e.g.
/// `slice::Split<'a, T, P>`). No plain unbounded lifetime+2-type-param
/// variant exists — every real std type with that shape turned out to need
/// a bound on the second parameter, so there was nothing left to genuinely
/// call unbounded.
macro_rules! impl_rust_std_type_lifetime2_pred1 {
    ($ty:ident, $source_crate:literal, $source_module:literal, $url:expr, $summary:expr) => {
        impl<'a, T, P: FnMut(&T) -> bool> $crate::RustStdType for $ty<'a, T, P> {
            fn rust_language_provenance() -> $crate::RustLanguageProvenance {
                $crate::RustLanguageProvenance::for_source($source_crate, $source_module)
            }

            fn rust_doc_url() -> &'static str {
                $url
            }

            fn rust_semantics_summary() -> &'static str {
                $summary
            }
        }
    };
}

/// As [`impl_rust_std_type_lifetime2_pred1`], for a binary predicate `P:
/// FnMut(&T, &T) -> bool` (e.g. `slice::ChunkBy<'a, T, P>`).
macro_rules! impl_rust_std_type_lifetime2_pred2 {
    ($ty:ident, $source_crate:literal, $source_module:literal, $url:expr, $summary:expr) => {
        impl<'a, T, P: FnMut(&T, &T) -> bool> $crate::RustStdType for $ty<'a, T, P> {
            fn rust_language_provenance() -> $crate::RustLanguageProvenance {
                $crate::RustLanguageProvenance::for_source($source_crate, $source_module)
            }

            fn rust_doc_url() -> &'static str {
                $url
            }

            fn rust_semantics_summary() -> &'static str {
                $summary
            }
        }
    };
}

pub(crate) use {
    impl_rust_std_type, impl_rust_std_type_generic1, impl_rust_std_type_generic2,
    impl_rust_std_type_generic3, impl_rust_std_type_lifetime0, impl_rust_std_type_lifetime1,
    impl_rust_std_type_lifetime1_concrete, impl_rust_std_type_lifetime2_pred1,
    impl_rust_std_type_lifetime2_pred2,
};

/// Register `RustStdStandard<T>`'s evidence link once per concrete `T`.
///
/// `RustStdStandard<T>` is generic, so `#[derive(Standard)]`'s own
/// auto-registration can't cover it — `inventory::submit!` needs one
/// concrete registration per call site, not one for the whole generic
/// family (see `amenable_core::link`). The naming convention here
/// (`concat!("amenable_std::rust_std::RustStdStandard<", stringify!($ty), ">")`)
/// is deliberately hardcoded rather than built from `module_path!()`, so
/// that downstream verifier crates registering `ProofRecord`s against the
/// same evidence can reproduce an identical string from their own module
/// without needing to agree on anything beyond this literal convention.
macro_rules! register_rust_std_standard_evidence {
    ($($ty:ty),* $(,)?) => {
        $(
            inventory::submit! {
                amenable_core::EvidenceLink {
                    name: concat!("amenable_std::rust_std::RustStdStandard<", stringify!($ty), ">"),
                    basis: concat!("amenable_std::rust_std::RustStdStandard<", stringify!($ty), ">"),
                    index: 0,
                }
            }
        )*
    };
}

pub(crate) use register_rust_std_standard_evidence;
