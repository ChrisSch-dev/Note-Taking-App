use eframe::egui;
use crate::note::Note;
use crate::storage::Storage;
use crate::theme::set_theme;

pub struct NoteApp {
    pub notes: Vec<Note>,
    pub filter: String,
    pub selected: Option<usize>,
    pub editor_title: String,
    pub editor_content: String,
    pub is_editing: bool,
    pub dark_mode: bool,
}

impl Default for NoteApp {
    fn default() -> Self {
        let notes = Storage::load_notes();
        Self {
            notes,
            filter: String::new(),
            selected: None,
            editor_title: String::new(),
            editor_content: String::new(),
            is_editing: false,
            dark_mode: true,
        }
    }
}

impl eframe::App for NoteApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        set_theme(ctx, self.dark_mode);

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("🔎");
                if ui.text_edit_singleline(&mut self.filter).changed() {
                    self.selected = None;
                }
                if ui.button(if self.dark_mode {"🌙"} else {"🔆"}).clicked() {
                    self.dark_mode = !self.dark_mode;
                }
            });
        });

        egui::SidePanel::left("sidebar").min_width(220.0).show(ctx, |ui| {
            ui.heading("Notes");
            if ui.button("➕ New Note").clicked() {
                self.editor_title.clear();
                self.editor_content.clear();
                self.is_editing = true;
                self.selected = None;
            }
            ui.separator();

            let filter = self.filter.to_lowercase();
            for (i, note) in self.notes.iter().enumerate() {
                if filter.is_empty() || note.title.to_lowercase().contains(&filter) || note.content.to_lowercase().contains(&filter) {
                    let selected = Some(i) == self.selected;
                    if ui.selectable_label(selected, &note.title).clicked() {
                        self.selected = Some(i);
                        self.editor_title = note.title.clone();
                        self.editor_content = note.content.clone();
                        self.is_editing = false;
                    }
                }
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.is_editing {
                ui.heading("Note Editor");
                ui.text_edit_singleline(&mut self.editor_title);
                ui.add(egui::TextEdit::multiline(&mut self.editor_content).desired_rows(20));
                ui.horizontal(|ui| {
                    if ui.button("💾 Save").clicked() {
                        if !self.editor_title.is_empty() {
                            match self.selected {
                                Some(idx) => {
                                    let note = &mut self.notes[idx];
                                    note.title = self.editor_title.clone();
                                    note.content = self.editor_content.clone();
                                    note.edited = Note::now_ts();
                                }
                                None => {
                                    self.notes.push(Note {
                                        title: self.editor_title.clone(),
                                        content: self.editor_content.clone(),
                                        created: Note::now_ts(),
                                        edited: Note::now_ts(),
                                    });
                                    self.selected = Some(self.notes.len() - 1);
                                }
                            }
                            Storage::save_notes(&self.notes);
                            self.is_editing = false;
                        }
                    }
                    if ui.button("❌ Cancel").clicked() {
                        self.is_editing = false;
                    }
                });
            } else if let Some(idx) = self.selected {
                // Clone note to avoid borrow checker issues
                let note = self.notes[idx].clone();
                ui.heading(&note.title);
                ui.label(format!(
                    "Created: {} | Edited: {}",
                    fmt_ts(note.created),
                    fmt_ts(note.edited)
                ));
                ui.separator();
                ui.label(&note.content);
                ui.separator();
                // Use indices for edit/delete to avoid borrow issues
                let selected = self.selected;
                ui.horizontal(|ui| {
                    if ui.button("✏️ Edit").clicked() {
                        if let Some(idx) = selected {
                            self.is_editing = true;
                            self.editor_title = self.notes[idx].title.clone();
                            self.editor_content = self.notes[idx].content.clone();
                        }
                    }
                    if ui.button("🗑️ Delete").clicked() {
                        if let Some(idx) = selected {
                            self.notes.remove(idx);
                            Storage::save_notes(&self.notes);
                            self.selected = None;
                            self.is_editing = false;
                        }
                    }
                });
            } else {
                ui.centered_and_justified(|ui| {
                    ui.label("Select a note or create a new one.");
                });
            }
        });
    }
}

fn fmt_ts(ts: u64) -> String {
    use chrono::{TimeZone, Utc};
    Utc.timestamp_opt(ts as i64, 0)
        .single()
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_else(|| "-".to_owned())
}