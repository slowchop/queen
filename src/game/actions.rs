use crate::game::mouse::MouseWorldPosition;
use crate::game::pathfinding::Path;
use crate::game::plugin::{ActionMode, PlayerState};
use crate::game::positions::SideIPos;
use crate::game::queen::Queen;
use crate::input::{InputAction, InputStates};
use bevy::prelude::Res;
use bevy::prelude::*;

pub fn primary_mouse_click(
    mouse_world_position: Res<MouseWorldPosition>,
    input_state: Res<InputStates>,
    player_state: Res<PlayerState>,
) {
    if !input_state.just_pressed(InputAction::PrimaryAction) {
        return;
    }

    info!("Left mouse click: {:?}", mouse_world_position);

    match &player_state.action_mode {
        _ => warn!(
            "TODO left_mouse_click: action_mode: {:?}",
            player_state.action_mode
        ),
    }
}
