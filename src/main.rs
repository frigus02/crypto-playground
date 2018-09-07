extern crate clap;
use clap::{App, AppSettings, Arg, SubCommand};

mod challenges;
mod encoding;

fn main() {
    let matches = App::new("Crypto Playground")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(
            SubCommand::with_name("hex-to-base64").arg(
                Arg::with_name("INPUT")
                    .help("input hex string")
                    .required(true)
                    .index(1),
            ),
        )
        .subcommand(
            SubCommand::with_name("fixed-xor")
                .arg(
                    Arg::with_name("INPUT1")
                        .help("input first hex string")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("INPUT2")
                        .help("input second hex string")
                        .required(true)
                        .index(2),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("hex-to-base64") {
        println!(
            "{}",
            challenges::hex_to_base64::run(matches.value_of("INPUT").unwrap())
        );
    } else if let Some(matches) = matches.subcommand_matches("fixed-xor") {
        println!(
            "{}",
            challenges::fixed_xor::run(
                matches.value_of("INPUT1").unwrap(),
                matches.value_of("INPUT2").unwrap()
            )
        );
    }
}
