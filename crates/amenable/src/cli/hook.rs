use tracing::instrument;

/// Install the pretty terminal miette handler (hyperlinks, Unicode
/// box-drawing) before rendering any report. A binary-only nicety, but
/// installing the hook itself defines no clap or error type, so it stays
/// a plain library function `main` calls once, not a reason for a
/// binary-only module.
#[cfg(feature = "cli")]
#[instrument(level = "debug")]
pub fn install_hook() {
    let _ = miette::set_hook(Box::new(|_| {
        Box::new(
            miette::MietteHandlerOpts::new()
                .terminal_links(true)
                .unicode(true)
                .build(),
        )
    }));
}
