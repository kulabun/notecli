use std::fs::{create_dir_all, File, remove_dir, remove_file};
use std::path::PathBuf;

use walkdir::{DirEntry, WalkDir};

const NOTE_FILE_NAME: &str = "@note";

pub struct NoteManager {
    note_file_name: String,
    notes_root: PathBuf,
}

impl NoteManager {
    pub fn new() -> NoteManager {
        return NoteManager {
            note_file_name: NOTE_FILE_NAME.to_string(),
            notes_root: notes_dir(),
        };
    }

    pub fn is_name_correct(&self, note_name: &str) -> bool {
        return note_name.matches("^[a-zA-Z0-9][a-zA-Z0-9\\-_\\.@]+(/[a-zA-Z0-9][a-zA-Z0-9\\-_\\.@]+)*&").next().is_some();
    }

    pub fn note_exists(&self, note_name: &str) -> bool {
        return self.path_to_note(note_name).exists();
    }

    pub fn touch_note(&self, note_name: &str) {
        let note_file = self.path_to_note(note_name);
        if note_file.exists() {
            return;
        }

        create_dir_all(note_file.parent().unwrap())
            .unwrap_or_else(|_| panic!("Failed to create note directory"));

        File::create(note_file)
            .unwrap_or_else(|_| panic!("Failed to create note file"));
    }

    pub fn delete_note(&self, note_name: &str) {
        let note_path = self.path_to_note(note_name);
        if !note_path.exists() {
            panic!("Note not found!")
        }

        remove_file(note_path.as_path())
            .unwrap_or_else(|_| panic!("Failed to delete note file"));

        fn is_empty_folder(folder: &PathBuf) -> bool {
            folder.read_dir().unwrap().next().is_none()
        }

        let mut parent = note_path.parent().unwrap().to_path_buf();
        while parent != self.notes_root && is_empty_folder(&mut parent) {
            remove_dir(parent.as_path())
                .unwrap_or_else(|_| panic!("Failed to delete note directory"));
            parent = parent.parent().unwrap().to_path_buf();
        }
    }

    pub fn path_to_note(&self, note_name: &str) -> PathBuf {
        let mut path = self.notes_root.join(note_name);
        for elem in self.note_file_name.split("/") {
            path = path.join(elem);
        }
        return path;
    }

    pub fn list_notes(&self, maybe_prefix: Option<String>) {
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
