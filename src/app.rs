use eframe::egui;
use crate::note::Note;
use crate::storage::Storage;
use crate::theme::set_theme;
use std::fs;

pub struct NoteApp {
    pub notes: Vec<Note>,
    pub filter: String,
    pub selected: Option<usize>,
    pub editor_title: String,
    pub editor_content: String,
    pub is_editing: bool,
    pub dark_mode: bool,
    pub changelogs: Option<String>,
}

impl Default for NoteApp {
    fn default() -> Self {
        let notes = Storage::load_notes();
        let changelogs = fs::read_to_string("changelogs.txt").ok();
        Self {
            notes,
            filter: String::new(),
            selected: None,
            editor_title: String::new(),
            editor_content: String::new(),
            is_editing: false,
            dark_mode: true,
            changelogs,
        }
    }
}

impl eframe::App for NoteApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        set_theme(ctx, self.dark_mode);

        // Top Panel: Search and Theme Toggle
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                // "Home" button, always visible on the top left
                if ui
                    .add(
                        egui::Button::new(
                            egui::RichText::new("🏠 Home")
                                .font(egui::FontId::proportional(18.0))
                                .strong(),
                        )
                            .min_size([70.0, 32.0].into()),
                    )
                    .on_hover_text("Return to Home Page")
                    .clicked()
                {
                    self.selected = None;
                    self.is_editing = false;
                    self.editor_title.clear();
                    self.editor_content.clear();
                }

                ui.add_space(8.0);
                ui.heading(
                    egui::RichText::new("📝 Purpose Notes")
                        .font(egui::FontId::proportional(26.0))
                        .strong(),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Theme Toggle
                    if ui
                        .add(
                            egui::Button::new(
                                egui::RichText::new(if self.dark_mode { "🌙" } else { "🔆" })
                                    .color(if self.dark_mode { egui::Color32::WHITE } else { egui::Color32::BLACK }),
                            )
                                .fill(if self.dark_mode { egui::Color32::DARK_GRAY } else { egui::Color32::LIGHT_GRAY })
                                .rounding(egui::Rounding::same(20.0)),
                        )
                        .on_hover_text("Toggle theme")
                        .clicked()
                    {
                        self.dark_mode = !self.dark_mode;
                    }
                });
                ui.add_space(8.0);
            });
        });

        // Sidebar Panel
        egui::SidePanel::left("sidebar")
            .min_width(250.0)
            .frame(
                egui::Frame::side_top_panel(&ctx.style())
                    .fill(if self.dark_mode {
                        egui::Color32::from_rgb(32, 36, 42)
                    } else {
                        egui::Color32::from_rgb(238, 241, 245)
                    })
                    .inner_margin(egui::Margin::same(12.0)),
            )
            .show(ctx, |ui| {
                // "Notes" and "+" button at the top
                ui.horizontal(|ui| {
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new("Notes")
                                .font(egui::FontId::proportional(30.0))
                                .strong(),
                        )
                            .wrap(false),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui
                            .add(
                                egui::Button::new(
                                    egui::RichText::new("＋").color(egui::Color32::WHITE),
                                )
                                    .fill(egui::Color32::from_rgb(60, 155, 255))
                                    .stroke(egui::Stroke::NONE)
                                    .rounding(egui::Rounding::same(16.0))
                                    .min_size([32.0, 32.0].into()),
                            )
                            .on_hover_text("New Note")
                            .clicked()
                        {
                            self.editor_title.clear();
                            self.editor_content.clear();
                            self.is_editing = true;
                            self.selected = None;
                        }
                    });
                });
                ui.add_space(8.0);
                ui.separator();

                // Search Bar BELOW "Notes" and the divider, but ABOVE all notes
                ui.add_space(8.0);
                let search_changed = ui.add_sized(
                    [ui.available_width() - 8.0, 32.0],
                    egui::TextEdit::singleline(&mut self.filter)
                        .hint_text("Search notes..."),
                ).changed();
                if search_changed {
                    self.selected = None;
                }
                ui.add_space(8.0);

                // Notes List
                let filter = self.filter.to_lowercase();
                let mut did_select = false;
                let mut filtered_count = 0;
                for (i, note) in self.notes.iter().enumerate() {
                    if filter.is_empty()
                        || note.title.to_lowercase().contains(&filter)
                        || note.content.to_lowercase().contains(&filter)
                    {
                        filtered_count += 1;
                        let selected = Some(i) == self.selected;
                        let label = egui::SelectableLabel::new(
                            selected,
                            egui::RichText::new(&note.title).color(
                                if selected {
                                    egui::Color32::from_rgb(60, 155, 255)
                                } else {
                                    ui.visuals().text_color()
                                },
                            ),
                        );
                        let response = egui::Frame::none()
                            .fill(if selected {
                                egui::Color32::from_rgb(220, 240, 255).gamma_multiply(0.25)
                            } else {
                                egui::Color32::TRANSPARENT
                            })
                            .rounding(egui::Rounding::same(8.0))
                            .inner_margin(egui::Vec2::new(6.0, 4.0))
                            .show(ui, |ui| ui.add(label))
                            .inner;

                        if response.clicked() && !did_select {
                            self.selected = Some(i);
                            self.editor_title = note.title.clone();
                            self.editor_content = note.content.clone();
                            self.is_editing = false;
                            did_select = true;
                        }
                    }
                }
                if self.notes.is_empty() {
                    ui.add_space(16.0);
                    ui.label(egui::RichText::new("No notes yet.").italics().weak());
                } else if filtered_count == 0 {
                    ui.add_space(16.0);
                    ui.label(egui::RichText::new("No results.").italics().weak());
                }
            });

        egui::CentralPanel::default().frame(
            egui::Frame::central_panel(&ctx.style())
                .fill(if self.dark_mode {
                    egui::Color32::from_rgb(40, 44, 52)
                } else {
                    egui::Color32::from_rgb(255, 255, 255)
                })
                .inner_margin(egui::Margin::same(24.0)),
        ).show(ctx, |ui| {
            ui.set_width(ui.available_width().min(680.0));

            if self.is_editing {
                ui.add_space(8.0);
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                    ui.heading(
                        egui::RichText::new(
                            if self.selected.is_some() { "Edit Note" } else { "New Note" }
                        )
                            .font(egui::FontId::proportional(34.0))
                            .strong(),
                    );
                });
                ui.add_space(20.0);
                ui.label(
                    egui::RichText::new("Title")
                        .font(egui::FontId::proportional(22.0))
                        .strong(),
                );
                ui.add(
                    egui::TextEdit::singleline(&mut self.editor_title)
                        .hint_text("Enter note title...")
                        .font(egui::FontId::proportional(19.0)),
                );
                ui.add_space(14.0);
                ui.label(
                    egui::RichText::new("Content")
                        .font(egui::FontId::proportional(22.0))
                        .strong(),
                );
                let available_height = ui.available_height() - 80.0;
                ui.add_sized(
                    [ui.available_width(), available_height.max(200.0)],
                    egui::TextEdit::multiline(&mut self.editor_content)
                        .hint_text("Type your note here...")
                        .font(egui::FontId::proportional(17.0)),
                );
                ui.add_space(16.0);
                ui.horizontal(|ui| {
                    if ui
                        .add(
                            egui::Button::new(
                                egui::RichText::new("💾 Save").color(egui::Color32::WHITE),
                            )
                                .fill(egui::Color32::from_rgb(60, 155, 255))
                                .rounding(egui::Rounding::same(12.0))
                                .min_size([80.0, 36.0].into()),
                        )
                        .clicked()
                    {
                        if !self.editor_title.trim().is_empty() {
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
                    if ui
                        .add(
                            egui::Button::new(
                                egui::RichText::new("❌ Cancel"),
                            )
                                .rounding(egui::Rounding::same(12.0))
                                .min_size([80.0, 36.0].into()),
                        )
                        .clicked()
                    {
                        self.is_editing = false;
                    }
                });
            } else if let Some(idx) = self.selected {
                let note = self.notes[idx].clone();
                ui.add_space(8.0);
                ui.heading(
                    egui::RichText::new(&note.title)
                        .font(egui::FontId::proportional(24.0))
                        .strong(),
                );
                ui.add_space(6.0);
                ui.label(
                    egui::RichText::new(format!(
                        "Created: {}   |   Edited: {}",
                        fmt_ts(note.created),
                        fmt_ts(note.edited)
                    ))
                        .size(13.0)
                        .italics()
                        .weak(),
                );
                ui.separator();
                ui.add_space(12.0);
                ui.label(
                    egui::RichText::new(&note.content)
                        .font(egui::FontId::proportional(17.0)),
                );
                ui.add_space(24.0);
                ui.horizontal(|ui| {
                    if ui
                        .add(
                            egui::Button::new(
                                egui::RichText::new("✏️ Edit"),
                            )
                                .rounding(egui::Rounding::same(12.0))
                                .min_size([70.0, 36.0].into()),
                        )
                        .on_hover_text("Edit this note")
                        .clicked()
                    {
                        self.is_editing = true;
                        self.editor_title = note.title.clone();
                        self.editor_content = note.content.clone();
                    }
                    if ui
                        .add(
                            egui::Button::new(
                                egui::RichText::new("🗑️ Delete").color(egui::Color32::WHITE),
                            )
                                .fill(egui::Color32::from_rgb(255, 80, 80))
                                .rounding(egui::Rounding::same(12.0))
                                .min_size([80.0, 36.0].into()),
                        )
                        .on_hover_text("Delete this note")
                        .clicked()
                    {
                        self.notes.remove(idx);
                        Storage::save_notes(&self.notes);
                        self.selected = None;
                        self.is_editing = false;
                    }
                });
            } else {
                // Home Page: Welcome, Instructions, and Changelogs
                ui.add_space(32.0);
                ui.heading(
                    egui::RichText::new("📝 Welcome to Purpose Notes!")
                        .size(28.0)
                        .strong(),
                );
                ui.add_space(10.0);

                // Instructions Section (BOLD)
                ui.group(|ui| {
                    ui.heading(
                        egui::RichText::new("Instructions")
                            .size(22.0)
                            .strong(),
                    );
                    ui.add_space(6.0);
                    ui.label(
                        egui::RichText::new(
                            r#"• To create a note, click the ＋ button on the left sidebar or 'New Note' at the top.
• Click a note in the sidebar to view it.
• Use the ✏️ Edit and 🗑️ Delete buttons to modify or remove notes.
• Search notes by typing in the search bar at the top.
• Switch between dark and light mode with the icon at the top right.
• Click the 🏠 Home button any time to return to this page.
"#,
                        )
                            .size(16.0),
                    );
                });

                ui.add_space(18.0);

                // Changelogs Section - fixed size, expands entire screen width, and scrollable
                let changelogs = self.changelogs.as_ref().map(|s| s.trim()).filter(|s| !s.is_empty());

                let available_height = ui.available_height().max(260.0);
                let available_width = ui.available_width();

                ui.group(|ui| {
                    ui.set_width(available_width);
                    ui.set_height(available_height);
                    // Changelogs Heading (BOLD)
                    ui.heading(
                        egui::RichText::new("Changelogs")
                            .size(22.0)
                            .strong(),
                    );
                    ui.add_space(6.0);

                    egui::ScrollArea::vertical()
                        .max_height(available_height - 40.0)
                        .show(ui, |ui| {
                            if let Some(changelogs) = changelogs {
                                ui.label(
                                    egui::RichText::new(changelogs)
                                        .size(15.0)
                                );
                            } else {
                                ui.add_space(8.0);
                                ui.group(|ui| {
                                    ui.set_width(ui.available_width());
                                    ui.set_height((available_height - 40.0).max(80.0));
                                    ui.centered_and_justified(|ui|{
                                        ui.label(
                                            egui::RichText::new("No ChangeLogs available")
                                                .size(16.0)
                                                .italics()
                                                .color(egui::Color32::DARK_GRAY)
                                        );
                                    });
                                });
                            }
                        });
                });

                ui.add_space(10.0);
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