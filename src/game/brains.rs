use crate::game::ants::AntType;
use crate::game::food::{
    AddFoodForAntToCarryEvent, CarryingDiscoveredFood, CarryingFood, DiscoveredFood, FoodState,
    FoodType,
};
use crate::game::hunger::Hunger;
use crate::game::map::ExitPositions;
use crate::game::pathfinding::Path;
use crate::game::positions::SideIPos;
use crate::game::queen::Queen;
use crate::game::time::GameTime;
use bevy::prelude::*;
use bevy_egui::egui::Align2;
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
    pub initial_time: Duration,
    pub time_left: Duration,
}

pub fn outside_map_discovering_food_action(
    time: Res<GameTime>,
    mut food_state: ResMut<FoodState>,
    mut ants: Query<(Entity, &Transform, &mut Visibility), With<AntType>>,
    mut query: Query<(
        &Actor,
        &mut ActionState,
        &mut OutsideMapDiscoveringNewFoodAction,
    )>,
    mut carry_food_writer: EventWriter<AddFoodForAntToCarryEvent>,
) {
    for (Actor(actor), mut state, mut action) in query.iter_mut() {
        let Ok((entity, transform, mut visibility)) = ants.get_mut(*actor) else {
            warn!(?actor, "No transform + visibility found with AntType.");
            continue;
        };

        match *state {
            ActionState::Requested => {
                let time_left = food_state.next_discover_time.get_and_increase();
                action.initial_time = time_left;
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
                carry_food_writer.send(AddFoodForAntToCarryEvent::discovered(
                    entity,
                    DiscoveredFood {
                        food: FoodType::MedicinePill,
                        position: SideIPos::from(transform),
                        time_to_discover: action.initial_time,
                        remaining: 20000,
                    },
                ));

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
pub struct OfferFoodDiscoveryToQueenAction;

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
pub fn offer_food_discovery_to_queen_action(
    mut commands: Commands,
    mut time: ResMut<GameTime>,
    mut contexts: EguiContexts,
    mut food_state: ResMut<FoodState>,
    mut ants: Query<(Entity, &Children), With<AntType>>,
    carrying_discovered_food: Query<&CarryingDiscoveredFood>,
    mut query: Query<(&Actor, &mut ActionState), With<OfferFoodDiscoveryToQueenAction>>,
) {
    for ((Actor(actor), mut state)) in query.iter_mut() {
        let Ok((entity, children)) = ants.get_mut(*actor) else {
            warn!(?actor, "No children found.");
            continue;
        };

        // TODO: Check if we are at the queen's position!

        // Grab the food type.
        let mut food_info = None;
        for &child in children.iter() {
            let Ok(f) = carrying_discovered_food.get(child) else {
                continue;
            };
            food_info = Some((child, f.clone()));
        }
        let Some((food_entity, food_info)) = food_info else {
            error!(?entity, "No food found in child ants.");
            continue;
        };

        match *state {
            ActionState::Requested => {
                time.system_pause(true);
                *state = ActionState::Executing;
            }
            ActionState::Executing => {
                // Show an egui dialog. TODO: Should probably be in another system!
                // TODO: This is flickering. Maybe putting it in another system might help?

                egui::Window::new("Queen's Choice")
                    .anchor(Align2::CENTER_CENTER, egui::Vec2::ZERO)
                    .show(&contexts.ctx_mut(), |ui| {
                        ui.heading("This scout has found new food!");
                        ui.label(format!("Food Type: {:?}", *food_info));
                        ui.heading("This is fake info for now (:");
                        ui.label(" + The Queen grows eggs 2x faster.");
                        ui.label(" - The Queen needs 3x as much food.");
                        ui.label(" + New ants walk 3x faster");
                        ui.label(" - Ants eat 2x slower");
                        ui.label("Do you want to add this food to the colony?");
                        ui.horizontal(|ui| {
                            let mut done = false;

                            if ui.button("Yes").clicked() {
                                // Add the food to the food state.
                                food_state.approve_food(**food_info);

                                // TODO: The queen eats the new food even if she is full.

                                done = true;
                            }
                            if ui.button("No").clicked() {
                                // TODO: The queen eats the ant even if she is full.
                                // Food just vanishes.

                                food_state.reject_food(food_info.food);
                                done = true;
                            }

                            if done {
                                commands.entity(food_entity).despawn_recursive();
                                time.system_pause(false);
                                *state = ActionState::Success;
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
