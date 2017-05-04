#![deny(warnings)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate dotenv;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
#[cfg(not(test))]
use dotenv::dotenv;
#[cfg(not(test))]
use std::env;
use std::error::Error as StdError;
use std::process::exit;
#[cfg(test)]
use std::io;

type Result<T> = std::result::Result<T, error::Error>;

pub fn handle_error<E: StdError, T>(error: E) -> T {
    println!("{:?}", error);
    exit(1);
}

#[cfg(not(test))]
pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let home_path = env::home_dir().expect("Home Dir to exist");

    let database_url = home_path
        .join(".history.sql")
        .to_str()
        .unwrap()
        .to_owned();

    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[cfg(test)]
pub fn establish_connection() -> SqliteConnection {
    let connection = SqliteConnection::establish(":memory:").unwrap();
    let migrations_dir = diesel::migrations::find_migrations_directory().unwrap();
    diesel::migrations::run_pending_migrations_in_directory(&connection, &migrations_dir, &mut io::sink()).unwrap();

    connection
}
