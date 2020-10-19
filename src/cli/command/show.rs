use clap::Clap;

use crate::note_manager::NoteManager;
use std::process::Command;
use std::env::var;

#[derive(Clap)]
pub struct Show {
    pub path: String,
    //TODO: add show cmd
}

impl Show {
    pub fn execute(&self) {
        let note_name = &self.path;

        let nm = NoteManager::new();

        if nm.is_name_correct(note_name) {
            panic!("Incorrect note name format!");
        }

        if !nm.note_exists(note_name) {
            panic!("Note not found!");
        }

        let path_to_note = nm.path_to_note(note_name);

        let pager = var("PAGER").unwrap_or("less".to_string());
        Command::new(&pager)
            .arg(path_to_note)
            .spawn()
            .expect(format!("Failed to spawn {}", pager).as_str())
            .wait()
            .unwrap_or_else(|_| panic!(format!("{} exit abnormally", pager)));
    }
}
