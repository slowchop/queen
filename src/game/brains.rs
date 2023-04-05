use crate::game::ants::AntType;
use crate::game::food::{
    AddFoodForAntToCarryEvent, AssignedFoodId, CarryingDiscoveredFood, CarryingFood,
    DiscoveredFood, FoodId, FoodState, FoodType,
};
use crate::game::hunger::Hunger;
use crate::game::map::{ExitPositions, FoodCell, SideMapPosToEntities};
use crate::game::pathfinding::Path;
use crate::game::plugin::PlayerState;
use crate::game::positions::SideIPos;
use crate::game::queen::Queen;
use crate::game::time::GameTime;
use bevy::prelude::*;
use bevy_egui::egui::Align2;
use bevy_egui::{egui, EguiContexts};
use big_brain::actions::StepsBuilder;
use big_brain::prelude::*;
use rand::prelude::SliceRandom;
use std::time::Duration;

pub fn eat_food() -> StepsBuilder {
    // TODO: expand this
    Steps::build()
        .label("MoveAndEat")
        .step(MoveToFoodAction)
        .step(EatAction)
}

pub fn discover_food_and_offer_to_the_queen() -> StepsBuilder {
    Steps::build()
        .label("DiscoverFood")
        .step(SetPathToRandomOutsideAction)
        .step(PathfindingAction)
        .step(MapTransitionAction::exit())
        .step(OutsideMapDiscoveringNewFoodAction::default())
        .step(MapTransitionAction::enter())
        .step(SetPathToQueenAction)
        .step(PathfindingAction)
        .step(OfferFoodDiscoveryToQueenAction)
}

pub fn gather_food_from_outside() -> StepsBuilder {
    Steps::build()
        .label("GatherFoodFromOutside")
        .step(SetPathToDiscoveredFoodAction)
        .step(PathfindingAction)
        .step(MapTransitionAction::exit())
        .step(OutsideMapGatheringExistingFoodAction::default())
        .step(MapTransitionAction::enter())
        .step(SetPathToStoreFoodAction)
        .step(PathfindingAction)
        .step(PlaceFoodIfPossibleAction)
}

/// Actor is on a path. This action is to follow the path and finish when the path is finished.
#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct PathfindingAction;

