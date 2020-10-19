
use cli::Cli;

mod cli;

#[test]
fn create_note_file() {
    let cli = Cli::new();
    let note_name = "foo/moo/bar";
    let note_path = cli.note_path(note_name);

    assert!(!note_path.exists(), "Note file exists before execution!");

    cli.create_note_file(note_name);

    assert!(note_path.exists(), "Note file was not created!");
}

#[test]
fn delete_note_file() {
    let cli = Cli::new();
    let note_name = "foo/moo/bar";
    let note_path = cli.note_path(note_name);

    cli.create_note_file(note_name);

    assert!(note_path.exists(), "Note file was not created!");

    cli.delete_note_file(note_name);

    assert!(!note_path.exists(), "Note file was not deleted!");
}
