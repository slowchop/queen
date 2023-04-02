use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy::utils::HashMap;
use jobs::Jobs;
use map::CellContent;
use positions::SideIPos;

mod actions;
mod camera;
mod jobs;
mod map;
mod mouse;
mod pathfinding;
mod positions;
mod setup;
mod ui;

pub const SIDE_CELL_SIZE: u8 = 16;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerState {
            queen_breeding_cell: Some(SideIPos::new(10, -10)),
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
    queen_breeding_cell: Option<SideIPos>,
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
pub struct SideMapPosToEntities(HashMap<SideIPos, Entity>);

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
