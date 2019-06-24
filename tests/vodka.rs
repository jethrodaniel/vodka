use std::env;
use std::process::Command;

use assert_cmd::prelude::*;

#[test]
fn calling_with_no_args() {
    // Clap exits with 1 in case no arguments were provided
    Command::cargo_bin("vodka").unwrap().assert().failure();
}

#[test]
#[should_panic]
fn calling_start_with_bad_display() {
    env::set_var("DISPLAY", ":99");
    Command::cargo_bin("vodka start").unwrap().assert().failure();
}
