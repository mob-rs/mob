use clap::ArgMatches;
use error::Error;
use rand::{thread_rng, Rng};
use reqwest::Client;
use std::fmt;
use super::Result;

const SERVER_URL: &'static str = "http://localhost:8000";

pub fn create(matches: &ArgMatches) -> Result<Vec<Member>> {
    let names = extract_names(matches);
    let new_members = build(names)?;
    let members = persist(new_members)?;
    Ok(members)
}

fn extract_names<'a>(matches: &'a ArgMatches) -> Vec<&'a str> {
    matches
        .value_of("members")
        .expect("members")
        .split(",")
        .collect()
}

fn build(names: Vec<&str>) -> Result<Vec<NewMember>> {
    let mut new_members: Vec<NewMember> = names
        .into_iter()
        .map(|name| NewMember::new(name))
        .collect();

    let mut rng = thread_rng();
    rng.shuffle(&mut new_members);

    Ok(new_members)
}

fn persist(new_members: Vec<NewMember>) -> Result<Vec<Member>> {
    let client = Client::new()?;

    let url = format!("{}/members", SERVER_URL);
    let mut response = client.post(&url).json(&new_members).send()?;
    response.json::<Vec<Member>>().map_err(|error| Error::Http(error))
}

#[derive(Debug, Serialize)]
pub struct NewMember {
    name: String,
}

impl NewMember {
    pub fn new(name: &str) -> NewMember {
        NewMember {
            name: name.into(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Member {
    pub id: i32,
    pub name: String,
}

impl fmt::Display for Member {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.name)
    }
}
