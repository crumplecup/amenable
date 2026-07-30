//! Shared miette reporting for integration tests.
//!
//! Library code keeps `AmenableResult`; tests convert at the boundary
//! with [`library`] so a test function can return `miette::Result<()>`
//! and use `?` directly, the same way the CLI's own `boundary.rs` does.

use amenable::AmenableError;
use miette::Diagnostic;

/// Install the fancy miette hook once per test process.
pub fn init() {
    let _ = miette::set_hook(Box::new(|_| {
        Box::new(
            miette::MietteHandlerOpts::new()
                .terminal_links(true)
                .unicode(true)
                .build(),
        )
    }));
}

fn ensure_init() {
    static INIT: std::sync::Once = std::sync::Once::new();
    INIT.call_once(init);
}

/// Convert a library result into a miette result for test functions.
pub fn library<T>(result: amenable::AmenableResult<T>) -> miette::Result<T> {
    ensure_init();
    result.map_err(|err| miette::Report::new(TestError(err)))
}

#[derive(Debug)]
struct TestError(AmenableError);

impl std::fmt::Display for TestError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(formatter)
    }
}

impl std::error::Error for TestError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

impl Diagnostic for TestError {}
