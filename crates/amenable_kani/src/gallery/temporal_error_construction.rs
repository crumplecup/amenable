//! Branch-level bisection of the `TemporalErrorKind` tree for the
//! `DateTime<FixedOffset>`/`DateTime<Local>` round-trip timeout
//! investigation. `check_temporal_error_construction` already showed
//! the trunk (`TemporalError`, boxing the full ~43-variant
//! `TemporalErrorKind`) times out even for the cheapest single
//! variant. This file tests each *branch* -- one representative
//! `*Source` type per error-source file, each file already a thematic
//! grouping by shared foreign-type dependency -- bare and unboxed, no
//! enum wrapper at all, to find which branch(es) are the real driver
//! before drilling into individual leaves.

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::temporal_error_construction::check_temporal_error_construction".to_owned(),
            "gallery::temporal_error_construction::check_temporal_error_construction".to_owned(),
            "amenable_kani".to_owned(),
            "trunk probe: bare TemporalError construction (ComponentOutOfRange variant), decoupled from any chrono/bridge logic -- confirmed PASSES now that every jiff::Error-wrapping *Source field is dropped entirely under cfg(kani) (branch bisection found jiff_sources specifically; String removal alone, done first, was necessary but not sufficient)".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_TEMPORAL_ERROR_CONSTRUCTION_SRC, {
        #[kani::proof]
        fn check_temporal_error_construction() {
            let value: u32 = kani::any();
            let _err = amenable_time::TemporalError::new(amenable_time::TemporalErrorKind::ComponentOutOfRange(
                amenable_time::ComponentOutOfRangeSource::new(amenable_time::TemporalComponent::Hour, value),
            ));
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::temporal_error_construction::check_temporal_error_kind_unboxed".to_owned(),
            "gallery::temporal_error_construction::check_temporal_error_kind_unboxed".to_owned(),
            "amenable_kani".to_owned(),
            "incremental step 1/2: the bare TemporalErrorKind enum (sum of all ~43 variants), unboxed, as the Result's error type directly -- isolates whether the sum of branches alone (no Box, no TemporalError wrapper) is still cheap after the jiff::Error fix".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_TEMPORAL_ERROR_KIND_UNBOXED_SRC, {
        #[kani::proof]
        fn check_temporal_error_kind_unboxed() {
            let value: u32 = kani::any();
            let _err: Result<u8, amenable_time::TemporalErrorKind> = Err(
                amenable_time::TemporalErrorKind::ComponentOutOfRange(
                    amenable_time::ComponentOutOfRangeSource::new(
                        amenable_time::TemporalComponent::Hour,
                        value,
                    ),
                ),
            );
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::temporal_error_construction::check_temporal_error_kind_boxed".to_owned(),
            "gallery::temporal_error_construction::check_temporal_error_kind_boxed".to_owned(),
            "amenable_kani".to_owned(),
            "incremental step 2/2: the same TemporalErrorKind, now boxed (Box<TemporalErrorKind>), matching TemporalError's own kind field shape exactly -- isolates whether boxing itself adds cost on top of the unboxed sum-of-variants".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_TEMPORAL_ERROR_KIND_BOXED_SRC, {
        #[kani::proof]
        fn check_temporal_error_kind_boxed() {
            let value: u32 = kani::any();
            let _err: Result<u8, Box<amenable_time::TemporalErrorKind>> = Err(Box::new(
                amenable_time::TemporalErrorKind::ComponentOutOfRange(
                    amenable_time::ComponentOutOfRangeSource::new(
                        amenable_time::TemporalComponent::Hour,
                        value,
                    ),
                ),
            ));
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::temporal_error_construction::check_validation_branch".to_owned(),
            "gallery::temporal_error_construction::check_validation_branch".to_owned(),
            "amenable_kani".to_owned(),
            "branch probe: validation.rs's ComponentOutOfRangeSource, bare, unboxed, no enum wrapper".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_VALIDATION_BRANCH_SRC, {
        #[kani::proof]
        fn check_validation_branch() {
            let value: u32 = kani::any();
            let _err: Result<u8, amenable_time::ComponentOutOfRangeSource> = Err(
                amenable_time::ComponentOutOfRangeSource::new(amenable_time::TemporalComponent::Hour, value),
            );
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::temporal_error_construction::check_endpoint_branch".to_owned(),
            "gallery::temporal_error_construction::check_endpoint_branch".to_owned(),
            "amenable_kani".to_owned(),
            "branch probe: endpoint.rs's LeapSecondNotRepresentableSource, bare, unboxed, no enum wrapper".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_ENDPOINT_BRANCH_SRC, {
        #[kani::proof]
        fn check_endpoint_branch() {
            let nanosecond: u32 = kani::any();
            let _err: Result<u8, amenable_time::LeapSecondNotRepresentableSource> = Err(
                amenable_time::LeapSecondNotRepresentableSource::new(nanosecond),
            );
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::temporal_error_construction::check_capability_branch".to_owned(),
            "gallery::temporal_error_construction::check_capability_branch".to_owned(),
            "amenable_kani".to_owned(),
            "branch probe: capability.rs's UnsupportedSource/UnsupportedDetail (itself a nested enum), bare, unboxed, no outer enum wrapper".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_CAPABILITY_BRANCH_SRC, {
        #[kani::proof]
        fn check_capability_branch() {
            let digits: u8 = kani::any();
            let _err: Result<u8, amenable_time::UnsupportedSource> = Err(
                amenable_time::UnsupportedSource::new(
                    amenable_time::UnsupportedDetail::FractionalDigitsOutOfRange(digits),
                ),
            );
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::temporal_error_construction::check_narrowing_branch".to_owned(),
            "gallery::temporal_error_construction::check_narrowing_branch".to_owned(),
            "amenable_kani".to_owned(),
            "branch probe: narrowing.rs's EpochSecondsEndpointsReversedSource, bare, unboxed, no enum wrapper".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_NARROWING_BRANCH_SRC, {
        #[kani::proof]
        fn check_narrowing_branch() {
            let start: i64 = kani::any();
            let end: i64 = kani::any();
            let _err: Result<u8, amenable_time::EpochSecondsEndpointsReversedSource> = Err(
                amenable_time::EpochSecondsEndpointsReversedSource::new(start, end),
            );
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::temporal_error_construction::check_builders_branch".to_owned(),
            "gallery::temporal_error_construction::check_builders_branch".to_owned(),
            "amenable_kani".to_owned(),
            "branch probe: builders.rs's UtcOffsetBuilderFailedSource, wrapping a real derive_builder-generated error, bare, unboxed, no enum wrapper".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_BUILDERS_BRANCH_SRC, {
        #[kani::proof]
        fn check_builders_branch() {
            let build_err = amenable_time::UtcOffsetDescriptorBuilder::default()
                .build()
                .unwrap_err();
            let _err: Result<u8, amenable_time::UtcOffsetBuilderFailedSource> = Err(
                amenable_time::UtcOffsetBuilderFailedSource::new(build_err),
            );
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::temporal_error_construction::check_chrono_sources_branch".to_owned(),
            "gallery::temporal_error_construction::check_chrono_sources_branch".to_owned(),
            "amenable_kani".to_owned(),
            "branch probe: chrono_sources.rs's NonPositiveDeltaSource, wrapping a real chrono::Duration, bare, unboxed, no enum wrapper".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

#[cfg(feature = "chrono")]
amenable_derive::gallery_harness! {
    kani, CHECK_CHRONO_SOURCES_BRANCH_SRC, {
        #[kani::proof]
        fn check_chrono_sources_branch() {
            let seconds: i64 = kani::any();
            let Some(delta) = chrono::Duration::try_seconds(seconds) else {
                return;
            };
            let _err: Result<u8, amenable_time::NonPositiveDeltaSource> = Err(
                amenable_time::NonPositiveDeltaSource::new(delta),
            );
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::temporal_error_construction::check_chrono_tz_sources_branch".to_owned(),
            "gallery::temporal_error_construction::check_chrono_tz_sources_branch".to_owned(),
            "amenable_kani".to_owned(),
            "branch probe: chrono_tz_sources.rs's NamedZoneResolutionFailedSource, wrapping a real chrono_tz::ParseError, bare, unboxed, no enum wrapper".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

#[cfg(feature = "chrono-tz")]
amenable_derive::gallery_harness! {
    kani, CHECK_CHRONO_TZ_SOURCES_BRANCH_SRC, {
        #[kani::proof]
        fn check_chrono_tz_sources_branch() {
            let parse_err = "not-a-real-zone".parse::<chrono_tz::Tz>().unwrap_err();
            let _err: Result<u8, amenable_time::NamedZoneResolutionFailedSource> = Err(
                amenable_time::NamedZoneResolutionFailedSource::new(parse_err, "not-a-real-zone"),
            );
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::temporal_error_construction::check_jiff_sources_branch".to_owned(),
            "gallery::temporal_error_construction::check_jiff_sources_branch".to_owned(),
            "amenable_kani".to_owned(),
            "branch probe: jiff_sources.rs's JiffDateConstructionFailedSource, wrapping a real jiff::Error, bare, unboxed, no enum wrapper".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

#[cfg(feature = "jiff")]
amenable_derive::gallery_harness! {
    kani, CHECK_JIFF_SOURCES_BRANCH_SRC, {
        #[kani::proof]
        fn check_jiff_sources_branch() {
            let jiff_err = jiff::civil::Date::new(2024, 13, 1).unwrap_err();
            let _err: Result<u8, amenable_time::JiffDateConstructionFailedSource> = Err(
                amenable_time::JiffDateConstructionFailedSource::new(jiff_err),
            );
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::temporal_error_construction::check_jiff_interval_sources_branch".to_owned(),
            "gallery::temporal_error_construction::check_jiff_interval_sources_branch".to_owned(),
            "amenable_kani".to_owned(),
            "branch probe: jiff_interval_sources.rs's IntervalEndpointsReversedSource, wrapping real jiff::Timestamp values, bare, unboxed, no enum wrapper".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

#[cfg(feature = "jiff")]
amenable_derive::gallery_harness! {
    kani, CHECK_JIFF_INTERVAL_SOURCES_BRANCH_SRC, {
        #[kani::proof]
        fn check_jiff_interval_sources_branch() {
            let start = jiff::Timestamp::UNIX_EPOCH;
            let end = jiff::Timestamp::UNIX_EPOCH;
            let _err: Result<u8, amenable_time::IntervalEndpointsReversedSource> = Err(
                amenable_time::IntervalEndpointsReversedSource::new(start, end),
            );
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::temporal_error_construction::check_parsing_branch".to_owned(),
            "gallery::temporal_error_construction::check_parsing_branch".to_owned(),
            "amenable_kani".to_owned(),
            "branch probe: parsing.rs's ParseRejectedSource (ParseProfile + ParseRejectionReason, both real typed enums), bare, unboxed, no outer enum wrapper".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::temporal_error_construction::check_bare_jiff_error_leaf".to_owned(),
            "gallery::temporal_error_construction::check_bare_jiff_error_leaf".to_owned(),
            "amenable_kani".to_owned(),
            "leaf probe within the jiff_sources branch: a bare jiff::Error, with NO wrapping Source struct at all, to confirm the cost lives in jiff::Error itself rather than in JiffDateConstructionFailedSource's own derive_more::Error/Display dispatch".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::temporal_error_construction::check_jiff_date_new_success_path".to_owned(),
            "gallery::temporal_error_construction::check_jiff_date_new_success_path".to_owned(),
            "amenable_kani".to_owned(),
            "decomposition: the real production call (instant.rs's complete_date_descriptor_to_jiff_date, Calendar branch) on a value that SUCCEEDS -- isolates whether merely calling jiff::civil::Date::new is expensive regardless of outcome, or only its failure path is".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

#[cfg(feature = "jiff")]
amenable_derive::gallery_harness! {
    kani, CHECK_JIFF_DATE_NEW_SUCCESS_PATH_SRC, {
        #[kani::proof]
        fn check_jiff_date_new_success_path() {
            let _date = jiff::civil::Date::new(2024, 6, 15).unwrap();
        }
    }
}

#[cfg(feature = "jiff")]
amenable_derive::gallery_harness! {
    kani, CHECK_BARE_JIFF_ERROR_LEAF_SRC, {
        #[kani::proof]
        fn check_bare_jiff_error_leaf() {
            // Cheap construction (jiff::Error::from_args, no real
            // validation call at all) -- isolates whether the cost is
            // in jiff::Error's own recursive, Arc-indirected shape
            // (confirmed in its own source: `inner: Option<Arc<
            // ErrorInner>>`, `ErrorInner { cause: Option<Error>, .. }`)
            // or specifically in computing one via a real jiff
            // validation path.
            let jiff_err = jiff::Error::from_args(format_args!("x"));
            let _err: Result<u8, jiff::Error> = Err(jiff_err);
        }
    }
}

amenable_derive::gallery_harness! {
    kani, CHECK_PARSING_BRANCH_SRC, {
        #[kani::proof]
        fn check_parsing_branch() {
            let month: u8 = kani::any();
            let _err: Result<u8, amenable_time::ParseRejectedSource> = Err(
                amenable_time::ParseRejectedSource::new(
                    amenable_time::ParseProfile::Iso8601ReducedPrecisionCalendarDate,
                    amenable_time::ParseRejectionReason::MonthOutOfRange(month),
                    "0000-13",
                ),
            );
        }
    }
}
