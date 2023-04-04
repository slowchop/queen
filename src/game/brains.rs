use crate::game::ants::AntType;
use crate::game::food::{CarryFoodEvent, FoodState, FoodType};
use crate::game::hunger::Hunger;
use crate::game::map::ExitPositions;
use crate::game::pathfinding::Path;
use crate::game::positions::SideIPos;
use crate::game::queen::Queen;
use crate::game::time::GameTime;
use bevy::prelude::*;
use big_brain::prelude::*;
use rand::prelude::SliceRandom;
use std::time::Duration;

/// A scout or cargo ant to leave the map for food.
#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct LeaveMapAction;

pub fn leave_map_action(
    exit_positions: Res<ExitPositions>,
    mut ants: Query<(&mut Path, &mut Visibility), With<AntType>>,
    mut query: Query<(&Actor, &mut ActionState), With<LeaveMapAction>>,
) {
    for (Actor(actor), mut state) in query.iter_mut() {
        let Ok((mut path, mut visibility)) = ants.get_mut(*actor) else {
            warn!("No path for actor {:?}", actor);
            continue;
        };

        match *state {
            ActionState::Requested => {
                info!("Requested");

                // Find a random exit position
                // TODO: Have an optional exit position to go to for cargo ants.
                let exit_position = exit_positions
                    .choose(&mut rand::thread_rng())
                    .expect("No exit positions");

                path.set_target(*exit_position);

                *state = ActionState::Executing;
            }
            ActionState::Executing => {
                // Check if path is complete and is at the exit position.
                if path.did_complete() {
                    *visibility = Visibility::Hidden;
                    *state = ActionState::Success;
                } else if path.did_fail() {
                    *state = ActionState::Failure;
                }
                // Still going to the exit position.
            }
            _ => {}
        }
    }
}

/// The ant is off the map going to get new food.
#[derive(Clone, Component, Debug, ActionBuilder, Default)]
pub struct OutsideMapDiscoveringNewFoodAction {
    pub time_left: Duration,
}

pub fn outside_map_discovering_food_action(
    time: Res<GameTime>,
    mut food_state: ResMut<FoodState>,
    mut ants: Query<(Entity, &mut Visibility), With<AntType>>,
    mut query: Query<(
        &Actor,
        &mut ActionState,
        &mut OutsideMapDiscoveringNewFoodAction,
    )>,
    mut carry_food_writer: EventWriter<CarryFoodEvent>,
) {
    for (Actor(actor), mut state, mut action) in query.iter_mut() {
        let Ok((entity, mut visibility)) = ants.get_mut(*actor) else {
            warn!(?actor, "No visibility found.");
            continue;
        };

        match *state {
            ActionState::Requested => {
                let time_left = food_state.next_discover_time.get_and_increase();
                action.time_left = time_left;
                *state = ActionState::Executing;
            }
            ActionState::Executing => {
                action.time_left = action.time_left.saturating_sub(time.delta());
                if action.time_left != Duration::ZERO {
                    continue;
                }
                // Ant is back on the map!
                *visibility = Visibility::Visible;

                // Give the ant some food to carry.
                carry_food_writer.send(CarryFoodEvent::new(entity, FoodType::MedicinePill));

                *state = ActionState::Success;
            }
            _ => {}
        }
    }
}

/// The ant is off the map going to get approved food.
#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct OutsideMapGettingApprovedFoodAction;

/// Move to The Queen!
#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct MoveToQueenAction;

pub fn move_to_queen_action(
    queen: Query<&Transform, With<Queen>>,
    mut ants: Query<&mut Path, With<AntType>>,
    mut query: Query<(&Actor, &mut ActionState, &mut MoveToQueenAction)>,
) {
    for (Actor(actor), mut state, mut action) in query.iter_mut() {
        let Ok(mut path) = ants.get_mut(*actor) else {
            warn!(?actor, "No path found.");
            continue;
        };

        match *state {
            ActionState::Requested => {
                let queen_transform = queen.single();
                let queen_position = SideIPos::from(queen_transform);

                path.set_target(queen_position);

                *state = ActionState::Executing;
            }
            ActionState::Executing => {
                if path.did_complete() {
                    *state = ActionState::Success;
                } else if path.did_fail() {
                    *state = ActionState::Failure;
                }
            }
            _ => {}
        }
    }
}

/// The scout ant is back on the map
#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct OfferNewFoodToQueenAction;

/// When hungry or needs food for the queen, move to some food.
#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct MoveToFoodAction;

/// At should be at a food cell and will eat it.
#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct EatAction;

/// Should be at a food cell and will pick it up (to carry to the queen).
#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct CarryFoodAction;

#[derive(Clone, Component, Debug, ScorerBuilder)]
pub struct HungryScorer;
