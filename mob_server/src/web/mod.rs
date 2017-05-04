use rocket::{self, Rocket};

mod teams;

pub fn app() -> Rocket {
    rocket::ignite()
        .mount("/teams", teams::routes())
}
