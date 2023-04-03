use crate::game::actions::SetQueenLayingPositionEvent;
use crate::game::ants::AntType;
use crate::game::eggs::Egg;
use crate::game::pathfinding::Path;
use crate::game::plugin::PlayerState;
use crate::game::positions::SideIPos;
use bevy::prelude::*;

#[derive(Debug)]
pub struct EggLaidEvent {
    pub egg: Egg,
    pub position: SideIPos,
}

#[derive(Component, Default, Copy, Clone)]
pub struct Queen {
    pub egg_progress: f32,
}

/// Used in [PlayerState].
#[derive(PartialEq, Debug, Default)]
pub enum QueenMode {
    Working,
    #[default]
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
pub fn stop_queen_path_when_laying_position_changed(
    set_queen_laying_position_reader: EventReader<SetQueenLayingPositionEvent>,
    mut query: Query<&mut Path, With<Queen>>,
) {
    if set_queen_laying_position_reader.is_empty() {
        return;
    }

    for mut path in query.iter_mut() {
        path.cancel();
    }
}

/// If the queen is in laying mode, ensure that it is pathing to the laying spot.
pub fn ensure_path_queen_to_laying_spot(
    player_state: Res<PlayerState>,
    mut query: Query<(&mut Path, &Transform), With<Queen>>,
) {
    let Some(queen_laying_position) = player_state.queen_laying_position else {
        warn!("Queen laying position not set");
        return;
    };

    for (mut path, transform) in query.iter_mut() {
        if path.has_target() {
            continue;
        }

        // Already at the laying spot.
        if SideIPos::from(transform) == queen_laying_position {
            continue;
        }

        path.set_target(queen_laying_position);
        info!(?queen_laying_position, "Queen laying position set");
    }
}

/// If the queen is at the laying spot and is set to laying mode, increase the egg progress.
pub fn grow_and_lay_eggs(
    time: Res<Time>,
    player_state: Res<PlayerState>,
    mut query: Query<(&mut Queen, &Transform, &Path)>,
    mut egg_laid_writer: EventWriter<EggLaidEvent>,
) {
    for (mut queen, transform, path) in query.iter_mut() {
        if path.has_target() {
            continue;
        }

        let Some(queen_laying_position) = player_state.queen_laying_position else {
            warn!("Queen laying position not set.");
            continue;
        };

        let queen_position = SideIPos::from(transform);
        if queen_position != queen_laying_position {
            // This should be handled by [ensure_path_queen_to_laying_spot].
            warn!("Queen isn't at laying position and Path is None.");
            continue;
        }

        queen.egg_progress += time.delta_seconds();
        // info!("Egg progress: {}", queen.egg_progress);

        if queen.egg_progress >= 3f32 {
            queen.egg_progress = 0f32;

            egg_laid_writer.send(EggLaidEvent {
                egg: Egg::new(player_state.queen_laying_ant_type, 3f32),
                position: queen_position,
            });
            info!("Egg laid!");
        }
    }
}
