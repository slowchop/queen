use crate::game::map::SIDE_CELL_SIZE;
use crate::game::queen::EggLaidEvent;
use crate::game::setup::sprite;
use bevy::prelude::*;

#[derive(Debug, Eq, PartialEq, Default, Copy, Clone)]
pub enum AntType {
    #[default]
    Scout,
    Cargo,
    Nurse,
    Soldier,
}
