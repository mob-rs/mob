use Result;
use db::Conn;
use models::{NewMember, Member};
use schema::members::dsl::{members as all_members};
use schema::members;
use std::ops::Deref;

use rocket::Route;
use rocket_contrib::JSON;

use diesel;
use diesel::prelude::*;

pub fn routes() -> Vec<Route> {
    routes![index, create]
}

#[get("/", format = "application/json")]
fn index(conn: Conn) -> Result<JSON<Vec<Member>>> {
    let members = all_members.load(conn.deref())?;

    Ok(JSON(members))
}

#[post("/", format = "application/json", data = "<new_members>")]
fn create(new_members: JSON<Vec<NewMember>>, conn: Conn) -> Result<JSON<Vec<Member>>> {
    diesel::insert(&new_members.into_inner())
        .into(members::table)
        .execute(conn.deref())?;

    let members = all_members.load(conn.deref())?;

    Ok(JSON(members))
}

#[cfg(test)]
mod test {
    use db::default_pool;
    use web::app;
    use models::Member;

    use rocket::http::Method::*;
    use rocket::http::{ContentType, Status};
    use rocket::testing::MockRequest;
    use serde_json;

    #[test]
    fn test_index() {
        let app = app(default_pool());

        let mut req = MockRequest::new(Get, "/members").header(ContentType::JSON);
        let response = req.dispatch_with(&app);

        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_create() {
        let pool = default_pool();

        let app = app(pool.clone());

        let request_body = json!([{ "name": "Mike" }]).to_string();

        let mut req = MockRequest::new(Post, "/members")
            .header(ContentType::JSON)
            .body(request_body);

        let mut response = req.dispatch_with(&app);

        let body = response.body().unwrap().into_string().unwrap();
        let members: Vec<Member> = serde_json::from_str(&body).unwrap();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(members.first().unwrap().name, "Mike")
    }
}
