mod get {
    use ::rocket::http::Status;
    use ::rocket::local::Client;

    use crate::rocket;
    use serde_json::Value;

    #[test]
    fn version() {
        let client = Client::new(rocket()).unwrap();
        let mut response = client.get("/tokens").dispatch();
        assert_eq!(response.status(), Status::Ok);
        let response_json: Value = serde_json::from_str(&response.body_string().unwrap()).unwrap();
        assert_eq!(response_json["data"]["version"], env!("CARGO_PKG_VERSION"));
    }
}
