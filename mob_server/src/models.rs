use schema::*;

#[derive(Queryable, Debug)]
pub struct Team {
    pub id: i32,
    pub driver_id: i32,
}

#[derive(Insertable, Deserialize, Debug, PartialEq)]
#[table_name = "teams"]
pub struct NewTeam {
    driver_id: i32,
}

#[derive(Debug, Queryable)]
pub struct Member {
    pub id: i32,
    pub name: String,
}

// impl NewTeam {
//     pub fn new(driver: Member) -> NewTeam {
//         NewTeam {
//             driver: driver,
//         }
//     }
// }

// impl Team {
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
