#[get("/")]
fn hello() -> &'static str {
    "Hello World!"
}

#[cfg(test)]
mod test {
    use rocket;
    use rocket::testing::MockRequest;
    use rocket::http::Status;
    use rocket::http::Method::*;

    #[test]
    fn test_hello() {
        let rocket = rocket::ignite().mount("/", routes![super::hello]);

        let mut req = MockRequest::new(Get, "/");
        let mut response = req.dispatch_with(&rocket);

        let body_string = response.body().and_then(|b| b.into_string());
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(body_string, Some("Hello World!".into()));
    }
}
