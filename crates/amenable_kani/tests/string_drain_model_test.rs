use amenable_kani::{KaniStringDrainObservation, KaniUtf8Buffer};

#[test]
fn drain_model_preserves_empty_utf8_content() -> miette::Result<()> {
    let source =
        KaniUtf8Buffer::<2>::new([0, 0], 0).map_err(|error| miette::miette!("{error:?}"))?;
    let observation = KaniStringDrainObservation::new(source);

    assert_eq!(observation.yielded_bytes(), b"");
    assert_eq!(observation.yielded_len(), 0);
    assert_eq!(observation.source_len_after_drain(), 0);
    assert!(observation.source_is_empty());
    Ok(())
}

#[test]
fn drain_model_preserves_bounded_utf8_content_in_order() -> miette::Result<()> {
    let source =
        KaniUtf8Buffer::<2>::new(*b"ab", 2).map_err(|error| miette::miette!("{error:?}"))?;
    let observation = KaniStringDrainObservation::new(source);

    assert_eq!(observation.yielded_bytes(), b"ab");
    assert_eq!(observation.yielded_len(), 2);
    assert_eq!(observation.source_len_after_drain(), 0);
    assert!(observation.source_is_empty());
    Ok(())
}
