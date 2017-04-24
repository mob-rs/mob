use std::thread::sleep;
use std::time::Duration;
use super::Result;
use team::{Member,Team};
use tmux;

pub fn run(time_per_driver_in_minutes: &f64, team: &mut Team) -> Result<()> {
    let time_per_driver_in_seconds = time_per_driver_in_minutes * 60.0;

    let mut elapsed_time = 0.0;

    loop {
        if is_time_for_next_driver(&time_per_driver_in_seconds, elapsed_time) {
            team.next_driver();
            tmux::flash_background()?;
            tmux::send_message(next_driver_message(&team.driver))?;
        };

        println!("{}", team.driver);
        elapsed_time += 1.0;
        sleep(Duration::from_secs(1))
    }
}

fn is_time_for_next_driver(time_per_driver: &f64, elapsed_time: f64) -> bool {
    if elapsed_time != 0.0 && elapsed_time % time_per_driver == 0.0 {
        true
    } else {
        false
    }
}

fn next_driver_message(driver: &Member) -> String {
    format!("Next Driver: {}", driver)
}
