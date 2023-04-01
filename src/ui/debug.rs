use bevy::prelude::ResMut;
use bevy_egui::{egui, EguiContexts};

pub fn debug_ui(
    mut contexts: EguiContexts,
) {
    egui::Window::new("Debug").show(&contexts.ctx_mut(), |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("Hello!");
        });
    });
}
