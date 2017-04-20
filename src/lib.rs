#![deny(warnings)]

extern crate clap;
extern crate rand;

pub mod cli;
pub mod error;
pub mod team;
pub mod timer;
pub mod tmux;

use clap::ArgMatches;
use std::error::Error as StdError;
use std::process::exit;
use team::{Member,Team};

type Result<T> = std::result::Result<T, error::Error>;

pub fn run(matches: ArgMatches) -> Result<()> {
    let mut team = match matches.value_of("members") {
        Some(members_string) => {
            let members: Vec<Member> = members_string
                .split(",")
                .map(|string| string.to_owned() as Member)
                .collect();

            Team::new(members)
        }
        None => exit(1),
    };

    timer::run(&mut team)
}

pub fn handle_error<E: StdError, T>(error: E) -> T {
    println!("{:?}", error);
    exit(1);
}
