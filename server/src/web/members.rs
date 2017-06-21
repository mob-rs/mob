use Result;
use db::Conn;
use models::{Member, MemberChangeset};
use schema::members;
use std::ops::Deref;

use rocket::Route;
use rocket_contrib::JSON;

use diesel;
use diesel::prelude::*;

pub fn routes() -> Vec<Route> {
    routes![index, update]
}

#[get("/", format = "application/json")]
fn index(conn: Conn) -> Result<JSON<Vec<Member>>> {
    let members = members::dsl::members.load(conn.deref())?;

    Ok(JSON(members))
}

#[patch("/<id>", format = "application/json", data = "<member_changeset>")]
fn update(id: i32, member_changeset: JSON<MemberChangeset>, conn: Conn) -> Result<JSON<Member>> {
    let member: Member = diesel::update(members::table.find(id))
        .set(&member_changeset.into_inner())
        .get_result::<Member>(conn.deref())?;

    Ok(JSON(member))
}

#[cfg(test)]
mod test {
    use db::default_pool;
    use models::{NewTeam, Team, NewMember, Member};
    use schema::{members, teams};
    use web::app;

    use diesel;
    use diesel::prelude::*;
    use rocket::http::Method::*;
    use rocket::http::{ContentType, Status};
    use rocket::testing::MockRequest;
    use serde_json;
    use std::ops::Deref;

    #[test]
    fn test_index() {
        let app = app(default_pool());

        let mut req = MockRequest::new(Get, "/members").header(ContentType::JSON);
        let response = req.dispatch_with(&app);

        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_update() {
        let pool = default_pool();
        let app = app(pool.clone());
        let conn = pool.get().unwrap();

        let new_team = NewTeam::new(5.0, "example");
        let team = diesel::insert(&new_team)
            .into(teams::table)
            .get_result::<Team>(conn.deref())
            .unwrap();

        let mike = NewMember::new(&team, "Mike", 1, true);
        let member = diesel::insert(&mike)
            .into(members::table)
            .get_result::<Member>(conn.deref())
            .unwrap();

        let request_body = json!({ "driver": false });
        let mut req = MockRequest::new(Patch, format!("/members/{}", member.id))
            .header(ContentType::JSON)
            .body(request_body.to_string());

        let mut response = req.dispatch_with(&app);
        let body = response.body().unwrap().into_string().unwrap();
        let member_response: Member = serde_json::from_str(&body).unwrap();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(member_response.driver, false);
    }
}
