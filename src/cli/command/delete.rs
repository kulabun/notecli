use clap::Clap;

use crate::note_manager::NoteManager;
use std::error;
use super::command;

#[derive(Clap)]
pub struct Delete {
    pub path: String,
}

impl command::Command for Delete {
    fn execute(&self) -> Result<(), Box<dyn error::Error>> {
        let note_name = &self.path;

        let nm = NoteManager::new();

        nm.verify_name_format(note_name)?;
        nm.verify_note_exists(note_name)?;
        nm.delete_note(note_name);

        Ok(())
    }
}

