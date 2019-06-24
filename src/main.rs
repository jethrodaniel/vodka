use std::fs::File;

#[macro_use]
extern crate clap;
use clap::{App, AppSettings, Arg};

#[macro_use]
extern crate log;
extern crate simplelog;
use simplelog::{CombinedLogger, Config, LevelFilter, TermLogger, TerminalMode, WriteLogger};

fn setup_loggers(level: LevelFilter) {
    CombinedLogger::init(vec![
        TermLogger::new(level, Config::default(), TerminalMode::Mixed).unwrap(),
        WriteLogger::new(level, Config::default(), File::create("vodka.log").unwrap()),
    ])
    .unwrap();
}

fn main() {
    let matches = App::new(crate_name!())
        .setting(AppSettings::ArgRequiredElseHelp)
        .author(crate_authors!("\n"))
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::with_name("verbose")
                .help("Enable verbose output")
                .multiple(true)
                .short("v"),
        )
        .arg(
            Arg::with_name("start")
                .index(1)
                .required(true)
                .help("Starts the program"),
        )
        .get_matches();

    match matches.occurrences_of("verbose") {
        0 => (),
        1 => setup_loggers(LevelFilter::Info),
        _ => setup_loggers(LevelFilter::Trace),
    }

    match vodka::run(matches) {
        Ok(_) => (),
        Err(e) => {
            error!("Application error: {}", e);
            std::process::exit(1)
        }
    }
}
