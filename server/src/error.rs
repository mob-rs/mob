use diesel::result::Error as DieselError;
use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Diesel(DieselError),
}

use self::Error::*;

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Diesel(ref diesel_error) => diesel_error.description(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.description().fmt(f)
    }
}

impl From<DieselError> for Error {
    fn from(error: DieselError) -> Error {
        Diesel(error)
    }
}
