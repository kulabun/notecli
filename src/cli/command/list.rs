use std::error;
use super::command;

use clap::Clap;

use crate::note_manager::NoteManager;

#[derive(Clap)]
pub struct List {
    pub path: Option<String>,
}

impl command::Command for List {
    fn execute(&self) -> Result<(), Box<dyn error::Error>> {
        let nm = NoteManager::new();
        let maybe_path = self.path.as_ref().map(|p| p.to_string());
        nm.list_notes(maybe_path)?;

        Ok(())
    }
}
