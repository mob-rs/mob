#![deny(warnings)]

extern crate mob;

use mob::*;

fn main() {
    let matches = cli::build_cli().get_matches();

    run(matches).unwrap_or_else(handle_error);
}
