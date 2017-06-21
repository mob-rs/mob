use schema::*;

#[derive(Associations, Identifiable, Queryable, Serialize, Debug)]
#[has_many(members)]
pub struct Team {
    pub id: i32,
    pub time: f64,
    pub hostname: String,
}

#[derive(Insertable, Deserialize, Debug, PartialEq)]
#[table_name = "teams"]
pub struct NewTeam {
    pub time: f64,
    pub hostname: String,
}

impl NewTeam {
    pub fn new(time: f64, hostname: &str) -> NewTeam {
        NewTeam {
            time: time,
            hostname: hostname.into(),
        }
    }
}

#[derive(Associations, Identifiable, Queryable, Serialize, Deserialize, Debug)]
#[belongs_to(Team)]
pub struct Member {
    pub id: i32,
    pub name: String,
    pub team_id: i32,
    pub active: bool,
    pub driver: bool,
    pub position: i32,
}

#[derive(Insertable, AsChangeset, Deserialize, Debug)]
#[table_name = "members"]
pub struct MemberChangeset {
    pub driver: Option<bool>,
}

#[derive(Insertable, Deserialize, Debug, PartialEq)]
#[table_name = "members"]
pub struct NewMember {
    pub name: String,
    pub team_id: i32,
    pub active: bool,
    pub driver: bool,
    pub position: i32,
}

impl NewMember {
    pub fn new(team: &Team, name: &str, position: i32, driver: bool) -> NewMember {
        NewMember {
            name: name.into(),
            team_id: team.id,
            active: true,
            driver: driver,
            position: position,
        }
    }
}
