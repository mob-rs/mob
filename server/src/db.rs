use diesel::pg::PgConnection;
use dotenv::dotenv;
use r2d2;
use r2d2_diesel::ConnectionManager;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
use std::env;
use std::ops::Deref;

embed_migrations!("migrations");

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn default_pool() -> Pool {
    let config = r2d2::Config::default();
    let manager = ConnectionManager::<PgConnection>::new(database_url());
    let pool = r2d2::Pool::new(config, manager).expect("db pool");

    let conn = pool.get().unwrap();
    embedded_migrations::run(conn.deref()).unwrap();

    pool
}

pub fn database_url() -> String {
    dotenv().ok();

    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub struct Conn(r2d2::PooledConnection<ConnectionManager<PgConnection>>);

impl Deref for Conn {
    type Target = PgConnection;

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
