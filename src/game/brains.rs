pub mod pathfinding;

use self::pathfinding::SetPathToQueenAction;
use self::pathfinding::{
    SetPathToDiscoveredFoodAction, SetPathToFoodStorageAction, SetPathToRandomOutsideAction,
};
use crate::game::ants::AntType;
use crate::game::food::{
    AddFoodForAntToCarryEvent, AssignedFoodId, CarryingDiscoveredFood, CarryingFood,
    DiscoveredFood, FeedEvent, FoodState, DEFAULT_CARGO_CAPACITY,
};
use crate::game::hunger::Hunger;
use crate::game::map::{SideMapPosToEntities, TileNeedsFoodRenderingUpdate};
use crate::game::plugin::{PlayerState, QueensChoice};
use crate::game::positions::SideIPos;
use crate::game::queen::Queen;
use crate::game::skill::SkillMode;
use crate::game::time::GameTime;
use bevy::prelude::*;
use big_brain::actions::StepsBuilder;
use big_brain::prelude::*;
use rand::prelude::SliceRandom;
use std::fmt::Debug;
use std::time::Duration;

// pub fn eat_food() -> StepsBuilder {
//     // TODO: expand this
//     Steps::build()
//         .label("Eat")
//         // .step(SetPathToStoredFoodAction)
//         .step(PathfindingAction)
//     // .step(EatAction::default())
// }

// pub fn discover_food_and_offer_to_the_queen_steps() -> StepsBuilder {
//     Steps::build()
//         .label("DiscoverFood")
//         .step(SetPathToRandomOutsideAction)
//         .step(PathfindingAction)
//         .step(MapTransitionAction::exit())
//         .step(OutsideMapDiscoveringNewFoodAction::default())
//         .step(MapTransitionAction::enter())
//         .step(SetPathToQueenAction)
//         .step(PathfindingAction)
//         .step(OfferFoodDiscoveryToQueenAction)
// }
//
// pub fn gather_food_from_outside_steps() -> StepsBuilder {
//     Steps::build()
//         .label("GatherFoodFromOutside")
//         .step(SetPathToDiscoveredFoodAction)
//         .step(PathfindingAction)
//         .step(MapTransitionAction::exit())
//         .step(OutsideMapGatheringExistingFoodAction::default())
//         .step(MapTransitionAction::enter())
//         .step(SetPathToFoodStorageAction)
//         .step(PathfindingAction)
//         .step(PlaceFoodIfPossibleAction)
// }
//
// pub fn feed_queen_steps() -> StepsBuilder {
//     Steps::build()
//         .label("FeedQueen")
//         .step(SetPathToStoredFoodAction)
//         .step(PathfindingAction)
//         .step(PickUpFoodAction)
//         .step(SetPathToQueenAction)
//         .step(PathfindingAction)
//         .step(FeedQueenAction)
// }

// /// At should be at a food cell and will eat it.
// #[derive(Clone, Component, Debug, ActionBuilder, Default)]
// pub struct EatAction {
//     finish_eating_at: Duration,
// }
//
// pub fn eat_action(
//     time: Res<GameTime>,
//     mut food_state: ResMut<FoodState>,
//     mut feed_writer: EventWriter<FeedEvent>,
//     ants: Query<&Transform>,
//     mut query: Query<(&Actor, &mut ActionState, &mut EatAction)>,
//     mut update_food_rendering_writer: EventWriter<UpdateFoodRenderingEvent>,
// ) {
//     for (Actor(entity), mut state, mut eat_action) in query.iter_mut() {
//         if *state != ActionState::Requested {
//             // TODO: Change it so the ant takes some time to eat.
//             continue;
//         }
//
//         match *state {
//             ActionState::Requested => {
//                 let Ok(transform) = ants.get(*entity) else {
//                     warn!("Ant has no transform in eat_action");
//                     *state = ActionState::Failure;
//                     continue;
//                 };
//
//                 let pos = SideIPos::from(transform);
//
//                 let Some(carrying_food) = food_state.take_food_from_position(pos, 1f32) else {
//                     warn!("Tried to eat food but there was none.");
//                     *state = ActionState::Failure;
//                     continue;
//                 };
//
//                 info!(?carrying_food, "Eating food");
//
//                 eat_action.finish_eating_at = time.since_startup() + Duration::from_secs(5);
//
//                 feed_writer.send(FeedEvent {
//                     target: *entity,
//                     carrying_food,
//                 });
//                 *state = ActionState::Executing;
//
//                 update_food_rendering_writer.send(UpdateFoodRenderingEvent(pos));
//             }
//             ActionState::Executing => {
//                 if time.since_startup() >= eat_action.finish_eating_at {
//                     *state = ActionState::Success;
//                 }
//             }
//             ActionState::Cancelled => {
//                 *state = ActionState::Failure;
//             }
//             _ => {}
//         }
//     }
// }

/// Will attempt to place food at the destination.
///
/// TODO: This will work all the time for now.
#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct PlaceFoodIfPossibleAction;

