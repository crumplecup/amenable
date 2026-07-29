use amenable_kani::KaniEscapeAsciiObservation;

#[test]
fn escape_ascii_observation_keeps_the_source_and_escaped_bytes() {
    let observation = KaniEscapeAsciiObservation::new(b'A');

    assert_eq!(observation.source(), [b'A', b'\n']);
    assert_eq!(observation.escaped(), [b'A', b'\\', b'n']);
}
