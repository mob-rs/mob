use clap::ArgMatches;
use client::Client;
use regex::Regex;
use std::io::{self, Write};
use std::process::exit;
use super::Result;
use termion::color;

pub fn run<C: Client>(matches: &ArgMatches, client: &C) -> Result<()> {
    let next_driver = matches.value_of("next_driver").expect("Next Driver");

    let previous_driver_id = matches
        .value_of("previous_driver_id")
        .expect("Previous Driver ID")
        .parse::<i32>()?;

    let next_driver_id = matches
        .value_of("next_driver_id")
        .expect("Next Driver ID")
        .parse::<i32>()?;

    let team_id = matches
        .value_of("team_id")
        .expect("Team ID")
        .parse::<i32>()?;

    print_next_driver(&next_driver);
    print_continue()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    if Regex::new("y")?.is_match(&input) {
        update_driver(client, previous_driver_id, next_driver_id)?;
    } else {
        client.delete_team(team_id)?;
    }

    exit(0);
}

fn print_next_driver(next_driver: &str) {
    println!("Next Driver is: {red}{next_driver}{reset}",
             red = color::Fg(color::Red),
             next_driver = next_driver,
             reset = color::Fg(color::Reset));
}

fn print_continue() -> Result<()> {
    print!("Continue [y/n]? ");
    io::stdout().flush()?;
    Ok(())
}

fn update_driver<C: Client>(client: &C, previous_driver_id: i32, next_driver_id: i32) -> Result<()> {
    client.update_member(previous_driver_id, false)?;
    client.update_member(next_driver_id, true)?;
    Ok(())
}
