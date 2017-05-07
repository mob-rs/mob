use clap::ArgMatches;
use error::Error;
use rand::{thread_rng, Rng};
use reqwest::Client;
use std::fmt;
use super::Result;

const SERVER_URL: &'static str = "http://localhost:8000";

pub fn create(matches: &ArgMatches) -> Result<()> {
    let time_per_driver_in_minutes = matches.value_of("minutes")
        .map(|minutes| minutes.parse::<f64>())
        .unwrap_or(Ok(5.0))?;

    let new_members = matches
        .value_of("members")
        .expect("members")
        .split(",")
        .map(|name| NewMember::new(name))
        .collect();

    let members = create_members(new_members)?;

    let team = NewTeam::new(members, time_per_driver_in_minutes);
    create_team(&team)?;

    Ok(())
}

fn create_members(new_members: Vec<NewMember>) -> Result<Vec<Member>> {
    let client = Client::new()?;

    let url = format!("{}/members", SERVER_URL);
    let mut response = client.post(&url).json(&new_members).send()?;
    response.json::<Vec<Member>>().map_err(|error| Error::Http(error))
}

fn create_team(new_team: &NewTeam) -> Result<()> {
    let client = Client::new()?;

    let url = format!("{}/teams", SERVER_URL);
    client.post(&url).json(&new_team).send()?;

    Ok(())
}

#[derive(Debug, Serialize)]
struct NewTeam {
    driver_id: i32,
    time: f64,
}

impl NewTeam {
    fn new(members: Vec<Member>, time: f64) -> NewTeam {
        let mut randomized_members = members.clone();
        let mut rng = thread_rng();
        rng.shuffle(&mut randomized_members);

        let first_driver = randomized_members.first()
            .expect("At least one member")
            .clone();

        NewTeam {
            driver_id: first_driver.id,
            time: time,
        }
    }
}

#[derive(Debug, Serialize)]
struct NewMember {
    name: String,
}

impl NewMember {
    fn new(name: &str) -> NewMember {
        NewMember {
            name: name.into(),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
struct Member {
    id: i32,
    name: String,
}

impl fmt::Display for Member {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.name)
    }
}

#[cfg(test)]
mod test {
    use super::{Member, Team};

    #[test]
    fn test_new() {
        let members: Vec<Member> = vec!["Mike".into(), "Brian".into(), "Patrick".into()];

        let time = 5.0;

        let team = Team::new(members.clone(), time);

        assert_eq!(team.driver, team.members[0]);
        assert_eq!(team.time, time);
    }
}
