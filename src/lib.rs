#![deny(warnings)]

extern crate clap;

pub mod cli;
pub mod error;

use clap::ArgMatches;
use std::error::Error as StdError;
use std::process::exit;

type Result<T> = std::result::Result<T, error::Error>;

pub fn run(matches: ArgMatches) -> Result<()> {
    let names = matches.value_of("names");

    println!("{:?}", names);
    Ok(())
}

pub fn handle_error<E: StdError, T>(error: E) -> T {
    println!("{:?}", error);
    exit(1);
}
