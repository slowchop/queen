use crate::game::SidePosition;
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::window::PrimaryWindow;

#[derive(Resource, Deref, DerefMut)]
pub struct MouseToWorld(SidePosition);

pub fn setup(mut commands: Commands) {
    commands.insert_resource(MouseToWorld(SidePosition::new(0f32, 0f32)));
}

pub fn mouse_to_world(
    windows: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) {
    let (camera, camera_transform) = camera.single();

    // PrimaryWindow assumes only one instance of this component.
    let window = windows.single();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        eprintln!("World coords: {}/{}", world_position.x, world_position.y);
    }
}
