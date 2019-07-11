mod get {
    use ::rocket::http::Status;
    use ::rocket::local::Client;

    use crate::rocket;
    use serde_json::Value;

    #[test]
    fn version() {
        let client = Client::new(rocket()).unwrap();
        let mut response = client.get("/version").dispatch();
        assert_eq!(response.status(), Status::Ok);
        let response_json: Value = serde_json::from_str(&response.body_string().unwrap()).unwrap();
        assert_eq!(response_json["data"]["version"], env!("CARGO_PKG_VERSION"));
    }

    #[test]
    fn version_major() {
        let client = Client::new(rocket()).unwrap();
        let mut response = client.get("/version/major").dispatch();
        assert_eq!(response.status(), Status::Ok);
        let response_json: Value = serde_json::from_str(&response.body_string().unwrap()).unwrap();
        assert_eq!(
            response_json["data"]["version"]["major"],
            env!("CARGO_PKG_VERSION_MAJOR")
        );
    }

    #[test]
    fn version_minor() {
        let client = Client::new(rocket()).unwrap();
        let mut response = client.get("/version/minor").dispatch();
        assert_eq!(response.status(), Status::Ok);
        let response_json: Value = serde_json::from_str(&response.body_string().unwrap()).unwrap();
        println!("{:?}", response_json["data"]["version"]);
        assert_eq!(
            response_json["data"]["version"]["minor"],
            env!("CARGO_PKG_VERSION_MINOR")
        );
    }

    #[test]
    fn version_patch() {
        let client = Client::new(rocket()).unwrap();
        let mut response = client.get("/version/patch").dispatch();
        assert_eq!(response.status(), Status::Ok);
        let response_json: Value = serde_json::from_str(&response.body_string().unwrap()).unwrap();
        assert_eq!(
            response_json["data"]["version"]["patch"],
            env!("CARGO_PKG_VERSION_PATCH")
        );
    }

    #[test]
    fn version_error() {
        let client = Client::new(rocket()).unwrap();
        let mut response = client.get("/version/error").dispatch();
        assert_eq!(response.status(), Status::NotFound);
        let response_json: Value = serde_json::from_str(&response.body_string().unwrap()).unwrap();
    }
}
