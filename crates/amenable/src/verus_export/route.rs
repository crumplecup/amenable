//! A position in the artifact tree, for error messages and local-name
//! collision hints.

use super::identifiers::normalize_identifier;
use tracing::instrument;

#[derive(Debug, Clone)]
pub(super) enum RouteSegment {
    Member(String),
    /// A root-level enum's own variant, e.g. `Balanced` -- lets sibling
    /// variants' same-named params (a `value: char` in `Balanced` and
    /// another, semantically unrelated `value: char` in `Fallback`)
    /// disambiguate through `NameAllocator` exactly like sibling members
    /// already do, instead of silently aliasing.
    Variant(String),
}

#[instrument(level = "debug", skip(route))]
pub(super) fn route_display(route: &[RouteSegment]) -> String {
    if route.is_empty() {
        return "root".to_owned();
    }

    route
        .iter()
        .map(|segment| match segment {
            RouteSegment::Member(label) => format!("member {label}"),
            RouteSegment::Variant(name) => format!("variant {name}"),
        })
        .collect::<Vec<_>>()
        .join(" -> ")
}

#[instrument(level = "debug", skip(route))]
pub(super) fn route_hint_name(route: &[RouteSegment]) -> String {
    if route.is_empty() {
        return "root".to_owned();
    }

    route
        .iter()
        .map(route_segment_name)
        .collect::<Vec<_>>()
        .join("_")
}

#[instrument(level = "debug", skip(segment))]
fn route_segment_name(segment: &RouteSegment) -> String {
    match segment {
        RouteSegment::Member(label) | RouteSegment::Variant(label) => normalize_identifier(label),
    }
}
