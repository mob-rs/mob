use schema::*;

#[derive(Queryable, Serialize, Debug)]
pub struct Team {
    pub id: i32,
    pub driver_id: i32,
    pub time: f64,
}

#[derive(Insertable, Deserialize, Debug, PartialEq)]
#[table_name = "teams"]
pub struct NewTeam {
    driver_id: i32,
    time: f64,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Member {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Deserialize, Debug, PartialEq)]
#[table_name = "members"]
pub struct NewMember {
    pub name: String,
}
