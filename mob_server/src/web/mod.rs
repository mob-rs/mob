use db::{Pool, default_pool};
use rocket::{self, Rocket};

mod teams;

pub fn app(pool: Option<Pool>) -> Rocket {
    let pool = pool.unwrap_or(default_pool());
    rocket::ignite()
        .manage(pool)
        .mount("/teams", teams::routes())
}
