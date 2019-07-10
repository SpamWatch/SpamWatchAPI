#[get("/")]
pub fn info() -> String {
    format!("{} v{} by {}\n{}\n\n{}",
            &env!("CARGO_PKG_NAME"),
            &env!("CARGO_PKG_VERSION"),
            &env!("CARGO_PKG_AUTHORS"),
            &env!("CARGO_PKG_DESCRIPTION"),
            &env!("CARGO_PKG_REPOSITORY"))
}


#[get("/version")]
pub fn version() -> &'static str {
    &env!("CARGO_PKG_VERSION")
}
