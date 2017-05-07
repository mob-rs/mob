// #![deny(warnings)]

extern crate clap;
extern crate mob_server;
extern crate rand;
extern crate reqwest;
extern crate termion;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

pub mod cli;
pub mod error;
pub mod prompt;
pub mod team;
pub mod tmux;

use clap::ArgMatches;
use mob_server::web;
use std::error::Error as StdError;
use std::process::exit;

type Result<T> = std::result::Result<T, error::Error>;

pub fn run(matches: ArgMatches) -> Result<()> {
    match matches.subcommand() {
        ("prompt", Some(subcommand_matches)) => prompt::run(subcommand_matches),
        ("server", Some(_matches)) => {
            web::app(None).launch();
            Ok(())
        },
        ("create", Some(subcommand_matches)) => team::create(subcommand_matches),
        _ => unreachable!("Should not get here"),
    }
}

pub fn handle_error<E: StdError, T>(error: E) -> T {
    println!("{:?}", error);
    exit(1);
}
