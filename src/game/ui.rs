use crate::game::{ActionMode, PlayerState, QueenMode};
use bevy::prelude::*;
use bevy_egui::egui::style::Spacing;
use bevy_egui::egui::FontFamily::Proportional;
use bevy_egui::egui::FontId;
use bevy_egui::{egui, EguiContext, EguiContexts, EguiInput};

// Left click to select an ant or queen, enemy, or a zone.
// Right click drag to pan the camera.
// Scroll to zoom in and out.

// To set the queen breeding zone, click on "New Queen Breeding Spot", then click on a cell.
// To set the queen mode, switch between "Work" and "Breed" radio

#[derive(PartialEq)]
enum ControlMode {
    First,
    Second,
    Third,
}

pub fn setup(mut contexts: EguiContexts) {
    let mut style = (*contexts.ctx_mut().style()).clone();
    style
        .text_styles
        .insert(egui::TextStyle::Button, FontId::proportional(30.0));
    contexts.ctx_mut().set_style(style);
}

pub fn control(mut contexts: EguiContexts, mut player_state: ResMut<PlayerState>) {
    let PlayerState {
        queen_mode,
        action_mode,
        ..
    } = &mut *player_state;

    egui::TopBottomPanel::bottom("top_panel")
        .exact_height(80f32)
        .show(contexts.ctx_mut(), |ui| {
            ui.add_space(10f32);
            ui.horizontal_centered(|ui| {
                ui.vertical(|ui| {
                    ui.heading("Queen Mode");
                    ui.horizontal_centered(|ui| {
                        ui.selectable_value(queen_mode, QueenMode::Working, "Work");
                        ui.selectable_value(queen_mode, QueenMode::Breeding, "Breed");
                    });
                });

                ui.separator();

                ui.vertical(|ui| {
                    ui.heading("Actions");
                    ui.horizontal_centered(|ui| {
                        // ui.button("Camera");
                        // ui.button("Dig");
                        // ui.button("Set Breeding Spot");
                        // ui.button("Set food storage");
                        // ui.button("Set nest");

                        ui.selectable_value(action_mode, ActionMode::Select, "Select");
                        ui.selectable_value(action_mode, ActionMode::Dig, "Dig");
                    });
                });
            });
        });
}
