use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

const PROOF_ID: &str = "amenable_kani::calculator::add_impl_computes_exact_sum";

fn temporary_path(name: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after the Unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "amenable-{name}-{}-{nonce}.jsonl",
        std::process::id()
    ))
}

fn record(path: &Path, reviewer: &str, score: &str, recommendation: &str, comment: &str) {
    let output = Command::new(env!("CARGO_BIN_EXE_amenable"))
        .args([
            "assess",
            "proof",
            "--proof",
            PROOF_ID,
            "--reviewer",
            reviewer,
            "--claim-alignment",
            score,
            "--assumption-adequacy",
            score,
            "--model-fidelity",
            score,
            "--assertion-strength",
            score,
            "--adversarial-coverage",
            score,
            "--clarity",
            score,
            "--recommendation",
            recommendation,
            "--comment",
            comment,
            "--assessments",
            path.to_str().expect("temporary path should be UTF-8"),
        ])
        .output()
        .expect("assessment CLI should start");

    assert!(
        output.status.success(),
        "assessment failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn assessment_appends_multiline_comment_as_one_json_record() {
    let path = temporary_path("assessment-record");
    let comment_path = temporary_path("assessment-comment");
    let comment = "The arithmetic claim is exact under the stated precondition.\nAdd a boundary-focused proof next.";
    fs::write(&comment_path, comment).expect("comment file should be written");

    let output = Command::new(env!("CARGO_BIN_EXE_amenable"))
        .args([
            "assess",
            "proof",
            "--proof",
            PROOF_ID,
            "--reviewer",
            "skeptical-reviewer",
            "--claim-alignment",
            "3",
            "--assumption-adequacy",
            "2",
            "--model-fidelity",
            "4",
            "--assertion-strength",
            "3",
            "--adversarial-coverage",
            "1",
            "--clarity",
            "4",
            "--recommendation",
            "strengthen",
            "--comment-file",
            comment_path
                .to_str()
                .expect("temporary path should be UTF-8"),
            "--assessments",
            path.to_str().expect("temporary path should be UTF-8"),
        ])
        .output()
        .expect("assessment CLI should start");

    assert!(
        output.status.success(),
        "assessment failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let contents = fs::read_to_string(&path).expect("assessment artifact should be written");
    assert_eq!(
        contents.lines().count(),
        1,
        "one assessment is one JSONL line"
    );

    let record: serde_json::Value =
        serde_json::from_str(contents.trim()).expect("assessment record should be JSON");
    assert_eq!(record["version"], "0.1.0");
    assert_eq!(record["proof_id"], PROOF_ID);
    assert!(record["timestamp"].as_u64().is_some());
    assert_eq!(record["recommendation"], "strengthen");
    assert_eq!(record["comment"], comment);
    assert!(
        record.get("schema_version").is_none(),
        "new records should not emit the legacy schema_version field"
    );
    assert!(
        record.get("timestamp_unix_seconds").is_none(),
        "new records should not emit the legacy timestamp_unix_seconds field"
    );

    fs::remove_file(path).expect("assessment artifact should be removed");
    fs::remove_file(comment_path).expect("comment file should be removed");
}

#[test]
fn report_aggregates_independent_assessments_by_axis_and_recommendation() {
    let path = temporary_path("assessment-report");
    record(
        &path,
        "reviewer-one",
        "4",
        "accept",
        "Strong, focused proof.",
    );
    record(
        &path,
        "reviewer-two",
        "2",
        "strengthen",
        "The model needs broader boundary coverage.",
    );

    let output = Command::new(env!("CARGO_BIN_EXE_amenable"))
        .args([
            "assess",
            "report",
            "--proof",
            PROOF_ID,
            "--assessments",
            path.to_str().expect("temporary path should be UTF-8"),
        ])
        .output()
        .expect("assessment report CLI should start");

    assert!(
        output.status.success(),
        "report failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).expect("report should be UTF-8");
    assert!(stdout.contains("2 assessment(s)"));
    assert!(stdout.contains("claim alignment: mean 3.00; 0:0 1:0 2:1 3:0 4:1"));
    assert!(stdout.contains("recommendations: accept:1 strengthen:1"));

    fs::remove_file(path).expect("assessment artifact should be removed");
}

#[test]
fn score_outside_the_rubric_range_is_rejected_before_recording() {
    let path = temporary_path("assessment-invalid-score");
    let output = Command::new(env!("CARGO_BIN_EXE_amenable"))
        .args([
            "assess",
            "proof",
            "--proof",
            PROOF_ID,
            "--reviewer",
            "reviewer",
            "--claim-alignment",
            "5",
            "--assumption-adequacy",
            "3",
            "--model-fidelity",
            "3",
            "--assertion-strength",
            "3",
            "--adversarial-coverage",
            "3",
            "--clarity",
            "3",
            "--recommendation",
            "strengthen",
            "--comment",
            "This should not be recorded.",
            "--assessments",
            path.to_str().expect("temporary path should be UTF-8"),
        ])
        .output()
        .expect("assessment CLI should start");

    assert!(!output.status.success());
    assert!(
        !path.exists(),
        "invalid assessment must not create a record"
    );
}

#[test]
fn queue_omits_assessed_proofs_and_keeps_other_registered_proofs_actionable() {
    let path = temporary_path("assessment-queue");
    record(
        &path,
        "reviewer",
        "3",
        "accept",
        "This proof has received its first assessment.",
    );

    let output = Command::new(env!("CARGO_BIN_EXE_amenable"))
        .args([
            "assess",
            "queue",
            "--assessments",
            path.to_str().expect("temporary path should be UTF-8"),
        ])
        .output()
        .expect("assessment queue CLI should start");

    assert!(
        output.status.success(),
        "queue failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).expect("queue should be UTF-8");
    assert!(stdout.contains("Unassessed Kani proofs:"));
    assert!(!stdout.contains(PROOF_ID));

    fs::remove_file(path).expect("assessment artifact should be removed");
}

#[test]
fn report_accepts_legacy_assessment_records_for_back_compatibility() {
    let path = temporary_path("assessment-legacy-report");
    let legacy_record = format!(
        "{{\"schema_version\":1,\"proof_id\":\"{PROOF_ID}\",\"reviewer\":\"legacy-reviewer\",\"timestamp_unix_seconds\":1785357757,\"rubric\":{{\"claim_alignment\":4,\"assumption_adequacy\":3,\"model_fidelity\":3,\"assertion_strength\":4,\"adversarial_coverage\":2,\"clarity\":4}},\"recommendation\":\"accept\",\"comment\":\"Legacy record.\"}}\n"
    );
    fs::write(&path, legacy_record).expect("legacy assessment artifact should be written");

    let output = Command::new(env!("CARGO_BIN_EXE_amenable"))
        .args([
            "assess",
            "report",
            "--proof",
            PROOF_ID,
            "--assessments",
            path.to_str().expect("temporary path should be UTF-8"),
        ])
        .output()
        .expect("legacy assessment report CLI should start");

    assert!(
        output.status.success(),
        "legacy report failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).expect("legacy report should be UTF-8");
    assert!(stdout.contains("1 assessment(s)"));
    assert!(stdout.contains("recommendations: accept:1"));

    fs::remove_file(path).expect("legacy assessment artifact should be removed");
}
