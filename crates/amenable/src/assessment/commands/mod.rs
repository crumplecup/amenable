//! Assessment command dispatch plus shared formatting helpers.

mod queries;
mod recording;
mod reporting;

use super::cli::{
    AssessmentListArgs, AssessmentQueueArgs, AssessmentReportArgs, AssessmentSummaryArgs,
    RecordAssessmentArgs, VerificationFailuresArgs,
};
use crate::{AmenableError, AmenableResult};
use serde::Serialize;
use time::{Date, OffsetDateTime, format_description::well_known::Rfc3339};
use tracing::instrument;

#[instrument(level = "debug", skip(args))]
pub(super) fn record(args: RecordAssessmentArgs) -> AmenableResult<()> {
    recording::record(args)
}

#[instrument(level = "debug", skip(args))]
pub(super) fn summary(args: AssessmentSummaryArgs) -> AmenableResult<()> {
    reporting::summary(args)
}

#[instrument(level = "debug", skip(args))]
pub(super) fn failures(args: VerificationFailuresArgs) -> AmenableResult<()> {
    queries::failures(args)
}

#[instrument(level = "debug", skip(args))]
pub(super) fn list(args: AssessmentListArgs) -> AmenableResult<()> {
    queries::list(args)
}

#[instrument(level = "debug", skip(args))]
pub(super) fn report(args: AssessmentReportArgs) -> AmenableResult<()> {
    reporting::report(args)
}

#[instrument(level = "debug", skip(args))]
pub(super) fn queue(args: AssessmentQueueArgs) -> AmenableResult<()> {
    queries::queue(args)
}

#[instrument(level = "debug", skip(date))]
fn start_of_utc_date_timestamp(date: Date) -> AmenableResult<u64> {
    let timestamp = date.midnight().assume_utc().unix_timestamp();
    u64::try_from(timestamp).map_err(|error| AmenableError::pre_epoch_date(date.to_string(), error))
}

#[instrument(level = "debug")]
fn format_timestamp(timestamp: u64) -> AmenableResult<String> {
    let seconds = i64::try_from(timestamp)
        .map_err(|error| AmenableError::timestamp_too_large(timestamp, error))?;
    let recorded_at = OffsetDateTime::from_unix_timestamp(seconds)?;
    Ok(recorded_at.format(&Rfc3339)?)
}

#[instrument(level = "debug", skip(value))]
fn print_json<T: Serialize>(value: &T) -> AmenableResult<()> {
    let json = serde_json::to_string_pretty(value)?;
    crate::write_stdout_line(json)
}
