use slog::{Drain, Logger};
use slog_async;
use slog_term;

use lazy_static::lazy_static;

pub type BoxResult<T> = Result<T, Box<std::error::Error>>;

fn logger() -> Logger {
    let decorator = slog_term::TermDecorator::new().force_color().build();
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    Logger::root(drain, o!())
}

lazy_static! {
    pub static ref LOGGER: Logger = logger();
}
