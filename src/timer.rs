use member::Member;
use std::env::current_exe;
use std::thread::sleep;
use std::time::Duration;
use super::Result;
use team::{self, Team};
use tmux;

pub fn run(team: &mut Team) -> Result<()> {
    let time_per_driver_in_seconds = team.time * 60.0;

    let mut elapsed_time = 0.0;

    loop {
        let team = team::fetch()?;
        println!("{}", team.driver);

        if is_time_for_next_driver(&time_per_driver_in_seconds, elapsed_time) {
            prompt_user(&team)?;

            while current_driver()? != team.next_driver() {
                sleep(Duration::from_millis(500));
            }
        };

        elapsed_time += 1.0;
        sleep(Duration::from_secs(1))
    }
}

fn current_driver() -> Result<Member> {
    match team::fetch() {
        Ok(team) => Ok(team.driver),
        Err(_error) => {
            println!("Thanks for mobbing!");
            ::std::process::exit(0);
        }
    }
}

fn prompt_user(team: &Team) -> Result<()> {
    let bin = current_exe()?.to_str().expect("Binary path").to_owned();
    let next_driver = team.next_driver();

    let prompt_command = format!("{} prompt {}", bin, next_driver);
    tmux::new_window_with_command(&prompt_command)?;

    Ok(())
}

fn is_time_for_next_driver(time_per_driver: &f64, elapsed_time: f64) -> bool {
    elapsed_time != 0.0 && elapsed_time % time_per_driver == 0.0
}
