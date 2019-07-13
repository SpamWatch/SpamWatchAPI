use slog::Logger;
use sloggers::Build;
use sloggers::terminal::{Destination, TerminalLoggerBuilder};
use sloggers::types::Severity;

use crate::settings;

pub fn logger() -> Logger {
    TerminalLoggerBuilder::new()
        .level(Severity::Debug)
        .destination(Destination::Stderr)
        .build()
        .unwrap()
}

#[macro_export]
macro_rules! config {
    ($part1:tt) => (settings::Settings::load()?.$part1);
    // Don't know how else I could do this, so this is the temporary solution
    ($part1:tt.$part2:tt) => (settings::Settings::load()?.$part1.$part2);
}
