use super::{queries, recording, reporting};
use crate::AmenableResult;
use crate::assessment::cli::{
    AssessmentListArgs, AssessmentQueueArgs, AssessmentReportArgs, AssessmentSummaryArgs,
    RecordAssessmentArgs, VerificationFailuresArgs,
};
use tracing::instrument;

#[instrument(level = "debug", skip(args))]
pub(crate) fn record(args: RecordAssessmentArgs) -> AmenableResult<()> {
    recording::record(args)
}

#[instrument(level = "debug", skip(args))]
pub(crate) fn summary(args: AssessmentSummaryArgs) -> AmenableResult<()> {
    reporting::summary(args)
}

#[instrument(level = "debug", skip(args))]
pub(crate) fn failures(args: VerificationFailuresArgs) -> AmenableResult<()> {
    queries::failures(args)
}

#[instrument(level = "debug", skip(args))]
pub(crate) fn list(args: AssessmentListArgs) -> AmenableResult<()> {
    queries::list(args)
}

#[instrument(level = "debug", skip(args))]
pub(crate) fn report(args: AssessmentReportArgs) -> AmenableResult<()> {
    reporting::report(args)
}

#[instrument(level = "debug", skip(args))]
pub(crate) fn queue(args: AssessmentQueueArgs) -> AmenableResult<()> {
    queries::queue(args)
}
