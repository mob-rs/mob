use db::Pool;
use rocket::{self, Rocket};

mod teams;
mod members;

pub fn app(pool: Pool) -> Rocket {
    rocket::ignite()
        .manage(pool)
        .mount("/team", teams::routes())
        .mount("/members", members::routes())
}
