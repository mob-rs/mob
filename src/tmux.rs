use error::Error;
use std::process::{Command,ExitStatus};
use std::thread::sleep;
use std::time::Duration;
use super::Result;

const RED: &'static str = "red";
const BLACK: &'static str = "black";

pub fn flash_background() -> Result<()> {
    for _ in 0..3 {
        set_background(RED)?;
        sleep(Duration::from_millis(300));
        set_background(BLACK)?;
        sleep(Duration::from_millis(300));
    }

    Ok(())
}

fn set_background(color: &str) -> Result<ExitStatus> {
    Command::new("tmux")
        .arg("select-pane")
        .arg("-P")
        .arg(format!("bg={}", color))
        .status()
        .map_err(|error| Error::Io(error))
}
