use bevy::prelude::*;
use bevy::window::WindowMode;

#[derive(Resource, Default)]
pub struct GameSettings {
    pub window_mode: WindowMode,
    pub resolution: (f32, f32),

    /// Show the egui debug Window.
    pub show_debug_ui: bool,

    /// Show the stats text on the top right.
    pub show_stats: bool,
}
