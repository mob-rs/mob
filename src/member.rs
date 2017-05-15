use clap::ArgMatches;
use rand::{thread_rng, Rng};
use std::fmt;

pub fn build(matches: &ArgMatches) -> Vec<NewMember> {
    let mut names = extract_names(matches);

    let mut rng = thread_rng();
    rng.shuffle(&mut names);

    names
        .into_iter()
        .enumerate()
        .map(|(index, name)| {
            let position = index as i32 + 1;
            NewMember::new(name, position)
        })
        .collect()
}

fn extract_names<'a>(matches: &'a ArgMatches) -> Vec<&'a str> {
    matches
        .value_of("members")
        .expect("members")
        .split(",")
        .collect()
}

#[derive(Debug, Serialize)]
pub struct NewMember {
    name: String,
    position: i32,
}

impl NewMember {
    pub fn new(name: &str, position: i32) -> NewMember {
        NewMember {
            name: name.into(),
            position: position,
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
