#[cfg(test)]
mod get {
    use actix_service::Service;
    use actix_web::{App, guard, HttpResponse, web};
    use actix_web::http::StatusCode;
    use actix_web::test;

    use crate::routes;

    #[test]
    fn test_version() {
        let mut app = test::init_service(App::new()
            .service(web::resource("/version").route(
                web::route()
                    .guard(guard::Get())
                    .to(|| HttpResponse::MethodNotAllowed())
                    .to(routes::root::version))));
        // Create request object
        let req = test::TestRequest::get().uri("/version").to_request();

        // Execute application
        let resp = test::block_on(app.call(req)).unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
}

#[cfg(test)]
mod post {
    use actix_service::Service;
    use actix_web::{App, guard, HttpResponse, web};
    use actix_web::http::StatusCode;
    use actix_web::test;

    use crate::routes;

    #[test]
    fn test_version() {
        let mut app = test::init_service(App::new()
            .service(web::resource("/version").route(
                web::route()
                    .guard(guard::Get())
                    .to(|| HttpResponse::MethodNotAllowed())
                    .to(routes::root::version))));
        // Create request object
        let req = test::TestRequest::post().uri("/version").to_request();

        // Execute application
        let resp = test::block_on(app.call(req)).unwrap();
        assert_eq!(resp.status(), StatusCode::METHOD_NOT_ALLOWED);
    }
}

#[cfg(test)]
mod put {
    use actix_service::Service;
    use actix_web::{App, guard, HttpResponse, web};
    use actix_web::http::StatusCode;
    use actix_web::test;

    use crate::routes;

    #[test]
    fn test_version() {
        let mut app = test::init_service(App::new()
            .service(web::resource("/version").route(
                web::route()
                    .guard(guard::Get())
                    .to(|| HttpResponse::MethodNotAllowed())
                    .to(routes::root::version))));
        // Create request object
        let req = test::TestRequest::put().uri("/version").to_request();

        // Execute application
        let resp = test::block_on(app.call(req)).unwrap();
        assert_eq!(resp.status(), StatusCode::METHOD_NOT_ALLOWED);
    }
}

#[cfg(test)]
mod patch {
    use actix_service::Service;
    use actix_web::{App, guard, HttpResponse, web};
    use actix_web::http::StatusCode;
    use actix_web::test;

    use crate::routes;

    #[test]
    fn test_version() {
        let mut app = test::init_service(App::new()
            .service(web::resource("/version").route(
                web::route()
                    .guard(guard::Get())
                    .to(|| HttpResponse::MethodNotAllowed())
                    .to(routes::root::version))));
        // Create request object
        let req = test::TestRequest::patch().uri("/version").to_request();

        // Execute application
        let resp = test::block_on(app.call(req)).unwrap();
        assert_eq!(resp.status(), StatusCode::METHOD_NOT_ALLOWED);
    }
}

#[cfg(test)]
mod delete {
    use actix_service::Service;
    use actix_web::{App, guard, HttpResponse, web};
    use actix_web::http::StatusCode;
    use actix_web::test;

    use crate::routes;

    #[test]
    fn test_version() {
        let mut app = test::init_service(App::new()
            .service(web::resource("/version").route(
                web::route()
                    .guard(guard::Get())
                    .to(|| HttpResponse::MethodNotAllowed())
                    .to(routes::root::version))));
        // Create request object
        let req = test::TestRequest::delete().uri("/version").to_request();

        // Execute application
        let resp = test::block_on(app.call(req)).unwrap();
        assert_eq!(resp.status(), StatusCode::METHOD_NOT_ALLOWED);
    }
}
