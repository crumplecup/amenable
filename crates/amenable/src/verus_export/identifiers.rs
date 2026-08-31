//! Pure identifier-casing utilities shared by the route, tree-walk, and
//! render layers -- no route or artifact concepts of their own.

use tracing::instrument;

/// `module_stem` (already `snake_case`) to `PascalCase`, for synthetic
/// local type names (`{Stem}Selector`/`{Stem}Result`) that need to look
/// like real Rust type identifiers, not the `snake_case` function name
/// they're derived from.
#[instrument(level = "debug")]
pub(super) fn to_pascal_case(snake_case: &str) -> String {
    snake_case
        .split('_')
        .filter(|segment| !segment.is_empty())
        .map(|segment| {
            let mut chars = segment.chars();
            match chars.next() {
                Some(first) => first.to_ascii_uppercase().to_string() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect()
}

#[instrument(level = "debug")]
pub(super) fn normalize_identifier(value: &str) -> String {
    let mut normalized = String::new();
    let mut previous_was_underscore = false;

    for (index, ch) in value.chars().enumerate() {
        if ch.is_ascii_alphanumeric() {
            if ch.is_ascii_uppercase() {
                if index > 0 && !previous_was_underscore {
                    normalized.push('_');
                }
                normalized.push(ch.to_ascii_lowercase());
            } else {
                normalized.push(ch.to_ascii_lowercase());
            }
            previous_was_underscore = false;
        } else if !previous_was_underscore {
            normalized.push('_');
            previous_was_underscore = true;
        }
    }

    let normalized = normalized.trim_matches('_').to_owned();
    if normalized.is_empty() {
        "_".to_owned()
    } else {
        normalized
    }
}
