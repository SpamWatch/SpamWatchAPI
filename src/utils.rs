use slog::{Drain, Logger};
use slog_async;
use slog_term;

pub type BoxResult<T> = Result<T, Box<std::error::Error>>;

pub fn logger() -> Logger {
    let decorator = slog_term::TermDecorator::new().force_color().build();
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    Logger::root(drain, o!())
}

#[macro_export]
macro_rules! config {
    ($part1:tt) => (settings::Settings::load()?.$part1);
    // Don't know how else I could do this, so this is the temporary solution
    ($part1:tt.$part2:tt) => (settings::Settings::load()?.$part1.$part2);
}
