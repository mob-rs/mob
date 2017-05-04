use clap::{App, AppSettings, Arg, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
    let prompt_subcommand = SubCommand::with_name("prompt")
        .setting(AppSettings::Hidden)
        .arg(Arg::with_name("next_driver")
            .index(1)
            .required(true));

    let server_subcommand = SubCommand::with_name("server");

    App::new("history")
        .version("0.1")
        .setting(AppSettings::SubcommandsNegateReqs)
        .arg(Arg::with_name("members")
            .help("Names for mob")
            .index(1)
            .required(true))
        .arg(Arg::with_name("minutes")
            .help("Amount of time per driver")
            .takes_value(true)
            .long("minutes")
            .short("m"))
        .subcommand(prompt_subcommand)
        .subcommand(server_subcommand)
}
