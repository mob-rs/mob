use clap::ArgMatches;
use client::Client;
use rand::{thread_rng, Rng};
use std::fmt;
use super::Result;

pub fn create<C: Client>(matches: &ArgMatches, client: &C) -> Result<Vec<Member>> {
    let names = extract_names(matches);
    let new_members = build(names)?;
    let members = client.create_members(new_members)?;
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
