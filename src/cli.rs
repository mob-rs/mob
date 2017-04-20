use clap::{App, Arg};

pub fn build_cli() -> App<'static, 'static> {
    App::new("history")
        .version("0.1")
        .arg(Arg::with_name("names")
             .help("Names for mob")
             .index(1)
             .required(true))
}