pub fn place_food_if_possible_action(
    mut commands: Commands,
    mut food_state: ResMut<FoodState>,
    side_map_pos_to_entities: Res<SideMapPosToEntities>,
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
            error!(?entity, "/////////// No CarryingFood found in children.");
            continue;
        };

        info!("//////// Removing food from cargo ant");
        // Remove food from ant.
        commands.entity(child_food_entity).despawn_recursive();

        // Spawn food on ground.
        // 1) Get the cell position
        // 2) Get the FoodCell component.
        // 3) Update FoodCell
        // 4) The sprites for the food will update elsewhere when changed.
        let pos = SideIPos::from(transform);
        food_state.add_food_at_position(pos, &carrying_food);

        commands.entity(entity).insert(TileNeedsFoodRenderingUpdate);

        *state = ActionState::Success;
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
    skill_mode: Res<SkillMode>,
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

                let food_info = skill_mode.next_food(time.since_startup());

                // Give the ant some food to carry.
                carry_food_writer.send(AddFoodForAntToCarryEvent::discovered(
                    entity,
                    DiscoveredFood {
                        food_info,
                        position: SideIPos::from(transform),
                        time_to_discover: action.initial_time,
                        stash_remaining: 1000f32,
                    },
                ));

                *state = ActionState::Success;
            }
            ActionState::Cancelled => {
                *state = ActionState::Failure;
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
            ActionState::Cancelled => {
                *state = ActionState::Failure;
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
    mut food_state: ResMut<FoodState>,
    mut player_state: ResMut<PlayerState>,
    mut ants: Query<(Entity, &Children), With<AntType>>,
    carrying_discovered_food: Query<&CarryingDiscoveredFood>,
    mut query: Query<(&Actor, &mut ActionState), With<OfferFoodDiscoveryToQueenAction>>,
    queen: Query<Entity, With<Queen>>,
    mut feed_writer: EventWriter<FeedEvent>,
) {
    let queen_entity = queen.single();

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
        let Some((child_food_entity, carrying)) = food_info else {
            error!(?entity, "No food found in child ants.");
            continue;
        };

        match *state {
            ActionState::Requested => {
                time.system_pause(true);
                player_state.queens_choice = QueensChoice::Undecided(carrying.food_info.clone());
                *state = ActionState::Executing;
            }
            ActionState::Executing => {
                let mut done = false;
                match player_state.queens_choice {
                    QueensChoice::None => {
                        warn!("Not in a state to offer food to the queen.");
                        *state = ActionState::Failure;
                    }
                    QueensChoice::Undecided(_) => {
                        // Still waiting
                    }
                    QueensChoice::Approve => {
                        // Add the food to the food state.
                        food_state.approve_food((*carrying).clone());

                        feed_writer.send(FeedEvent {
                            target: queen_entity,
                            carrying_food: CarryingFood {
                                food_id: carrying.food_info.food_id,
                                amount: 10f32,
                            },
                        });

                        done = true;
                    }
                    QueensChoice::Deny => {
                        // TODO: The queen eats the ant even if she is full.
                        // Food just vanishes.
                        food_state.reject_food(carrying.food_info.food_id);
                        done = true;
                    }
                }

                if done {
                    commands.entity(child_food_entity).despawn_recursive();
                    time.system_pause(false);
                    *state = ActionState::Success;
                }
            }
            ActionState::Cancelled => {
                // *state = ActionState::Failure;
                todo!();
            }
            _ => {}
        }
    }
}

#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct PickUpFoodAction;

pub fn pick_up_food_action(
    mut food_state: ResMut<FoodState>,
    mut ants: Query<(Entity, &Transform), With<AntType>>,
    mut query: Query<(&Actor, &mut ActionState), With<PickUpFoodAction>>,
    mut carry_food_writer: EventWriter<AddFoodForAntToCarryEvent>,
) {
    for ((Actor(actor), mut state)) in query.iter_mut() {
        if *state != ActionState::Requested {
            continue;
        }

        let Ok((entity, transform)) = ants.get_mut(*actor) else {
            warn!(?actor, ">>>>>>>>>>> No children found.");
            *state = ActionState::Failure;
            continue;
        };

        let pos = SideIPos::from(transform);

        // Make sure there's still food here.
        let Some(carrying_food) = food_state.take_food_from_position(pos, DEFAULT_CARGO_CAPACITY) else {
            warn!(">>>>>>>>> No food left at {:?}", pos);
            *state = ActionState::Failure;
            continue;
        };

        info!(">>>>>>>>>> Picked up food at {:?}", pos);
        carry_food_writer.send(AddFoodForAntToCarryEvent::food(entity, carrying_food));

        *state = ActionState::Success;
    }
}

#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct FeedQueenAction;

pub fn feed_queen_action(
    mut commands: Commands,
    mut ants: Query<(Entity, &Transform, &Children), With<AntType>>,
    carrying_food: Query<&CarryingFood>,
    queen: Query<Entity, With<Queen>>,
    mut actions: Query<(&Actor, &mut ActionState), With<FeedQueenAction>>,
    mut feed_writer: EventWriter<FeedEvent>,
) {
    let queen_entity = queen.single();

    for ((Actor(actor), mut state)) in actions.iter_mut() {
        if *state != ActionState::Requested {
            continue;
        }

        let Ok((entity, transform, children)) = ants.get_mut(*actor) else {
            warn!(?actor, "transform + children not found for action");
            *state = ActionState::Failure;
            continue;
        };

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

        // Feed the queen and remove the food.
        commands.entity(child_food_entity).despawn_recursive();

        feed_writer.send(FeedEvent {
            target: queen_entity,
            carrying_food,
        });

        *state = ActionState::Success;
    }
}

#[derive(Clone, Component, Debug, ScorerBuilder)]
pub struct HungryScorer;

pub fn hungry_scorer(
    hungers: Query<&Hunger>,
    mut query: Query<(&Actor, &mut Score), With<HungryScorer>>,
) {
    for (Actor(actor), mut score) in query.iter_mut() {
        if let Ok(hunger) = hungers.get(*actor) {
            score.set(hunger.hunger_score());
        }
    }
}
