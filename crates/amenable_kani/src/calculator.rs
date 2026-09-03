//! A small worked calculation: two evidence types (`Debit`, `Credit`),
//! each a thin domain wrapper around `i64` rather than a root in its own
//! right, compose into one calculation (`add`), backed by a real Kani
//! harness.
//!
//! This exists to give the evidence chain something with more than one
//! hop to walk — `AddEvidence -> [Debit, Credit] -> [RustStdStandard<i64>,
//! RustStdStandard<i64>]` — unlike the bare root types in `rust_std`.
//! `Debit`/`Credit` don't invent their own authority: their provenance
//! *is* `i64`'s, the same way a newtype wrapper's validity ultimately
//! rests on whatever it wraps. It's a worked example of the trait family,
//! not product surface: real users compose their own calculations the
//! same way, on their own domain types.

#[cfg(kani)]
use amenable_core::Ensures;
#[cfg(kani)]
use amenable_core::Requires;
use amenable_core::{Establish, MetadataEntry, ProofToken, Provenance, Witness};
use amenable_derive::{Standard, calculation};
use amenable_std::RustStdStandard;

#[cfg(kani)]
use crate::FieldAccessRecoversTheStoredValue;
use crate::KaniVerifier;

/// A debit amount: a domain wrapper around `i64`, resting on `i64`'s own
/// standard rather than standing as a root itself.
#[derive(Debug, Clone, PartialEq, Eq, Default, Standard)]
#[standard(basis = "RustStdStandard<i64>")]
pub struct Debit {
    value: i64,
}

impl Debit {
    /// Create a debit of the given amount.
    pub const fn new(value: i64) -> Self {
        Self { value }
    }
}

impl Provenance for Debit {
    type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn metadata(&self) -> Self::MetadataIter {
        Box::new(vec![MetadataEntry::new("value", self.value.to_string())].into_iter())
    }
}

::inventory::submit! {
    ::amenable_core::EvidenceLink::new(
        concat!(module_path!(), "::", stringify!(Debit)),
        "amenable_std::rust_std::RustStdStandard<i64>",
        0,
    )
}

/// Access proof for [`Debit`]: nothing statically guarantees a private
/// field survives its constructor unchanged the way a `pub` field's
/// identity is guaranteed by the type system, so the access path from a
/// constructed `Debit` back to the `i64` it wraps earns its own proof,
/// separate from anything [`AddEvidence`] proves about the calculation
/// built on top of it.
impl Witness<KaniVerifier> for Debit {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "verify_debit_access_preserves_value".to_owned(),
            VERIFY_DEBIT_ACCESS_PRESERVES_VALUE_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        concat!(module_path!(), "::", stringify!(Debit)),
        "kani",
        || <Debit as Witness<KaniVerifier>>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_DEBIT_ACCESS_PRESERVES_VALUE_SRC, {
        /// `Debit::new` preserves the underlying `i64` unchanged — the
        /// access path used wherever a `Debit`'s `.value` is read as
        /// evidence (e.g. inside `add_impl`) is faithful to what was
        /// constructed.
        #[kani::proof]
        fn verify_debit_access_preserves_value() {
            let x: i64 = kani::any();
            let debit = Debit::new(x);

            assert!(FieldAccessRecoversTheStoredValue::ensures((debit.value, x)));
        }
    }
}

/// A credit amount: a domain wrapper around `i64`, resting on `i64`'s own
/// standard rather than standing as a root itself.
#[derive(Debug, Clone, PartialEq, Eq, Default, Standard)]
#[standard(basis = "RustStdStandard<i64>")]
pub struct Credit {
    value: i64,
}

impl Credit {
    /// Create a credit of the given amount.
    pub const fn new(value: i64) -> Self {
        Self { value }
    }
}

impl Provenance for Credit {
    type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn metadata(&self) -> Self::MetadataIter {
        Box::new(vec![MetadataEntry::new("value", self.value.to_string())].into_iter())
    }
}

::inventory::submit! {
    ::amenable_core::EvidenceLink::new(
        concat!(module_path!(), "::", stringify!(Credit)),
        "amenable_std::rust_std::RustStdStandard<i64>",
        0,
    )
}

