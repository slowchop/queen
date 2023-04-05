use crate::input::{InputAction, InputState, InputStates};
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct WorldInspectorActive(bool);

pub fn toggle_world_inspector(
    input_states: Res<InputStates>,
    mut world_inspector_active: ResMut<WorldInspectorActive>,
) {
    if input_states.just_pressed(InputAction::Debug1) {
        info!("Toggling world inspector");
        world_inspector_active.0 = !world_inspector_active.0;
    }
}

pub fn world_inspector_is_active(world_inspector_active: Res<WorldInspectorActive>) -> bool {
    world_inspector_active.0
}
