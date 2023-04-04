use crate::game::positions::SideIPos;
use bevy::prelude::*;
use bevy::utils::HashSet;
use rand::Rng;
use std::time::Duration;

enum FoodType {
    Water,
    Apple,
    Banana,
    Carrot,
    Fly,
    Worm,
    Coffee,
    Manure,
    MedicinePill,
    Honey,
    Sugar,
    FrogsLeg,
}

pub struct FoundFood {
    food: FoodType,
    position: SideIPos,
    time: f32,
}

#[derive(Resource)]
pub struct FoodState {
    pub approved: Vec<FoundFood>,
    pub rejected: HashSet<FoodType>,
    pub next_discover_time: NextDiscoverTime,
}

// TODO: 10s?
const MIN_FOOD_TIME: f32 = 1f32;

#[derive(Deref)]
struct NextDiscoverTime(Duration);

impl NextDiscoverTime {
    pub fn increase(&mut self) {
        self.0 *= 2;
    }

    /// Random between MIN_FOOD_TIME and self
    /// Also increases the time for the next call.
    pub fn get_and_increase(&mut self) -> Duration {
        let mut rng = rand::thread_rng();
        let time = rng.gen_range(MIN_FOOD_TIME..self.0.as_secs_f32());
        let duration = Duration::from_secs_f32(time);
        self.increase();
        duration
    }
}

impl Default for NextDiscoverTime {
    fn default() -> Self {
        // TODO: 20s?
        Self(Duration::from_secs(5))
    }
}
