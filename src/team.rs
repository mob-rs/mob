use clap::ArgMatches;
use client::Client;
use member::{self, Member};
use super::Result;

pub fn create<C: Client>(matches: &ArgMatches, client: &C) -> Result<Team> {
    let time_per_driver_in_minutes = matches.value_of("minutes")
        .map(|minutes| minutes.parse::<f64>())
        .unwrap_or(Ok(5.0))?;

    let members = member::create(matches, client)?;

    let new_team = NewTeam::new(members, time_per_driver_in_minutes);
    let team = client.create_team(&new_team)?;

    Ok(team)
}

#[derive(Debug, Serialize)]
pub struct NewTeam {
    driver_id: i32,
    time: f64,
}

impl NewTeam {
    fn new(members: Vec<Member>, time: f64) -> NewTeam {
        let first_driver = members.first()
            .expect("At least one member")
            .clone();

        NewTeam {
            driver_id: first_driver.id,
            time: time,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Team {
    pub id: i32,
    pub driver: Member,
    pub time: f64,
    pub members: Vec<Member>,
}

impl Team {
    pub fn next_driver(&self) -> Member {
        let current_driver_index = self.members
            .iter()
            .position(|ref member| member == &&self.driver)
            .expect("Valid index for current driver");

        let next_driver_index = current_driver_index + 1;

        if next_driver_index == self.members.len() {
            self.members
                .first()
                .expect("At least one member")
                .clone()
        } else {
            self.members[next_driver_index].clone()
        }
    }

    pub fn change_driver(&mut self, next_driver: &Member) {
        self.driver = next_driver.to_owned()
    }
}

#[cfg(test)]
mod test {
    use super::{Member, Team};

    #[test]
    fn test_next_driver() {
        let members: Vec<Member> = vec![
            Member { id: 1, name: "Mike".into() },
            Member { id: 2, name: "Brian".into() }];

        let team = Team {
            id: 1,
            driver: members.first().unwrap().clone(),
            time: 5.0,
            members: members,
        };

        assert_eq!(team.next_driver(), team.members[1]);
    }

    #[test]
    fn test_change_driver() {
        let members: Vec<Member> = vec![
            Member { id: 1, name: "Mike".into() },
            Member { id: 2, name: "Brian".into() }];

        let mut team = Team {
            id: 1,
            driver: members.first().unwrap().clone(),
            time: 5.0,
            members: members,
        };

        let next_driver = team.next_driver();
        team.change_driver(&next_driver);

        assert_eq!(next_driver, team.driver);
    }
}
