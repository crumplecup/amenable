use std::{
    fs,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

use miette::{IntoDiagnostic, WrapErr};

#[test]
fn dump_registry_emits_witness_export_records_section() -> miette::Result<()> {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .into_diagnostic()
        .wrap_err("clock should be after the Unix epoch")?
        .as_nanos();
    let out = std::env::temp_dir().join(format!("amenable-dump-registry-{stamp}.json"));

    let output = Command::new(env!("CARGO_BIN_EXE_amenable"))
        .args(["dump-registry", "--out"])
        .arg(&out)
        .output()
        .into_diagnostic()
        .wrap_err("dump-registry CLI should start")?;

    assert!(
        output.status.success(),
        "dump-registry failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let json = fs::read_to_string(&out)
        .into_diagnostic()
        .wrap_err("registry dump should be readable")?;
    let dump: serde_json::Value = serde_json::from_str(&json)
        .into_diagnostic()
        .wrap_err("registry dump should be valid JSON")?;

    let witness_exports = dump
        .get("witness_export_records")
        .ok_or_else(|| miette::miette!("registry dump should include witness exports"))?;
    assert!(
        witness_exports.is_array(),
        "witness_export_records should be an array: {witness_exports}"
    );
    if let Some(first_export) = witness_exports
        .as_array()
        .and_then(|exports| exports.first())
    {
        assert!(
            first_export.get("artifact").is_some(),
            "witness export entries should include a structured artifact tree: {first_export}"
        );
        assert!(
            first_export
                .get("artifact")
                .and_then(|artifact| artifact.get("metadata"))
                .is_some(),
            "witness export artifacts should include structured leaf metadata: {first_export}"
        );
    }

    let _ = fs::remove_file(&out);
    Ok(())
}
