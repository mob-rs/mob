use std::error::Error as StdError;
use std::fmt;
use std::io::Error as IoError;
use std::string::FromUtf8Error;
use std::num::ParseFloatError;

#[derive(Debug)]
pub enum Error {
    FromUtf8(FromUtf8Error),
    Io(IoError),
    ParseFloat(ParseFloatError),
}

use self::Error::*;

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            FromUtf8(ref from_utf8_error) => from_utf8_error.description(),
            Io(ref io_error) => io_error.description(),
            ParseFloat(ref parse_int_error) => parse_int_error.description(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.description().fmt(f)
    }
}

impl From<IoError> for Error {
    fn from(error: IoError) -> Error {
        Io(error)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(error: FromUtf8Error) -> Error {
        FromUtf8(error)
    }
}

impl From<ParseFloatError> for Error {
    fn from(error: ParseFloatError) -> Error {
        ParseFloat(error)
    }
}
