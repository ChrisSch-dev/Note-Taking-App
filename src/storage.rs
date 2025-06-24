use crate::note::Note;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::Path;

const NOTES_FILE: &str = "notes.json";

pub struct Storage;

impl Storage {
    /// Load notes from a JSON file. If the file does not exist or is invalid, returns an empty Vec.
    pub fn load_notes() -> Vec<Note> {
        let path = Path::new(NOTES_FILE);
        if !path.exists() {
            return Vec::new();
        }
        let file = match File::open(path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Error opening notes file: {e}");
                return Vec::new();
            }
        };
        let reader = BufReader::new(file);
        match serde_json::from_reader(reader) {
            Ok(notes) => notes,
            Err(e) => {
                eprintln!("Error parsing notes file: {e}");
                Vec::new()
            }
        }
    }

    /// Save notes to a JSON file. Overwrites the file if it exists.
    pub fn save_notes(notes: &[Note]) {
        let path = Path::new(NOTES_FILE);
        let file = match OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)
        {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Error opening notes file for writing: {e}");
                return;
            }
        };
        let writer = BufWriter::new(file);
        if let Err(e) = serde_json::to_writer_pretty(writer, notes) {
            eprintln!("Error saving notes to file: {e}");
        }
    }
}