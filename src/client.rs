use error::Error;
use member::{NewMember, Member};
use reqwest::Client as ReqwestClient;
use super::Result;
use team::{NewTeam, Team, TeamId};

const SERVER_URL: &'static str = "http://localhost:8000";

pub trait Client {
    fn new() -> Self;
    fn fetch_team(&self, team_id: TeamId) -> Result<Team>;
    fn create_team(&self, new_team: &NewTeam) -> Result<Team>;
    fn update_team(&self, id: i32, driver_id: i32) -> Result<()>;
    fn delete_team(&self, id: i32) -> Result<()>;
    fn create_members(&self, new_members: Vec<NewMember>) -> Result<Vec<Member>>;
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

    fn fetch_team(&self, team_id: TeamId) -> Result<Team> {
        let url = match team_id {
            TeamId::Id(id) => format!("{}/teams/{}", SERVER_URL, id),
            TeamId::Hostname(hostname) => format!("{}/teams/hostname/{}", SERVER_URL, hostname),
        };

        let mut response = self.inner.get(&url).send()?;
        response.json::<Team>().map_err(|error| Error::Http(error))
    }

    fn create_team(&self, new_team: &NewTeam) -> Result<Team> {
        let url = format!("{}/teams", SERVER_URL);
        let mut response = self.inner.post(&url).json(&new_team).send()?;
        response.json::<Team>().map_err(|error| Error::Http(error))
    }

    fn update_team(&self, id: i32, driver_id: i32) -> Result<()> {
        let url = format!("{}/teams/{}", SERVER_URL, id);
        let body = json!({ "driver_id": driver_id });
        self.inner.patch(&url).json(&body).send()?;
        Ok(())
    }

    fn delete_team(&self, id: i32) -> Result<()> {
        let url = format!("{}/teams/{}", SERVER_URL, id);
        self.inner.delete(&url).send()?;
        Ok(())
    }

    fn create_members(&self, new_members: Vec<NewMember>) -> Result<Vec<Member>> {
        let url = format!("{}/members", SERVER_URL);
        let mut response = self.inner.post(&url).json(&new_members).send()?;
        response.json::<Vec<Member>>().map_err(|error| Error::Http(error))
    }
}

#[cfg(test)]
pub struct MockClient {}

#[cfg(test)]
impl Client for MockClient {
    fn new() -> MockClient {
        MockClient {}
    }

    fn fetch_team(&self, _team_id: TeamId) -> Result<Team> {
        let mike = Member { id: 1, name: "Mike".into() };
        let brian = Member { id: 2, name: "Brian".into() };
        let members = vec![mike.clone(), brian];
        let team = Team {
            id: 1,
            driver: mike,
            hostname: "example".into(),
            time: 5.0,
            members: members,
        };

        Ok(team)
    }

    fn create_team(&self, _new_team: &NewTeam) -> Result<Team> {
        let mike = Member { id: 1, name: "Mike".into() };
        let brian = Member { id: 2, name: "Brian".into() };
        let members = vec![mike.clone(), brian];

        let team = Team {
            id: 1,
            driver: mike,
            hostname: "example".into(),
            time: 5.0,
            members: members,
        };

        Ok(team)
    }

    fn update_team(&self, _id: i32, _driver_id: i32) -> Result<()> {
        Ok(())
    }

    fn delete_team(&self, _id: i32) -> Result<()> {
        Ok(())
    }

    fn create_members(&self, _new_members: Vec<NewMember>) -> Result<Vec<Member>> {
        let mike = Member { id: 1, name: "Mike".into() };
        let brian = Member { id: 2, name: "Brian".into() };
        let members = vec![mike, brian];

        Ok(members)
    }
}
