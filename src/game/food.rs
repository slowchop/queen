use crate::game::positions::SideIPos;
use bevy::prelude::*;
use bevy::utils::HashSet;
use rand::Rng;
use std::time::Duration;

/// Add food to an ant to carry. If they have some food already we should probably drop it.
pub struct AddFoodForAntToCarryEvent {
    pub entity: Entity,
    pub data: CarryFoodType,
}

pub enum CarryFoodType {
    Food(FoodType),
    DiscoveredFood(DiscoveredFood),
}

impl AddFoodForAntToCarryEvent {
    pub fn food(entity: Entity, food: FoodType) -> Self {
        Self {
            entity,
            data: CarryFoodType::Food(food),
        }
    }

    pub fn discovered(entity: Entity, discovered: DiscoveredFood) -> Self {
        Self {
            entity,
            data: CarryFoodType::DiscoveredFood(discovered),
        }
    }
}

/// Specifically for scout ants that have discovered a new food.
/// Note: Attached to a child of the ant.
#[derive(Component, Deref, Debug)]
pub struct CarryingDiscoveredFood(DiscoveredFood);

/// All other cases of carrying food, e.g.:
/// - carrying food from outside to the food store
/// - carrying food for the queen to eat
///
/// Note: Attached to a child of the ant.
#[derive(Component, Deref, Debug)]
pub struct CarryingFood(FoodType);

pub fn attach_food_to_ant(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut events: EventReader<AddFoodForAntToCarryEvent>,
) {
    for event in events.iter() {
        // TODO: Drop the food if the ant already has some.

        // Add to the child of the ant.
        let mut child_entity = commands.spawn(SpriteBundle {
            texture: asset_server.load("food/food.png"),
            ..Default::default()
        });

        match event.data {
            CarryFoodType::DiscoveredFood(discovered) => {
                child_entity.insert(CarryingDiscoveredFood(discovered))
            }
            CarryFoodType::Food(food) => child_entity.insert(CarryingFood(food)),
        };
        let child_entity = child_entity.id();

        commands.entity(event.entity).push_children(&[child_entity]);
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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

#[derive(Copy, Clone, Debug)]
pub struct DiscoveredFood {
    pub food: FoodType,
    pub position: SideIPos,
    pub time_to_discover: Duration,
    pub remaining: u32,
}

#[derive(Resource, Default)]
pub struct FoodState {
    pub approved: Vec<DiscoveredFood>,
    pub rejected: HashSet<FoodType>,
    pub next_discover_time: NextDiscoverTime,
}

impl FoodState {
    pub fn approve_food(&mut self, found: DiscoveredFood) {
        self.approved.push(found);
    }

    pub fn reject_food(&mut self, food: FoodType) {
        self.rejected.insert(food);
    }
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
