use clap::ArgMatches;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use super::Result;

pub fn run(matches: &ArgMatches) -> Result<()> {
    let next_driver = matches.value_of("next_driver").expect("Next Driver");

    write(next_driver)?;

    Ok(())
}

fn write(input: &str) -> Result<()> {
    let path = Path::new("/tmp").join("mob");
    let mut file = File::create(path)?;
    file.write_all(input.as_bytes())?;
    Ok(())
}
