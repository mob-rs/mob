use errors::Result;
use std::process::{Command, ExitStatus};

pub fn new_window_with_command(command: &str) -> Result<ExitStatus> {
    Command::new("tmux")
        .arg("new-window")
        .arg(command)
        .status()
        .map_err(|error| error.into())
}
