use diesel::sqlite::SqliteConnection;
use r2d2;
use r2d2_diesel::ConnectionManager;

use std::ops::Deref;

use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};

embed_migrations!("migrations");

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn default_pool() -> Pool {
    let config = r2d2::Config::default();
    let manager = ConnectionManager::<SqliteConnection>::new(database_url());
    let pool = r2d2::Pool::new(config, manager).expect("db pool");

    let connection = pool.get().unwrap();
    embedded_migrations::run(connection.deref()).unwrap();

    pool
}

#[cfg(not(test))]
fn database_url() -> String {
    use std::env;

    let home_path = env::home_dir().expect("Home Dir to exist");

    home_path
        .join(".mob.sql")
        .to_str()
        .unwrap()
        .to_owned()

}

#[cfg(test)]
fn database_url() -> String {
    use uuid::Uuid;

    format!("file:{}.db?mode=memory&cache=shared", Uuid::new_v4())
}

pub struct Conn(r2d2::PooledConnection<ConnectionManager<SqliteConnection>>);

impl Deref for Conn {
    type Target = SqliteConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Conn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Conn, ()> {
        let pool = match <State<Pool> as FromRequest>::from_request(request) {
            Outcome::Success(pool) => pool,
            Outcome::Failure(e) => return Outcome::Failure(e),
            Outcome::Forward(_) => return Outcome::Forward(()),
        };

        match pool.get() {
            Ok(conn) => Outcome::Success(Conn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}
