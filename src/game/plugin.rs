use crate::game;
use crate::game::ants::AntType;
use crate::game::eggs::SpawnAntEvent;
use crate::game::food_types::FoodId;
use crate::game::map::{AddFoodZoneEvent, UpdateFoodRenderingEvent, UpdateTileDirtAmountEvent};
use crate::game::pathfinding::{PathfindingLinesDebug, VisitedNodeEvent};
use crate::game::positions::SideIPos;
use crate::game::queen::{EggLaidEvent, Queen};
use crate::game::setup::queen_start;
use crate::game::time::GameTime;
use crate::game::zones::FoodStorageZones;
use crate::game::{actions, brains, camera, food, mouse, setup, time, ui};
use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};
use big_brain::prelude::*;
use rand::prelude::{IteratorRandom, ThreadRng};
use rand::Rng;
use crate::game::food::FoodInfo;
use crate::game::skill::SkillMode;

pub const DIRT_Z: f32 = 0f32;
pub const QUEEN_Z: f32 = 1f32;
pub const FOOD_Z: f32 = 1.5f32;
pub const ANT_Z: f32 = 2f32;
pub const EGG_Z: f32 = 3f32;

pub struct GamePlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum InputSet {
    /// Reset the state of the UI.
    Reset,

    /// Ui goes before Input so we can detect if the mouse is over a UI element when a click action
    /// happens.
    Ui,

    /// If we know we're over a UI element, some of these Input sets will be skipped.
    GetInput,

    /// After the input has been collected, do things with the input, that should happen before
    /// ray-casting and the rest of the game logic.
    ProcessInput,

    /// Work out any ray-casting.
    /// TODO: There's only one system in here which might be a small bottleneck.
    ///       Maybe that system can just move to ProcessInput with a `run_after()`.
    Raycast,

    ///
    Game,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BigBrainPlugin);

        app.add_event::<VisitedNodeEvent>();
        app.add_event::<EggLaidEvent>();
        app.add_event::<SpawnAntEvent>();
        app.add_event::<food::AddFoodForAntToCarryEvent>();
        app.add_event::<food::FeedEvent>();
        app.add_event::<UpdateTileDirtAmountEvent>();
        app.add_event::<UpdateFoodRenderingEvent>();
        app.add_event::<AddFoodZoneEvent>();

        app.insert_resource(GameTime::default());
        app.insert_resource(ui::IsHoveringOverUi::default());
        app.insert_resource(PlayerState::default());
        app.insert_resource(food::FoodState::default());
        app.insert_resource(PathfindingLinesDebug::default());
        app.insert_resource(SkillMode::Career);

        app.add_startup_systems((
            camera::setup,
            setup::setup_map,
            setup::setup_queen,
            // setup::setup_test_eggs,
            mouse::setup,
            ui::setup,
        ));

        // Reset
        app.add_systems((time::new_frame, ui::reset_hovering_over_ui_flag).in_set(InputSet::Reset));

        // Ui
        app.add_systems((ui::control, ui::show_queens_choice).in_set(InputSet::Ui));

        // ProcessInput
        app.add_systems(
            (
                camera::control,
                actions::primary_mouse_click,
                time::input,
                game::pathfinding::toggle_pathfinding_debug_lines,
            )
                .in_set(InputSet::ProcessInput),
        );

        // Raycast
        app.add_systems((mouse::mouse_to_world, ).in_set(InputSet::Raycast));

        // Game
        app.add_systems(
            (
                camera::update,
                game::pathfinding::needs_path,
                game::pathfinding::move_along_path,
                game::map::passive_dig_when_visiting_a_cell,
                game::map::update_tile_rendering,
                game::map::update_food_tile_rendering,
                game::map::detect_cell_content_changes_and_update_graph,
                game::queen::grow_and_lay_eggs,
                game::queen::update_queen_egg_progress_speed,
                game::eggs::spawn_eggs,
                game::eggs::grow_eggs,
                game::animation::animate_sprites,
                game::ants::spawn_ants,
                game::hunger::hunger_system,
                game::food::attach_food_to_ant,
            )
                .in_set(InputSet::Game),
        );

        app.add_systems(
            (
                game::zones::add_food_zones,
                game::side_effects::remove_expired_side_effects,
                game::side_effects::calculate_total_side_effects,
                game::pathfinding::show_debug_lines,
                game::debug::check_for_f3_to_offer_queen_new_food,
                game::food::feed_and_apply,
        )
            .in_set(InputSet::Game),
        );

        app.configure_set(InputSet::Reset.before(InputSet::Ui));
        app.configure_set(InputSet::Ui.before(InputSet::GetInput));
        app.configure_set(InputSet::GetInput.before(InputSet::ProcessInput));
        app.configure_set(InputSet::ProcessInput.before(InputSet::Raycast));
        app.configure_set(InputSet::Raycast.before(InputSet::Game));

        // Brain things
        app.add_systems(
            (
                brains::pathfinding::pathfinding_action,
                brains::pathfinding::set_path_to_outside_action,
                brains::pathfinding::set_path_and_assign_food_to_discovered_food_action,
                brains::pathfinding::set_path_to_food_storage_action,
                brains::pathfinding::set_path_to_stored_food_action,
                brains::pathfinding::set_path_to_queen_action,
                brains::map_transition_action,
                brains::outside_map_discovering_food_action,
                brains::outside_map_gathering_existing_food_action,
                brains::offer_food_discovery_to_queen_action,
                brains::place_food_if_possible_action,
                brains::pick_up_food_action,
                brains::feed_queen_action,
            )
                .in_set(BigBrainSet::Actions),
        );
        // app.add_system_to_stage(BigBrainStage::Scorers, ());
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
    pub action_mode: ActionMode,
    pub queen_laying_ant_type: AntType,
    pub queens_choice: QueensChoice,
}

#[derive(Debug, Default, Clone)]
pub enum QueensChoice {
    #[default]
    None,
    Undecided(FoodInfo),
    Approve,
    Deny,
}

#[derive(PartialEq, Debug, Default)]
pub enum ActionMode {
    #[default]
    Select,
    // MoveCamera,
    // ZoomIn,
    // ZoomOut,
    Dig,
    // SetLayingPosition,
}

#[derive(Component)]
pub struct Crawler;

#[derive(Component, Deref, DerefMut)]
pub struct Speed(f32);

impl Speed {
    pub fn new(speed: f32) -> Self {
        Self(speed)
    }
}

impl Default for Speed {
    fn default() -> Self {
        Self::new(64.0)
    }
}

pub fn apply_speed(speed: &Speed, time: &GameTime) -> f32 {
    speed.0 * time.delta_seconds()
}