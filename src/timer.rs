use std::thread::sleep;
use std::time::Duration;
use super::Result;
use team::Team;
use tmux;

pub fn run(time_per_driver_in_minutes: &i64, team: &mut Team) -> Result<()> {
    let time_per_driver_in_seconds = time_per_driver_in_minutes * 60;

    let mut elapsed_time = 0;

    loop {
        if is_time_for_next_driver(&time_per_driver_in_seconds, elapsed_time) {
            tmux::flash_background()?;
            team.next_driver();
        };

        println!("{}", team.driver);
        elapsed_time += 1;
        sleep(Duration::from_secs(1))
    }
}

fn is_time_for_next_driver(time_per_driver: &i64, elapsed_time: i64) -> bool {
    if elapsed_time != 0 && elapsed_time % time_per_driver == 0 {
        true
    } else {
        false
    }
}
