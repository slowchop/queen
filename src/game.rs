use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy::utils::HashMap;
use dirt::CellContent;
use jobs::Jobs;

mod actions;
mod camera;
mod dirt;
mod jobs;
mod mouse;
mod pathfinding;
mod setup;
mod ui;

pub const SIDE_CELL_SIZE: u8 = 16;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerState {
            queen_breeding_cell: Some(SideCell::new(10, -10)),
            queen_mode: QueenMode::Breeding,
            action_mode: ActionMode::Select,
        });

        app.insert_resource(Jobs::default());

        app.add_startup_systems((
            camera::setup,
            setup::setup_map,
            setup::setup_queen,
            setup::setup_ants,
            mouse::setup,
        ));
        app.add_systems((camera::control, camera::update).chain());

        // TODO: camera::control -> mouse::mouse_to_world -> other...
        app.add_system(mouse::mouse_to_world);

        app.add_startup_system(ui::setup);
        app.add_system(ui::control);

        app.add_systems((actions::left_mouse_click,));
    }

    fn name(&self) -> &str {
        "Game"
    }

    fn is_unique(&self) -> bool {
        true
    }
}

#[derive(Resource, Debug)]
pub struct PlayerState {
    queen_breeding_cell: Option<SideCell>,
    queen_mode: QueenMode,
    action_mode: ActionMode,
}

#[derive(PartialEq, Debug)]
pub enum QueenMode {
    Working,
    Breeding,
}

#[derive(PartialEq, Debug)]
pub enum ActionMode {
    Select,
    // MoveCamera,
    // ZoomIn,
    // ZoomOut,
    Dig,
    SetBreedCell,
}

/// The side view of the world. The idea is that if we have time we can do a top down view on the
/// surface of the world.
#[derive(Resource)]
pub struct SideDirtCells(HashMap<SideCell, Entity>);

fn morton_encode(x: i32, y: i32) -> i64 {
    let mut z = 0i64;
    let x = x as i64;
    let y = y as i64;

    for i in 0..32 {
        z |= ((x >> i) & 1) << (2 * i);
        z |= ((y >> i) & 1) << (2 * i + 1);
    }

    z
}

/// The side position of a fixed position, e.g. a dirt cell.
#[derive(Component, Deref, DerefMut, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct SideCell(IVec2);

impl SideCell {
    pub fn sides(&self) -> [SideCell; 4] {
        [
            SideCell::new(self.0.x + 1, self.0.y),
            SideCell::new(self.0.x - 1, self.0.y),
            SideCell::new(self.0.x, self.0.y + 1),
            SideCell::new(self.0.x, self.0.y - 1),
        ]
    }
}

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

impl PartialOrd<Self> for SideCell {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SideCell {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        morton_encode(self.0.x, self.0.y).cmp(&morton_encode(other.0.x, other.0.y))
    }
}

/// The side position of a floating point position. Used for crawler positions.
#[derive(Deref, DerefMut, Copy, Clone, Debug)]
pub struct SidePosition(Vec2);

impl SidePosition {
    pub fn new(x: f32, y: f32) -> Self {
        Self(Vec2::new(x, y))
    }

    pub fn to_cell(&self) -> SideCell {
        SideCell::new(
            (self.0.x / SIDE_CELL_SIZE as f32).floor() as i32,
            (self.0.y / SIDE_CELL_SIZE as f32).floor() as i32,
        )
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
