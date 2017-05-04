use rocket_contrib::{JSON, Value};

#[get("/", format = "application/json")]
fn hello() -> JSON<Value> {
    JSON(json!({
        "message": "Hello World!",
    }))
}

#[cfg(test)]
mod test {
    use rocket;
    use rocket::testing::MockRequest;
    use rocket::http::{ContentType, Status};
    use rocket::http::Method::*;

    #[test]
    fn test_hello() {
        let rocket = rocket::ignite().mount("/", routes![super::hello]);

        let mut req = MockRequest::new(Get, "/").header(ContentType::JSON);
        let mut response = req.dispatch_with(&rocket);

        assert_eq!(response.status(), Status::Ok);

        let body = response.body().unwrap().into_string().unwrap();
        assert!(body.contains("Hello World!"));
    }
}
