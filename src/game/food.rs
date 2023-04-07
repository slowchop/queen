use crate::game::food_types::FoodId;
use crate::game::positions::SideIPos;
use crate::game::setup::queen_start;
use crate::game::zones::FoodStorageZones;
use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};
use rand::{random, Rng};
use std::time::Duration;
use crate::game::hunger::Hunger;
use crate::game::queen::Queen;
use crate::game::side_effects::{AppliedFoodSideEffects, SideEffect};
use crate::game::time::GameTime;

pub const DEFAULT_CARGO_CAPACITY: f32 = 10f32;

pub struct FeedEvent {
    pub target: Entity,
    pub carrying_food: CarryingFood,
}

/// Add food to an ant to carry. If they have some food already we should probably drop it.
pub struct AddFoodForAntToCarryEvent {
    pub entity: Entity,
    pub data: CarryFoodType,
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct FoodInfo {
    pub food_id: FoodId,
    pub side_effects: Vec<SideEffect>,
}

pub enum CarryFoodType {
    Food(CarryingFood),
    DiscoveredFood(DiscoveredFood),
}

/// A food and amount.
///
/// This could mean an ant carrying food, or a cell containing food.
///
/// Note: Attached to a child of an ant or cell for the sake of rendering something else.
#[derive(Component, Debug, Clone, Copy)]
pub struct CarryingFood {
    pub food_id: FoodId,
    pub amount: f32,
}

#[derive(Clone, Debug)]
pub struct DiscoveredFood {
    pub food_info: FoodInfo,
    pub position: SideIPos,
    pub time_to_discover: Duration,
    pub stash_remaining: f32,
}

impl AddFoodForAntToCarryEvent {
    pub fn food(entity: Entity, carrying_food: CarryingFood) -> Self {
        Self {
            entity,
            data: CarryFoodType::Food(carrying_food),
        }
    }

    pub fn discovered(entity: Entity, discovered: DiscoveredFood) -> Self {
        Self {
            entity,
            data: CarryFoodType::DiscoveredFood(discovered),
        }
    }
}

#[derive(Component, Deref, DerefMut, Debug, Default)]
pub struct AssignedFoodId(pub Option<FoodId>);

/// Specifically for scout ants that have discovered a new food.
/// Note: Attached to a child of the ant.
#[derive(Component, Deref, Debug)]
pub struct CarryingDiscoveredFood(DiscoveredFood);

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
            transform: Transform::from_translation(Vec3::new(
                random::<f32>(),
                random::<f32>(),
                0.0,
            )),
            ..Default::default()
        });

        match &event.data {
            CarryFoodType::DiscoveredFood(discovered) => {
                child_entity.insert(CarryingDiscoveredFood(discovered.clone()))
            }
            CarryFoodType::Food(food) => child_entity.insert(food.clone()),
        };
        let child_entity = child_entity.id();

        commands.entity(event.entity).push_children(&[child_entity]);
    }
}

#[derive(Resource, Default)]
pub struct FoodState {
    pub approved: Vec<DiscoveredFood>,
    pub rejected: HashSet<FoodId>,
    pub next_discover_time: NextDiscoverTime,
    pub food_zones: FoodStorageZones,
    pub food_position_cells: HashMap<SideIPos, FoodCell>,
}

impl FoodState {
    pub fn get_discovered_food(&self, food_id: FoodId) -> Option<&DiscoveredFood> {
        self.approved.iter().find(|f| f.food_info.food_id == food_id)
    }

    /// This won't fail. It will always pick some spot.
    ///
    /// First try a random zone. If not, somewhere near the queen.
    pub fn find_destination_to_place_food(&self) -> SideIPos {
        if let Some(position) = self.food_zones.random() {
            return position;
        };

        // TODO: More random?
        // TODO: Make sure it's not on top of the queen.
        queen_start()
    }

