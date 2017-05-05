use Result;
use db::Conn;
use models::{NewTeam, Team};
use schema::teams::dsl::{teams as all_teams};
use schema::teams;
use std::ops::Deref;

use rocket::Route;
use rocket_contrib::{JSON, Value};

use diesel;
use diesel::prelude::*;

pub fn routes() -> Vec<Route> {
    routes![index, create]
}

#[get("/", format = "application/json")]
fn index(conn: Conn) -> Result<JSON<Vec<Team>>> {
    let teams = all_teams.load(conn.deref())?;

    Ok(JSON(teams))
}

#[post("/", format = "application/json", data = "<new_team>")]
fn create(new_team: JSON<NewTeam>, conn: Conn) -> Result<JSON<Value>> {
    diesel::insert(&new_team.into_inner())
        .into(teams::table)
        .execute(conn.deref())?;

    Ok(JSON(json!({ "message": "created" })))
}

#[cfg(test)]
mod test {
    extern crate tempdir;

    use db::Pool;
    use web::app;

    use diesel::Connection;
    use diesel::sqlite::SqliteConnection;
    use r2d2;
    use r2d2_diesel::ConnectionManager;
    use rocket::http::Method::*;
    use rocket::http::{ContentType, Status};
    use rocket::testing::MockRequest;
    use std::ops::Deref;
    use self::tempdir::TempDir;

    embed_migrations!("migrations");

    fn test_pool() -> Pool {
        let database_url = TempDir::new("mob")
            .unwrap()
            .path()
            .join("db")
            .to_str()
            .unwrap()
            .to_owned();

        let config = r2d2::Config::builder().pool_size(1).build();
        let manager = ConnectionManager::<SqliteConnection>::new(database_url);
        let pool = r2d2::Pool::new(config, manager).expect("db pool");

        let connection = pool.get().unwrap();
        embedded_migrations::run(connection.deref()).unwrap();

        pool
    }

    #[test]
    fn test_index() {
        let app = app(None);

        let mut req = MockRequest::new(Get, "/teams").header(ContentType::JSON);
        let response = req.dispatch_with(&app);

        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_create() {
        let pool = test_pool();
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
