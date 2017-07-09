use diesel::result::Error as DieselError;
use rocket::Request;
use rocket::http::{ContentType, Status};
use rocket::response::{Responder, Response};
use std::io::Cursor;
use std::num::ParseIntError;
use std::result::Result as StdResult;

error_chain! {
    foreign_links {
        Diesel(DieselError);
        ParseInt(ParseIntError);
    }
}

use self::ErrorKind::*;

impl<'r> Responder<'r> for Error {
    fn respond_to(self, _: &Request) -> StdResult<Response<'r>, Status> {
        match *self.kind() {
            Diesel(ref diesel_error) => handle_diesel_error(diesel_error),
            ParseInt(ref _parse_int_error) => respond_with_500(),
            _ => respond_with_500(),
        }
    }
}

fn handle_diesel_error<'r>(diesel_error: &DieselError) -> StdResult<Response<'r>, Status> {
    match *diesel_error {
        DieselError::NotFound => respond_with_404(),
        _ => respond_with_500(),
    }
}

fn respond_with_500<'r>() -> StdResult<Response<'r>, Status> {
    let body = json!({ "message": "Internal Server Error" }).to_string();
    build_response(body, Status::InternalServerError)
}

fn respond_with_404<'r>() -> StdResult<Response<'r>, Status> {
    let body = json!({ "message": "Not Found" }).to_string();
    build_response(body, Status::NotFound)
}

fn build_response<'r>(body: String, status: Status) -> StdResult<Response<'r>, Status> {
    Response::build()
        .status(status)
        .header(ContentType::JSON)
        .sized_body(Cursor::new(body))
        .ok()
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
