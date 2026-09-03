use std::io::SeekFrom;

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use crate::KaniWitness;
use crate::rust_std::CheckedProof;
use crate::rust_std::macros::{bridge_kani_witness, kani_ensures};

impl KaniWitness for RustStdStandard<SeekFrom> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_seek_from_round_trips_each_variants_offset".to_owned(),
            VERIFY_SEEK_FROM_ROUND_TRIPS_EACH_VARIANTS_OFFSET_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<SeekFrom>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<SeekFrom>",
        "kani",
        || <RustStdStandard<SeekFrom> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<SeekFrom>,
    "amenable_std::rust_std::RustStdStandard<SeekFrom>",
    (i64, i64),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_SEEK_FROM_ROUND_TRIPS_EACH_VARIANTS_OFFSET_SRC, {
        /// Each `SeekFrom` variant preserves the offset it was
        /// constructed with, and stays its own variant. The assertions
        /// call `RustStdStandard::<SeekFrom>::ensures` directly rather
        /// than restating the comparison.
        #[kani::proof]
        fn verify_seek_from_round_trips_each_variants_offset() {
            let start_offset: u64 = kani::any();
            let end_offset: i64 = kani::any();
            let current_offset: i64 = kani::any();

            match SeekFrom::Start(start_offset) {
                SeekFrom::Start(value) => {
                    assert!(
                        RustStdStandard::<SeekFrom>::ensures((value as i64, start_offset as i64)),
                        "SeekFrom::Start preserves its offset"
                    );
                }
                _ => panic!("SeekFrom::Start must construct the Start variant"),
            }
            match SeekFrom::End(end_offset) {
                SeekFrom::End(value) => {
                    assert!(
                        RustStdStandard::<SeekFrom>::ensures((value, end_offset)),
                        "SeekFrom::End preserves its offset"
                    );
                }
                _ => panic!("SeekFrom::End must construct the End variant"),
            }
            match SeekFrom::Current(current_offset) {
                SeekFrom::Current(value) => {
                    assert!(
                        RustStdStandard::<SeekFrom>::ensures((value, current_offset)),
                        "SeekFrom::Current preserves its offset"
                    );
                }
                _ => panic!("SeekFrom::Current must construct the Current variant"),
            }
        }
    }
}

impl KaniWitness for RustStdStandard<std::io::Sink> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_sink_write_reports_full_length_and_discards_content".to_owned(),
            VERIFY_SINK_WRITE_REPORTS_FULL_LENGTH_AND_DISCARDS_CONTENT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::io::Sink>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::io::Sink>",
        "kani",
        || <RustStdStandard<std::io::Sink> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_SINK_WRITE_REPORTS_FULL_LENGTH_AND_DISCARDS_CONTENT_SRC, {
        /// `std::io::sink()`'s writer always reports the full length
        /// written and never errors, regardless of the content offered.
        #[kani::proof]
        fn verify_sink_write_reports_full_length_and_discards_content() {
            use std::io::Write;

            let data: [u8; 4] = kani::any();
            let mut writer = std::io::sink();
            let written = writer.write(&data).expect("Sink::write never errors");
            assert!(
                RustStdStandard::<usize>::ensures((written, data.len())),
                "Sink::write reports the full length written"
            );
            writer.flush().expect("Sink::flush never errors");
        }
    }
}

impl KaniWitness for RustStdStandard<std::io::Chain<&'static [u8], &'static [u8]>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_chain_reads_the_first_source_then_the_second".to_owned(),
            VERIFY_CHAIN_READS_THE_FIRST_SOURCE_THEN_THE_SECOND_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::io::Chain<&'static [u8], &'static [u8]>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::io::Chain<&'static [u8], &'static [u8]>>",
        "kani",
        || <RustStdStandard<std::io::Chain<&'static [u8], &'static [u8]>> as KaniWitness>::proof()
            .to_string(),
    )
}

kani_ensures!(
    RustStdStandard<std::io::Chain<&'static [u8], &'static [u8]>>,
    "amenable_std::rust_std::RustStdStandard<std::io::Chain<&'static [u8], &'static [u8]>>",
    ([u8; 2], [u8; 2]),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_CHAIN_READS_THE_FIRST_SOURCE_THEN_THE_SECOND_SRC, {
        /// `.chain()` reads its first source to exhaustion before it
        /// starts reading the second.
        #[kani::proof]
        fn verify_chain_reads_the_first_source_then_the_second() {
            use std::io::Read;

            let first: [u8; 2] = kani::any();
            let second: [u8; 2] = kani::any();
            let mut chain = (&first[..]).chain(&second[..]);
            let mut buffer = [0u8; 2];

            let read_first = chain
                .read(&mut buffer)
                .expect("Chain::read over in-memory slices never errors");
            assert!(
                RustStdStandard::<usize>::ensures((read_first, buffer.len())),
                "Chain::read drains the first source fully before touching the second"
            );
            assert!(
                RustStdStandard::<std::io::Chain<&'static [u8], &'static [u8]>>::ensures((
                    buffer, first
                )),
                "Chain::read yields the first source's bytes first"
            );

            let read_second = chain
                .read(&mut buffer)
                .expect("Chain::read over in-memory slices never errors");
            assert!(
                RustStdStandard::<usize>::ensures((read_second, buffer.len())),
                "Chain::read continues into the second source once the first is exhausted"
            );
            assert!(
                RustStdStandard::<std::io::Chain<&'static [u8], &'static [u8]>>::ensures((
                    buffer, second
                )),
                "Chain::read yields the second source's bytes once the first is drained"
            );
        }
    }
}
