use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy::utils::HashMap;
use dirt::Dirt;

mod camera;
mod dirt;
mod mouse;
mod setup;

pub const SIDE_CELL_SIZE: u8 = 16;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_systems((
            camera::setup,
            setup::setup_map,
            setup::setup_queen,
            mouse::setup,
        ));
        app.add_systems((camera::control, camera::update).chain());

        // TODO: After control camera
        app.add_system(mouse::mouse_to_world);
    }

    fn name(&self) -> &str {
        "Game"
    }

    fn is_unique(&self) -> bool {
        true
    }
}

#[derive(Resource)]
pub struct PlayerState {
    queen_breeding_cell: Option<SideCell>,
}

/// The side view of the world. The idea is that if we have time we can do a top down view on the
/// surface of the world.
#[derive(Resource)]
pub struct SideDirtCells(HashMap<SideCell, Entity>);

/// The side position of a fixed position, e.g. a dirt cell.
#[derive(Component, Deref, DerefMut, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct SideCell(IVec2);

impl SideCell {
    pub fn new(x: i32, y: i32) -> Self {
        Self(IVec2::new(x, y))
    }

    pub fn to_world_vec2(&self) -> Vec2 {
        Vec2::new(
            self.0.x as f32 * SIDE_CELL_SIZE as f32,
            self.0.y as f32 * SIDE_CELL_SIZE as f32,
        )
    }

    pub fn to_world_vec3(&self) -> Vec3 {
        self.to_world_vec2().extend(0.0)
    }
}

/// The side position of a floating point position. Used for crawler positions.
#[derive(Deref, DerefMut)]
pub struct SidePosition(Vec2);

impl SidePosition {
    pub fn new(x: f32, y: f32) -> Self {
        Self(Vec2::new(x, y))
    }
}

/// A creature that crawls around the world and uses pathfinding.
///
/// This is removed from the queen when in breeding mode.
#[derive(Component)]
pub struct Crawler;

#[derive(Component)]
pub struct Speed(f32);

impl Speed {
    pub fn new(speed: f32) -> Self {
        Self(speed)
    }
}

impl Default for Speed {
    fn default() -> Self {
        Self::new(1.0)
    }
}

#[derive(Component)]
pub struct Hunger {
    current: u16,
    hungry_at: u16,
    starving_at: u16,
}

impl Hunger {
    pub fn new(hungry_at: u16, starving_at: u16) -> Self {
        Self {
            current: 0,
            hungry_at,
            starving_at,
        }
    }

    pub fn starving_offset(&self) -> u16 {
        self.starving_at.saturating_sub(self.current)
    }
}

impl Default for Hunger {
    fn default() -> Self {
        Self::new(100, 200)
    }
}
