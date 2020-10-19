use assert_cmd::Command;
use predicates::str::contains;

use cli::Cli;
use std::ffi::OsString;
use std::iter;

mod cli;

#[test]
fn test_no_args() {
    let cli = Cli::new();
    cli.cmd(iter::empty::<&str>())
        .assert()
        .stderr(contains("USAGE"))
        .stderr(contains("FLAGS"))
        .stderr(contains("SUBCOMMANDS"))
        .failure();
}

#[test]
fn test_help() {
    let cli = Cli::new();
    cli.cmd(&["help"])
        .assert()
        .stdout(contains("USAGE"))
        .stdout(contains("FLAGS"))
        .stdout(contains("SUBCOMMANDS"))
        .success();
}

