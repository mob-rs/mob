use db::Conn;
use errors::Result;
use models::{NewTeam, Team, NewMember, Member};
use schema::{members, teams};
use std::ops::Deref;

use rocket::Route;
use rocket_contrib::{Json, Value};

use diesel;
use diesel::prelude::*;

pub fn routes() -> Vec<Route> {
    routes![create, show, show_last, delete]
}

#[derive(Debug, Deserialize)]
struct NewTeamBody {
    time: f64,
    members: Vec<NewMemberBody>,
}

impl NewTeamBody {
    fn new_team(&self) -> NewTeam {
        NewTeam::new(self.time)
    }

    fn new_members(self, team: &Team) -> Vec<NewMember> {
        self.members.into_iter().map(|member| {
            NewMember::new(
                team,
                &member.name,
                member.position,
                member.position == 1)
        }).collect::<Vec<NewMember>>()
    }
}

#[derive(Debug, Deserialize)]
struct NewMemberBody {
    name: String,
    position: i32,
}

#[post("/", format = "application/json", data = "<new_team_body>")]
fn create(new_team_body: Json<NewTeamBody>, conn: Conn) -> Result<Json<Value>> {
    let body = new_team_body.into_inner();

    let team = diesel::insert(&body.new_team())
        .into(teams::table)
        .get_result::<Team>(conn.deref())?;

    diesel::insert(&body.new_members(&team))
        .into(members::table)
        .execute(conn.deref())?;

    render_team(team, conn)
}

#[get("/<id>", format = "application/json")]
fn show(id: i32, conn: Conn) -> Result<Json<Value>> {
    let team: Team = teams::table.find(id).first(conn.deref())?;
    render_team(team, conn)
}

#[get("/last", format = "application/json")]
fn show_last(conn: Conn) -> Result<Json<Value>> {
    let team: Team = teams::table.order(teams::dsl::id.desc()).first(conn.deref())?;
    render_team(team, conn)
}

#[delete("/<id>")]
fn delete(id: i32, conn: Conn) -> Result<Json<Value>> {
    diesel::delete(teams::table.find(id)).execute(conn.deref())?;
    Ok(Json(json!({ "message": "deleted" })))
}

fn render_team(team: Team, conn: Conn) -> Result<Json<Value>> {
    let driver: Member = Member::belonging_to(&team)
        .filter(members::driver.eq(true))
        .first(conn.deref())?;

    let members: Vec<Member> = Member::belonging_to(&team).load(conn.deref())?;

    Ok(Json(json!({
        "id": team.id,
        "time": team.time,
        "members": members,
        "driver": driver,
    })))
}

#[cfg(test)]
mod test {
    extern crate serde_json;

    use db::default_pool;
    use web::app;
    use models::{NewMember, Member, NewTeam, Team};
    use schema::{members, teams};

    use diesel::prelude::*;
    use diesel;
    use rocket::http::{ContentType, Status};
    use rocket::local::Client;
    use self::serde_json::Value;
    use std::ops::Deref;

    fn assert_team_response(team: Team, driver: Member, json: Value) {
        assert_eq!(json["id"], team.id, "json has team id");
        assert_eq!(json["time"], team.time, "json has team time");
        assert_eq!(json["driver"]["id"], driver.id, "json has driver id");
        assert_eq!(json["driver"]["name"], driver.name, "json has driver name");
    }

    #[test]
    fn test_create() {
        let app = app(default_pool());

        let request_body = json!({
            "time": 5.0,
            "members": [
                { "name": "Mike", "position": 1 },
                { "name": "Brian", "position": 2 },
                { "name": "Patrick", "position": 3 },
            ],
        });

        let client = Client::new(app).unwrap();

        let mut response = client
            .post("/teams")
            .header(ContentType::JSON)
            .body(request_body.to_string())
            .dispatch();

        let body = response.body().unwrap().into_string().unwrap();
        let json: Value = serde_json::from_str(&body).unwrap();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(json["driver"]["name"], "Mike");
    }

    #[test]
    fn test_show_last() {
        let pool = default_pool();
        let app = app(pool.clone());
        let conn = pool.get().unwrap();

        let new_team = NewTeam::new(5.0);
        let team = diesel::insert(&new_team)
            .into(teams::table)
            .get_result::<Team>(conn.deref())
            .unwrap();

        let new_member = NewMember::new(&team, "Mike", 1, true);
        let member = diesel::insert(&new_member)
            .into(members::table)
            .get_result::<Member>(conn.deref())
            .unwrap();

        let client = Client::new(app).unwrap();
        let mut response = client
            .get("/teams/last")
            .header(ContentType::JSON)
            .dispatch();

        let body = response.body().unwrap().into_string().unwrap();
        let json: Value = serde_json::from_str(&body).unwrap();

        assert_eq!(response.status(), Status::Ok);
        assert_team_response(team, member, json);
    }

    #[test]
    fn test_show() {
        let pool = default_pool();
        let app = app(pool.clone());
        let conn = pool.get().unwrap();

        let new_team = NewTeam::new(5.0);
        let team = diesel::insert(&new_team)
            .into(teams::table)
            .get_result::<Team>(conn.deref())
            .unwrap();

        let new_member = NewMember::new(&team, "Mike", 1, true);
        let member = diesel::insert(&new_member)
            .into(members::table)
            .get_result::<Member>(conn.deref())
            .unwrap();

        let client = Client::new(app).unwrap();
        let mut response = client
            .get(format!("/teams/{}", team.id))
            .header(ContentType::JSON)
            .dispatch();

        let body = response.body().unwrap().into_string().unwrap();
        let json: Value = serde_json::from_str(&body).unwrap();

        assert_eq!(response.status(), Status::Ok);
        assert_team_response(team, member, json);
    }

    #[test]
    fn test_delete() {
        let pool = default_pool();
        let app = app(pool.clone());
        let conn = pool.get().unwrap();

        let new_team = NewTeam::new(5.0);
        let team = diesel::insert(&new_team)
            .into(teams::table)
            .get_result::<Team>(conn.deref())
            .unwrap();

        let client = Client::new(app).unwrap();

        let mut response = client
            .delete(format!("/teams/{}", team.id))
            .dispatch();

        let body = response.body().unwrap().into_string().unwrap();
        let json: Value = serde_json::from_str(&body).unwrap();

        let error: diesel::result::Error = teams::table
            .find(team.id)
            .first::<Team>(conn.deref())
            .unwrap_err();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(json["message"], "deleted");
        assert_eq!(error, diesel::result::Error::NotFound);
    }
}
