use regex::Error as RegexError;
use reqwest::Error as ReqwestError;
use std::env::VarError;
use std::io::Error as IoError;
use std::num::{ParseFloatError, ParseIntError};
use std::string::FromUtf8Error;

error_chain! {
    foreign_links {
        Env(VarError);
        FromUtf8(FromUtf8Error);
        Http(ReqwestError);
        Io(IoError);
        ParseFloat(ParseFloatError);
        ParseInt(ParseIntError);
        Regex(RegexError);
    }
}