    pub fn find_destination_to_take_food(&self) -> Option<SideIPos> {
        if self.food_position_cells.is_empty() {
            return None;
        }

        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.food_position_cells.len());
        self.food_position_cells.keys().nth(index).copied()
    }

    pub fn discover_food(&mut self) -> FoodId {
        // The loop here is to stop infinite loops if we reject all food.
        for _ in 0..100_000 {
            let food_id = FoodId::random();
            if self.rejected.contains(&food_id) {
                continue;
            }

            // We don't want approved either.
            if self.approved.iter().any(|f| f.food_info.food_id == food_id) {
                continue;
            }

            return food_id;
        }

        return FoodId::random();
    }

    pub fn approve_food(&mut self, found: DiscoveredFood) {
        self.approved.push(found);
    }

    pub fn reject_food(&mut self, food: FoodId) {
        self.rejected.insert(food);
    }

    pub fn random_food_source(&self) -> Option<FoodId> {
        if self.approved.is_empty() {
            return None;
        }

        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.approved.len());
        self.approved.iter().nth(index).map(|f| f.food_info.food_id)
    }

    pub fn position_of_food_source(&self, food_id: FoodId) -> Option<SideIPos> {
        self.approved
            .iter()
            .find(|f| f.food_info.food_id == food_id)
            .map(|f| f.position)
    }

    /// Return None if food has run out or not found.
    pub fn take_food_from_discovered_source(&mut self, food_id: &FoodId) -> Option<CarryingFood> {
        let mut food = self.approved.iter_mut().find(|f| f.food_info.food_id == *food_id)?;

        if food.stash_remaining <= 0f32 {
            return None;
        }

        /// At most DEFAULT_CARGO_CAPACITY, but no more than is left in the stash.
        let amount = food.stash_remaining.min(DEFAULT_CARGO_CAPACITY);
        food.stash_remaining -= amount;

        Some(CarryingFood {
            food_id: food.food_info.food_id,
            amount,
        })
    }

    pub fn info_at_position(&self, position: &SideIPos) -> Option<&FoodCell> {
        self.food_position_cells.get(position)
    }

    pub fn add_food_at_position(&mut self, position: SideIPos, food: &CarryingFood) {
        let food_cell = self.food_position_cells.entry(position).or_default();
        food_cell.add(food);
    }

    pub fn take_food_from_position(&mut self, position: SideIPos) -> Option<CarryingFood> {
        let food_cell = self.food_position_cells.get_mut(&position)?;
        if food_cell.is_empty() {
            error!("Food cell shouldn't be empty 1!");
            return None;
        }

        let maybe_carrying_food = food_cell.take_any_food_up_to_max_amount(DEFAULT_CARGO_CAPACITY);
        if maybe_carrying_food.is_none() {
            error!("Food cell shouldn't be empty 2!");
            return None;
        }

        if food_cell.is_empty() {
            info!("cell is now empty");
            self.food_position_cells.remove(&position);
        }

        maybe_carrying_food
    }

    pub fn eta(&self, food_id: &FoodId) -> Option<Duration> {
        self.approved
            .iter()
            .find(|f| f.food_info.food_id == *food_id)
            .map(|f| f.time_to_discover)
    }
}

/// A container for all the food stored in a cell.
#[derive(Deref, DerefMut, Default)]
pub struct FoodCell(HashMap<FoodId, f32>);

impl FoodCell {
    /// If the food exists we add to the number.
    pub fn add(&mut self, food: &CarryingFood) {
        if let Some(current_amount) = self.0.get_mut(&food.food_id) {
            *current_amount += food.amount;
        } else {
            self.0.insert(food.food_id, food.amount);
        }
    }

    /// We can only carry one type of food at once.
    ///
    /// Find the first food and get as much as possible up to the amount specified.
    ///
    /// If there is nothing left in the hash entry, remove it.
    pub fn take_any_food_up_to_max_amount(&mut self, amount: f32) -> Option<CarryingFood> {
        let food_id = *self.0.keys().next()?;
        let current_amount = self.0.get_mut(&food_id)?;

        let amount_to_take = amount.min(*current_amount);
        debug_assert!(amount_to_take > 0f32);

        *current_amount -= amount_to_take;

        if *current_amount <= 0f32 {
            self.0.remove(&food_id);
        }

        Some(CarryingFood {
            food_id,
            amount: amount_to_take,
        })
    }
}

// TODO: 10s?
const MIN_FOOD_TIME: f32 = 1f32;

#[derive(Deref)]
pub struct NextDiscoverTime(Duration);

impl NextDiscoverTime {
    pub fn increase(&mut self) {
        self.0 = Duration::from_secs_f32(self.0.as_secs_f32() * 1.1f32) as Duration;
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

pub fn feed_and_apply(
    time: Res<GameTime>,
    food_state: Res<FoodState>,
    mut feed_reader: EventReader<FeedEvent>,
    mut query: Query<(&mut Hunger, &mut AppliedFoodSideEffects)>,
) {
    for event in feed_reader.iter() {
        for (mut hunger, mut applied) in query.iter_mut() {
            hunger.feed(event.carrying_food.amount);

            let carrying_food = &event.carrying_food;
            let Some(discovered_food) = food_state.get_discovered_food(carrying_food.food_id) else {
                error!("Food not found!");
                continue;
            };

            applied.add_or_update(discovered_food.food_info.clone(), time.since_startup() + Duration::from_secs(5 * 60));
        }
    }
}

