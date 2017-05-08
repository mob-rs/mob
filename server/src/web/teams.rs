use Result;
use db::Conn;
use models::{NewTeam, Team, Member};
use schema::members::dsl::{members as all_members};
use schema::{members, teams};
use std::ops::Deref;

use rocket::Route;
use rocket_contrib::{JSON, Value};

use diesel;
use diesel::prelude::*;

pub fn routes() -> Vec<Route> {
    routes![create, update_driver, delete, show]
}

#[post("/", format = "application/json", data = "<new_team>")]
fn create(new_team: JSON<NewTeam>, conn: Conn) -> Result<JSON<Value>> {
    diesel::insert(&new_team.into_inner())
        .into(teams::table)
        .execute(conn.deref())?;

    render_team(conn)
}

#[patch("/", format = "application/json", data = "<member>")]
fn update_driver(member: JSON<Value>, conn: Conn) -> Result<JSON<Value>> {
    use schema::teams::dsl::*;
    use schema::members::dsl::*;

    let member_name = member
        .get("name")
        .and_then(|value| value.as_str())
        .expect("Member Name");

    let member: Member = members
        .filter(name.eq(member_name))
        .first(conn.deref())?;

    diesel::update(teams.find(1))
        .set(driver_id.eq(member.id))
        .execute(conn.deref())?;

    render_team(conn)
}

#[delete("/")]
fn delete(conn: Conn) -> Result<JSON<Value>> {
    diesel::delete(teams::table.find(1)).execute(conn.deref())?;
    Ok(JSON(json!({ "message": "deleted" })))
}

#[get("/", format = "application/json")]
fn show(conn: Conn) -> Result<JSON<Value>> {
    render_team(conn)
}

fn render_team(conn: Conn) -> Result<JSON<Value>> {
    let team: Team = teams::table.find(1).first(conn.deref())?;
    let driver: Member = members::table
        .find(team.driver_id)
        .first(conn.deref())?;
    let members: Vec<Member> = all_members.load(conn.deref())?;

    Ok(JSON(json!({
        "id": team.id,
        "time": team.time,
        "members": members,
        "driver": driver,
    })))
}

#[cfg(test)]
mod test {
    use db::default_pool;
    use web::app;
    use models::{NewMember, Member, NewTeam, Team};
    use schema::{members, teams};

    use diesel;
    use diesel::prelude::*;
    use rocket::http::Method::*;
    use rocket::http::{ContentType, Status};
    use rocket::testing::MockRequest;
    use std::ops::Deref;
    use serde_json::{self, Value};

    #[test]
    fn test_create() {
        let pool = default_pool();
        let conn = pool.get().unwrap();

        let app = app(pool.clone());

        let new_member = NewMember { name: "Mike".into() };
        diesel::insert(&new_member).into(members::table).execute(conn.deref()).unwrap();
        let driver: Member = members::table.filter(members::dsl::name.eq("Mike")).first(conn.deref()).unwrap();

        let request_body = json!({ "driver_id": driver.id, "time": 5.0 });

        let mut req = MockRequest::new(Post, "/team")
            .header(ContentType::JSON)
            .body(request_body.to_string());

        let mut response = req.dispatch_with(&app);
        let body = response.body().unwrap().into_string().unwrap();
        let json: Value = serde_json::from_str(&body).unwrap();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(json["driver"]["id"], driver.id);
    }

    #[test]
    fn test_update_driver() {
        let pool = default_pool();
        let app = app(pool.clone());
        let conn = pool.get().unwrap();

        let mike = NewMember { name: "Mike".into() };
        diesel::insert(&mike).into(members::table).execute(conn.deref()).unwrap();
        let driver: Member = members::table.filter(members::dsl::name.eq("Mike")).first(conn.deref()).unwrap();

        let brian = NewMember { name: "Brian".into() };
        diesel::insert(&brian).into(members::table).execute(conn.deref()).unwrap();
        let next_driver: Member = members::table.filter(members::dsl::name.eq("Brian")).first(conn.deref()).unwrap();

        let new_team = NewTeam { driver_id: driver.id, time: 5.0 };
        diesel::insert(&new_team).into(teams::table).execute(conn.deref()).unwrap();
        let team: Team = teams::table.find(1).first(conn.deref()).unwrap();

        let request_body = json!({ "name": next_driver.name });
        let mut req = MockRequest::new(Patch, "/team")
            .header(ContentType::JSON)
            .body(request_body.to_string());

        let mut response = req.dispatch_with(&app);
        let body = response.body().unwrap().into_string().unwrap();
        let json: Value = serde_json::from_str(&body).unwrap();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(json["id"], team.id);
        assert_eq!(json["time"], team.time);
        assert_eq!(json["driver"]["id"], next_driver.id);
        assert_eq!(json["driver"]["name"], next_driver.name);
    }

    #[test]
    fn test_delete() {
        let pool = default_pool();
        let app = app(pool.clone());
        let conn = pool.get().unwrap();

        let mike = NewMember { name: "Mike".into() };
        diesel::insert(&mike).into(members::table).execute(conn.deref()).unwrap();
        let driver: Member = members::table.filter(members::dsl::name.eq("Mike")).first(conn.deref()).unwrap();

        let new_team = NewTeam { driver_id: driver.id, time: 5.0 };
        diesel::insert(&new_team).into(teams::table).execute(conn.deref()).unwrap();

        let mut req = MockRequest::new(Delete, "/team");

        let mut response = req.dispatch_with(&app);
        let body = response.body().unwrap().into_string().unwrap();
        let json: Value = serde_json::from_str(&body).unwrap();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(json["message"], "deleted");
        assert_eq!(teams::table.count().first(conn.deref()), Ok(0));
    }

    #[test]
    fn test_show() {
        let pool = default_pool();
        let app = app(pool.clone());
        let conn = pool.get().unwrap();

        let new_member = NewMember { name: "Mike".into() };
        diesel::insert(&new_member).into(members::table).execute(conn.deref()).unwrap();
        let driver: Member = members::table.filter(members::dsl::name.eq("Mike")).first(conn.deref()).unwrap();

        let new_team = NewTeam { driver_id: driver.id, time: 5.0 };
        diesel::insert(&new_team).into(teams::table).execute(conn.deref()).unwrap();
        let team: Team = teams::table.find(1).first(conn.deref()).unwrap();

        let mut req = MockRequest::new(Get, "/team").header(ContentType::JSON);
        let mut response = req.dispatch_with(&app);
        let body = response.body().unwrap().into_string().unwrap();
        let json: Value = serde_json::from_str(&body).unwrap();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(json["id"], team.id);
        assert_eq!(json["time"], team.time);
        assert_eq!(json["driver"]["id"], driver.id);
        assert_eq!(json["driver"]["name"], driver.name);
    }
}
