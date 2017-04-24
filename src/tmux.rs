use error::Error;
use std::process::{Command,ExitStatus};
use std::thread::sleep;
use std::thread;
use std::time::Duration;
use super::Result;

const RED: &'static str = "red";
const BLACK: &'static str = "black";

type Pane = String;

pub fn flash_background() -> Result<()> {
    let mut handlers = Vec::new();

    for pane in list_panes()? {
        handlers.push(thread::spawn(move || {
            flash_background_for_pane(&pane).unwrap();
        }))
    };

    for handler in handlers {
        handler.join().unwrap();
    }

    Ok(())
}

fn flash_background_for_pane(pane: &Pane) -> Result<()> {
    for _ in 0..3 {
        set_background(pane, RED)?;
        sleep(Duration::from_millis(300));
        set_background(pane, BLACK)?;
        sleep(Duration::from_millis(300));
    }

    Ok(())
}

fn set_background(pane: &Pane, color: &str) -> Result<ExitStatus> {
    Command::new("tmux")
        .arg("select-pane")
        .arg("-t")
        .arg(pane)
        .arg("-P")
        .arg(format!("bg={}", color))
        .status()
        .map_err(|error| Error::Io(error))
}

fn list_panes() -> Result<Vec<Pane>> {
    let output = Command::new("tmux").arg("list-panes").arg("-a").output()?;
    let raw = String::from_utf8(output.stdout)?;
    Ok(parse_panes(&raw))
}

fn parse_panes(raw: &str) -> Vec<Pane> {
    raw
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line[2..5].to_owned() as Pane)
        .collect()
}

#[cfg(test)]
mod test {
    use super::parse_panes;

    #[test]
    fn test_parse_panes() {
        let raw = "
0:1.1: [238x58] [history 21/100000, 14403 bytes] %0 (active)
0:2.1: [238x29] [history 23/100000, 20204 bytes] %1
0:2.2: [238x28] [history 3/100000, 2744 bytes] %3 (active)";
        let expected = vec!["1.1", "2.1", "2.2"];
        let actual = parse_panes(raw);

        assert_eq!(expected, actual);
    }
}
