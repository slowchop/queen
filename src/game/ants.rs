use crate::game::queen::EggLaidEvent;
use bevy::prelude::*;

#[derive(Debug, Eq, PartialEq, Default, Copy, Clone)]
pub enum AntType {
    #[default]
    Scout,
    Cargo,
    Nurse,
    Soldier,
}

pub fn spawn_eggs(mut commands: Commands, mut egg_laid_reader: EventReader<EggLaidEvent>) {
    //
}
