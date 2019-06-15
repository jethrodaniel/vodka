use assert_cmd::prelude::*;
use std::env;
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

#[test]
fn get_display_env_var() {
    unsafe {
        match vodka::Display::new() {
            Ok(display) => {
                assert_eq!(display.display_env_var, ":0");
            }
            Err(msg) => panic!(msg),
        };
    }
}

#[test]
#[should_panic]
fn get_missing_display_env_var() {
    env::set_var("DISPLAY", "");
    get_display_env_var();
}
