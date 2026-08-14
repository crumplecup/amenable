use std::{
    fs,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

#[test]
fn dump_registry_emits_witness_export_records_section() {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be after the Unix epoch")
        .as_nanos();
    let out = std::env::temp_dir().join(format!("amenable-dump-registry-{stamp}.json"));

    let output = Command::new(env!("CARGO_BIN_EXE_amenable"))
        .args(["dump-registry", "--out"])
        .arg(&out)
        .output()
        .expect("dump-registry CLI should start");

    assert!(
        output.status.success(),
        "dump-registry failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let json = fs::read_to_string(&out).expect("registry dump should be readable");
    let dump: serde_json::Value =
        serde_json::from_str(&json).expect("registry dump should be valid JSON");

    let witness_exports = dump
        .get("witness_export_records")
        .expect("registry dump should include witness exports");
    assert!(
        witness_exports.is_array(),
        "witness_export_records should be an array: {witness_exports}"
    );

    let _ = fs::remove_file(&out);
}
