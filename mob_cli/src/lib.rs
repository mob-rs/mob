#![deny(warnings)]

extern crate clap;
extern crate mob_server;
extern crate rand;
extern crate termion;

pub mod cli;
pub mod error;
pub mod prompt;
pub mod team;
pub mod timer;
pub mod tmux;

use clap::ArgMatches;
use mob_server::web;
use std::error::Error as StdError;
use std::process::exit;
use team::{Member, Team};

type Result<T> = std::result::Result<T, error::Error>;

pub fn run(matches: ArgMatches) -> Result<()> {
    match matches.subcommand() {
        ("prompt", Some(subcommand_matches)) => prompt::run(subcommand_matches),
        ("server", Some(_matches)) => {
            web::app(None).launch();
            Ok(())
        },
        _ => timer(&matches),
    }
}

fn timer(matches: &ArgMatches) -> Result<()> {
    let time_per_driver_in_minutes = matches.value_of("minutes")
        .map(|minutes| minutes.parse::<f64>())
        .unwrap_or(Ok(5.0))?;

    let mut team = match matches.value_of("members") {
        Some(members_string) => {
            let members: Vec<Member> = members_string.split(",")
                .map(|string| string.to_owned())
                .collect::<Vec<Member>>();

            Team::new(members)
        }
        None => exit(1),
    };

    timer::run(&time_per_driver_in_minutes, &mut team)
}

pub fn handle_error<E: StdError, T>(error: E) -> T {
    println!("{:?}", error);
    exit(1);
}
