#[macro_use]
extern crate clap;
use clap::{App, AppSettings, Arg};

#[macro_use]
extern crate log;
extern crate simplelog;
use simplelog::*;
use std::fs::File;

fn setup_basic_loggers() {
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed).unwrap(),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create("vodka.log").unwrap(),
        ),
    ])
    .unwrap();
}

fn setup_verbose_loggers() {
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Trace, Config::default(), TerminalMode::Mixed).unwrap(),
        WriteLogger::new(
            LevelFilter::Trace,
            Config::default(),
            File::create("vodka.log").unwrap(),
        ),
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
        1 => setup_basic_loggers(),
        _ => setup_verbose_loggers(),
    }

    match vodka::run(matches) {
        Ok(_) => (),
        Err(e) => {
            error!("Application error: {}", e);
            std::process::exit(1)
        }
    }
}
