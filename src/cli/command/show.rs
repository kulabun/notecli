use std::env::var;
use std::error;
use std::process;

use clap::Clap;

use crate::note_manager::NoteManager;

use super::command;

#[derive(Clap)]
pub struct Show {
    pub path: String,
    //TODO: add show cmd
}

impl command::Command for Show {
    fn execute(&self) -> Result<(), Box<dyn error::Error>> {
        let note_name = &self.path;

        let nm = NoteManager::new();

        nm.verify_name_format(note_name)?;
        nm.verify_note_exists(note_name)?;

        let path_to_note = nm.path_to_note(note_name);

        let pager = var("PAGER").unwrap_or("less".to_string());
        process::Command::new(&pager)
            .arg(path_to_note)
            .spawn()
            .expect(format!("Failed to spawn {}", pager).as_str())
            .wait()
            .unwrap_or_else(|_| panic!(format!("{} exit abnormally", pager)));

        Ok(())
    }
}
