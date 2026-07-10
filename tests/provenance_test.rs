use amenable::{KaniVerifier, KaniVerifierMetadata, Provenance, Verifier};

#[test]
fn verifier_metadata_marker_is_zero_sized() {
    assert_eq!(std::mem::size_of::<KaniVerifierMetadata>(), 0);
}

#[test]
fn verifier_metadata_iterates_lazily_generated_entries() {
    let metadata = KaniVerifier::metadata();

    assert!(!metadata.is_empty());
    assert_eq!(metadata.len(), metadata.iter().count());

    let entry = metadata
        .get("verifier_family")
        .expect("verifier_family fact present");
    assert_eq!(entry.value(), "kani");

    assert!(metadata.contains_key("authority"));
    assert!(!metadata.contains_key("nonexistent_key"));
    assert!(metadata.get("nonexistent_key").is_none());
}
