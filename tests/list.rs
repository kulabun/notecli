use assert_cmd::Command;
use predicates::str::contains;
use tempfile::tempdir;

use cli::Cli;

mod cli;


#[test]
fn test_list_no_args() {
    let cli = Cli::new();
    let notes = [
        "personal/js/notes",
        "personal/rust/notes",
        "personal/ts/notes",
        "work/meetings/10102020",
        "work/meetings/11102020",
        "work/meetings/12102020",
        "work/meetings/13102020",
    ].to_vec();
    cli.create_note_files(notes.to_owned());

    let notes_list = join(notes);
    cli.cmd(&["list"])
        .assert()
        .stdout(notes_list.to_owned())
        .success();
}

#[test]
fn test_list_with_path() {
    let cli = Cli::new();
    let work_notes = [
        "work/meetings/10102020",
        "work/meetings/11102020",
        "work/meetings/12102020",
        "work/meetings/13102020",
    ].to_vec();
    cli.create_note_files(work_notes.to_owned());

    let personal_notes = [
        "personal/rust/notes",
        "personal/js/notes",
        "personal/ts/notes",
    ].to_vec();
    cli.create_note_files(personal_notes.to_owned());

    let notes_list = join(work_notes);
    cli.cmd(&["list", "work/meetings"])
        .assert()
        .stdout(notes_list.to_owned())
        .success();

    cli.cmd(&["list", "work"])
        .assert()
        .stdout(notes_list.to_owned())
        .success();
}

fn join(work_notes: Vec<&str>) -> String {
    let mut notes_list = "".to_string();
    for note in work_notes.to_owned() {
        notes_list.push_str(note);
        notes_list.push_str("\n");
    }
    notes_list
}

