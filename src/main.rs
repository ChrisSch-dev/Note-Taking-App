mod note;
mod storage;
mod theme;
mod app;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Advanced Note Taking App",
        native_options,
        Box::new(|_cc| Box::new(app::NoteApp::default())),
    );
}