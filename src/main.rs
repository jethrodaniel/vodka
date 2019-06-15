#[macro_use]
extern crate clap;
use clap::{App, AppSettings, Arg};

fn main() {
    let matches = App::new(crate_name!())
        .setting(AppSettings::ArgRequiredElseHelp)
        .author(crate_authors!("\n"))
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::with_name("start")
                .index(1)
                .required(true)
                .help("Starts the program"),
        )
        .get_matches();

    unsafe {
        match vodka::run(matches) {
            Ok(_) => (),
            Err(e) => {
                println!("Application error: {}", e);
                std::process::exit(1)
            }
        }
    }
}
