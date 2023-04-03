use crate::game::ants::AntType;
use crate::game::plugin::{ActionMode, PlayerState};
use crate::game::queen::QueenMode;
use bevy::prelude::*;
use bevy_egui::egui::style::Spacing;
use bevy_egui::egui::FontFamily::Proportional;
use bevy_egui::egui::FontId;
use bevy_egui::{egui, EguiContext, EguiContexts, EguiInput};

#[derive(Resource, Default)]
pub struct IsHoveringOverUi(bool);

pub fn setup(mut contexts: EguiContexts) {
    let mut style = (*contexts.ctx_mut().style()).clone();
    style
        .text_styles
        .insert(egui::TextStyle::Button, FontId::proportional(30.0));
    contexts.ctx_mut().set_style(style);
}

pub fn reset_hovering_over_ui_flag(mut is_hovering_over_ui: ResMut<IsHoveringOverUi>) {
    *is_hovering_over_ui = IsHoveringOverUi(false);
}

pub fn not_using_ui(is_hovering_over_ui: Res<IsHoveringOverUi>) -> bool {
    !is_hovering_over_ui.0
}

pub fn control(
    mut contexts: EguiContexts,
    mut player_state: ResMut<PlayerState>,
    mut is_hovering_over_ui: ResMut<IsHoveringOverUi>,
) {
    let PlayerState {
        queen_mode,
        action_mode,
        queen_laying_ant_type,
        ..
    } = &mut *player_state;

    let response = egui::TopBottomPanel::bottom("top_panel")
        .exact_height(80f32)
        .show(contexts.ctx_mut(), |ui| {
            ui.add_space(10f32);
            ui.horizontal_centered(|ui| {
                ui.vertical(|ui| {
                    ui.heading("Queen Mode");
                    ui.horizontal_centered(|ui| {
                        ui.selectable_value(queen_mode, QueenMode::Working, "Work");
                        ui.selectable_value(queen_mode, QueenMode::Laying, "Lay Eggs");
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
                        ui.selectable_value(
                            action_mode,
                            ActionMode::SetLayingPosition,
                            "Set Laying Position",
                        );
                        ui.selectable_value(action_mode, ActionMode::Dig, "Dig");
                    });
                });

                ui.separator();

                ui.vertical(|ui| {
                    ui.heading("Next Ant Type");
                    ui.horizontal_centered(|ui| {
                        ui.selectable_value(queen_laying_ant_type, AntType::Scout, "Scout");
                        ui.selectable_value(queen_laying_ant_type, AntType::Cargo, "Cargo");
                        ui.selectable_value(queen_laying_ant_type, AntType::Soldier, "Soldier");
                        ui.selectable_value(queen_laying_ant_type, AntType::Nurse, "Nurse");
                    });
                });
            });
        });

    if response.response.hovered() {
        *is_hovering_over_ui = IsHoveringOverUi(true);
    }
}
