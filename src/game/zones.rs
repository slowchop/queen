use crate::game::food::FoodState;
use crate::game::map::AddFoodZoneEvent;
use crate::game::positions::SideIPos;
use bevy::prelude::*;
use bevy::utils::HashSet;
use rand::Rng;

#[derive(Default, Debug)]
pub struct Zones {
    cells: HashSet<SideIPos>,
}

impl Zones {
    pub fn add(&mut self, position: SideIPos) {
        self.cells.insert(position);
    }

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
pub struct FoodStorageZones(Zones);

/// Grab some zone events, check if they exist and remove them if they do to accommodate different zone types.
pub fn add_food_zones(
    mut commands: Commands,
    mut food_state: ResMut<FoodState>,
    mut add_zone_reader: EventReader<AddFoodZoneEvent>,
) {
    for AddFoodZoneEvent(position) in add_zone_reader.iter() {
        // TODO: Add child sprite

        food_state.food_zones.add(*position);
    }
}
