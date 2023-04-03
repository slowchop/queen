use crate::game::positions::SidePosition;
use crate::game::setup::queen_start;
use crate::input::{InputAction, InputStates};
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;

/// 0 means pixel perfect
/// 1 means 1 sprite pixels wide per 2 screen pixel (2x zoom)
/// 2 means 1 sprite pixel wide per 4 screen pixels (4x zoom)
#[derive(Deref, DerefMut)]
pub struct ZoomLevel(i8);

impl ZoomLevel {
    pub fn camera_projection_scale(&self) -> f32 {
        2f32.powi(-self.0 as i32)
    }
}

#[derive(Resource)]
pub struct CameraFocus {
    target: SidePosition,
    zoom: ZoomLevel,
}

impl CameraFocus {
    pub fn new(target: SidePosition) -> Self {
        Self {
            target,
            zoom: ZoomLevel(1),
        }
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::INDIGO),
            ..Default::default()
        },
        ..Default::default()
    });

    commands.insert_resource(CameraFocus::new(queen_start().into()));
}

pub fn control(
    time: Res<Time>,
    mut camera_focus: ResMut<CameraFocus>,
    input_states: Res<InputStates>,
) {
    let mut movement = Vec2::ZERO;
    if input_states.is_pressed(InputAction::Left) {
        movement.x -= 1.0;
    }
    if input_states.is_pressed(InputAction::Right) {
        movement.x += 1.0;
    }
    if input_states.is_pressed(InputAction::Up) {
        movement.y += 1.0;
    }
    if input_states.is_pressed(InputAction::Down) {
        movement.y -= 1.0;
    }

    let movement = movement.normalize_or_zero();
    let camera_pan_speed =
        1000f32 * camera_focus.zoom.camera_projection_scale() * time.delta().as_secs_f32();
    *camera_focus.target += movement * camera_pan_speed;
}

pub fn update(
    camera_focus: Res<CameraFocus>,
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera2d>>,
) {
    let (mut transform, mut projection) = query.single_mut();
    projection.scale = camera_focus.zoom.camera_projection_scale();

    transform.translation.x = camera_focus.target.x;
    transform.translation.y = camera_focus.target.y;
}
