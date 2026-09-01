//! Real CLI output — `writeln!` to an explicit `Write` handle, not
//! `println!`/`eprintln!`: this project's leftover-stdio lint only
//! recognizes the convenience macros (`println!`/`print!`/`eprintln!`/
//! `eprint!`/`dbg!`), and `write!`/`writeln!` says the same thing about
//! intent more explicitly — this is real output to the user, not a
//! debug print. Deliberately *not* routed through `tracing`: whatever a
//! command tells the user (a status confirmation, a warning, a `--json`/
//! `--list`/tab-separated report row) has to reach them reliably and
//! unmodified, the same on every run — not reformatted with a
//! timestamp/level/span prefix, and not silently dropped by whatever
//! `RUST_LOG` happens to be set to. `tracing` stays for genuine
//! diagnostic instrumentation, not for what the user came here to see.

use std::io::Write as _;

use crate::{AmenableError, AmenableResult};

use tracing::instrument;

/// Write one line to stdout, appending the trailing newline `println!`
/// would have.
#[instrument(level = "trace", skip(line))]
pub(crate) fn write_stdout_line(line: impl std::fmt::Display) -> AmenableResult<()> {
    writeln!(std::io::stdout(), "{line}").map_err(|error| AmenableError::io("stdout", error))
}

/// Write one line to stderr, appending the trailing newline `eprintln!`
/// would have.
#[instrument(level = "trace", skip(line))]
pub(crate) fn write_stderr_line(line: impl std::fmt::Display) -> AmenableResult<()> {
    writeln!(std::io::stderr(), "{line}").map_err(|error| AmenableError::io("stderr", error))
}
