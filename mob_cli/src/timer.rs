use std::env::current_exe;
use std::fs::{File, remove_file};
use std::io::Read;
use std::path::Path;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;
use super::Result;
use team::Team;
use tmux;

const MOB_FILE_PATH: &'static str = "/tmp/mob";

pub fn run(team: &mut Team) -> Result<()> {
    let time_per_driver_in_seconds = team.time * 60.0;

    let mut elapsed_time = 0.0;

    loop {
        println!("{}", team.driver);
        if is_time_for_next_driver(&time_per_driver_in_seconds, elapsed_time) {
            prompt_user(team)?;
        };

        elapsed_time += 1.0;
        sleep(Duration::from_secs(1))
    }
}

fn prompt_user(team: &mut Team) -> Result<()> {
    let bin = current_exe()?.to_str().expect("Binary path").to_owned();
    let next_driver = team.next_driver();

    let prompt_command = format!("{} prompt {}", bin, next_driver);
    let exit_status = tmux::new_window_with_command(&prompt_command)?;

    wait_for_file();

    if exit_status.success() && is_continue()? {
        team.change_driver(&next_driver);
        Ok(())
    } else {
        exit(1);
    }
}

fn wait_for_file() {
    let path = Path::new(MOB_FILE_PATH);

    while !path.exists() {
        sleep(Duration::from_millis(500));
    }
}

fn is_continue() -> Result<bool> {
    let path = Path::new(MOB_FILE_PATH);

    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    remove_file(path)?;

    Ok(contents == "y")
}

fn is_time_for_next_driver(time_per_driver: &f64, elapsed_time: f64) -> bool {
    if elapsed_time != 0.0 && elapsed_time % time_per_driver == 0.0 {
        true
    } else {
        false
    }
}
