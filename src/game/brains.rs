use crate::game::ants::AntType;
use crate::game::food::{CarryFoodEvent, CarryingFood, FoodState, FoodType};
use crate::game::hunger::Hunger;
use crate::game::map::ExitPositions;
use crate::game::pathfinding::Path;
use crate::game::positions::SideIPos;
use crate::game::queen::Queen;
use crate::game::time::GameTime;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
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

/// The scout ant is offering new food to the queen.
#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct OfferNewFoodToQueenAction;

/// Offer the food to the queen.
///
/// * Pause the game using a special system pause in GameTime. (TODO)
/// * Show a dialog that looks modal with a summary of the food.
/// * Wait for player to accept or reject the food.
/// * If accepted:
///   * The food is added to the food state as approved food.
///   * The queen eats it even if she is full.
/// * If rejected, the food is added to the food state as rejected food.
///   * The food vanishes (for now)
/// * The game is unpaused.
pub fn offer_new_food_to_queen_action(
    mut time: ResMut<GameTime>,
    mut contexts: EguiContexts,
    mut food_state: ResMut<FoodState>,
    mut ants: Query<(Entity, &Children), With<AntType>>,
    carrying_food: Query<&CarryingFood>,
    mut query: Query<(&Actor, &mut ActionState), With<OfferNewFoodToQueenAction>>,
) {
    for ((Actor(actor), mut state)) in query.iter_mut() {
        let Ok((entity, children)) = ants.get_mut(*actor) else {
            warn!(?actor, "No children found.");
            continue;
        };

        // TODO: Check if we are at the queen's position!

        // Grab the food type.
        let mut food_type = None;
        for &child in children.iter() {
            let Ok(f) = carrying_food.get(child) else {
                continue;
            };
            food_type = Some(f.clone());
        }
        let Some(food_type) = food_type else {
            error!(?entity, "No food found in child ants.");
            continue;
        };

        info!(?food_type, "Offering new food to queen.");

        match *state {
            ActionState::Requested => {
                time.system_pause(true);
                *state = ActionState::Executing;
            }
            ActionState::Executing => {
                // Show an egui dialog. TODO: Should probably be in another system!

                info!("Executing offer");

                egui::Window::new("Queen's Choice").show(&contexts.ctx_mut(), |ui| {
                    ui.heading("This scout has found new food!");
                    ui.label(format!("Food Type: {:?}", *food_type));
                    ui.label(" + The Queen's eggs hatch 2x as many ants.");
                    ui.label(" - The Queen needs 3x as much food.");
                    ui.label(" + New ants are 3x faster");
                    ui.label(" - Ants eat 2x slower");
                    ui.label("Do you want to add this food to the colony?");
                    ui.horizontal(|ui| {
                        if ui.button("Yes").clicked() {
                            // Add the food to the food state.
                            // food_state.add_approved_food(food_type);
                            // The queen eats it even if she is full.
                        }
                        if ui.button("No").clicked() {
                            // Add the food to the food state.
                            // food_state.add_rejected_food(food_type);
                            // The food vanishes (for now)
                        }
                    });
                });
            }
            _ => {}
        }
    }
}

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
