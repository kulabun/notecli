use assert_cmd::Command;
use predicates::str::contains;
use tempfile::tempdir;

use cli::Cli;

mod cli;


#[test]
fn test_create_no_args() {
    let cli = Cli::new();
    cli.cmd(&["create"])
        .assert()
        .stderr(contains("USAGE"))
        .stderr(contains("For more information try --help"))
        .failure();
}

#[test]
fn test_create_with_path() {
    let cli = Cli::new();
    let note_name = "work/meetings/10102020";
    let note_path = cli.note_path(note_name);

    assert!(!note_path.exists(), "note file exists before test execution!");

    cli.cmd(&["create", note_name])
        .assert()
        .stdout(contains(note_path.to_str().unwrap()))
        .success();

    assert!(note_path.exists(), "note file was not created!");
}

#[test]
fn test_create_with_exiting_path() {
    let cli = Cli::new();
    let note_name = "work/meetings/10102020";

    cli.create_note_file(note_name);

    cli.cmd(&["create", note_name])
        .assert()
        .stderr(contains("Note already exists!"))
        .failure();
}
