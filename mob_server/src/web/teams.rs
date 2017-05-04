use db::Conn;
use models::{NewTeam, Team};
use Result;

use rocket::Route;
use rocket_contrib::{JSON, Value};

#[get("/", format = "application/json")]
fn index(conn: Conn) -> Result<JSON<Vec<Team>>> {
    let teams = Team::all(&conn)?;

    Ok(JSON(teams))
}

#[post("/", format = "application/json", data = "<team>")]
fn create(team: JSON<NewTeam>) -> JSON<Value> {
    println!("{:?}", team);
    JSON(json!({ "message": "created" }))
}

pub fn routes() -> Vec<Route> {
    routes![index, create]
}

#[cfg(test)]
mod test {
    use rocket::http::Method::*;
    use rocket::http::{ContentType, Status};
    use rocket::testing::MockRequest;
    use web::app;

    #[test]
    fn test_index() {
        let app = app();

        let mut req = MockRequest::new(Get, "/teams").header(ContentType::JSON);
        let mut response = req.dispatch_with(&app);

        assert_eq!(response.status(), Status::Ok);

        let body = response.body().unwrap().into_string().unwrap();
        assert!(body.contains("Hello World!"));
    }

    #[test]
    fn test_create() {
        let app = app();

        let mut req = MockRequest::new(Post, "/teams")
            .header(ContentType::JSON)
            .body(r#"{ "driver_id": 1 }"#);

        let mut response = req.dispatch_with(&app);

        assert_eq!(response.status(), Status::Ok);
        let body = response.body().unwrap().into_string().unwrap();
        assert!(body.contains("created"));
    }
}
