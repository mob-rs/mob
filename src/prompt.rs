use clap::ArgMatches;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use std::process::exit;
use super::Result;

pub fn run(matches: &ArgMatches) -> Result<()> {
    print_next_driver(matches);
    print_continue()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    write(&input.trim().to_lowercase())?;
    exit(0);
}

fn print_next_driver(matches: &ArgMatches) {
    let next_driver = matches.value_of("next_driver").expect("Next Driver");
    println!("Next Driver is: {}", next_driver);
}

fn print_continue() -> Result<()> {
    print!("Continue [y/n]? ");
    io::stdout().flush()?;
    Ok(())
}

fn write(input: &str) -> Result<()> {
    let path = Path::new("/tmp").join("mob");
    let mut file = File::create(path)?;
    file.write_all(input.as_bytes())?;
    Ok(())
}
