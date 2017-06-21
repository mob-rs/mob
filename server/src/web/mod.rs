use db::Pool;
use rocket::{self, Rocket, Request};
use rocket_contrib::{JSON, Value};

mod teams;
mod members;

pub fn app(pool: Pool) -> Rocket {
    rocket::ignite()
        .manage(pool)
        .mount("/teams", teams::routes())
        .mount("/members", members::routes())
        .catch(errors![not_found, server_error])
}

#[error(404)]
fn not_found(_req: &Request) -> JSON<Value> {
    JSON(json!({ "message": "Not Found" }))
}

#[error(500)]
fn server_error(_req: &Request) -> JSON<Value> {
    JSON(json!({ "message": "Internal Server Error" }))
}
