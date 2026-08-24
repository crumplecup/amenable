//! Binary-only error reporting with miette.
//!
//! Not linked from the library — keeps miette out of the library API and
//! error routing.

use amenable::{AmenableError, AmenableErrorKind};
use miette::{Diagnostic, Report};

use crate::Cli;

/// Run the CLI and render failures with miette.
pub(crate) fn run(cli: Cli) -> Result<(), Report> {
    install_hook();
    crate::dispatch(cli).map_err(|err| Report::from(CliError(err)))
}

/// Print a diagnostic report to stderr.
pub(crate) fn exit_on_error(report: &Report) {
    eprintln!("{report:?}");
}

fn install_hook() {
    let _ = miette::set_hook(Box::new(|_| {
        Box::new(
            miette::MietteHandlerOpts::new()
                .terminal_links(true)
                .unicode(true)
                .build(),
        )
    }));
}

/// Local wrapper so `Diagnostic` can be implemented without touching
/// library types.
#[derive(Debug)]
struct CliError(AmenableError);

impl std::fmt::Display for CliError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(formatter)
    }
}

impl std::error::Error for CliError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

impl Diagnostic for CliError {
    fn code<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        Some(Box::new(format!(
            "amenable::{}",
            error_kind_code(&self.0.kind)
        )))
    }

    fn help<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        Some(Box::new(
            "See the CSV/JSONL ledgers under artifacts/ for verification and assessment history.",
        ))
    }
}

fn error_kind_code(kind: &AmenableErrorKind) -> &'static str {
    use AmenableErrorKind::{
        AssessmentCount, Chain, InvalidScore, InvalidUtcDate, Invariant, Io, JsonLine,
        PreEpochDate, Serde, Std, SystemTime, TimeComponentRange, TimeFormat,
        TimeFormatDescription, TimeParse, TimestampTooLarge,
    };
    match kind {
        Io(_) => "Io",
        JsonLine(_) => "JsonLine",
        Serde(_) => "Serde",
        SystemTime(_) => "SystemTime",
        TimeFormatDescription(_) => "TimeFormatDescription",
        TimeParse(_) => "TimeParse",
        InvalidUtcDate(_) => "InvalidUtcDate",
        TimeComponentRange(_) => "TimeComponentRange",
        TimeFormat(_) => "TimeFormat",
        Invariant(_) => "Invariant",
        Chain(_) => "Chain",
        Std(_) => "Std",
        InvalidScore(_) => "InvalidScore",
        PreEpochDate(_) => "PreEpochDate",
        TimestampTooLarge(_) => "TimestampTooLarge",
        AssessmentCount(_) => "AssessmentCount",
    }
}