pub fn pathfinding_action(
    mut ants: Query<&mut Path>,
    mut query: Query<(&Actor, &mut ActionState), With<PathfindingAction>>,
) {
    for (Actor(actor), mut state) in query.iter_mut() {
        let Ok(mut path) = ants.get_mut(*actor) else {
            warn!("No path for actor {:?}", actor);
            continue;
        };

        match *state {
            ActionState::Requested => {
                // We expect to already have a path set.
                if !path.is_progressing() {
                    *state = ActionState::Failure;
                    continue;
                }

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

/// A scout or cargo ant needs to find a destination to the outside.
#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct SetPathToRandomOutsideAction;

pub fn set_path_to_outside_action(
    exit_positions: Res<ExitPositions>,
    mut ants: Query<&mut Path>,
    mut query: Query<(&Actor, &mut ActionState), With<SetPathToRandomOutsideAction>>,
) {
    for (Actor(actor), mut state) in query.iter_mut() {
        let Ok(mut path) = ants.get_mut(*actor) else {
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

                *state = ActionState::Success;
            }
            _ => {}
        }
    }
}

#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct SetPathToStoreFoodAction;

pub fn set_path_to_store_food_action(
    player_state: Res<PlayerState>,
    mut ants: Query<&mut Path>,
    mut query: Query<(&Actor, &mut ActionState), With<SetPathToStoreFoodAction>>,
) {
    for (Actor(actor), mut state) in query.iter_mut() {
        let Ok(mut path) = ants.get_mut(*actor) else {
            warn!("No path for actor {:?}", actor);
            continue;
        };

        match *state {
            ActionState::Requested => {
                let target = player_state.find_destination_to_place_food();
                path.set_target(target);

                *state = ActionState::Success;
            }
            _ => {}
        }
    }
}

/// Will attempt to place food at the destination.
///
/// TODO: This will work all the time for now.
#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct PlaceFoodIfPossibleAction;

pub fn place_food_if_possible_action(
    mut commands: Commands,
    side_map_pos_to_entities: Res<SideMapPosToEntities>,
    mut food_cells: Query<&mut FoodCell>,
    mut ants: Query<(Entity, &Children, &Transform)>,
    carrying_food: Query<&CarryingFood>,
    mut query: Query<(&Actor, &mut ActionState), With<PlaceFoodIfPossibleAction>>,
) {
    for (Actor(actor), mut state) in query.iter_mut() {
        let Ok((entity, children, transform)) = ants.get_mut(*actor) else {
            warn!("No children + transform for actor {:?}", actor);
            continue;
        };

        if *state != ActionState::Requested {
            continue;
        }

        // Grab the food type.
        let mut food_info = None;
        for &child in children.iter() {
            let Ok(f) = carrying_food.get(child) else {
                continue;
            };
            food_info = Some((child, f.clone()));
        }
        let Some((child_food_entity, carrying_food)) = food_info else {
            error!(?entity, "No CarryingFood found in children.");
            continue;
        };

        // Remove food from ant.
        commands.entity(child_food_entity).despawn();

        // Spawn food on ground.
        // 1) Get the cell entity
        // 2) Get the FoodCell component.
        // 3) Update FoodCell
        // 4) The sprites for the food will update elsewhere when changed.
        let pos = SideIPos::from(transform);
        let cell_entity = side_map_pos_to_entities
            .get(&pos)
            .expect("No cell entity found for position");

        let Ok(mut food_cell) = food_cells.get_mut(*cell_entity) else {
            error!(?cell_entity, "No FoodCell found for cell entity.");
            continue;
        };

        food_cell.add(&carrying_food);

        *state = ActionState::Success;
    }
}

/// A cargo ant needs to find a destination to the outside.
#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct SetPathToDiscoveredFoodAction;

pub fn set_path_and_assign_food_to_discovered_food_action(
    food_state: Res<FoodState>,
    mut ants: Query<(&mut Path, &mut AssignedFoodId)>,
    mut query: Query<(&Actor, &mut ActionState), With<SetPathToDiscoveredFoodAction>>,
) {
    for (Actor(actor), mut state) in query.iter_mut() {
        let Ok((mut path, mut assigned_food_id)) = ants.get_mut(*actor) else {
            error!("No path for actor {:?}", actor);
            continue;
        };

        match *state {
            ActionState::Requested => {
                let Some(food_id) = food_state.random_food_source() else {
                    warn!("No random food source found");
                    *state = ActionState::Failure;
                    continue;
                };
                let Some(exit_position) = food_state.position_of_food_source(food_id) else {
                    warn!("No position for food {:?}", assigned_food_id);
                    *state = ActionState::Failure;
                    continue;
                };

                **assigned_food_id = Some(food_id);
                path.set_target(exit_position);

                *state = ActionState::Success;
            }
            _ => {}
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransitionDirection {
    Enter,
    Exit,
}

/// When an ant has hit the map exit, we make them invisible, or vice versa.
#[derive(Clone, Component, Debug, ActionBuilder, Deref)]
pub struct MapTransitionAction(TransitionDirection);

impl MapTransitionAction {
    pub fn enter() -> Self {
        Self(TransitionDirection::Enter)
    }

    pub fn exit() -> Self {
        Self(TransitionDirection::Exit)
    }
}

pub fn map_transition_action(
    mut ants: Query<&mut Visibility>,
    mut query: Query<(&Actor, &mut ActionState, &MapTransitionAction)>,
) {
    for (Actor(actor), mut state, transition) in query.iter_mut() {
        let Ok(mut visibility) = ants.get_mut(*actor) else {
            warn!("No visibility for actor {:?}", actor);
            continue;
        };

        match *state {
            ActionState::Requested => {
                *visibility = if **transition == TransitionDirection::Enter {
                    Visibility::Visible
                } else {
                    Visibility::Hidden
                };

                *state = ActionState::Success;
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
    mut ants: Query<(Entity, &Transform)>,
    mut query: Query<(
        &Actor,
        &mut ActionState,
        &mut OutsideMapDiscoveringNewFoodAction,
    )>,
    mut carry_food_writer: EventWriter<AddFoodForAntToCarryEvent>,
) {
    for (Actor(actor), mut state, mut action) in query.iter_mut() {
        let Ok((entity, transform)) = ants.get_mut(*actor) else {
            warn!(?actor, "No transform found.");
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

                // Give the ant some food to carry.
                carry_food_writer.send(AddFoodForAntToCarryEvent::discovered(
                    entity,
                    DiscoveredFood {
                        food_id: FoodId(FoodType::MedicinePill),
                        position: SideIPos::from(transform),
                        time_to_discover: action.initial_time,
                        stash_remaining: 1000,
                    },
                ));

                *state = ActionState::Success;
            }
            _ => {}
        }
    }
}

/// The ant is off the map going to get existing food.
#[derive(Clone, Component, Debug, ActionBuilder, Default)]
pub struct OutsideMapGatheringExistingFoodAction {
    pub time_left: Duration,
}

pub fn outside_map_gathering_existing_food_action(
    time: Res<GameTime>,
    mut food_state: ResMut<FoodState>,
    mut ants: Query<(Entity, &mut AssignedFoodId)>,
    mut query: Query<(
        &Actor,
        &mut ActionState,
        &mut OutsideMapGatheringExistingFoodAction,
    )>,
    mut carry_food_writer: EventWriter<AddFoodForAntToCarryEvent>,
) {
    for (Actor(actor), mut state, mut action) in query.iter_mut() {
        let Ok((entity, mut assigned_food_id)) = ants.get_mut(*actor) else {
            warn!(?actor, "No transform found.");
            continue;
        };

        let Some(food_id) = **assigned_food_id else {
            warn!("No food assigned for actor {:?}", actor);
            continue;
        };

        match *state {
            ActionState::Requested => {
                let Some(time_left) = food_state.eta(&food_id) else {
                    warn!("No food left at {:?}", food_id);
                    *state = ActionState::Failure;
                    continue;
                };
                action.time_left = time_left;

                *state = ActionState::Executing;
            }
            ActionState::Executing => {
                action.time_left = action.time_left.saturating_sub(time.delta());
                if action.time_left != Duration::ZERO {
                    continue;
                }

                let Some(carrying_food) = food_state.take_food_from_discovered_source(&food_id) else {
                    warn!("No food left at {:?}", food_id);
                    *state = ActionState::Failure;
                    continue;
                };

                // Give the ant the food to carry.
                carry_food_writer.send(AddFoodForAntToCarryEvent::food(entity, carrying_food));

                // Remove the food assignment.
                **assigned_food_id = None;

                *state = ActionState::Success;
            }
            _ => {}
        }
    }
}

/// Move to The Queen!
#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct SetPathToQueenAction;

pub fn set_path_to_queen_action(
    queen: Query<&Transform, With<Queen>>,
    mut ants: Query<&mut Path>,
    mut query: Query<(&Actor, &mut ActionState), With<SetPathToQueenAction>>,
) {
    for (Actor(actor), mut state) in query.iter_mut() {
        let Ok(mut path) = ants.get_mut(*actor) else {
            warn!(?actor, "No path found.");
            continue;
        };

        match *state {
            ActionState::Requested => {
                let queen_transform = queen.single();
                let queen_position = SideIPos::from(queen_transform);
                path.set_target(queen_position);

                *state = ActionState::Success;
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

                                food_state.reject_food(food_info.food_id);
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
