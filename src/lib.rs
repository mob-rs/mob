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
pub mod client;
pub mod error;
pub mod member;
pub mod prompt;
pub mod status;
pub mod team;
pub mod timer;
pub mod tmux;

use clap::ArgMatches;
use client::{Client, HttpClient};
use mob_server::{db, web};
use std::error::Error as StdError;
use std::io;
use std::process::exit;
use std::thread::sleep;
use std::thread;
use std::time::Duration;

type Result<T> = std::result::Result<T, error::Error>;

pub fn run(matches: ArgMatches) -> Result<()> {
    let client = HttpClient::new();

    let mut stdout = io::stdout();

    match matches.subcommand() {
        ("prompt", Some(subcommand_matches)) => {
            prompt::run(subcommand_matches, &client)
        },
        ("status", Some(_matches)) => status::run(&mut stdout, &client),
        ("start", Some(subcommand_matches)) => {
            thread::spawn(|| web::app(db::default_pool()).launch());
            sleep(Duration::from_millis(500));

            let mut team = team::create(subcommand_matches, &client)?;
            timer::run(&mut team, &client)
        },
        _ => unreachable!("Should not get here"),
    }
}

pub fn handle_error<E: StdError, T>(error: E) -> T {
    println!("{:?}", error);
    exit(1);
}
