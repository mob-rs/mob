use db::Pool;
use rocket::{self, Rocket, Request};
use rocket_contrib::{Json, Value};
use std::env;

mod teams;
mod members;

pub fn app(pool: Pool) -> Rocket {
    set_log_level();
    rocket::ignite()
        .manage(pool)
        .mount("/teams", teams::routes())
        .mount("/members", members::routes())
        .catch(errors![not_found, server_error])
}

#[error(404)]
fn not_found(_req: &Request) -> Json<Value> {
    Json(json!({ "message": "Not Found" }))
}

#[error(500)]
fn server_error(_req: &Request) -> Json<Value> {
    Json(json!({ "message": "Internal Server Error" }))
}

fn set_log_level() {
    env::set_var("ROCKET_LOG", "critical");
}
