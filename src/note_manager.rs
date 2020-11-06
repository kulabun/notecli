use std::fs::{self, File};
use std::path::PathBuf;
use regex::Regex;

use walkdir::WalkDir;
use std::error;
use core::fmt;

const NOTE_FILE_NAME: &str = "@note";

pub struct NoteManager {
    note_file_name: String,
    notes_root: PathBuf,
}

#[derive(Debug)]
struct NoteCliError {
    message: &'static str
}

impl fmt::Display for NoteCliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl NoteCliError {
    fn new(message: &'static str) -> NoteCliError {
        NoteCliError {
            message
        }
    }
}

impl error::Error for NoteCliError {}

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

impl NoteManager {
    pub fn new() -> NoteManager {
        return NoteManager {
            note_file_name: NOTE_FILE_NAME.to_string(),
            notes_root: notes_dir(),
        };
    }

    pub fn verify_name_format(&self, note_name: &str) -> Result<()> {
        let first_char_pattern = regex::Regex::new("^([a-zA-Z0-9][^/]*)(/[a-zA-Z0-9][^/]*)*$")?;
        if !first_char_pattern.is_match(note_name) {
            return Err(Box::new(NoteCliError::new("Note name elements must start with alphanumeric character!")));
        }

        let name_characters = regex::Regex::new("^[a-zA-Z0-9][a-zA-Z0-9\\-_\\.@#]*(/[a-zA-Z0-9][a-zA-Z0-9\\-_\\.@#]*)*$")?;
        if !name_characters.is_match(note_name) {
            return Err(Box::new(NoteCliError::new("Note name elements are allowed to contain only following symbols: '-', '_', '@', '.', '#'")));
        }

        Ok(())
    }

    pub fn verify_note_exists(&self, note_name: &str) -> Result<()> {
        if self.note_exists(note_name) {
            Ok(())
        } else {
            Err(Box::new(NoteCliError::new("Note")))
        }
    }

    pub fn note_exists(&self, note_name: &str) -> bool {
        return self.path_to_note(note_name).exists();
    }

    pub fn touch_note(&self, note_name: &str) -> Result<()> {
        self.verify_name_format(note_name)?;

        if !self.note_exists(note_name) {
            self.create_note(note_name)?;
        }

        Ok(())
    }

    pub fn create_note(&self, note_name: &str) -> Result<()> {
        self.verify_name_format(note_name)?;
        if self.note_exists(note_name) {
            return Err(Box::new(NoteCliError::new("Note already exists!")));
        };

        let note_file = self.path_to_note(note_name);
        let parent = self.get_parent(&note_file)?;
        fs::create_dir_all(parent)?;
        File::create(note_file.as_path())?;

        Ok(())
    }

    fn get_parent(&self, child: &PathBuf) -> Result<PathBuf> {
        let parent_path = child.parent()
            .ok_or(NoteCliError::new("Failed to resolve note's parent directory"))?
            .to_path_buf();

        Ok(parent_path)
    }

    pub fn delete_note(&self, note_name: &str) -> Result<()> {
        self.verify_note_exists(note_name);

        let note_path = self.path_to_note(note_name);

        fs::remove_file(note_path.as_path())?;

        let mut parent = self.get_parent(&note_path)?;
        while parent != self.notes_root && parent.read_dir()?.next().is_none() {
            fs::remove_dir(parent.as_path())?;
            parent = self.get_parent(&parent)?;
        }

        Ok(())
    }

    pub fn path_to_note(&self, note_name: &str) -> PathBuf {
        let mut path = self.notes_root.join(note_name);
        for elem in self.note_file_name.split("/") {
            path = path.join(elem);
        }
        return path;
    }

    pub fn list_notes(&self, maybe_prefix: Option<String>) -> Result<()> {
        let root = match maybe_prefix {
            Some(p) => self.notes_root.as_path().join(p),
            None => self.notes_root.to_path_buf(),
        };

        for entry in WalkDir::new(root)
            .sort_by(|a, b| a.file_name().cmp(b.file_name()))
        {
            let absolute_path = entry.unwrap().path().to_path_buf();
            if !absolute_path.ends_with(self.note_file_name.as_str()) {
                continue;
            }

            let note_name = absolute_path
                .parent().unwrap()
                .strip_prefix(self.notes_root.as_path()).unwrap()
                .to_str().unwrap()
                .replace("\\", "/");
            println!("{}", note_name);
        }

        Ok(())
    }

    pub fn search_notes(&self) {}
}

fn notes_dir() -> PathBuf {
    let crate_name = clap::crate_name!();
    return dirs::data_local_dir().map(|p| p.join(crate_name).join("notes"))
        .unwrap_or_else(|| panic!("Failed to resolve local data directory"));
}

// fn config_dir() -> PathBuf {
//     let crate_name = clap::crate_name!();
//     return dirs::config_dir().unwrap().join(crate_name);
// }
