use std::env::var;
use std::error;
use std::process;
use super::command;

use clap::Clap;

use crate::note_manager::NoteManager;

#[derive(Clap)]
pub struct Create {
    pub path: String,
    //TODO: add editor property
}

impl command::Command for Create {
    fn execute(&self) -> Result<(), Box<dyn error::Error>> {
        let note_name = &self.path;

        let nm = NoteManager::new();

        nm.create_note(note_name)?;

        let path_to_note = nm.path_to_note(note_name);
        let editor = var("EDITOR").unwrap_or("vim".to_string());
        process::Command::new(&editor)
            .arg(path_to_note)
            .spawn()
            .expect(format!("Failed to spawn {}", editor).as_str())
            .wait()
            .unwrap_or_else(|_| panic!(format!("{} exit abnormally", editor)));

        Ok(())
    }
}

