#![cfg(feature = "verus")]

use amenable_core::{
    ContractRecord, Ensures, Evidence, MetadataEntry, Provenance, Requires, Verifier, Witness,
    WitnessSupportSummary,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct VerusVerifier;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct VerusVerifierMetadata;

impl Provenance for VerusVerifierMetadata {
    type MetadataIter = std::vec::IntoIter<MetadataEntry>;

    fn metadata(&self) -> Self::MetadataIter {
        Vec::new().into_iter()
    }
}

impl Verifier for VerusVerifier {
    type Metadata = VerusVerifierMetadata;

    fn name() -> &'static str {
        "verus"
    }
}

struct CharRoundtrip;

impl Evidence for CharRoundtrip {
    type Basis = Self;
    type Audit = ();

    fn basis() -> Self::Basis {
        Self
    }

    fn audit(&self) -> Self::Audit {}

    fn is_root() -> bool {
        true
    }
}

impl Witness<VerusVerifier> for CharRoundtrip {
    type SupportingEvidence = Self;
    type ProofArtifact = ();

    fn proof() -> Self::ProofArtifact {}

    fn support() -> WitnessSupportSummary {
        WitnessSupportSummary::checked_leaf()
    }
}

amenable_derive::verus_ensures_witness!(
    CharRoundtrip,
    "verus_contract_test::CharRoundtrip",
    "verify_char_roundtrip"
);

struct ValidUnicodeScalarLike;

impl Evidence for ValidUnicodeScalarLike {
    type Basis = Self;
    type Audit = ();

    fn basis() -> Self::Basis {
        Self
    }

    fn audit(&self) -> Self::Audit {}

    fn is_root() -> bool {
        true
    }
}

impl Witness<VerusVerifier> for ValidUnicodeScalarLike {
    type SupportingEvidence = Self;
    type ProofArtifact = ();

    fn proof() -> Self::ProofArtifact {}

    fn support() -> WitnessSupportSummary {
        WitnessSupportSummary::checked_leaf()
    }
}

// Reuses verify_char_roundtrip's harness like CharRoundtrip above, but
// claims only its second real clause (the unicode-scalar one, not the
// roundtrip one) -- the subset-index feature real ValidUnicodeScalar
// needs.
amenable_derive::verus_ensures_witness!(
    ValidUnicodeScalarLike,
    "verus_contract_test::ValidUnicodeScalarLike",
    "verify_char_roundtrip",
    [1]
);

struct EscapeAsciiModel;

impl Evidence for EscapeAsciiModel {
    type Basis = Self;
    type Audit = ();

    fn basis() -> Self::Basis {
        Self
    }

    fn audit(&self) -> Self::Audit {}

    fn is_root() -> bool {
        true
    }
}

impl Witness<VerusVerifier> for EscapeAsciiModel {
    type SupportingEvidence = Self;
    type ProofArtifact = ();

    fn proof() -> Self::ProofArtifact {}

    fn support() -> WitnessSupportSummary {
        WitnessSupportSummary::checked_leaf()
    }
}

amenable_derive::verus_requires_witness!(
    EscapeAsciiModel,
    "verus_contract_test::EscapeAsciiModel",
    "verify_escape_ascii_model_leaves_printable_bytes_unescaped"
);

#[test]
fn ensures_bound_is_the_real_multi_clause_slice() {
    let clauses: &[&str] = CharRoundtrip::ensures(());

    assert_eq!(
        clauses,
        [
            "char_roundtrip_preserves_value(result, c)",
            "char_is_valid_unicode_scalar(c)",
        ]
    );
}

#[test]
fn requires_bound_is_the_real_clause_slice() {
    let clauses: &[&str] = EscapeAsciiModel::requires(());

    assert_eq!(
        clauses,
        ["escape_ascii_input_is_printable_ascii(printable)"]
    );
}

#[test]
fn indexed_subset_claims_only_the_selected_real_clause() {
    let clauses: &[&str] = ValidUnicodeScalarLike::ensures(());

    assert_eq!(clauses, ["char_is_valid_unicode_scalar(c)"]);
}

#[test]
fn one_contract_record_is_registered_per_real_clause() {
    // `inventory::iter` doesn't promise any particular order across
    // separate `submit!` blocks, so compare as sets, not sequences.
    let mut ensures_records = inventory::iter::<ContractRecord>()
        .filter(|record| {
            record.evidence == "verus_contract_test::CharRoundtrip" && record.kind == "ensures"
        })
        .map(|record| (record.fragment)())
        .collect::<Vec<_>>();
    ensures_records.sort_unstable();

    let mut expected_ensures = vec![
        "char_roundtrip_preserves_value(result, c)",
        "char_is_valid_unicode_scalar(c)",
    ];
    expected_ensures.sort_unstable();
    assert_eq!(ensures_records, expected_ensures);

    let requires_records = inventory::iter::<ContractRecord>()
        .filter(|record| {
            record.evidence == "verus_contract_test::EscapeAsciiModel" && record.kind == "requires"
        })
        .map(|record| (record.fragment)())
        .collect::<Vec<_>>();

    assert_eq!(
        requires_records,
        vec!["escape_ascii_input_is_printable_ascii(printable)"]
    );
}
