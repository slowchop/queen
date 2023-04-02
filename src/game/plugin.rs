use crate::game;
use crate::game::jobs::Jobs;
use crate::game::map::CellChangedEvent;
use crate::game::pathfinding::VisitedNodeEvent;
use crate::game::positions::SideIPos;
use crate::game::{actions, camera, mouse, setup, ui};
use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy::utils::HashMap;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<VisitedNodeEvent>();
        app.add_event::<CellChangedEvent>();

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
        app.add_systems((
            game::pathfinding::needs_path,
            game::pathfinding::move_along_path,
            game::map::passive_dig_when_visiting_a_cell,
            game::map::detect_cell_content_changes_and_update_rendering,
        ));
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
    pub queen_breeding_cell: Option<SideIPos>,
    pub queen_mode: QueenMode,
    pub action_mode: ActionMode,
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
