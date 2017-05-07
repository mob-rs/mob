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
    extern crate uuid;

    use db::Pool;
    use web::app;
    use models::Team;

    use diesel::prelude::*;
    use diesel::sqlite::SqliteConnection;
    use r2d2;
    use r2d2_diesel::ConnectionManager;
    use rocket::http::Method::*;
    use rocket::http::{ContentType, Status};
    use rocket::testing::MockRequest;
    use self::uuid::Uuid;
    use std::ops::Deref;

    embed_migrations!("migrations");

    fn test_pool() -> Pool {
        let config = r2d2::Config::default();
        let database_url = format!("file:{}.db?mode=memory&cache=shared", Uuid::new_v4());
        let manager = ConnectionManager::<SqliteConnection>::new(database_url);
        let pool = r2d2::Pool::new(config, manager).expect("db pool");

        let connection = pool.get().unwrap();
        embedded_migrations::run(connection.deref()).unwrap();

        pool
    }

    #[test]
    fn test_index() {
        let app = app(Some(test_pool()));

        let mut req = MockRequest::new(Get, "/teams").header(ContentType::JSON);
        let response = req.dispatch_with(&app);

        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_create() {
        use schema::teams::dsl::*;
        use schema::teams::table as teams;

        let pool = test_pool();

        let app = app(Some(pool.clone()));

        let request_body = json!({ "driver_id": 1, "time": 5.0 });

        let mut req = MockRequest::new(Post, "/teams")
            .header(ContentType::JSON)
            .body(request_body.to_string());

        let connection = pool.get().unwrap();

        let mut response = req.dispatch_with(&app);

        assert_eq!(response.status(), Status::Ok);
        let body = response.body().unwrap().into_string().unwrap();
        assert!(body.contains("created"));

        let team: Team = teams.filter(driver_id.eq(1)).first(connection.deref()).unwrap();
        assert_eq!(team.driver_id, 1);
    }
}
