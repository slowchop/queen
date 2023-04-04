use crate::game::positions::SideIPos;
use bevy::prelude::*;
use bevy::utils::HashSet;
use rand::Rng;
use std::time::Duration;

/// Add food to an ant to carry. If they have some food already we should probably drop it.
pub struct CarryFoodEvent {
    pub entity: Entity,
    pub food: FoodType,
}

impl CarryFoodEvent {
    pub fn new(entity: Entity, food: FoodType) -> Self {
        Self { entity, food }
    }
}

/// Note: attached as child of the ant.
#[derive(Component, Deref, Debug)]
pub struct CarryingFood(FoodType);

pub fn attach_food_to_ant(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut events: EventReader<CarryFoodEvent>,
) {
    for event in events.iter() {
        // TODO: Drop the food if the ant already has some.

        // Add to the child of the ant.
        let child_entity = commands
            .spawn(SpriteBundle {
                texture: asset_server.load("food/food.png"),
                ..Default::default()
            })
            .insert(CarryingFood(event.food))
            .id();

        commands.entity(event.entity).push_children(&[child_entity]);
    }
}

#[derive(Clone, Copy, Debug)]
pub enum FoodType {
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
    pub food: FoodType,
    pub position: SideIPos,
    pub time: f32,
}

#[derive(Resource, Default)]
pub struct FoodState {
    pub approved: Vec<FoundFood>,
    pub rejected: HashSet<FoodType>,
    pub next_discover_time: NextDiscoverTime,
}

// TODO: 10s?
const MIN_FOOD_TIME: f32 = 1f32;

#[derive(Deref)]
pub struct NextDiscoverTime(Duration);

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
