use crate::game::actions::SetQueenLayingPositionEvent;
use crate::game::pathfinding::Path;
use crate::game::plugin::PlayerState;
use crate::game::positions::SideIPos;
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Queen {
    // pub current_egg_type: AntType,
    pub egg_progress: u32,
}

/// Used in [PlayerState].
#[derive(PartialEq, Debug)]
pub enum QueenMode {
    Working,
    Laying,
}

impl Queen {
    /// A run_if condition.
    pub fn is_laying(player_state: Res<PlayerState>) -> bool {
        player_state.queen_mode == QueenMode::Laying
    }
}

pub fn set_queen_laying_position(
    mut set_queen_laying_position_reader: EventReader<SetQueenLayingPositionEvent>,
    mut player_state: ResMut<PlayerState>,
) {
    for event in set_queen_laying_position_reader.iter() {
        player_state.queen_laying_position = Some(**event);
    }
}

/// Cancel pathfinding for the queen so that it doesn't move to the previous one.
/// But only if the queen has laying mode set.
/// In other words: If the queen is in working mode, we don't want to cancel the pathfinding.
pub fn stop_queen_pathfinding_when_laying_target_changed(
    mut set_queen_laying_position_reader: EventReader<SetQueenLayingPositionEvent>,
    mut query: Query<&mut Path, With<Queen>>,
) {
    if set_queen_laying_position_reader.is_empty() {
        return;
    }

    for mut path in query.iter_mut() {
        path.cancel();
    }
}

pub fn ensure_path_queen_to_laying_spot(
    player_state: Res<PlayerState>,
    mut query: Query<&mut Path, With<Queen>>,
) {
    let Some(queen_laying_position) = player_state.queen_laying_position else {
        warn!("Queen laying position not set");
        return;
    };

    for mut path in query.iter_mut() {
        if path.is_moving() {
            continue;
        }

        path.set_target(queen_laying_position);
        info!(?queen_laying_position, "Queen laying position set");
    }
}

/// If the queen is at the laying spot and is set to laying mode, increase the egg progress.
pub fn lay_eggs(player_state: Res<PlayerState>, mut query: Query<(&mut Queen, &Transform, &Path)>) {
    for (mut queen, transform, path) in query.iter_mut() {
        if path.is_moving() {
            continue;
        }

        let Some(queen_laying_position) = player_state.queen_laying_position else {
            warn!("Queen laying position not set.");
            continue;
        };

        let queen_position = SideIPos::from(transform);
        if queen_position != queen_laying_position {
            warn!("Queen isn't at laying position and Path is None.");
            continue;
        }

        queen.egg_progress += 1;
        info!("Egg progress: {}", queen.egg_progress);
    }
}
