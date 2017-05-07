use schema::*;

#[derive(Queryable, Serialize, Debug)]
pub struct Team {
    pub id: i32,
    pub driver_id: i32,
    pub time: f64,
}

#[derive(Insertable, Deserialize, Debug, PartialEq)]
#[table_name = "teams"]
pub struct NewTeam {
    driver_id: i32,
    time: f64,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Member {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Deserialize, Debug, PartialEq)]
#[table_name = "members"]
pub struct NewMember {
    name: String,
}

// impl NewTeam {
//     pub fn new(driver: Member) -> NewTeam {
//         NewTeam {
//             driver: driver,
//         }
//     }
// }

//     pub fn next_driver(&self) -> Member {
//         let current_driver_index = self.members
//             .iter()
//             .position(|ref member| member == &&self.driver)
//             .expect("Valid index for current driver");

//         let next_driver_index = current_driver_index + 1;

//         if next_driver_index == self.members.len() {
//             self.members
//                 .first()
//                 .expect("At least one member")
//                 .clone()
//         } else {
//             self.members[next_driver_index].clone()
//         }
//     }

//     pub fn change_driver(&mut self, next_driver: &Member) {
//         self.driver = next_driver.to_owned()
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::{Member, Team};

//     #[test]
//     fn test_next_driver() {
//         let members: Vec<Member> = vec!["Mike".into(), "Brian".into(), "Patrick".into()];

//         let team = Team::new(members.clone());

//         assert_eq!(team.next_driver(), team.members[1]);
//     }

//     #[test]
//     fn test_change_driver() {
//         let members: Vec<Member> = vec!["Mike".into(), "Brian".into(), "Patrick".into()];

//         let mut team = Team::new(members.clone());

//         let next_driver = team.next_driver();
//         team.change_driver(&next_driver);

//         assert_eq!(next_driver, team.driver);
//     }
// }
