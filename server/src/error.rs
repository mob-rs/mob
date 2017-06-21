use diesel::result::Error as DieselError;
use rocket::http::{ContentType, Status};
use rocket::response::{Responder, Response};
use std::error::Error as StdError;
use std::fmt;
use std::io::Cursor;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum Error {
    Diesel(DieselError),
    ParseInt(ParseIntError),
}

use self::Error::*;

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Diesel(ref diesel_error) => diesel_error.description(),
            ParseInt(ref parse_int_error) => parse_int_error.description(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.description().fmt(f)
    }
}

impl<'r> Responder<'r> for Error {
    fn respond(self) -> Result<Response<'r>, Status> {
        match self {
            Diesel(ref diesel_error) => handle_diesel_error(diesel_error),
            ParseInt(ref _parse_int_error) => respond_with_500(),
        }
    }
}

fn handle_diesel_error<'r>(diesel_error: &DieselError) -> Result<Response<'r>, Status> {
    match *diesel_error {
        DieselError::NotFound => respond_with_404(),
        _ => respond_with_500(),
    }
}

fn respond_with_500<'r>() -> Result<Response<'r>, Status> {
    let body = json!({ "message": "Internal Server Error" }).to_string();
    build_response(body, Status::InternalServerError)
}

fn respond_with_404<'r>() -> Result<Response<'r>, Status> {
    let body = json!({ "message": "Not Found" }).to_string();
    build_response(body, Status::NotFound)
}

fn build_response<'r>(body: String, status: Status) -> Result<Response<'r>, Status> {
    Response::build()
        .status(status)
        .header(ContentType::JSON)
        .sized_body(Cursor::new(body))
        .ok()
}

impl From<DieselError> for Error {
    fn from(error: DieselError) -> Error {
        Diesel(error)
    }
}

impl From<ParseIntError> for Error {
    fn from(error: ParseIntError) -> Error {
        ParseInt(error)
    }
}

#[cfg(test)]
mod test {
    use diesel::result::Error as DieselError;
    use rocket::http::Status;
    use super::*;

    #[test]
    fn test_handle_diesel_error_not_found() {
        let not_found = DieselError::NotFound;

        let response = handle_diesel_error(&not_found).unwrap();

        assert_eq!(response.status(), Status::NotFound, "returns 404");
    }

    #[test]
    fn test_handle_diesel_error_catch_all() {
        let rollback_transaction = DieselError::RollbackTransaction;

        let response = handle_diesel_error(&rollback_transaction).unwrap();

        assert_eq!(response.status(), Status::InternalServerError, "returns 500");
    }

    #[test]
    fn test_respond_with_500() {
        let response = respond_with_500().unwrap();

        assert_eq!(response.status(), Status::InternalServerError, "returns 500");
    }

    #[test]
    fn test_respond_with_404() {
        let response = respond_with_404().unwrap();

        assert_eq!(response.status(), Status::NotFound, "returns 404");
    }
}
