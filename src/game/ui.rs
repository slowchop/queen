use crate::game::ants::AntType;
use crate::game::hunger::Hunger;
use crate::game::plugin::{ActionMode, PlayerState, QueensChoice};
use crate::game::queen::Queen;
use bevy::prelude::*;
use bevy_egui::egui::style::Spacing;
use bevy_egui::egui::FontFamily::Proportional;
use bevy_egui::egui::{Align2, FontId};
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
    queen: Query<(&Hunger, &Queen)>,
) {
    let PlayerState {
        action_mode,
        queen_laying_ant_type,
        ..
    } = &mut *player_state;

    let (queen_hunger, queen_info) = queen.single();

    let response = egui::TopBottomPanel::bottom("top_panel")
        .exact_height(80f32)
        .show(contexts.ctx_mut(), |ui| {
            ui.add_space(10f32);

            ui.horizontal_centered(|ui| {
                ui.vertical(|ui| {
                    ui.heading("Queen");
                    ui.label(format!("Hunger: {:.1}%", queen_hunger.hunger_fraction()));

                    // egg progress total goes from 0 - 1, and the egg_progress_speed is how much per second.
                    // let rate_per_hour = 3600f32 / queen_info.egg_progress_speed;
                    // ui.label(format!("Egg: {:.1}% ({:.02} per hour)", queen_info.egg_progress * 100f32, rate_per_hour));
                    let seconds_per_egg = 1f32 / queen_info.egg_progress_speed;
                    ui.label(format!("Egg: {:.1}% ({:.0}s per egg)", queen_info.egg_progress * 100f32, seconds_per_egg));
                });

                ui.separator();

                // ui.vertical(|ui| {
                //     ui.heading("Actions");
                //     ui.horizontal_centered(|ui| {
                //         ui.selectable_value(action_mode, ActionMode::Select, "Select");
                //         ui.selectable_value(action_mode, ActionMode::Dig, "Dig");
                //     });
                // });
                //
                // ui.separator();
                //
                ui.vertical(|ui| {
                    ui.heading("Next Ant Type");
                    ui.horizontal_centered(|ui| {
                        ui.selectable_value(queen_laying_ant_type, AntType::Scout, "Scout");
                        ui.selectable_value(queen_laying_ant_type, AntType::Cargo, "Cargo");
                        // ui.selectable_value(queen_laying_ant_type, AntType::Soldier, "Soldier");
                        ui.selectable_value(queen_laying_ant_type, AntType::Nurse, "Nurse");
                    });
                });
            });
        });

    if response.response.hovered() {
        *is_hovering_over_ui = IsHoveringOverUi(true);
    }
}

pub fn show_queens_choice(mut contexts: EguiContexts, mut player_state: ResMut<PlayerState>) {
    let QueensChoice::Undecided(food_info) =  player_state.queens_choice.clone() else {
        return;
    };

    egui::Window::new("Queen's Choice")
        .anchor(Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(&contexts.ctx_mut(), |ui| {
            ui.heading("This scout has found new food!");
            ui.label(format!("Food Type: {}", food_info.food_id));

            for side_effect in &food_info.side_effects {
                ui.label(side_effect.short_name());
            }

            ui.label("Do you want to add this food to the colony?");
            ui.horizontal(|ui| {
                if ui.button("Yes").clicked() {
                    player_state.queens_choice = QueensChoice::Approve;
                }
                if ui.button("No").clicked() {
                    player_state.queens_choice = QueensChoice::Deny;
                }
            });
        });
}
