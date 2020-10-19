use assert_cmd::Command;
use predicates::str::contains;

use cli::Cli;

mod cli;

#[test]
fn test_completion_no_args() {
    let cli = Cli::new();
    cli.cmd(&["completion"])
        .assert()
        .stderr(contains("USAGE"))
        .stderr(contains("For more information try --help"))
        .failure();
}

#[test]
fn test_completion_unsupported_shell() {
    let cli = Cli::new();
    cli.cmd(&["completion", "badsh"])
        .assert()
        .stderr(contains("USAGE"))
        .stderr(contains("For more information try --help"))
        .failure();
}

#[test]
fn test_completion_fish() {
    let cli = Cli::new();
    cli.cmd(&["completion", "fish"])
        .assert()
        .stdout(contains("complete -c notecli"))
        .success();
}

#[test]
fn test_completion_bash() {
    let cli = Cli::new();
    cli.cmd(&["completion", "bash"])
        .assert()
        .stdout(contains("compgen -W"))
        .success();
}

#[test]
fn test_completion_zsh() {
    let cli = Cli::new();
    cli.cmd(&["completion", "zsh"])
        .assert()
        .stdout(contains("#compdef notecli"))
        .success();
}

#[test]
fn test_completion_powershell() {
    let cli = Cli::new();
    cli.cmd(&["completion", "powershell"])
        .assert()
        .stdout(contains("[CompletionResult]::new("))
        .success();
}

#[test]
fn test_completion_elvish() {
    let cli = Cli::new();
    cli.cmd(&["completion", "elvish"])
        .assert()
        .stdout(contains("&'notecli;completion'= {"))
        .success();
}
