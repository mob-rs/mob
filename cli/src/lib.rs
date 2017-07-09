#![deny(warnings)]

extern crate clap;
#[macro_use] extern crate error_chain;
extern crate mob_server;
extern crate rand;
extern crate regex;
extern crate reqwest;
extern crate serde;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate termion;

pub mod cli;
pub mod client;
pub mod errors;
pub mod member;
pub mod prompt;
pub mod status;
pub mod team;
pub mod timer;
pub mod tmux;

use clap::ArgMatches;
use client::{Client, HttpClient};
use errors::{Error, Result};
use mob_server::{db, web};
use std::io::Write;
use std::io;
use std::process::exit;
use std::thread::sleep;
use std::thread;
use std::time::Duration;

pub fn run(matches: ArgMatches) -> Result<()> {
    let client = HttpClient::new();

    let mut stdout = io::stdout();

    match matches.subcommand() {
        ("prompt", Some(subcommand_matches)) => {
            prompt::run(subcommand_matches, &client)
        },
        ("status", Some(subcommand_matches)) => status::run(subcommand_matches, &mut stdout, &client),
        ("start", Some(subcommand_matches)) => {
            thread::spawn(|| web::app(db::default_pool()).launch());
            sleep(Duration::from_millis(500));
            let mut team = team::create(subcommand_matches, &client)?;
            timer::run(&mut team, &client)
        },
        _ => unreachable!("Should not get here"),
    }
}

pub fn handle_error(error: Error) {
    let stderr = &mut ::std::io::stderr();
    let errmsg = "Error writing to stderr";

    writeln!(stderr, "error: {}", error).expect(errmsg);

    for e in error.iter().skip(1) {
        writeln!(stderr, "caused by: {}", e).expect(errmsg);
    }

    // The backtrace is not always generated. Try to run this example
    // with `RUST_BACKTRACE=1`.
    if let Some(backtrace) = error.backtrace() {
        writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
    }

    exit(1);
}
