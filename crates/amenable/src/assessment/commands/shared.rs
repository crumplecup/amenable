use crate::{AmenableError, AmenableResult};
use serde::Serialize;
use time::{Date, OffsetDateTime, format_description::well_known::Rfc3339};
use tracing::instrument;

#[instrument(level = "debug", skip(date))]
pub(super) fn start_of_utc_date_timestamp(date: Date) -> AmenableResult<u64> {
    let timestamp = date.midnight().assume_utc().unix_timestamp();
    u64::try_from(timestamp).map_err(|error| AmenableError::pre_epoch_date(date.to_string(), error))
}

#[instrument(level = "debug")]
pub(super) fn format_timestamp(timestamp: u64) -> AmenableResult<String> {
    let seconds = i64::try_from(timestamp)
        .map_err(|error| AmenableError::timestamp_too_large(timestamp, error))?;
    let recorded_at = OffsetDateTime::from_unix_timestamp(seconds)?;
    Ok(recorded_at.format(&Rfc3339)?)
}

#[instrument(level = "debug", skip(value))]
pub(super) fn print_json<T: Serialize>(value: &T) -> AmenableResult<()> {
    let json = serde_json::to_string_pretty(value)?;
    crate::write_stdout_line(json)
}
