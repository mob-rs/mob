#![deny(warnings)]

extern crate clap;

pub mod cli;
pub mod error;
pub mod tmux;

use clap::ArgMatches;
use std::error::Error as StdError;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

type Result<T> = std::result::Result<T, error::Error>;

pub fn run(matches: ArgMatches) -> Result<()> {
    let mut elapsed_time = 0;

    let names = matches.value_of("names");

    loop {
        if elapsed_time == 3 {
            tmux::flash_background()?;
        };

        println!("{:?}", names);
        println!("{:?}", elapsed_time);

        elapsed_time += 1;
        sleep(Duration::new(1, 0));
    }
}

pub fn handle_error<E: StdError, T>(error: E) -> T {
    println!("{:?}", error);
    exit(1);
}
