use crate::game;
use crate::game::actions::SetQueenLayingPositionEvent;
use crate::game::ants::AntType;
use crate::game::jobs::Jobs;
use crate::game::map::CellChangedEvent;
use crate::game::pathfinding::VisitedNodeEvent;
use crate::game::positions::SideIPos;
use crate::game::queen::{EggLaidEvent, Queen, QueenMode};
use crate::game::{actions, camera, mouse, setup, ui};
use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy::utils::HashMap;

pub struct GamePlugin;

pub const DIRT_Z: f32 = 0f32;
pub const EGG_Z: f32 = 1f32;
pub const ANT_Z: f32 = 2f32;
pub const QUEEN_Z: f32 = 3f32;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<VisitedNodeEvent>();
        app.add_event::<CellChangedEvent>();
        app.add_event::<SetQueenLayingPositionEvent>();
        app.add_event::<EggLaidEvent>();

        app.insert_resource(PlayerState::default());
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

        app.add_systems((actions::primary_mouse_click,));
        app.add_systems((
            game::pathfinding::needs_path,
            game::pathfinding::move_along_path,
            game::map::passive_dig_when_visiting_a_cell,
            game::map::detect_cell_content_changes_and_update_rendering,
            game::queen::set_queen_laying_position,
            game::queen::stop_queen_path_when_laying_position_changed.run_if(Queen::is_laying),
            game::queen::ensure_path_queen_to_laying_spot.run_if(Queen::is_laying),
            game::queen::grow_and_lay_eggs.run_if(Queen::is_laying),
            game::eggs::spawn_eggs,
            game::animation::animate_sprites,
        ));
    }

    fn name(&self) -> &str {
        "Game"
    }

    fn is_unique(&self) -> bool {
        true
    }
}

#[derive(Resource, Debug, Default)]
pub struct PlayerState {
    pub queen_laying_position: Option<SideIPos>,
    pub queen_laying_ant_type: AntType,
    pub queen_mode: QueenMode,
    pub action_mode: ActionMode,
}

#[derive(PartialEq, Debug, Default)]
pub enum ActionMode {
    #[default]
    Select,
    // MoveCamera,
    // ZoomIn,
    // ZoomOut,
    Dig,
    SetLayingPosition,
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
