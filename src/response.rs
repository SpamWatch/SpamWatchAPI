use rocket::http::Status;
use serde_json::{json, Value};

pub fn reponse(status: Status, data: Value) -> String {
    json!({
        "code": status.code,
        "message": status.reason,
        "data": data
    }).to_string()
}

pub fn error(status: Status, reason: String) -> String {
    reponse(status, json!({"reason": reason}))
}
