use clap::Clap;

use crate::note_manager::NoteManager;

#[derive(Clap)]
pub struct Delete {
    pub path: String,
}

impl Delete {
    pub fn execute(&self) {
        let note_name = &self.path;

        let nm = NoteManager::new();

        if nm.is_name_correct(note_name) {
            panic!("Incorrect note name format!");
        }

        if !nm.note_exists(note_name) {
            panic!("Note not found!");
        }

        nm.delete_note(note_name);
    }
}

