mod note;
mod storage;
mod theme;
mod app;

fn main() {
    let app = app::NoteApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}