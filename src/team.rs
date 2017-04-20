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

        let first_driver = randomized_members
            .first()
            .expect("At least one member")
            .clone();

        Team {
            members: randomized_members.clone(),
            driver: first_driver,
        }
    }

    pub fn next_driver(self: &mut Team) {
        let current_driver_index = self
            .members
            .iter()
            .position(|ref member| member == &&self.driver)
            .expect("Valid index for current driver");

        let next_driver_index = current_driver_index + 1;

        if next_driver_index == self.members.len() {
            self.driver = self.members
                .first()
                .expect("At least one member")
                .clone();
        } else {
            self.driver = self.members[next_driver_index].clone();
        }
    }
}
