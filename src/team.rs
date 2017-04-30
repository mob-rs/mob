use rand::{thread_rng, Rng};

pub type Member = String;

#[derive(Debug)]
pub struct Team {
    pub members: Vec<Member>,
    pub driver: Member,
}

impl Team {
    pub fn new(members: Vec<Member>) -> Team {
        let mut randomized_members = members.clone();
        let mut rng = thread_rng();
        rng.shuffle(&mut randomized_members);

        let first_driver = randomized_members.first()
            .expect("At least one member")
            .clone();

        Team {
            members: randomized_members,
            driver: first_driver,
        }
    }

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
    fn test_new() {
        let members: Vec<Member> = vec!["Mike".into(), "Brian".into(), "Patrick".into()];

        let team = Team::new(members.clone());

        assert_eq!(team.driver, team.members[0]);
    }

    #[test]
    fn test_next_driver() {
        let members: Vec<Member> = vec!["Mike".into(), "Brian".into(), "Patrick".into()];

        let team = Team::new(members.clone());

        assert_eq!(team.next_driver(), team.members[1]);
    }

    #[test]
    fn test_change_driver() {
        let members: Vec<Member> = vec!["Mike".into(), "Brian".into(), "Patrick".into()];

        let mut team = Team::new(members.clone());

        let next_driver = team.next_driver();
        team.change_driver(&next_driver);

        assert_eq!(next_driver, team.driver);
    }
}
