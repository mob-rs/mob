use client::Client;
use std::io::Write;
use super::Result;

pub fn run<W: Write, C: Client>(buffer: &mut W, client: &C) -> Result<()> {
    match client.fetch_team() {
        Ok(team) => {
            writeln!(buffer, "Current Driver: {}", team.driver)?;
            Ok(())
        },
        Err(_error) => {
            println!("No mob");
            Ok(())
        }
    }
}

#[cfg(test)]
mod test {
    use super::run;
    use client::{Client, MockClient};

    #[test]
    fn test_run() {
        let client = MockClient::new();
        let mut buffer = vec![];

        run(&mut buffer, &client).unwrap();

        let actual = String::from_utf8(buffer).unwrap();
        assert_eq!(actual, "Current Driver: Mike\n");
    }
}
