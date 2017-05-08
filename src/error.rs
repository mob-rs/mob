use reqwest::Error as ReqwestError;
use std::error::Error as StdError;
use std::fmt;
use std::io::Error as IoError;
use std::num::{ParseFloatError, ParseIntError};
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum Error {
    FromUtf8(FromUtf8Error),
    Http(ReqwestError),
    Io(IoError),
    ParseFloat(ParseFloatError),
    ParseInt(ParseIntError),
}

use self::Error::*;

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            FromUtf8(ref from_utf8_error) => from_utf8_error.description(),
            Http(ref reqwest_error) => reqwest_error.description(),
            Io(ref io_error) => io_error.description(),
            ParseFloat(ref parse_float_error) => parse_float_error.description(),
            ParseInt(ref parse_int_error) => parse_int_error.description(),
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

impl From<ParseIntError> for Error {
    fn from(error: ParseIntError) -> Error {
        ParseInt(error)
    }
}

impl From<ReqwestError> for Error {
    fn from(error: ReqwestError) -> Error {
        Http(error)
    }
}
