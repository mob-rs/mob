use errors::Result;
use member::Member;
use reqwest::Client as ReqwestClient;
use team::{NewTeam, Team};

const SERVER_URL: &'static str = "http://localhost:8000";

pub trait Client {
    fn new() -> Self;
    fn create_team(&self, new_team: &NewTeam) -> Result<Team>;
    fn fetch_team(&self, id: i32) -> Result<Team>;
    fn fetch_last_team(&self) -> Result<Team>;
    fn delete_team(&self, id: i32) -> Result<()>;
    fn update_member(&self, id: i32, driver: bool) -> Result<Member>;
}

pub struct HttpClient {
    inner: ReqwestClient,
}

impl Client for HttpClient {
    fn new() -> HttpClient {
        let inner = ReqwestClient::new().expect("Create client");

        HttpClient {
            inner: inner,
        }
    }

    fn fetch_team(&self, id: i32) -> Result<Team> {
        let url = format!("{}/teams/{}", SERVER_URL, id);
        let mut response = self.inner.get(&url).send()?;
        response.json::<Team>().map_err(|error| error.into())
    }

    fn fetch_last_team(&self) -> Result<Team> {
        let url = format!("{}/teams/last", SERVER_URL);
        let mut response = self.inner.get(&url).send()?;
        response.json::<Team>().map_err(|error| error.into())
    }

    fn create_team(&self, new_team: &NewTeam) -> Result<Team> {
        let url = format!("{}/teams", SERVER_URL);
        let mut response = self.inner.post(&url).json(&new_team).send()?;
        response.json::<Team>().map_err(|error| error.into())
    }

    fn delete_team(&self, id: i32) -> Result<()> {
        let url = format!("{}/teams/{}", SERVER_URL, id);
        self.inner.delete(&url).send()?;
        Ok(())
    }

    fn update_member(&self, id: i32, driver: bool) -> Result<Member> {
        let url = format!("{}/members/{}", SERVER_URL, id);
        let body = json!({ "driver": driver });
        let mut response = self.inner.patch(&url).json(&body).send()?;
        response.json::<Member>().map_err(|error| error.into())
    }
}

#[cfg(test)]
pub struct MockClient {}

#[cfg(test)]
impl Client for MockClient {
    fn new() -> MockClient {
        MockClient {}
    }

    fn fetch_team(&self, _id: i32) -> Result<Team> {
        let mike = Member::new(1, "Mike", 1, true, true);
        let brian = Member::new(2, "Brian", 2, true, false);
        let members = vec![mike.clone(), brian];
        let team = Team {
            id: 1,
            driver: mike,
            time: 5.0,
            members: members,
        };

        Ok(team)
    }

    fn fetch_last_team(&self) -> Result<Team> {
        let mike = Member::new(1, "Mike", 1, true, true);
        let brian = Member::new(2, "Brian", 2, true, false);
        let members = vec![mike.clone(), brian];
        let team = Team {
            id: 1,
            driver: mike,
            time: 5.0,
            members: members,
        };

        Ok(team)
    }

    fn create_team(&self, _new_team: &NewTeam) -> Result<Team> {
        let mike = Member::new(1, "Mike", 1, true, true);
        let brian = Member::new(2, "Brian", 2, true, false);
        let members = vec![mike.clone(), brian];

        let team = Team {
            id: 1,
            driver: mike,
            time: 5.0,
            members: members,
        };

        Ok(team)
    }

    fn delete_team(&self, _id: i32) -> Result<()> {
        Ok(())
    }

    fn update_member(&self, _id: i32, driver: bool) -> Result<Member> {
        Ok(Member::new(1, "Mike", 1, true, driver))
    }
}
