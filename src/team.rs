use clap::ArgMatches;
use client::Client;
use hostname::get_hostname;
use member::{self, NewMember, Member};
use super::Result;

pub enum TeamId {
    Id(i32),
    Hostname(String),
}

pub fn create<C: Client>(matches: &ArgMatches, client: &C) -> Result<Team> {
    let time_per_driver_in_minutes = matches.value_of("minutes")
        .map(|minutes| minutes.parse::<f64>())
        .unwrap_or(Ok(5.0))?;

    let new_members = member::build(matches);

    let new_team = NewTeam::new(new_members, time_per_driver_in_minutes, hostname());
    let team = client.create_team(&new_team)?;

    Ok(team)
}

fn hostname() -> String {
    get_hostname().expect("system to have a hostname")
}

#[derive(Debug, Serialize)]
pub struct NewTeam {
    time: f64,
    hostname: String,
    members: Vec<NewMember>,
}

impl NewTeam {
    fn new(new_members: Vec<NewMember>, time: f64, hostname: String) -> NewTeam {
        NewTeam {
            time: time,
            hostname: hostname,
            members: new_members,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Team {
    pub id: i32,
    pub driver: Member,
    pub hostname: String,
    pub time: f64,
    pub members: Vec<Member>,
}

impl Team {
    pub fn next_driver(&self) -> Member {
        let next_driver_position = next_driver_position(
            self.driver.position + 1,
            &self.members);

        let mut next_driver = self.members
            .clone()
            .into_iter()
            .find(|member| member.position == next_driver_position)
            .expect("Member to exist at position");
        next_driver.driver = true;
        next_driver
    }
}

fn next_driver_position(next_position: i32, members: &Vec<Member>) -> i32 {
    match members.into_iter().find(|member| member.position == next_position) {
        Some(driver) => {
            if driver.is_active() {
                driver.position
            } else {
                next_driver_position(driver.position + 1, members)
            }
        }
        None => next_driver_position(0, members)

    }
}

#[cfg(test)]
mod test {
    use super::{Member, Team};

    #[test]
    fn test_next_driver() {
        let current_driver = Member::new(1, "Mike", 1, true, true);
        let next_driver = Member::new(2, "Brian", 2, true, false);
        let members: Vec<Member> = vec![
            current_driver.clone(),
            next_driver.clone(),
            Member::new(3, "Patrick", 3, true, false)];

        let team = Team {
            id: 1,
            driver: current_driver,
            hostname: "example".into(),
            time: 5.0,
            members: members,
        };

        assert_eq!(team.next_driver(), next_driver);
    }

    #[test]
    fn test_next_driver_ignores_inactive() {
        let current_driver  = Member::new(1, "Mike", 1, true, true);
        let inactive_driver  = Member::new(2, "Brian", 2, false, false);
        let next_driver  = Member::new(3, "Patrick", 3, true, false);

        let members: Vec<Member> = vec![
            current_driver.clone(),
            inactive_driver,
            next_driver.clone()];

        let team = Team {
            id: 1,
            driver: current_driver,
            hostname: "example".into(),
            time: 5.0,
            members: members,
        };

        assert_eq!(team.next_driver(), next_driver);
    }
}
