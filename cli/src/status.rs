use clap::ArgMatches;
use client::Client;
use hostname::get_hostname;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;
use super::Result;
use team::TeamId;
use termion;

pub fn run<W: Write, C: Client>(matches: &ArgMatches, buffer: &mut W, client: &C) -> Result<()> {
    if let Some(interval_string) = matches.value_of("interval") {
        let interval = interval_string.parse::<u64>()?;
        loop {
            write!(buffer, "{}{}", termion::clear::All, termion::cursor::Goto(1,1))?;
            print_status(buffer, client)?;
            buffer.flush()?;
            sleep(Duration::from_secs(interval));
        }
    };

    print_status(buffer, client)?;
    Ok(())
}

fn print_status<W: Write, C: Client>(buffer: &mut W, client: &C) -> Result<()> {
    match client.fetch_team(TeamId::Hostname(hostname())) {
        Ok(team) => {
            write!(buffer, "Current Driver: {}", team.driver)?;
            Ok(())
        },
        Err(_error) => {
            write!(buffer, "No mob")?;
            Ok(())
        }
    }
}

fn hostname() -> String {
    get_hostname().expect("system to have a hostname")
}

#[cfg(test)]
mod test {
    use client::{Client, MockClient};
    use super::print_status;

    #[test]
    fn test_print_status() {
        let client = MockClient::new();
        let mut buffer = vec![];

        print_status(&mut buffer, &client).unwrap();

        let actual = String::from_utf8(buffer).unwrap();
        assert_eq!(actual, "Current Driver: Mike");
    }
}
