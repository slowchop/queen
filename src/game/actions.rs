use crate::game::jobs::{Job, Jobs};
use crate::game::mouse::MouseWorldPosition;
use crate::game::plugin::{ActionMode, PlayerState};
use crate::input::{InputAction, InputStates};
use bevy::prelude::Res;
use bevy::prelude::*;

pub fn left_mouse_click(
    mouse_world_position: Res<MouseWorldPosition>,
    input_state: Res<InputStates>,
    mut player_state: ResMut<PlayerState>,
    mut jobs: ResMut<Jobs>,
) {
    if !input_state.just_pressed(InputAction::PrimaryAction) {
        return;
    }

    match &player_state.action_mode {
        ActionMode::Dig => {
            //
            jobs.insert(mouse_world_position.to_cell(), Job::Dig);
        }
        _ => warn!(
            "TODO left_mouse_click: action_mode: {:?}",
            player_state.action_mode
        ),
    }
}
