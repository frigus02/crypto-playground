extern crate clap;
use clap::{Arg, App, AppSettings, SubCommand};

mod hex_to_base64;

fn main() {
    let matches = App::new("Crypto Playground")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(SubCommand::with_name("hex-to-base64")
            .arg(Arg::with_name("INPUT")
                .help("input hex string")
                .required(true)
                .index(1)))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("hex-to-base64") {
        println!("{}", hex_to_base64::run(matches.value_of("INPUT").unwrap()));
    }
}
