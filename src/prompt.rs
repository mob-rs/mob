use clap::ArgMatches;
use client::Client;
use std::io::{self, Write};
use std::process::exit;
use super::Result;
use termion::color;

pub fn run<C: Client>(matches: &ArgMatches, client: &C) -> Result<()> {
    let next_driver = matches.value_of("next_driver").expect("Next Driver");

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

    match input.trim().to_lowercase().as_ref() {
        "y" => client.update_team(team_id, next_driver_id)?,
        "n" => client.delete_team(team_id)?,
        _ => {
            println!("Invalid input");
            exit(1);
        }
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
