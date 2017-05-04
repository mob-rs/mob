use db::init_pool;
use rocket::{self, Rocket};

mod teams;

pub fn app() -> Rocket {
    rocket::ignite()
        .manage(init_pool())
        .mount("/teams", teams::routes())
}
