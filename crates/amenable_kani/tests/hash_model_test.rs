use amenable_kani::KaniRandomStateObservation;

#[test]
fn same_input_hashes_report_matching_digests() {
    let observation = KaniRandomStateObservation::same_input("some value", 7);

    assert_eq!(observation.input(), "some value");
    assert_eq!(observation.first_digest(), 7);
    assert_eq!(observation.second_digest(), 7);
    assert!(observation.same_input_hashes_agree());
}
