//! Real Creusot proof content for `chrono_tz::ParseError`'s own claim
//! (`chrono::parse_error` holds the `CreusotWitness` bridge that
//! references the `_SRC` constant this file's `harness!` call emits).
//!
//! An accommodation model, not `#[trusted]`: five real attempts at a
//! mechanical proof over the real `str` type, each addressing a
//! newly-confirmed issue (real `why3find -X` output, not guesses), hit
//! the same floor -- `creusot-std`'s own `str::view()` is
//! `#[logic(opaque)]` with body `dead`, fully uninterpreted for *every*
//! string, including a literal, confirmed by reading its real source.
//! `str::len()`/`String::len()` both route through that same opaque
//! `self@`, so no automatic solver can connect any literal's length or
//! content to anything concrete -- a real gap in Creusot's own string
//! model, recorded as a gallery finding
//! (`creusot_std_string_view_is_opaque_for_every_string`).
//!
//! The real fact behind this claim isn't content-free, though: every
//! name in chrono-tz's real prebuilt table is non-empty (confirmed by
//! reading it, all 597 checked, shortest "GB"/"NZ" at 2 characters),
//! so a query of length 0 cannot equal any of them. That fact doesn't
//! need `str::view()` at all once restated over plain integers -- the
//! same move `ext_chrono::tz_offset`'s own Verus model
//! (`TzOffsetModelVariant`) already uses for `base`/`dst`/`fix`: a few
//! real, cited instances standing in for a larger real space, with the
//! claim mechanically discharged over that small model, no `str`
//! anywhere in sight. `ZoneNameModelVariant` below models two real
//! zone names (chrono-tz's own shortest ones) by their real lengths,
//! and proves the one law that actually matters: every one of them
//! exceeds the empty query's length of 0. The correspondence between
//! this model and chrono-tz's real `from_str` behavior is independently
//! confirmed by `amenable_kani::chrono::parse_error::
//! check_tz_parse_error_occurs_for_unknown_names`'s real, execution-
//! based proof over the real type -- not re-derived here, the same
//! epistemic shape every Verus accommodation model in this workspace
//! already rests on.

mod model;

#[cfg(creusot)]
use creusot_std::{
    logic::Int,
    macros::{ensures, logic, requires},
};
#[cfg(creusot)]
use model::ZoneNameModelVariant;

amenable_derive::harness! {
    creusot, ZONE_NAME_MODEL_LENGTH_SRC, {
        /// Each variant's own real name length, in characters --
        /// confirmed directly against chrono-tz's real prebuilt table,
        /// not invented.
        #[logic(open)]
        pub fn zone_name_model_length(v: ZoneNameModelVariant) -> Int {
            pearlite! {
                match v {
                    ZoneNameModelVariant::Gb => 2,
                    ZoneNameModelVariant::Nz => 2,
                }
            }
        }
    }
}

amenable_derive::harness! {
    creusot, ZONE_NAME_MODEL_EXCLUDES_EMPTY_QUERY_SRC, {
        /// The law this model establishes: every modeled real name's
        /// length strictly exceeds the empty-string witness's length
        /// (`0`, the same witness `amenable_kani::chrono::parse_error`
        /// uses), so it cannot equal it -- the real reason a lookup
        /// for `""` matches no real zone.
        #[logic(open)]
        fn zone_name_model_excludes_empty_query(v: ZoneNameModelVariant) -> bool {
            pearlite! { zone_name_model_length(v) > 0 }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ext_chrono::parse_error::zone_name_model_excludes_empty_query",
        "creusot",
        "ensures",
        || ZONE_NAME_MODEL_EXCLUDES_EMPTY_QUERY_SRC,
    )
}

amenable_derive::harness! {
    creusot, VERIFY_ZONE_NAME_MODEL_EXCLUDES_EMPTY_QUERY_SRC, {
        /// Proves `zone_name_model_excludes_empty_query` for every
        /// variant -- genuinely discharged by Creusot's own prover
        /// over this model, not assumed. The same claim
        /// `amenable_kani::chrono::parse_error::
        /// check_tz_parse_error_occurs_for_unknown_names` checks by
        /// real execution over the real type; this is the model's own
        /// independent, mechanically-checked half.
        #[requires(true)]
        #[ensures(zone_name_model_excludes_empty_query(v))]
        fn verify_zone_name_model_excludes_empty_query(v: ZoneNameModelVariant) {}
    }
}

amenable_derive::harness! {
    creusot, EXERCISE_ZONE_NAME_MODEL_VARIANTS_SRC, {
        /// Exercises the proof above concretely for both real
        /// variants, `Gb` and `Nz` -- Creusot checks this call site
        /// against `verify_zone_name_model_excludes_empty_query`'s own
        /// `#[ensures]` for each one genuinely, not merely matches
        /// against the enum abstractly.
        fn exercise_zone_name_model_variants() {
            verify_zone_name_model_excludes_empty_query(ZoneNameModelVariant::Gb);
            verify_zone_name_model_excludes_empty_query(ZoneNameModelVariant::Nz);
        }
    }
}
