use crate::game::positions::SideIPos;
use bevy::prelude::*;
use bevy::utils::HashSet;
use rand::Rng;

#[derive(Default, Debug)]
pub struct Zone {
    cells: HashSet<SideIPos>,
}

impl Zone {
    pub fn random(&self) -> Option<SideIPos> {
        if self.cells.is_empty() {
            return None;
        }

        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.cells.len());
        self.cells.iter().nth(index).copied()
    }
}

#[derive(Default, Debug, Deref, DerefMut)]
pub struct FoodStorageZone(Zone);
