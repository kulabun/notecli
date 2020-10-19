use std::ffi;
use std::fs::{create_dir_all, File, remove_file};
use std::path::PathBuf;

use assert_cmd::Command;
use regex::Regex;
use tempfile::tempdir;

pub struct Cli {
    home_dir: PathBuf,
    data_dir: PathBuf,
    config_dir: PathBuf,
    notes_root: PathBuf,
    editor: String,
    pager: String,
    note_file_name: String,
}

impl Cli {
    pub fn new() -> Cli {
        let home = tempdir().unwrap().into_path();
        let data = home.to_owned().join(".local").join("share");
        let config = home.to_owned().join(".config");
        let notes_root = data.to_owned().join(clap::crate_name!()).join("notes");

        return Cli {
            home_dir: home,
            data_dir: data,
            config_dir: config,
            notes_root: notes_root,
            editor: "echo".to_string(),
            pager: "cat".to_string(),
            note_file_name: "@note".to_string(),
        };
    }

    pub fn cmd<I, S>(&self, args: I) -> Command
        where
            I: IntoIterator<Item=S>,
            S: AsRef<ffi::OsStr>,
    {
        let mut command: Command = Command::cargo_bin(clap::crate_name!()).unwrap();

        command.args(args)
            .env_clear()
            .env("HOME", self.home_dir.to_str().unwrap())
            .env("XDG_DATA_HOME", self.data_dir.to_str().unwrap())
            .env("EDITOR", self.editor.as_str())
            .env("PAGER", self.pager.as_str());

        return command;
    }

    pub fn note_path(&self, note_name: &str) -> PathBuf {
        let mut path = self.notes_root.to_owned();
        for elem in note_name.split("/") {
            path = path.join(elem);
        }
        return path.join(self.note_file_name.as_str());
    }

    pub fn create_note_file(&self, note_name: &str) {
        let note_path = self.note_path(note_name);

        create_dir_all(note_path.parent().unwrap())
            .unwrap_or_else(|_| panic!("Failed to create note directory"));
        File::create(note_path.as_path())
            .unwrap_or_else(|_| panic!("Failed to create note file"));
    }

    pub fn create_note_files<I, S>(&self, note_names: I)
        where
            I: IntoIterator<Item=S>,
            S: AsRef<str>,
    {
        for note_name in note_names {
            self.create_note_file(note_name.as_ref());
        }
    }

    pub fn delete_note_file(&self, note_name: &str) {
        let note_path = self.note_path(note_name);

        remove_file(note_path.as_path())
            .unwrap_or_else(|_| panic!("Failed to delete note file"));
    }

    pub fn notes_root(&self) -> PathBuf {
        return self.notes_root.to_owned();
    }
}
