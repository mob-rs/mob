#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate dotenv;

use self::diesel::pg::PgConnection;
use self::diesel::Connection;
use self::dotenv::dotenv;
use std::env;

embed_migrations!("migrations");

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = PgConnection::establish(&database_url).unwrap();
    embedded_migrations::run(&conn).unwrap();
}
