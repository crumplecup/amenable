//! `EvidenceLink`'s generic form: bounds and premises round-trip through the
//! accessors, and the concrete constructor leaves both empty.

use amenable_core::{EvidenceLink, Premise};

static BOUNDS: &[&str] = &["chrono::offset::TimeZone"];
static PREMISES: &[Premise] = &[Premise::new(
    "offset-round-trip",
    "a local offset applied back gives the UTC instant",
)];

#[test]
fn concrete_link_has_no_bounds_or_premises() -> miette::Result<()> {
    let link = EvidenceLink::new(
        "amenable_ext::ExtStandard<Foo>",
        "amenable_ext::ExtStandard<Foo>",
        0,
    );
    assert!(link.bounds().is_empty());
    assert!(link.premises().is_empty());
    Ok(())
}

#[test]
fn generic_link_carries_bounds_and_premises() -> miette::Result<()> {
    let link = EvidenceLink::generic(
        "amenable_ext::ExtGeneric<DateTime<Tz>>",
        "amenable_ext::ExtGeneric<DateTime<Tz>>",
        0,
        BOUNDS,
        PREMISES,
    );
    assert_eq!(link.bounds(), BOUNDS);
    assert_eq!(link.premises().len(), 1);
    let premise = &link.premises()[0];
    assert_eq!(premise.id(), "offset-round-trip");
    assert_eq!(
        premise.statement(),
        "a local offset applied back gives the UTC instant"
    );
    Ok(())
}

#[test]
fn generic_link_keeps_name_and_index() -> miette::Result<()> {
    let link = EvidenceLink::generic("generic-name", "generic-name", 3, BOUNDS, PREMISES);
    assert_eq!(link.name(), "generic-name");
    assert_eq!(link.basis(), "generic-name");
    assert_eq!(link.index(), 3);
    Ok(())
}
