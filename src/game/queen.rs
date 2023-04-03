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

/// If the queen is at the laying spot and is set to laying mode, increase the egg progress.
pub fn grow_and_lay_eggs(
    time: Res<Time>,
    player_state: Res<PlayerState>,
    mut query: Query<(&mut Queen, &Transform)>,
    mut egg_laid_writer: EventWriter<EggLaidEvent>,
) {
    for (mut queen, transform) in query.iter_mut() {
        let pos = SideIPos::from(transform);

        queen.egg_progress += time.delta_seconds();
        // info!("Egg progress: {}", queen.egg_progress);

        if queen.egg_progress >= 3f32 {
            queen.egg_progress = 0f32;

            egg_laid_writer.send(EggLaidEvent {
                egg: Egg::new(player_state.queen_laying_ant_type, 3f32),
                position: pos,
            });
            info!("Egg laid!");
        }
    }
}
