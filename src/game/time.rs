use crate::input::{InputAction, InputState, InputStates};
use bevy::prelude::*;
use std::time::Duration;

#[derive(Resource, Debug)]
pub struct GameTime {
    /// A special paused mode where the game is waiting for an important input.
    /// The player can't unpause this.
    system_game_pause: bool,

    /// 0 = paused, 1 = normal speed, 2 = double speed, etc.
    time_scale: u32,

    /// For unpausing back to the original time_scale.
    last_time_scale: u32,

    since_startup: Duration,
    delta: Duration,
}

impl GameTime {
    pub fn delta(&self) -> Duration {
        self.delta
    }

    pub fn delta_seconds(&self) -> f32 {
        self.delta.as_secs_f32()
    }

    pub fn system_pause(&mut self, paused: bool) {
        self.system_game_pause = paused;
    }
}

impl Default for GameTime {
    fn default() -> Self {
        Self {
            system_game_pause: false,
            last_time_scale: 1,
            time_scale: 1,
            since_startup: Duration::from_secs(0),
            delta: Duration::from_secs(0),
        }
    }
}

pub fn new_frame(time: Res<Time>, mut game_time: ResMut<GameTime>) {
    if game_time.system_game_pause {
        return;
    }

    let delta: Duration = time.delta().into();
    game_time.delta = delta * game_time.time_scale as u32;
    game_time.since_startup += delta;
}

pub fn input(input: Res<InputStates>, mut game_time: ResMut<GameTime>) {
    if game_time.system_game_pause {
        // System pause doesn't allow changes in time scale.
        return;
    }

    if input.just_pressed(InputAction::TogglePause) {
        if game_time.time_scale == 0 {
            println!("Unpausing...");
            game_time.time_scale = game_time.last_time_scale;
        } else {
            println!("Pausing...");
            game_time.last_time_scale = game_time.time_scale;
            game_time.time_scale = 0;
        }
    }

    if input.just_pressed(InputAction::Speed1) {
        println!("Normal speed");
        game_time.time_scale = 1;
    }

    if input.just_pressed(InputAction::Speed2) {
        println!("Double speed");
        game_time.time_scale = 2;
    }

    if input.just_pressed(InputAction::Speed3) {
        println!("Triple speed");
        game_time.time_scale = 3;
    }
}
