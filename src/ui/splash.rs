use crate::helpers::delete_entities_with_component;
use crate::state::{GameState, StateConfig, StateDisplay};
use bevy::prelude::*;
use bevy::utils::Instant;
use std::time::Duration;

#[derive(Component, Deref)]
pub struct EndSplashTime(pub Instant);

#[derive(Component)]
pub struct SplashComponent;

pub fn enter(
    time: Res<Time>,
    mut commands: Commands,
    state_config: Res<StateConfig>,
    state: Res<State<GameState>>,
    asset_server: Res<AssetServer>,
) {
    let StateDisplay::Splash(splash_state) = state_config.get(&state.0).unwrap() else {
        panic!("{state:?} is not a Splash: {state_config:?}");
    };
    dbg!(splash_state);
    let texture = asset_server.load(&splash_state.asset);

    commands
        .spawn(SpriteBundle {
            texture,
            ..Default::default()
        })
        .insert(SplashComponent);

    commands
        .spawn(EndSplashTime(
            time.startup() + time.elapsed() + Duration::from_millis(splash_state.ms),
        ))
        .insert(SplashComponent);
}

pub fn update(
    time: Res<Time>,
    state_config: Res<StateConfig>,
    mut state: ResMut<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    end_splash_time: Query<&EndSplashTime>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let StateDisplay::Splash(splash_state) = state_config.get(&state.0).unwrap() else {
        panic!("{state:?} is not a Splash: {state_config:?}");
    };

    if keyboard_input.get_just_pressed().len() > 0 {
        next_state.set(splash_state.next.clone());
        return;
    }

    let end_time = end_splash_time.single();
    if end_time.0 > time.startup() + time.elapsed() {
        return;
    }

    next_state.set(splash_state.next.clone());
}
