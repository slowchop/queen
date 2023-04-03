use crate::helpers::delete_entities_with_component;
use crate::settings::GameSettings;
use crate::state::{GameState, StateConfig};
use crate::ui::menu::MenuComponent;
use crate::ui::splash::SplashComponent;
use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use bevy::window::{WindowLevel, WindowMode, WindowResolution};
use bevy::DefaultPlugins;
use bevy_egui::EguiPlugin;
use bevy_prototype_debug_lines::DebugLinesPlugin;
use color_eyre::Result;
use std::time::Duration;

pub mod controller;
pub mod game;
pub mod helpers;
pub mod input;
pub mod settings;
pub mod setup;
pub mod state;
pub mod ui;

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut app = App::new();

    // Easier to just include state.yaml
    let state_config = StateConfig::load_str(include_str!("../assets/state.yaml")).unwrap();

    let game_settings = GameSettings {
        window_mode: WindowMode::Windowed,
        resolution: (1920f32, 1080f32),
        show_debug_ui: true,
        show_stats: true,
    };

    // Bevy plugins
    app.add_plugins(bevy_plugins(&game_settings));

    // Third party plugins
    app.add_plugin(EguiPlugin);
    app.add_plugin(DebugLinesPlugin::default());

    // Game plugins
    app.add_plugin(game::GamePlugin);

    // Game resources
    app.insert_resource(game_settings);
    app.insert_resource(state_config);
    app.add_state::<GameState>();

    // Game startup
    app.add_startup_system(setup::setup);
    // TODO: app.add_system(setup::enumerate_resolutions);

    app.add_startup_system(ui::stats::setup_stats);

    // Input
    app.add_startup_system(input::setup);
    app.add_systems(
        (
            input::process_keyboard_input,
            input::process_mouse_input.run_if(game::not_using_ui),
        )
            .in_set(game::InputSet::GetInput),
    );

    app.add_system(ui::splash::enter.in_schedule(OnEnter(GameState::Splash)));
    app.add_system(ui::splash::update.in_set(OnUpdate(GameState::Splash)));
    app.add_system(
        delete_entities_with_component::<SplashComponent>.in_schedule(OnExit(GameState::Splash)),
    );

    app.add_system(ui::menu::enter.in_schedule(OnEnter(GameState::MainMenu)));
    app.add_systems(
        (
            ui::menu::update,
            ui::menu::handle_action_events,
            ui::menu::handle_interaction_events,
        )
            .in_set(OnUpdate(GameState::MainMenu)),
    );
    app.add_system(ui::menu::update_visual_selection.in_set(OnUpdate(GameState::MainMenu)));
    app.add_system(
        delete_entities_with_component::<MenuComponent>.in_schedule(OnExit(GameState::MainMenu)),
    );

    // Fixed frame rate systems
    app.insert_resource(FixedTime::new(Duration::from_secs_f32(1.0 / 60.0)));
    app.add_systems(().in_schedule(CoreSchedule::FixedUpdate));

    // Video frame rate systems
    app.add_systems((ui::stats::update_stats,));

    app.run();

    Ok(())
}

fn bevy_plugins(settings: &GameSettings) -> PluginGroupBuilder {
    DefaultPlugins
        .set(WindowPlugin {
            primary_window: Some(Window {
                position: WindowPosition::Centered(MonitorSelection::Current),
                resolution: WindowResolution::new(settings.resolution.0, settings.resolution.1),
                mode: settings.window_mode,
                fit_canvas_to_parent: true,
                resizable: true,
                ..Default::default()
            }),
            ..Default::default()
        })
        .set(ImagePlugin::default_nearest())
}
