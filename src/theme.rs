use eframe::egui::Context;

pub fn set_theme(ctx: &Context, dark_mode: bool) {
    if dark_mode {
        ctx.set_visuals(eframe::egui::Visuals::dark());
    } else {
        ctx.set_visuals(eframe::egui::Visuals::light());
    }
}