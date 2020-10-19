use assert_cmd::Command;
use predicates::str::contains;
use tempfile::tempdir;

use cli::Cli;
use std::fs::{File, OpenOptions};
use std::io::Write;

mod cli;


#[test]
fn test_show_no_args() {
    let cli = Cli::new();
    cli.cmd(&["show"])
        .assert()
        .stderr(contains("USAGE"))
        .stderr(contains("For more information try --help"))
        .failure();
}

#[test]
fn test_show_with_path() {
    let cli = Cli::new();
    let note_name = "work/meetings/10102020";

    cli.create_note_file(note_name);
    let mut file = OpenOptions::new()
        .write(true)
        .open(cli.note_path(note_name)).unwrap();
    let content = "Hello World!";
    file.write_all(content.as_bytes());
    file.flush();

    cli.cmd(&["show", note_name])
        .assert()
        .stdout(content)
        .success();
}

