use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

use serde_json::Value;

const PROOF_ID: &str = "amenable_kani::calculator::add_impl_computes_exact_sum";
const SECOND_PROOF_ID: &str =
    "amenable_kani::rust_std::alloc_boxed::verify_box_derefs_and_writes_through";
const THIRD_PROOF_ID: &str = "amenable_kani::calculator::verify_credit_access_preserves_value";

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

fn record(
    path: &Path,
    reviewer: &str,
    score: &str,
    recommendation: &str,
    resolution_path: &str,
    comment: &str,
) {
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
            "--resolution-path",
            resolution_path,
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

fn write_assessment_artifact(path: &Path, contents: &str) {
    fs::write(path, contents).expect("assessment artifact should be written");
}

fn write_kani_results_artifact(path: &Path, contents: &str) {
    fs::write(path, contents).expect("Kani results artifact should be written");
}

struct AssessmentRecord<'a> {
    assessment_id: &'a str,
    proof_id: &'a str,
    reviewer: &'a str,
    timestamp: u64,
    score: u8,
    recommendation: &'a str,
    resolution_path: &'a str,
    comment: &'a str,
}

fn assessment_record(record: AssessmentRecord<'_>) -> String {
    format!(
        concat!(
            "{{\"version\":\"0.1.0\",\"assessment_id\":\"{assessment_id}\",\"proof_id\":\"{proof_id}\",",
            "\"reviewer\":\"{reviewer}\",\"timestamp\":{timestamp},",
            "\"rubric\":{{\"claim_alignment\":{score},\"assumption_adequacy\":{score},",
            "\"model_fidelity\":{score},\"assertion_strength\":{score},",
            "\"adversarial_coverage\":{score},\"clarity\":{score}}},",
            "\"recommendation\":\"{recommendation}\",\"resolution_path\":\"{resolution_path}\",",
            "\"comment\":\"{comment}\"}}\n"
        ),
        assessment_id = record.assessment_id,
        proof_id = record.proof_id,
        reviewer = record.reviewer,
        timestamp = record.timestamp,
        score = record.score,
        recommendation = record.recommendation,
        resolution_path = record.resolution_path,
        comment = record.comment,
    )
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
            "--resolution-path",
            "strengthen_current_proof",
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

    let record: Value =
        serde_json::from_str(contents.trim()).expect("assessment record should be JSON");
    assert_eq!(record["version"], "0.1.0");
    assert_eq!(record["proof_id"], PROOF_ID);
    assert!(
        record["assessment_id"]
            .as_str()
            .is_some_and(|assessment_id| assessment_id.starts_with("assessment-"))
    );
    assert!(record["timestamp"].as_u64().is_some());
    assert_eq!(record["recommendation"], "strengthen");
    assert_eq!(record["resolution_path"], "strengthen_current_proof");
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
fn report_aggregates_independent_assessments_by_recommendation_and_resolution_path() {
    let path = temporary_path("assessment-report");
    record(
        &path,
        "reviewer-one",
        "4",
        "accept",
        "keep_current_proof",
        "Strong, focused proof.",
    );
    record(
        &path,
        "reviewer-two",
        "2",
        "strengthen",
        "strengthen_current_proof",
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
    assert!(stdout.contains("resolution paths: keep_current_proof:1 strengthen_current_proof:1"));

    fs::remove_file(path).expect("assessment artifact should be removed");
}

#[test]
fn summary_counts_assessments_by_recommendation() {
    let path = temporary_path("assessment-summary");
    write_assessment_artifact(
        &path,
        &format!(
            "{}{}{}",
            assessment_record(AssessmentRecord {
                assessment_id: "assessment-1",
                proof_id: PROOF_ID,
                reviewer: "one",
                timestamp: 1785283200,
                score: 4,
                recommendation: "accept",
                resolution_path: "keep_current_proof",
                comment: "Accepted.",
            }),
            assessment_record(AssessmentRecord {
                assessment_id: "assessment-2",
                proof_id: PROOF_ID,
                reviewer: "two",
                timestamp: 1785286800,
                score: 3,
                recommendation: "replace",
                resolution_path: "replace_with_proof_specific_model",
                comment: "Replace it.",
            }),
            assessment_record(AssessmentRecord {
                assessment_id: "assessment-3",
                proof_id: SECOND_PROOF_ID,
                reviewer: "three",
                timestamp: 1785290400,
                score: 2,
                recommendation: "replace",
                resolution_path: "replace_with_accommodation_model",
                comment: "Also replace it.",
            }),
        ),
    );

    let output = Command::new(env!("CARGO_BIN_EXE_amenable"))
        .args([
            "assess",
            "summary",
            "--assessments",
            path.to_str().expect("temporary path should be UTF-8"),
        ])
        .output()
        .expect("assessment summary CLI should start");

    assert!(
        output.status.success(),
        "summary failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).expect("summary should be UTF-8");
    assert!(stdout.contains("recommendation"));
    assert!(stdout.contains("accept"));
    assert!(stdout.contains("replace"));
    assert!(stdout.contains("strengthen"));
    assert!(stdout.contains("retire"));
    let accept_row = stdout
        .lines()
        .find(|line| line.trim_start().starts_with("accept"))
        .expect("summary should include an accept row");
    let replace_row = stdout
        .lines()
        .find(|line| line.trim_start().starts_with("replace"))
        .expect("summary should include a replace row");
    assert!(accept_row.split_whitespace().eq(["accept", "1"]));
    assert!(replace_row.split_whitespace().eq(["replace", "2"]));

    fs::remove_file(path).expect("assessment artifact should be removed");
}

#[test]
fn summary_can_group_by_resolution_path_and_emit_json() {
    let path = temporary_path("assessment-summary-resolution-path");
    let legacy_record = format!(
        "{{\"schema_version\":1,\"proof_id\":\"{SECOND_PROOF_ID}\",\"reviewer\":\"legacy-reviewer\",\"timestamp_unix_seconds\":1785357757,\"rubric\":{{\"claim_alignment\":4,\"assumption_adequacy\":3,\"model_fidelity\":3,\"assertion_strength\":4,\"adversarial_coverage\":2,\"clarity\":4}},\"recommendation\":\"accept\",\"comment\":\"Legacy record.\"}}\n"
    );
    write_assessment_artifact(
        &path,
        &format!(
            "{}{}",
            assessment_record(AssessmentRecord {
                assessment_id: "assessment-1",
                proof_id: PROOF_ID,
                reviewer: "one",
                timestamp: 1785283200,
                score: 4,
                recommendation: "replace",
                resolution_path: "replace_with_accommodation_model",
                comment: "Model it.",
            }),
            legacy_record,
        ),
    );

    let output = Command::new(env!("CARGO_BIN_EXE_amenable"))
        .args([
            "assess",
            "summary",
            "--by",
            "resolution_path",
            "--json",
            "--assessments",
            path.to_str().expect("temporary path should be UTF-8"),
        ])
        .output()
        .expect("assessment summary CLI should start");

    assert!(
        output.status.success(),
        "summary failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).expect("summary should be UTF-8");
    let json: Value = serde_json::from_str(&stdout).expect("summary output should be JSON");
    assert_eq!(json["by"], "resolution_path");
    assert_eq!(json["counts"]["replace_with_accommodation_model"], 1);
    assert_eq!(json["counts"]["legacy_unspecified"], 1);

    fs::remove_file(path).expect("assessment artifact should be removed");
}

#[test]
fn list_filters_assessments_by_recommendation_and_resolution_path() {
    let path = temporary_path("assessment-list");
    write_assessment_artifact(
        &path,
        &format!(
            "{}{}",
            assessment_record(AssessmentRecord {
                assessment_id: "assessment-1",
                proof_id: PROOF_ID,
                reviewer: "one",
                timestamp: 1785283200,
                score: 4,
                recommendation: "replace",
                resolution_path: "replace_with_proof_specific_model",
                comment: "Replace it.",
            }),
            assessment_record(AssessmentRecord {
                assessment_id: "assessment-2",
                proof_id: SECOND_PROOF_ID,
                reviewer: "two",
                timestamp: 1785286800,
                score: 3,
                recommendation: "accept",
                resolution_path: "keep_current_proof",
                comment: "Accepted.",
            }),
        ),
    );

    let output = Command::new(env!("CARGO_BIN_EXE_amenable"))
        .args([
            "assess",
            "list",
            "--recommendation",
            "replace",
            "--resolution-path",
            "replace_with_proof_specific_model",
            "--assessments",
            path.to_str().expect("temporary path should be UTF-8"),
        ])
        .output()
        .expect("assessment list CLI should start");

    assert!(
        output.status.success(),
        "list failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).expect("list should be UTF-8");
    assert!(stdout.contains("assessment-1"));
    assert!(stdout.contains("replace"));
    assert!(stdout.contains("replace_with_proof_specific_model"));
    assert!(stdout.contains(PROOF_ID));
    assert!(stdout.contains("one"));
    assert!(!stdout.contains(SECOND_PROOF_ID));
    assert!(!stdout.contains("\taccept\t"));

    fs::remove_file(path).expect("assessment artifact should be removed");
}

#[test]
fn list_json_emits_full_structured_assessments() {
    let path = temporary_path("assessment-list-json");
    write_assessment_artifact(
        &path,
        &assessment_record(AssessmentRecord {
            assessment_id: "assessment-1",
            proof_id: PROOF_ID,
            reviewer: "one",
            timestamp: 1785283200,
            score: 4,
            recommendation: "accept",
            resolution_path: "keep_current_proof",
            comment: "Accepted.",
        }),
    );

    let output = Command::new(env!("CARGO_BIN_EXE_amenable"))
        .args([
            "assess",
            "list",
            "--json",
            "--assessments",
            path.to_str().expect("temporary path should be UTF-8"),
        ])
        .output()
        .expect("assessment list CLI should start");

    assert!(
        output.status.success(),
        "list failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).expect("list should be UTF-8");
    let json: Value = serde_json::from_str(&stdout).expect("list output should be JSON");
    let entry = json
        .as_array()
        .and_then(|entries| entries.first())
        .expect("list JSON should contain one entry");
    assert_eq!(entry["assessment_id"], "assessment-1");
    assert_eq!(entry["version"], "0.1.0");
    assert_eq!(entry["recommendation"], "accept");
    assert_eq!(entry["resolution_path"], "keep_current_proof");
    assert_eq!(entry["proof_id"], PROOF_ID);
    assert_eq!(entry["reviewer"], "one");
    assert_eq!(entry["recorded_at"], "2026-07-29T00:00:00Z");

    fs::remove_file(path).expect("assessment artifact should be removed");
}

#[test]
fn failures_list_nonpassing_latest_kani_results() {
    let path = temporary_path("assessment-failures");
    write_kani_results_artifact(
        &path,
        &format!(
            "proof_id,timestamp,status\n{PROOF_ID},1785283200,failed\n{SECOND_PROOF_ID},1785286800,passed\n{THIRD_PROOF_ID},1785290400,timeout\n"
        ),
    );

    let output = Command::new(env!("CARGO_BIN_EXE_amenable"))
        .args([
            "assess",
            "failures",
            "--results",
            path.to_str().expect("temporary path should be UTF-8"),
        ])
        .output()
        .expect("assessment failures CLI should start");

    assert!(
        output.status.success(),
        "failures failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).expect("failures should be UTF-8");
    assert!(stdout.contains("failed"));
    assert!(stdout.contains(PROOF_ID));
    assert!(stdout.contains("timeout"));
    assert!(stdout.contains(THIRD_PROOF_ID));
    assert!(!stdout.contains(SECOND_PROOF_ID));
    assert!(!stdout.contains("\tpassed\t"));

    fs::remove_file(path).expect("Kani results artifact should be removed");
}

#[test]
fn failures_can_filter_by_status_and_emit_json() {
    let path = temporary_path("assessment-failures-json");
    write_kani_results_artifact(
        &path,
        &format!(
            "proof_id,timestamp,status\n{PROOF_ID},1785283200,failed\n{SECOND_PROOF_ID},1785286800,passed\n{THIRD_PROOF_ID},1785290400,timeout\n"
        ),
    );

    let output = Command::new(env!("CARGO_BIN_EXE_amenable"))
        .args([
            "assess",
            "failures",
            "--status",
            "timeout",
            "--json",
            "--results",
            path.to_str().expect("temporary path should be UTF-8"),
        ])
        .output()
        .expect("assessment failures CLI should start");

    assert!(
        output.status.success(),
        "failures failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).expect("failures should be UTF-8");
    let json: Value = serde_json::from_str(&stdout).expect("failures output should be JSON");
    let entries = json.as_array().expect("failures JSON should be an array");
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0]["proof_id"], THIRD_PROOF_ID);
    assert_eq!(entries[0]["status"], "timeout");
    assert_eq!(entries[0]["timestamp"], 1785290400);
    assert_eq!(entries[0]["recorded_at"], "2026-07-29T02:00:00Z");

    fs::remove_file(path).expect("Kani results artifact should be removed");
}

#[test]
fn failures_can_list_only_unassessed_failing_proofs_for_a_fresh_sweep() {
    let results_path = temporary_path("assessment-failures-needs-assessment-results");
    let assessments_path = temporary_path("assessment-failures-needs-assessment-assessments");
    write_kani_results_artifact(
        &results_path,
        &format!(
            "proof_id,timestamp,status\n{PROOF_ID},1785283200,failed\n{SECOND_PROOF_ID},1785286800,failed\n{THIRD_PROOF_ID},1785290400,timeout\n"
        ),
    );
    write_assessment_artifact(
        &assessments_path,
        &format!(
            "{}{}",
            assessment_record(AssessmentRecord {
                assessment_id: "assessment-current",
                proof_id: PROOF_ID,
                reviewer: "reviewer-current",
                timestamp: 1785283200,
                score: 3,
                recommendation: "strengthen",
                resolution_path: "strengthen_current_proof",
                comment: "Current sweep assessment.",
            }),
            assessment_record(AssessmentRecord {
                assessment_id: "assessment-old",
                proof_id: THIRD_PROOF_ID,
                reviewer: "reviewer-old",
                timestamp: 1785196799,
                score: 2,
                recommendation: "replace",
                resolution_path: "document_verifier_limitation",
                comment: "Older sweep assessment should not satisfy the queue.",
            })
        ),
    );

    let output = Command::new(env!("CARGO_BIN_EXE_amenable"))
        .args([
            "assess",
            "failures",
            "--needs-assessment",
            "--since",
            "2026-07-29",
            "--results",
            results_path
                .to_str()
                .expect("temporary results path should be UTF-8"),
            "--assessments",
            assessments_path
                .to_str()
                .expect("temporary assessments path should be UTF-8"),
        ])
        .output()
        .expect("assessment failures CLI should start");

    assert!(
        output.status.success(),
        "failures failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).expect("failures should be UTF-8");
    assert!(!stdout.contains(PROOF_ID));
    assert!(stdout.contains(SECOND_PROOF_ID));
    assert!(stdout.contains(THIRD_PROOF_ID));

    fs::remove_file(results_path).expect("Kani results artifact should be removed");
    fs::remove_file(assessments_path).expect("assessment artifact should be removed");
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
            "--resolution-path",
            "strengthen_current_proof",
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
fn incompatible_resolution_path_is_rejected_before_recording() {
    let path = temporary_path("assessment-invalid-resolution-path");
    let output = Command::new(env!("CARGO_BIN_EXE_amenable"))
        .args([
            "assess",
            "proof",
            "--proof",
            PROOF_ID,
            "--reviewer",
            "reviewer",
            "--claim-alignment",
            "3",
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
            "accept",
            "--resolution-path",
            "replace_with_accommodation_model",
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
fn accept_can_record_a_documented_verifier_limitation() {
    let path = temporary_path("assessment-accepts-documented-verifier-limitation");
    let output = Command::new(env!("CARGO_BIN_EXE_amenable"))
        .args([
            "assess",
            "proof",
            "--proof",
            PROOF_ID,
            "--reviewer",
            "reviewer",
            "--claim-alignment",
            "4",
            "--assumption-adequacy",
            "4",
            "--model-fidelity",
            "4",
            "--assertion-strength",
            "3",
            "--adversarial-coverage",
            "3",
            "--clarity",
            "4",
            "--recommendation",
            "accept",
            "--resolution-path",
            "document_verifier_limitation",
            "--comment",
            "This preserved false trail is accepted boundary evidence.",
            "--assessments",
            path.to_str().expect("temporary path should be UTF-8"),
        ])
        .output()
        .expect("assessment CLI should start");

    assert!(
        output.status.success(),
        "recording should succeed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let contents = fs::read_to_string(path).expect("recorded assessment should exist");
    assert!(contents.contains("\"recommendation\":\"accept\""));
    assert!(contents.contains("\"resolution_path\":\"document_verifier_limitation\""));
}

#[test]
fn queue_omits_assessed_proofs_and_keeps_other_registered_proofs_actionable() {
    let path = temporary_path("assessment-queue");
    record(
        &path,
        "reviewer",
        "3",
        "accept",
        "keep_current_proof",
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
fn queue_since_treats_older_assessments_as_out_of_scope_for_a_fresh_sweep() {
    let path = temporary_path("assessment-queue-since");
    write_assessment_artifact(
        &path,
        &format!(
            "{}{}",
            assessment_record(AssessmentRecord {
                assessment_id: "assessment-1",
                proof_id: PROOF_ID,
                reviewer: "old-reviewer",
                timestamp: 1784592000,
                score: 3,
                recommendation: "accept",
                resolution_path: "keep_current_proof",
                comment: "Older assessment.",
            }),
            assessment_record(AssessmentRecord {
                assessment_id: "assessment-2",
                proof_id: SECOND_PROOF_ID,
                reviewer: "fresh-reviewer",
                timestamp: 1785283200,
                score: 4,
                recommendation: "accept",
                resolution_path: "keep_current_proof",
                comment: "Fresh assessment.",
            }),
        ),
    );

    let output = Command::new(env!("CARGO_BIN_EXE_amenable"))
        .args([
            "assess",
            "queue",
            "--since",
            "2026-07-28",
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
    assert!(stdout.contains(PROOF_ID));
    assert!(!stdout.contains(SECOND_PROOF_ID));

    fs::remove_file(path).expect("assessment artifact should be removed");
}

#[test]
fn queue_json_reports_structured_unassessed_proofs() {
    let path = temporary_path("assessment-queue-json");
    record(
        &path,
        "reviewer",
        "3",
        "accept",
        "keep_current_proof",
        "This proof has received its first assessment.",
    );

    let output = Command::new(env!("CARGO_BIN_EXE_amenable"))
        .args([
            "assess",
            "queue",
            "--json",
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
    let json: Value = serde_json::from_str(&stdout).expect("queue output should be JSON");
    assert!(json["count"].as_u64().is_some());
    assert!(
        json["proof_ids"]
            .as_array()
            .is_some_and(|proofs| proofs.iter().all(|proof| proof != PROOF_ID))
    );

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
    assert!(stdout.contains("resolution paths: legacy_unspecified:1"));

    fs::remove_file(path).expect("legacy assessment artifact should be removed");
}
