//! Exercises `Stoplight`'s `Amenable` impl -- the first real occupant of
//! `Amenable::kani_surface()`/`creusot_surface()`/`verus_surface()`/
//! `audit_surface()` anywhere in the tree (Step 5 of
//! `EXCHANGE_PROOF_DERIVATION_PLAN.md`). Confirms every surface reports
//! real, specific content, not an empty or vacuous default.

use amenable_core::Amenable;
use amenable_kani::Stoplight;

#[test]
fn kani_surface_reports_all_four_registered_stoplight_harnesses() {
    let surface = Stoplight::kani_surface();

    assert_eq!(
        surface,
        vec![
            "amenable_kani::stoplight::verify_full_cycle_composes".to_owned(),
            "amenable_kani::stoplight::verify_green_transitions_only_to_yellow".to_owned(),
            "amenable_kani::stoplight::verify_red_transitions_only_to_green".to_owned(),
            "amenable_kani::stoplight::verify_yellow_transitions_only_to_red".to_owned(),
        ]
    );
}

#[test]
fn kani_surface_only_reports_stoplight_s_own_harnesses() {
    let surface = Stoplight::kani_surface();

    assert!(
        surface
            .iter()
            .all(|id| id.starts_with("amenable_kani::stoplight::")),
        "kani_surface leaked a harness id from outside its own module: {surface:?}"
    );
}

#[cfg(feature = "creusot")]
#[test]
fn creusot_surface_reports_all_three_stoplight_edges() {
    let surface = Stoplight::creusot_surface();

    assert_eq!(
        surface,
        vec![
            "amenable_creusot::stoplight::green_to_yellow".to_owned(),
            "amenable_creusot::stoplight::yellow_to_red".to_owned(),
            "amenable_creusot::stoplight::red_to_green".to_owned(),
        ]
    );
}

#[cfg(not(feature = "creusot"))]
#[test]
fn creusot_surface_is_honestly_empty_without_the_creusot_feature() {
    assert_eq!(Stoplight::creusot_surface(), Vec::<String>::new());
}

#[test]
fn verus_surface_is_honestly_empty() {
    assert_eq!(Stoplight::verus_surface(), Vec::<String>::new());
}

#[test]
fn audit_surface_contains_the_real_captured_source_of_every_kani_harness() {
    let surface = Stoplight::audit_surface();

    assert_eq!(surface.len(), 4);
    assert!(surface.iter().any(|src| src.contains("green_to_yellow")));
    assert!(surface.iter().any(|src| src.contains("yellow_to_red")));
    assert!(surface.iter().any(|src| src.contains("red_to_green")));
    assert!(surface.iter().any(|src| src.contains("stub_verified")));
}

#[test]
fn invariant_name_reports_sequential_cycle() {
    assert!(Stoplight::invariant_name().ends_with("SequentialCycle"));
}