/// Access proof for [`Credit`] — see [`Debit`]'s impl for the rationale.
impl Witness<KaniVerifier> for Credit {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "verify_credit_access_preserves_value".to_owned(),
            VERIFY_CREDIT_ACCESS_PRESERVES_VALUE_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        concat!(module_path!(), "::", stringify!(Credit)),
        "kani",
        || <Credit as Witness<KaniVerifier>>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_CREDIT_ACCESS_PRESERVES_VALUE_SRC, {
        /// `Credit::new` preserves the underlying `i64` unchanged — the
        /// access path used wherever a `Credit`'s `.value` is read as
        /// evidence (e.g. inside `add_impl`) is faithful to what was
        /// constructed.
        #[kani::proof]
        fn verify_credit_access_preserves_value() {
            let x: i64 = kani::any();
            let credit = Credit::new(x);

            assert!(FieldAccessRecoversTheStoredValue::ensures((credit.value, x)));
        }
    }
}

/// The result of adding a [`Debit`] to a [`Credit`].
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Sum {
    value: i64,
}

impl Sum {
    /// Create a sum of the given amount.
    pub const fn new(value: i64) -> Self {
        Self { value }
    }
}

/// Add a debit and a credit, forming evidence rooted in both.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(a, b)))]
#[calculation(token = AddToken)]
pub fn add(a: Debit, b: Credit) -> Sum {
    Sum {
        value: a.value + b.value,
    }
}

/// Lawful token minted once `add`'s evidence has been established.
pub struct AddToken(());

impl ProofToken for AddToken {
    type Proposition = AddEvidence;
}

/// Proof artifact for [`AddEvidence`]'s Kani witness: names the harness
/// and carries its verbatim source, captured via
/// [`amenable_derive::harness!`] so the two can never drift apart.
#[derive(Debug, Clone, PartialEq, Eq, derive_getters::Getters, derive_new::new)]
pub struct CalculationProof {
    /// The Kani harness that checks this calculation's invariant.
    harness: String,
    /// The harness's own source — what it actually asserts, verbatim.
    claim: String,
}

impl std::fmt::Display for CalculationProof {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, f)))]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "harness: {}", self.harness)?;
        write!(f, "claim: {}", self.claim)
    }
}

// Bounding this impl on `Debit`/`Credit` each already having a Kani
// witness makes the access-proof obligation load-bearing rather than a
// convention someone has to remember: this impl simply doesn't exist —
// `AddEvidence` has no Kani proof at all — until Debit and Credit both
// have their own. Proving the calculation without also proving access to
// its basis is not an option the type system offers.
impl Witness<KaniVerifier> for AddEvidence
where
    Debit: Witness<KaniVerifier>,
    Credit: Witness<KaniVerifier>,
{
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "add_impl_computes_exact_sum".to_owned(),
            ADD_IMPL_COMPUTES_EXACT_SUM_SRC.to_owned(),
        )
    }
}

// `AddEvidence` itself, not the bare `Sum` it wraps, is the lawful
// credential: `Sum::new` is a public constructor (any `i64` value passes),
// so a `Sum` alone never demonstrated that `add_impl` actually ran. An
// `AddEvidence` value, by contrast, is only reachable from outside this
// module through `add::<V>()`, which already requires `Debit: Witness<V>`
// and `Credit: Witness<V>` — holding one *is* the proof that the
// calculation was legitimately performed.
impl ProofToken for AddEvidence {
    type Proposition = AddEvidence;
}

impl Establish<AddEvidence, KaniVerifier> for AddEvidence {
    type Token = AddToken;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(_credential)))]
    fn establish(_credential: AddEvidence) -> Self::Token {
        AddToken(())
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        concat!(module_path!(), "::", stringify!(AddEvidence)),
        "kani",
        || <AddEvidence as Witness<KaniVerifier>>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, ADD_IMPL_COMPUTES_EXACT_SUM_SRC, {
        /// For any `Debit`/`Credit` whose values don't overflow when added,
        /// `add_impl` computes the exact sum. This never runs under normal
        /// `cargo test` — only under `cargo kani`, which provides
        /// `#[cfg(kani)]` and the `kani` crate itself.
        #[kani::proof]
        fn add_impl_computes_exact_sum() {
            let a = Debit { value: kani::any() };
            let b = Credit { value: kani::any() };

            kani::assume(RustStdStandard::<i64>::requires((a.value, b.value)));

            let sum = add_impl(a.clone(), b.clone());

            assert!(FieldAccessRecoversTheStoredValue::ensures((
                sum.value,
                a.value + b.value
            )));
        }
    }
}
