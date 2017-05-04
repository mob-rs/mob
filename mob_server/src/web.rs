use rocket_contrib::{JSON, Value};
use models::NewTeam;

#[get("/", format = "application/json")]
fn hello() -> JSON<Value> {
    JSON(json!({
        "message": "Hello World!",
    }))
}

#[post("/teams", format = "application/json", data = "<team>")]
fn create_team(team: JSON<NewTeam>) -> JSON<Value> {
    println!("{:?}", team);
    JSON(json!({ "message": "created" }))
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

    #[test]
    fn test_create_team() {
        let rocket = rocket::ignite().mount("/", routes![super::create_team]);

        let mut req = MockRequest::new(Post, "/teams")
            .header(ContentType::JSON)
            .body(r#"{ "driver_id": 1 }"#);

        let mut response = req.dispatch_with(&rocket);

        assert_eq!(response.status(), Status::Ok);
        let body = response.body().unwrap().into_string().unwrap();
        assert!(body.contains("created"));
    }
}
