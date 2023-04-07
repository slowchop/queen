use crate::game::ants::AntType;
use crate::game::eggs::Egg;
use crate::game::pathfinding::Path;
use crate::game::plugin::PlayerState;
use crate::game::positions::SideIPos;
use crate::game::time::GameTime;
use bevy::prelude::*;
use crate::game::food::CarryingFood;
use crate::game::side_effects::{AppliedFoodSideEffects, CalculatedSideEffects, SideEffectDiscriminants};

#[derive(Debug)]
pub struct EggLaidEvent {
    pub egg: Egg,
    pub position: SideIPos,
}

#[derive(Component, Copy, Clone)]
pub struct Queen {
    pub egg_progress: f32,
    pub egg_progress_speed: f32,
}

impl Default for Queen {
    fn default() -> Self {
        Self {
            egg_progress: 0f32,
            egg_progress_speed: 0.1f32,
        }
    }
}

/// If the queen is at the laying spot and is set to laying mode, increase the egg progress.
pub fn grow_and_lay_eggs(
    time: Res<GameTime>,
    player_state: Res<PlayerState>,
    mut query: Query<(&mut Queen, &Transform)>,
    mut egg_laid_writer: EventWriter<EggLaidEvent>,
) {
    for (mut queen, transform) in query.iter_mut() {
        let pos = SideIPos::from(transform);

        queen.egg_progress += time.delta_seconds() * queen.egg_progress_speed;

        if queen.egg_progress >= 1f32 {
            queen.egg_progress = 0f32;

            egg_laid_writer.send(EggLaidEvent {
                egg: Egg::new(player_state.queen_laying_ant_type, 3f32),
                position: pos,
            });
        }
    }
}

pub fn update_queen_egg_progress_speed(
    mut query: Query<(&mut Queen, &CalculatedSideEffects)>,
) {
    for ((mut queen, side_effects)) in query.iter_mut() {
        let speed = 0.1f32 * side_effects.as_float(SideEffectDiscriminants::QueenEggRate);
        info!("Queen egg rate: {}", speed);
        queen.egg_progress_speed = speed;
    }
}