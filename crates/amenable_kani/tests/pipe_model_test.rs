use amenable_kani::{KaniCompose, KaniPipe};

#[test]
fn fresh_pipe_shares_one_resource_across_distinct_endpoints() {
    let pipe = KaniPipe::kani_depth0();
    let reader = pipe.reader();
    let writer = pipe.writer();

    assert_eq!(reader.resource_id(), writer.resource_id());
    assert_ne!(reader.raw_fd(), writer.raw_fd());
    assert!(reader.is_live());
    assert!(writer.is_live());
    assert!(pipe.reader_is_open());
    assert!(pipe.writer_is_open());
}

#[test]
fn write_close_read_round_trips_buffered_bytes() {
    let mut pipe = KaniPipe::kani_depth0();
    let reader = pipe.reader();
    let writer = pipe.writer();

    pipe.write_all(writer.clone(), b"piped".to_vec());
    pipe.close_writer(writer);

    assert_eq!(pipe.read_to_end(reader), b"piped");
    assert_eq!(pipe.buffered_len(), 0);
    assert!(!pipe.writer_is_open());
}
