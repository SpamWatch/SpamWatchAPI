use serde_json::{json, Value};
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

pub fn reponse(status: String, data: Value) -> String {
    json!({
        "code": status,
        "message": status,
        "data": data
    }).to_string()
}

pub fn error(status: String, reason: String) -> String {
    reponse(status, json!({ "reason": reason }))
}
