use std::fs::{create_dir_all, File};
use std::path::PathBuf;

use assert_cmd::Command;
use predicates::str::contains;
use tempfile::tempdir;

use cli::Cli;

mod cli;

#[test]
fn test_delete_no_args() {
    let cli = Cli::new();
    cli.cmd(&["delete"])
        .assert()
        .stderr(contains("USAGE"))
        .stderr(contains("For more information try --help"))
        .failure();
}

#[test]
fn test_delete_with_path() {
    let cli = Cli::new();

    let note_name = "work/meetings/10102020";
    cli.create_note_file(note_name);

    cli.cmd(&["delete", note_name])
        .assert()
        .success();

    assert!(!cli.note_path(note_name).exists(), "note file was not delete!");
    assert!(cli.notes_root().read_dir().unwrap().next().is_none(), "folders created as part of note namespace was not removed");
}


#[test]
fn test_delete_with_shared_path() {
    let cli = Cli::new();
    let another_note = "work/meetings/20102020";
    cli.create_note_file(another_note);

    let note_name = "work/meetings/10102020";
    cli.create_note_file(note_name);

    cli.cmd(&["delete", note_name])
        .assert()
        .success();

    assert!(!cli.note_path(note_name).exists(), "note file was not delete!");
    assert!(cli.note_path(another_note).exists(), "another note file was delete!");
}

