use slog::Logger;
use sloggers::Build;
use sloggers::terminal::{Destination, TerminalLoggerBuilder};
use sloggers::types::Severity;

pub fn logger() -> Logger {
    TerminalLoggerBuilder::new()
        .level(Severity::Debug)
        .destination(Destination::Stderr)
        .build()
        .unwrap()
}
