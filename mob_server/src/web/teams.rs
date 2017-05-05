use db::Conn;
use models::{NewTeam, Team};
use Result;

use rocket::Route;
use rocket_contrib::{JSON, Value};

pub fn routes() -> Vec<Route> {
    routes![index, create]
}

#[get("/", format = "application/json")]
fn index(conn: Conn) -> Result<JSON<Vec<Team>>> {
    let teams = Team::all(&conn)?;

    Ok(JSON(teams))
}

#[post("/", format = "application/json", data = "<team>")]
fn create(team: JSON<NewTeam>, conn: Conn) -> JSON<Value> {
    println!("{:?}", team);
    JSON(json!({ "message": "created" }))
}

#[cfg(test)]
mod test {
    use db;
    use rocket::http::Method::*;
    use rocket::http::{ContentType, Status};
    use rocket::testing::MockRequest;
    use std::ops::Deref;
    use web::app;
    use diesel::Connection;

    #[test]
    fn test_index() {
        let app = app(None);

        let mut req = MockRequest::new(Get, "/teams").header(ContentType::JSON);
        let mut response = req.dispatch_with(&app);

        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_create() {
        let pool = db::default_pool();
        let connection = pool.get().unwrap();

        let app = app(Some(pool));

        let mut req = MockRequest::new(Post, "/teams")
            .header(ContentType::JSON)
            .body(r#"{ "driver_id": 1 }"#);

        connection.begin_test_transaction().unwrap();

        let mut response = req.dispatch_with(&app);
        assert_eq!(response.status(), Status::Ok);
        let body = response.body().unwrap().into_string().unwrap();
        assert!(body.contains("created"));
    }
}
