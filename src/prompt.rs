use clap::ArgMatches;
use std::io::{self, Write};
use std::process::exit;
use super::Result;
use team;
use termion::color;

pub fn run(matches: &ArgMatches) -> Result<()> {
    let next_driver = matches.value_of("next_driver").expect("Next Driver");
    print_next_driver(&next_driver);
    print_continue()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    match input.trim().to_lowercase().as_ref() {
        "y" => team::update(next_driver)?,
        "n" => team::delete()?,
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
