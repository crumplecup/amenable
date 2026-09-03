//! The two `Ensures<KaniVerifier>` claims every `NonZero<T>` harness reuses:
//! `impl_nonzero_ensures_kani!` for `NonZero::new`'s construction
//! precondition (`value != 0`), and the generic [`NonZeroGetRoundTrips`]
//! for the `.get()` accessor postcondition.

use std::num::NonZero;

use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::signed::VERIFY_NONZERO_I8_SRC;
use crate::KaniWitness;
use crate::rust_std::CheckedProof;

/// The [`RustStdStandard<NonZero<T>>`] witnesses in the `signed` and
/// `unsigned` submodules all reuse their own harness for
/// [`amenable_core::Ensures<crate::KaniVerifier>`] — each harness calls
/// `RustStdStandard::<NonZero<T>>::ensures(value)` directly rather than
/// restating `value != 0`, identical across every width for the same
/// reason the witnesses themselves are literal per-width blocks (see
/// `nonzero`'s module doc): a generator macro's captured span would
/// resolve back to unsubstituted placeholders, not the real per-width
/// text. Nothing here needs `harness!`'s span capture, though, so this
/// half is a plain macro.
macro_rules! impl_nonzero_ensures_kani {
    ($($ty:ty),* $(,)?) => {
        $(
            impl amenable_core::Ensures<crate::KaniVerifier> for RustStdStandard<NonZero<$ty>> {
                type Input = $ty;
                type Bound = bool;

                fn ensures(value: $ty) -> bool {
                    value != 0
                }
            }

            ::inventory::submit! {
                ::amenable_core::ContractRecord::new(
                    concat!(
                        "amenable_std::rust_std::RustStdStandard<NonZero<",
                        stringify!($ty),
                        ">>"
                    ),
                    "kani",
                    "ensures",
                    || "value != 0",
                )
            }
        )*
    };
}

impl_nonzero_ensures_kani!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);

/// `NonZero<T>::get()` round-trips its wrapped value — a distinct claim
/// from `impl_nonzero_ensures_kani!`'s `RustStdStandard<NonZero<T>>`
/// impls above (those check `NonZero::new`'s *construction*
/// precondition, `value != 0`; this checks the *accessor*
/// postcondition), so it can't reuse that carrier's slot:
/// `RustStdStandard<NonZero<T>>` already has its one
/// `Ensures<KaniVerifier>` impl claimed by the precondition.
///
/// Generic over the wrapped width rather than twelve separate concrete
/// types (`NonZeroI8GetRoundTrips`, `NonZeroI16GetRoundTrips`, ...): every
/// one of those independently registered the identical fragment
/// `actual == expected`, the same trivial-equality claim this session's
/// other access-pattern types (`DerefReflectsTheStoredValue`,
/// `IndexRecoversTheStoredElement`, ...) already generalize over `T`.
/// Lives in `amenable_kani` rather than `amenable_std` — no Creusot/Verus
/// coverage of `NonZero::get()` exists yet, and every other Kani-only
/// generic contract type this session landed in `amenable_kani` for the
/// same reason.
pub struct NonZeroGetRoundTrips<T>(std::marker::PhantomData<T>);

impl<T> amenable_core::Standard for NonZeroGetRoundTrips<T> {
    type Provenance = amenable_std::RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn provenance(&self) -> Self::Provenance {
        <i32 as amenable_std::RustStdType>::provenance()
    }
}

impl<T> Evidence for NonZeroGetRoundTrips<T> {
    type Basis = RustStdStandard<i32>;
    type Audit = amenable_std::RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn basis() -> Self::Basis {
        RustStdStandard::<i32>::new()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn audit(&self) -> Self::Audit {
        <i32 as amenable_std::RustStdType>::provenance()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", ret))]
    fn is_root() -> bool {
        false
    }
}

impl<T> KaniWitness for NonZeroGetRoundTrips<T> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_nonzero_i8".to_owned(),
            VERIFY_NONZERO_I8_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

impl<T> amenable_core::Witness<crate::KaniVerifier> for NonZeroGetRoundTrips<T> {
    type SupportingEvidence = <Self as KaniWitness>::SupportingEvidence;
    type ProofArtifact = <Self as KaniWitness>::ProofArtifact;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        <Self as KaniWitness>::proof()
    }
}

impl<T: PartialEq> amenable_core::Ensures<crate::KaniVerifier> for NonZeroGetRoundTrips<T> {
    type Input = (T, T);
    type Bound = bool;

    fn ensures((actual, expected): (T, T)) -> bool {
        actual == expected
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_kani::NonZeroGetRoundTrips",
        "kani",
        "ensures",
        || stringify!(actual == expected),
    )
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::NonZeroGetRoundTrips",
        "kani",
        || <NonZeroGetRoundTrips<i8> as KaniWitness>::proof().to_string(),
    )
}
