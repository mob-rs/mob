use clap::{App, Arg};

pub fn build_cli() -> App<'static, 'static> {
    App::new("history")
        .version("0.1")
        .arg(Arg::with_name("members")
             .help("Names for mob")
             .index(1)
             .required(true))
        .arg(Arg::with_name("minutes")
             .help("Amount of time per driver")
             .takes_value(true)
             .long("minutes")
             .short("m"))
}
