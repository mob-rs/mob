use error::Error;
use std::process::{Command,ExitStatus};
use super::Result;

const RED: &'static str = "red";
const BLACK: &'static str = "black";

pub fn flash_background() -> Result<()> {
    set_background(RED)?;
    set_background(BLACK)?;
    set_background(RED)?;
    set_background(BLACK)?;
    Ok(())
}

fn set_background(color: &str) -> Result<ExitStatus> {
    Command::new("tmux")
        .arg("select-pane")
        .arg("-P")
        .arg(format!("'bg={}'", color))
        .status()
        .map_err(|error| Error::Io(error))
}
