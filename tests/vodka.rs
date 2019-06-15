use clap::{App, Arg};

use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn vodka() {
    // Clap exits with 1 in case no arguments were provided
    Command::cargo_bin("vodka").unwrap().assert().failure();
}

#[test]
fn display_init() {
    unsafe {
        match vodka::Display::new() {
            Ok(display) => {
                assert_eq!(display.display.is_null(), false);
            }
            Err(msg) => panic!(msg),
        };
    }
}
