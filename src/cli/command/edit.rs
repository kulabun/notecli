use std::env::var;
use std::process::Command;

use clap::Clap;

use crate::note_manager::NoteManager;

#[derive(Clap)]
pub struct Edit {
    pub path: String,
    //TODO: add editor property
}

impl Edit {
    pub fn execute(&self) {
        let note_name = &self.path;

        let nm = NoteManager::new();

        if nm.is_name_correct(note_name) {
            panic!("Incorrect note name format!");
        }

        nm.touch_note(note_name);

        let path_to_note = nm.path_to_note(note_name);
        let editor = var("EDITOR").unwrap_or("vim".to_string());
        Command::new(&editor)
            .arg(path_to_note)
            .spawn()
            .expect(format!("Failed to spawn {}", editor).as_str())
            .wait()
            .unwrap_or_else(|_| panic!(format!("{} exit abnormally", editor)));
    }
}
