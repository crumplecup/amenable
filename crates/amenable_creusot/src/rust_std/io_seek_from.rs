#[cfg(creusot)]
use creusot_std::macros::{check, ensures, extern_spec, logic, requires, trusted};
#[cfg(creusot)]
use std::io::SeekFrom;
amenable_derive::harness! {
    creusot, SEEK_FROM_ROUND_TRIPS_EACH_VARIANTS_OFFSET_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<SeekFrom>`
        /// postcondition -- real, callable Pearlite content, not just
        /// descriptive text alongside it.
        #[logic(open)]
        fn seek_from_round_trips_each_variants_offset(
            start_offset: u64,
            end_offset: i64,
            current_offset: i64,
            seek_result: (u64, i64, i64),
        ) -> bool {
            pearlite! {
                seek_result.0 == start_offset
                    && seek_result.1 == end_offset
                    && seek_result.2 == current_offset
            }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_SEEK_FROM_ROUND_TRIPS_EACH_VARIANTS_OFFSET_SRC, {
        /// Each `SeekFrom` variant preserves the offset it was constructed
        /// with and remains its own variant.
        #[requires(true)]
        #[ensures(seek_from_round_trips_each_variants_offset(start_offset, end_offset, current_offset, result))]
        fn verify_seek_from_round_trips_each_variants_offset(
            start_offset: u64,
            end_offset: i64,
            current_offset: i64,
        ) -> (u64, i64, i64) {
            let start_value = match SeekFrom::Start(start_offset) {
                SeekFrom::Start(value) => value,
                SeekFrom::End(_) | SeekFrom::Current(_) => start_offset,
            };
            let end_value = match SeekFrom::End(end_offset) {
                SeekFrom::End(value) => value,
                SeekFrom::Start(_) | SeekFrom::Current(_) => end_offset,
            };
            let current_value = match SeekFrom::Current(current_offset) {
                SeekFrom::Current(value) => value,
                SeekFrom::Start(_) | SeekFrom::End(_) => current_offset,
            };

            (start_value, end_value, current_value)
        }
    }
}
