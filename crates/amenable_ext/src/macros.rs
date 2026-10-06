//! Bulk-registration macros for `ExtType`, mirroring
//! `amenable_std::rust_std::macros`'s `impl_rust_std_type!` family.
//!
//! Only the plain (no generics) case exists so far — per
//! `docs/AMENABLE_EXT_PLAN.md` Phase 0, the arity/lifetime variants
//! (`impl_ext_type_generic1!`, …) get added when a real jiff type
//! actually needs one, not speculatively ahead of that need.
//!
//! These only implement `ExtType` itself (a static-metadata trait with no
//! bound on any type parameter) — they intentionally do not touch
//! `ExtStandard<T>` evidence registration or verifier proofs, which are a
//! separate, later pass (see `register_ext_standard_evidence!`).

/// Implement `ExtType` for a concrete (non-generic) type.
///
/// Takes an explicit `$authority` (unlike `impl_rust_std_type!`, which
/// hardcodes "Rust Project Developers" once for the whole crate via
/// `RustLanguageProvenance::for_source`) — see [`crate::ExtType`]'s own
/// doc comment for why: `amenable_ext` covers a different maintaining
/// project per target crate, with no single default to fall back to.
macro_rules! impl_ext_type {
    ($ty:ty, $authority:literal, $source_crate:literal, $source_module:literal, $url:expr, $summary:expr) => {
        impl $crate::ExtType for $ty {
            fn ext_language_provenance() -> $crate::ExtLanguageProvenance {
                $crate::ExtLanguageProvenance::for_source($authority, $source_crate, $source_module)
            }

            fn ext_doc_url() -> &'static str {
                $url
            }

            fn ext_semantics_summary() -> &'static str {
                $summary
            }
        }
    };
}

pub(crate) use impl_ext_type;

/// Register `ExtStandard<T>`'s evidence link once per concrete `T`.
///
/// `ExtStandard<T>` is generic, so `#[derive(Standard)]`'s own
/// auto-registration can't cover it — `inventory::submit!` needs one
/// concrete registration per call site, not one for the whole generic
/// family (see `amenable_core::link`). The naming convention here
/// (`concat!("amenable_ext::ExtStandard<", stringify!($ty), ">")`) is
/// deliberately hardcoded rather than built from `module_path!()`, so
/// that downstream verifier crates registering `ProofRecord`s against
/// the same evidence can reproduce an identical string from their own
/// module without needing to agree on anything beyond this literal
/// convention — the same reasoning `register_rust_std_standard_evidence!`
/// documents for its own naming.
macro_rules! register_ext_standard_evidence {
    ($($ty:ty),* $(,)?) => {
        $(
            inventory::submit! {
                amenable_core::EvidenceLink::new(
                    concat!("amenable_ext::ExtStandard<", stringify!($ty), ">"),
                    concat!("amenable_ext::ExtStandard<", stringify!($ty), ">"),
                    0,
                )
            }
        )*
    };
}

pub(crate) use register_ext_standard_evidence;

/// Register a generic evidence claim: `$ty` is generic over `bounds`, and
/// relies on `premises` beyond what those bounds enforce by signature.
///
/// Emits a link named `amenable_ext::ExtGeneric<$ty>`. The wrapper differs
/// from `ExtStandard<T>`, so a generic name can never equal a concrete one.
/// Kani cannot express generics, so each concrete instantiation still gets its
/// own witness type; this macro registers only the generic claim itself.
///
/// ```ignore
/// register_ext_generic_evidence!(
///     chrono::DateTime<Tz>,
///     bounds = ["chrono::offset::TimeZone"],
///     premises = [("offset-round-trip", "a local offset applied back gives the UTC instant")],
/// );
/// ```
macro_rules! register_ext_generic_evidence {
    (
        $ty:ty,
        bounds = [$($bound:expr),* $(,)?],
        premises = [$(($pid:expr, $pstmt:expr)),* $(,)?] $(,)?
    ) => {
        inventory::submit! {
            amenable_core::EvidenceLink::generic(
                concat!("amenable_ext::ExtGeneric<", stringify!($ty), ">"),
                concat!("amenable_ext::ExtGeneric<", stringify!($ty), ">"),
                0,
                &[$($bound),*],
                &[$(amenable_core::Premise::new($pid, $pstmt)),*],
            )
        }
    };
}

pub(crate) use register_ext_generic_evidence;
