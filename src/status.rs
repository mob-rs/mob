use client::Client;
use std::io::Write;
use super::Result;

pub fn run<W: Write, C: Client>(buffer: &mut W, client: &C) -> Result<()> {
    let team = client.fetch_team()?;

    writeln!(buffer, "Current Driver: {}", team.driver)?;
    Ok(())
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
