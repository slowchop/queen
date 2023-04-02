use crate::game::jobs::{Job, Jobs};
use crate::game::mouse::MouseWorldPosition;
use crate::game::pathfinding::Path;
use crate::game::plugin::{ActionMode, PlayerState};
use crate::game::positions::SideIPos;
use crate::game::queen::Queen;
use crate::input::{InputAction, InputStates};
use bevy::prelude::Res;
use bevy::prelude::*;

#[derive(Deref, DerefMut)]
pub struct SetQueenLayingPositionEvent(SideIPos);

pub fn primary_mouse_click(
    mouse_world_position: Res<MouseWorldPosition>,
    input_state: Res<InputStates>,
    player_state: Res<PlayerState>,
    mut jobs: ResMut<Jobs>,
    mut set_queen_laying_position_writer: EventWriter<SetQueenLayingPositionEvent>,
) {
    if !input_state.just_pressed(InputAction::PrimaryAction) {
        return;
    }

    info!("Left mouse click: {:?}", mouse_world_position);

    match &player_state.action_mode {
        ActionMode::Dig => {
            //
            jobs.insert(mouse_world_position.to_cell(), Job::Dig);
        }
        ActionMode::SetLayingPosition => {
            info!("Set laying position: {:?}", mouse_world_position);
            set_queen_laying_position_writer
                .send(SetQueenLayingPositionEvent(mouse_world_position.to_cell()));
        }
        _ => warn!(
            "TODO left_mouse_click: action_mode: {:?}",
            player_state.action_mode
        ),
    }
}
