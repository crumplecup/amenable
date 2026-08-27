//! Proves the `AmenableError` chain actually reaches the real foreign
//! error, not just that it compiles -- exercises `assessment::load`
//! directly (rather than shelling out to the CLI) so the test can inspect
//! the error's source chain and `Display` text, and can use `?` against
//! `AmenableResult` the same way the CLI's own `boundary.rs` does.

mod support;

use miette::IntoDiagnostic;
use std::error::Error as _;
use std::path::PathBuf;

fn temporary_path(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "amenable-error-chain-{name}-{}.jsonl",
        std::process::id()
    ))
}

#[test]
fn load_on_a_missing_artifact_returns_an_empty_list() -> miette::Result<()> {
    amenable::init_tracing();
    let path = temporary_path("missing");
    let assessments = support::library(amenable::assessment::load(&path))?;
    assert!(assessments.is_empty());
    Ok(())
}

#[test]
fn load_on_malformed_json_preserves_the_real_serde_error_in_the_chain() -> miette::Result<()> {
    amenable::init_tracing();
    let path = temporary_path("malformed");
    std::fs::write(&path, "{not valid json\n").into_diagnostic()?;

    let error = amenable::assessment::load(&path).expect_err("malformed JSON must fail");

    // The top-level Display names the file and 1-indexed line.
    let rendered = error.to_string();
    assert!(rendered.contains("line 1"), "rendered: {rendered}");
    assert!(
        rendered.contains(&path.display().to_string()),
        "rendered: {rendered}"
    );

    // The chain reaches all the way to a real serde_json::Error -- this is
    // the actual thing this whole error-handling pass exists to prove:
    // `.source()` isn't None, and isn't just another opaque AmenableError,
    // it's the genuine foreign error, still downcastable.
    let kind = error.source();
    assert!(
        kind.is_some(),
        "AmenableError must expose its kind as source (see #[error(source)] on `kind`)"
    );
    let json_line_source = kind.and_then(std::error::Error::source);
    assert!(
        json_line_source.is_some(),
        "AmenableErrorKind::JsonLine must expose the JsonLineSource wrapper as source"
    );
    let json_error = json_line_source.and_then(std::error::Error::source);
    assert!(
        json_error.is_some(),
        "JsonLineSource must expose the underlying serde_json::Error as source"
    );
    assert!(
        json_error.is_some_and(|error| error.downcast_ref::<serde_json::Error>().is_some()),
        "the deepest source must be the real serde_json::Error, not a stringified copy"
    );

    let _ = std::fs::remove_file(&path);
    Ok(())
}
