use std::thread::sleep;
use std::time::Duration;
use super::Result;
use team::Team;
use tmux;

pub fn run(team: &mut Team) -> Result<()> {
    let mut elapsed_time = 0;

    loop {
        if timer_is_done(elapsed_time) {
            tmux::flash_background()?;
            team.next_driver();
        };

        println!("{}", team.driver);
        elapsed_time += 1;
        sleep(Duration::new(1, 0));
    }
}

fn timer_is_done(elapsed_time: i64) -> bool {
    if elapsed_time != 0 && elapsed_time % 3 == 0 {
        true
    } else {
        false
    }
}
