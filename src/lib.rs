// #![deny(warnings)]

extern crate clap;
extern crate mob_server;
extern crate rand;
extern crate reqwest;
extern crate termion;
extern crate serde;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;

pub mod cli;
pub mod error;
pub mod member;
pub mod prompt;
pub mod team;
pub mod timer;
pub mod tmux;

use clap::ArgMatches;
use mob_server::{db, web};
use std::error::Error as StdError;
use std::process::exit;
use std::thread::sleep;
use std::thread;
use std::time::Duration;

type Result<T> = std::result::Result<T, error::Error>;

pub fn run(matches: ArgMatches) -> Result<()> {
    match matches.subcommand() {
        ("prompt", Some(subcommand_matches)) => prompt::run(subcommand_matches),
        ("server", Some(_matches)) => {
            web::app(db::default_pool()).launch();
            Ok(())
        },
        ("start", Some(subcommand_matches)) => {
            thread::spawn(|| web::app(db::default_pool()).launch());
            sleep(Duration::from_millis(500));

            let mut team = team::create(subcommand_matches)?;
            timer::run(&mut team)
        },
        _ => unreachable!("Should not get here"),
    }
}

pub fn handle_error<E: StdError, T>(error: E) -> T {
    println!("{:?}", error);
    exit(1);
}
